# Cache Context Isolation Fix

## Problem

`HierarchicalDataAccess::execute()` builds the cache key from `type_name::<V>()` + `CasId`, ignoring the `AuthorizationContext`. Two tenants requesting the same `CasId` share a cache entry, violating `user_context_isolation` and `no_unauthorized_cache_hit`.

## Decision: Canonical composite cache key

Add tenant, session, and subject identity to the cache key string. No struct changes to `CacheKey`; the fix stays in `data_access`.

Key format:

```text
<namespace>:<tenant_id>:<session_id>:<subject>:<request>
```

Components:
- `namespace` — existing `type_name::<V>()` cached in struct
- `tenant_id` — `context.tenant_id.unwrap_or_default().to_string()`
- `session_id` — `context.session_id.unwrap_or_default().to_string()`
- `subject` — `context.claims.get("sub").map_or("", |s| s.as_str())`
- `request` — `CasId` (already implements `ToString`)

Authorization is enforced before cache access. After success, `tenant_id`/`session_id` are guaranteed present for `BetterAuthProvider`. `unwrap_or_default()` provides a deterministic sentinel for other authorizers; a zero UUID will never collide with a real tenant.

## Changes

### `crates/data_access/src/lib.rs`

Replace the current key construction in `execute()`:

```rust
let cache_key = CacheKey::new(self.namespace.clone(), request.to_string());
let key_str = format!("{}:{}", cache_key.namespace, cache_key.key);
```

with:

```rust
let tenant_id = context
    .tenant_id
    .map(|id| id.to_string())
    .unwrap_or_default();
let session_id = context
    .session_id
    .map(|id| id.to_string())
    .unwrap_or_default();
let subject = context
    .claims
    .get("sub")
    .map_or("", |s| s.as_str());
let key_str = format!(
    "{}:{}:{}:{}:{}",
    self.namespace, tenant_id, session_id, subject, request
);
```

Remove the now-unused `CacheKey` import in `execute()` if desired.

### Regression test: `cross_tenant_cache_isolation`

Add to `#[cfg(test)] mod tests`:

```rust
#[tokio::test]
async fn cross_tenant_cache_isolation() {
    let repo = Arc::new(InMemoryRepository::<String>::new());
    let id = repo.create("secret".to_string()).await.unwrap();
    let cache = make_cache();
    let authorizer = Arc::new(theligi_authorization::BetterAuthProvider);
    let pipeline =
        HierarchicalDataAccess::new(cache.clone(), repo.clone(), authorizer.clone(), None, None);

    let tenant_a = uuid::Uuid::new_v4();
    let session_a = uuid::Uuid::new_v4();
    let context_a = AuthorizationContext::new()
        .with_tenant(tenant_a)
        .with_session(session_a)
        .with_claim("sub", "subject-a");

    let tenant_b = uuid::Uuid::new_v4();
    let session_b = uuid::Uuid::new_v4();
    let context_b = AuthorizationContext::new()
        .with_tenant(tenant_b)
        .with_session(session_b)
        .with_claim("sub", "subject-b");

    let result_a = pipeline.execute(&context_a, id).await;
    assert!(result_a.is_ok());
    assert_eq!(result_a.unwrap().unwrap(), "secret");

    repo.delete(id).await.unwrap();

    let result_b = pipeline.execute(&context_b, id).await;
    assert!(matches!(result_b, Err(DataAccessError::NotFound)));
}
```

Logic:
1. Tenant A populates cache under key scoped to A.
2. Repository entry is deleted.
3. Tenant B requests same `CasId`. With the old key, B would hit A's cached `"secret"`. With the new key, B gets a cache miss, then a repository `NotFound`.

## Authorization on cache hit

`authorization_on_hit = true` is specified in both `SPEC.toml` and `SUBSYSTEM.toml`. The fix includes an explicit re-authorization call inside the cache-hit branch before returning the cached value. This is a one-line addition to `execute()`:

```rust
let cached = self.cache.get(&key_str).await?;
if let Some(entry) = cached {
    self.authorizer
        .authorize(context, "data_access", "execute")
        .await?;
    let value = entry.value.downcast::<V>().map_err(|_| {
        CacheError::Internal(format!(
            "type mismatch in cache; expected {}",
            type_name::<V>()
        ))
    })?;
    return Ok(Ok((*value).clone()));
}
```

The existing authorization call before cache lookup is retained for the cache-miss path (which leads to repository access, satisfying `authorization_before_repository = true`).

## Final plan

1. Edit `crates/data_access/src/lib.rs`:
   - Replace key construction in `execute()` with context-scoped key.
   - Add `self.authorizer.authorize(...)` inside the `if let Some(entry) = cached` branch.
   - Remove the now-unused `CacheKey` import if desired.

2. Add regression test `cross_tenant_cache_isolation` to `#[cfg(test)] mod tests`.

3. Run `cargo fmt --check && cargo check && cargo clippy && cargo test` from `crates/data_access/` to validate.
