# Changelog

Chronological record of changes to theLiGI. Newest entries at the top.
Update after every turn.

## [Unreleased]

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
