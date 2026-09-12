# Plan: Executable Hierarchical Data-Access Pipeline

## Context

PR #3 established `HierarchicalDataAccess` and the L0-L5 cache vocabulary, but `execute()` is a stub that traces pipeline steps and returns `Ok(())`. PR #1's `OrchestrationPipeline` is also a stub. The two represent architectural duplication.

Additionally, `no_silent_fallback = true` and `fail_closed_security = true` are violated by:
- `HierarchicalCacheWrapper` silently falling back L5→L4 and L0→L1
- `daf_cache::HierarchicalCache` silently continuing on tier errors
- `HierarchicalDataAccess::execute()` discarding the authorization context

## Goal

Implement the actual `validate → authorize → cache_lookup → repository_lookup → algorithm → cache_population` pipeline in `HierarchicalDataAccess`, fix all `no_silent_fallback` violations, and converge the two pipeline implementations into one canonical path.

## Key Decisions

### 1. Canonical implementation: `HierarchicalDataAccess`

`OrchestrationPipeline` is removed. `HierarchicalDataAccess` becomes the single `DataAccess` implementation.

### 2. `HierarchicalDataAccess` is generic over `V`

```rust
pub struct HierarchicalDataAccess<V>
where
    V: Send + Sync + 'static,
{
    cache: Arc<daf_cache::HierarchicalCache>,
    repository: Arc<dyn Repository<Data = V>>,
    authorizer: Arc<dyn AuthorizationProvider>,
    algorithm: Option<Arc<dyn Algorithm<Input = V, Output = V>>>,
    validator: Option<Arc<dyn SeriesValidator>>,
}
```

`DataAccess::Request = CasId` and `DataAccess::Response = Result<V, DataAccessError>`.

`data_access` does NOT add a dependency on `theligi_content`. The caller specializes `V` (e.g., `ContentSeries`).

### 3. Pipeline execution order

Per `@specs/SUBSYSTEM.toml`:
1. `validate` — optional, only if `validator` is configured
2. `authorize` — mandatory, fail-closed
3. `cache_lookup` — L0 → L1 → L2 → L3 → L4 → L5, first hit wins
4. `repository_lookup` — on full cache miss
5. `algorithm` — optional, only if `algorithm` is configured
6. `cache_population` — write algorithm output (or raw repository data if no algorithm) to L1

### 4. Fail-closed tier semantics

**In `theLiGI/crates/cache/src/lib.rs` (`HierarchicalCacheWrapper`):**
- Absent L0 → `CacheError::Unavailable(CacheTier::L0)`
- Absent L5 → `CacheError::Unavailable(CacheTier::L5)`
- No L5→L4 or L0→L1 fallback

**In `theDAF/crates/daf-cache/src/hierarchical.rs` (`HierarchicalCache`):**
- Absent L0/L5 treated as configuration error
- Tier `get()` errors propagated (not silently continued)
- Tier `set()`/`delete()` errors propagated (not silently continued)
- `delete_prefix`, `clear`, `shake` aggregate across tiers but return error if any tier fails

**Rationale**: `no_silent_fallback = true` is a spec invariant. theDAF is shared infrastructure; fixing it here corrects the contract for all consumers.

### 5. Promotion policy

Promotion is handled by `daf_cache::HierarchicalCache::get()` (L2+ hits promote to L1). `HierarchicalCacheWrapper::get()` does NOT add additional promotion. L0 population happens only via explicit `set()` calls in the pipeline's `cache_population` step.

### 6. Write policy

Writes target L1 only. L0 is populated only on explicit `cache_population` after repository lookup. L2-L5 are not written to directly by the pipeline.

### 7. Invalidation policy

Not implemented in this PR. Deferred to a follow-up PR. `delete_prefix` and `clear` on the hierarchical cache remain functional.

### 8. Authorization on cache hits

Authorization happens BEFORE cache tier access, not after. This satisfies `authorization_on_cache_hit = true` and `authorization_before_repository = true`.

### 9. Type-erasure boundary

`daf_cache::Cache` stores `Arc<dyn Any + Send + Sync>`. `HierarchicalCacheWrapper` downcasts on `get()`. Type mismatches return `CacheError::Internal`. Cross-type collisions are prevented by typed `CacheKey` namespace; each `V` type uses a distinct namespace.

### 10. TTL ownership

`ttl_seconds` is passed through from `HierarchicalDataAccess` calls to the underlying cache. The pipeline does not set TTLs itself; callers specify them.

## Implementation Tasks

1. **Add `CacheError::Unavailable(CacheTier)`** in `crates/cache/src/lib.rs`
2. **Fix `HierarchicalCacheWrapper`** to return `Unavailable` for absent L0/L5 instead of falling back
3. **Fix `daf_cache::HierarchicalCache`** to propagate tier errors and treat absent L0/L5 as errors
4. **Rewrite `HierarchicalDataAccess`** to hold all pipeline components and implement `execute()` with the full pipeline
5. **Remove `OrchestrationPipeline`** (unused, architectural duplication)
6. **Add `theligi_validation` dependency** to `crates/data_access/Cargo.toml` for `SeriesValidator`
7. **Add tests** in `crates/data_access/src/lib.rs`:
   - Authorization enforced before cache access
   - Missing L0 returns error (not panic)
   - Missing L5 returns error (not silent fallback)
   - Full pipeline: cache miss → repository → algorithm → cache population → response
   - Cache hit short-circuits repository
8. **Update `@specs/SUBSYSTEM.toml`** execution_order documentation to match actual behavior
9. **Run `cargo fmt --check && cargo check && cargo clippy && cargo test`**

## Affected Files

- `crates/data_access/src/lib.rs`
- `crates/data_access/Cargo.toml`
- `crates/cache/src/lib.rs`
- `theDAF/crates/daf-cache/src/hierarchical.rs`
- `@specs/SUBSYSTEM.toml`

## Risks

- **theDAF change scope**: Fixing `daf_cache::HierarchicalCache` changes its public contract. Other workspace crates using it may need updates. Mitigation: review all `daf-cache` consumers after the change.
- **Generic complexity**: `HierarchicalDataAccess<V>` with `dyn Algorithm<Input = V, Output = V>` restricts algorithms to same-type transforms. Future algorithm signatures may need adjustment.
- **Validator coupling**: Adding `SeriesValidator` binds `data_access` to `validation`. This is intentional but widens the dependency graph.

## Validation

- `cargo fmt --check`
- `cargo check --workspace`
- `cargo clippy --workspace`
- `cargo test --workspace`
- Review all `daf-cache` consumers for tier-error behavior changes
