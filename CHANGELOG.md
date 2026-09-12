# Changelog

Chronological record of changes to theLiGI. Newest entries at the top.
Update after every turn.

## [Unreleased]

### 2026-09-12 — PR #4 review fixes (cache write policy + error conversion)

Addresses review feedback on PR #4 head:

#### theligi-data-access

- **Fixed cache write policy**: `HierarchicalDataAccess::execute()` now writes to L1
  only via `self.cache.l1().set(...)` instead of `self.cache.set(...)`.
  `daf_cache::HierarchicalCache::set()` propagates writes to L0 + L1, which
  contradicted the declared L1-only write policy in the plan.
- `namespace` is computed once in `new()` via `type_name::<V>()` and stored as a
  field, rather than recomputed on every `execute()` call. This makes the
  namespace explicit and overridable without changing the public API.

#### theligi-cache

- Improved `From<daf_core::CacheError>` for `CacheError` to match on the inner
  `String` directly (`e.0`) instead of round-tripping through `Display` and
  parsing the rendered string. Preserves full error text for unknown cases.

### 2026-09-12 — Hierarchical data-access pipeline (fail-closed)

Executed plan `.kilo/plans/1789226415812-hierarchical-pipeline-implementation.md`.
Fixes `no_silent_fallback` and `fail_closed_security` violations in the
cache and data-access layers. Converges `OrchestrationPipeline` and
`HierarchicalDataAccess` into one canonical pipeline.

#### theligi-cache

- Added `CacheError::Unavailable(CacheTier)` for absent mandatory tiers.
- Added `Display` for `CacheTier` (required by `thiserror` derive).
- `HierarchicalCacheWrapper::get()` returns `Unavailable` for absent L0/L5
  instead of panicking via `unwrap()` or silently falling back to L4/L1.
- `HierarchicalCacheWrapper::set()` returns `Unavailable` for absent L0/L5.
- `HierarchicalCacheWrapper::delete()` returns `Unavailable` for absent L0/L5.
- `ttl_seconds` intentionally discarded with `tracing::debug!` note
  (daf-cache tiers do not support TTL).
- Added tests: `hierarchical_wrapper_l0_unavailable`,
  `hierarchical_wrapper_l5_unavailable`.

#### daf-cache (`theDAF` shared infrastructure)

- `HierarchicalCache::get()` requires L0 and L5; tier errors propagate
  with `?` instead of `tracing::warn!` + continue.
- `HierarchicalCache::set()` propagates L0 errors; L0 write skipped only
  when absent (not silently swallowed).
- `HierarchicalCache::delete()`, `delete_prefix()`, `clear()`, `shake()`
  require L0/L5 and propagate tier errors.
- `HierarchicalCache::get()` L1 promotion errors also propagate (fail-closed).

#### theligi-data-access

- Removed `OrchestrationPipeline` (architectural duplication; unused
  outside this crate).
- Rewrote `HierarchicalDataAccess<V>` as the canonical `DataAccess` impl:
  generic over `V: Send + Sync + Clone + 'static`.
  Holds `cache`, `repository`, `authorizer`, `algorithm`, `validator`.
- `execute()` implements full pipeline:
  1. validate (invokes `SeriesValidator`; failures return `DataAccessError::Validation`)
  2. authorize (mandatory, fail-closed)
  3. cache_lookup (L0→L1→L2→L3→L4→L5 via `daf_cache::HierarchicalCache::get`)
  4. repository_lookup (on full cache miss)
  5. algorithm (optional)
  6. cache_population (writes to L1 via `daf_cache::HierarchicalCache::set`)
- `DataAccess::Request = CasId`, `DataAccess::Response = Result<V, DataAccessError>`.
- Added `From<daf_core::CacheError>` for `DataAccessError`.
- Added `theligi-validation` and `daf-core` dependencies.
- Added tests: `authorization_enforced_before_cache_access`,
  `full_pipeline_cache_miss_then_hit`, `cache_hit_short_circuits_repository`,
  `missing_l0_returns_error`, `missing_l5_returns_error`.

#### Validation

- `cargo fmt --check` — clean.
- `cargo check --workspace` — clean.
- `cargo clippy --workspace` — clean.
- `cargo test --workspace` — all crates pass (workspace-wide green).
- No `unsafe` introduced.

### 2026-09-12 — PR #4 review fixes (dependency reproducibility + test hardening)

Post-review fixups for PR #4 head `1659f8b`:

#### theDAF (shared infrastructure)

- Committed and pushed fail-closed `HierarchicalCache` changes to
  `feat/l0-l5-hierarchical-cache` branch (`37d54d7`).
- `HierarchicalCache::get/delete/delete_prefix/clear/shake` now require
  L0 and L5 and propagate tier errors with `?` instead of logging and
  continuing.
- L1 promotion errors in `get()` propagate instead of being swallowed.

#### theLiGI dependency pinning

 - Replaced local `theDAF` path dependencies for `daf-cache` and
   `daf-core` with git dependencies on
   `https://github.com/RAliane-REBORN/theDAF.git`
   `rev = "37d54d78f6e7d1e3baf73db4c2daa00e78266422"`.
 - Other theDAF crates (`daf-repository`, `daf-algorithms`,
   `daf-runtime`, `daf-messaging`, `daf-http`, `daf-application`)
   remain as path dependencies.
 - Removed redundant `package` keys from crate manifests; workspace
   deduplication in root `Cargo.toml` handles aliasing.
 - Regenerated `Cargo.lock` to remove duplicate local `daf-cache`
   entry.
 - Improved `From<daf_core::CacheError>` for `theligi-cache::CacheError`
   to parse theDAF tier-unavailability messages and map them to typed
   `CacheError::Unavailable(CacheTier::L0/L5)` variants.

#### Test hardening

 - `missing_l0_returns_error` and `missing_l5_returns_error` now use
   typed `matches!` assertions against
    `DataAccessError::Cache(CacheError::Unavailable(CacheTier::L0/L5))`
    instead of matching rendered error strings.

#### Dependency graph deduplication (PR #4 integration blocker)

  - Converted all remaining theDAF workspace dependencies
    (`daf-repository`, `daf-algorithms`, `daf-http`, `daf-application`)
    from local `path` dependencies to the same immutable git revision
    already used for `daf-core` and `daf-cache`:
    `https://github.com/RAliane-REBORN/theDAF.git`
    `rev = "37d54d78f6e7d1e3baf73db4c2daa00e78266422`.
  - Updated `crates/repository/Cargo.toml`, `crates/algorithm/Cargo.toml`,
    and `crates/http/Cargo.toml` to use `{ workspace = true }` for their
    theDAF dependencies.
  - Removed unused workspace entries for `daf-runtime` and
    `daf-messaging` from root `Cargo.toml`.
  - Regenerated `Cargo.lock`; `daf-core` and `daf-cache` now each have
    exactly one entry in the lockfile, both sourced from the pinned git
    revision. No duplicate local/git package pairs remain.
   - This closes the reproducibility gap identified in PR review comment
     #5647232913: the workspace no longer compiles two distinct instances
     of `daf-core` and `daf-cache`.

#### TTL contract decision

  - Documented explicit contract in `Cache::set` trait doc: `ttl_seconds`
    behavior is implementation-defined. `InMemoryCache` stores but does
    not enforce expiry. `HierarchicalCacheWrapper` discards `ttl_seconds`
    because `daf-cache` tiers do not support TTL, emitting a
    `tracing::debug!` note.
  - Added `hierarchical_wrapper_set_ttl_is_noop` test confirming that
    passing `Some(60)` does not error and the value remains retrievable.

#### Validation-order instrumentation

  - Added `RecordingAuthorizer` test helper that records whether
    `authorize` was invoked via `Arc<AtomicBool>`.
  - `validation_enforced_before_authorization` now asserts both that
    the result is `Err(DataAccessError::Validation(_))` AND that the
    authorizer was never called.

#### Validation

- `cargo fmt --check` — clean.
- `cargo check --workspace` — clean.
- `cargo clippy --workspace` — clean.
- `cargo test --workspace` — all crates pass (workspace-wide green).

## [0.1.0] - 2026-09-12

### Added

- Workspace scaffold: 37 crates under `crates/` with path dependencies to
  `../../theMQL/` and `../../theDAF/`.
- `@specs/SPEC.toml` and `@specs/SUBSYSTEM.toml` — authoritative
  specifications for all 37 crates.
- `AGENTS.md` — coding agent constitution with authority hierarchy,
  non-negotiable architecture, data-access pipeline, cache hierarchy,
  knowledge store boundaries, acquisition provenance, error handling,
  and TETANUS integration.
- `TETANUS.md` — NASA/JPL Power of Ten adapted for Rust; identifies
  `theligi-data_access`, `theligi-repository`, `theligi-inference`,
  `theligi-knowledge`, `theligi-content`, `theligi-validation`,
  `theligi-evidence` as safety-critical crates.
- Inherited implementations from theMQL and theDAF:
  - `theligi-core` — semantic primitives
  - `theligi-message` / `theligi-query` / `theligi-runtime` — theMQL inheritance
  - `theligi-data-access` — `DataAccess` trait, `DataAccessError`,
    `OrchestrationPipeline` stub, `HierarchicalDataAccess` stub
  - `theligi-repository` — `Repository` trait, `InMemoryRepository`
  - `theligi-cache` — `CacheTier`, `CacheKey`, `Cache` trait,
    `InMemoryCache`, `HierarchicalCacheWrapper`
  - `theligi-authorization` — `AuthorizationContext`, `AuthorizationProvider`,
    `BetterAuthProvider` stub
  - `theligi-algorithm` — `Algorithm` trait, 8 concrete implementations
    (`TopicRanker`, `TopicGenerator`, `SimilarityDetector`,
    `SaturationDetector`, `AudienceAffinity`, `PlatformAffinity`,
    `EvidenceScorer`, `GnnInference`)
  - `theligi-validation` — `SeriesValidator` trait, `ValidationResult`,
    `Violation`, `ValidationReport`
  - `theligi-factory` — `Factory` DI container with `()` placeholder types
- LiGI-specific intelligence layer: `knowledge`, `graph`, `sparql`,
  `acquisition`, `http`, `browser`, `document`, `platform`, `linkedin`,
  `x`, `instagram`, `topic`, `inference`, `content`, `validation`,
  `evidence`, `scheduler`, `telemetry`, `feedback`, `artifact`,
  `security`, `observability`, `server`, `cli`, `desktop`.
- `README.md` — architecture overview, layer table, pipeline diagram.
