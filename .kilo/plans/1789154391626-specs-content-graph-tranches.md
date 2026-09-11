# Plan: theLiGI Specs Extraction, Content Graph Model, and Implementation Tranches

## 1. Context

The project's authoritative architecture currently lives as inline TOML blocks inside `README.md` (lines 46–1463). There is no `@specs/` directory, no plan files, and no discrete-vs-continuous content graph concept.

This plan defines:
1. Extraction of `SPEC.toml` and `SUBSYSTEM.toml` into `@specs/`.
2. Introduction of a **discrete versus continuous rooted content graph** model.
3. Cross-checking and verification steps.
4. Implementation tranches with clear boundaries.

---

## 2. Specs Extraction (`@specs/`)

### 2.1 Create `@specs/SPEC.toml`

Source: `README.md` lines 46–834.

Action: Copy the TOML block verbatim into `@specs/SPEC.toml`.

Additive change only: append the new `[content.graph]` section (see §3 below).

Verify: `toml` parser accepts the file; no structural changes to existing keys.

### 2.2 Create `@specs/SUBSYSTEM.toml`

Source: `README.md` lines 838–1463.

Action: Copy the TOML block verbatim into `@specs/SUBSYSTEM.tOML`.

Additive change only: append the new `[content.graph]` subsystem section (see §3 below).

Verify: keys match existing crate ownership; no forbidden-flow violations introduced.

### 2.3 Update `README.md`

Remove the inline TOML blocks and replace them with references:

```markdown
# Authoritative specs live in `@specs/SPEC.toml` and `@specs/SUBSYSTEM.toml`.
# Do not edit inline; edit the source files.
```

This keeps `README.md` as the entry point without duplicating authoritative content.

---

## 3. Discrete vs Continuous Rooted Content Graph

### 3.1 Concept

| Dimension | Discrete | Continuous |
|-----------|----------|------------|
| Root | `ContentSeries` | `Topic` |
| Boundedness | Finite, exactly 6 posts | Unbounded temporal stream |
| Edges | Explicit `HAS_POST`, `NEXT`, `SUPPORTS`, `CAUSES` | Implicit chronological adjacency + `RELATED_TO` similarity edges |
| Mutability | Immutable once published | Append-only stream; no mutation of historical observations |
| Query shape | "All posts in series X", "Evidence for claim Y" | "Content published in window W on topic T", "Trajectory of topic T" |
| Lifecycle | Created → published → completed (6 posts) | Ongoing; no completion state |
| Cache key pattern | `series:{id}` | `topic:{id}:window:{start}:{end}` |

### 3.2 New spec additions to `@specs/SPEC.toml`

Append after the existing `[content]` block:

```toml
# ============================================================
# Content graph models
# ============================================================

[content.graph]
owner = "theligi-content"

[content.graph.discrete]
root = "ContentSeries"
node_types = ["ContentSeries", "ContentArtifact", "Claim", "Evidence"]
edge_types = ["HAS_POST", "NEXT_IN_SERIES", "SUPPORTS", "CAUSES", "DERIVES_FROM"]
cardinality = "exactly_6_posts_per_series"
lifecycle = "create_publish_complete"
immutable_after_publish = true

[content.graph.continuous]
root = "Topic"
node_types = ["ContentArtifact", "Interaction", "Signal", "TelemetryObservation"]
edge_types = ["CHRONOLOGICAL_ADJACENCY", "RELATED_TO", "INFLUENCED_BY"]
boundedness = "unbounded_stream"
lifecycle = "append_only"
immutable_observations = true

[content.graph.cross_model]
shared_roots_allowed = false
discrete_series_may_target_continuous_topic = true
continuous_stream_may_contain_discrete_series_references = true
```

### 3.3 New spec additions to `@specs/SUBSYSTEM.toml`

Append after the existing `[content]` section:

```toml
# ============================================================
# Content graph
# ============================================================

[content.graph]
crate = "theligi-content"

[content.graph.ownership]
discrete_model = "theligi-content"
continuous_model = "theligi-content"
graph_traversal = "theligi-graph"
sparql_queries = "theligi-sparql"
vector_similarity = "theligi-graph-via-helixdb"

[content.graph.boundaries]
discrete_must_not_bypass_series_root = true
continuous_must_not_use_series_edges = true
continuous_window_queries_must_not_scan_unbounded = true

[content.graph.data_access]
discrete_reads = "series_scoped"
continuous_reads = "topic_scoped_windowed"
```

---

## 4. Cross-Check and Verification

| Check | Method | Pass Criteria |
|-------|--------|---------------|
| TOML parses | `python -m toml @specs/SPEC.toml` | No error |
| TOML parses | `python -m toml @specs/SUBSYSTEM.toml` | No error |
| Existing keys unchanged | Diff `@specs/SPEC.toml` against README lines 46–834 | Only additive sections at end |
| Existing keys unchanged | Diff `@specs/SUBSYSTEM.toml` against README lines 838–1463 | Only additive sections at end |
| No forbidden flow | Review `[content.graph.ownership]` against `[dependency_boundary]` | No new forbidden-flow entries |
| Crate assignments | Verify every new `crate =` value exists in `[workspace.crates]` | All match |
| Discrete/continuous separation | Verify no shared edge types between the two models | Lists are disjoint |
| `@specs/` referenced | `README.md` points to source files | References present, inline blocks removed |

---

## 5. Implementation Tranches

### Tranche 1 — Specs Foundation

**Goal**: Extract specs, add discrete/continuous content graph model, verify.

**Deliverables**:
- `@specs/SPEC.toml`
- `@specs/SUBSYSTEM.toml`
- Updated `README.md`
- Verification checklist completed (all pass criteria met)

**Out of scope**: Any Rust code, any plan files beyond this one.

### Tranche 2 — Workspace and Core Crates

**Goal**: Initialize Cargo workspace; implement `theligi-core` semantic types.

**Deliverables**:
- `Cargo.toml` workspace
- `theligi-core` crate: `ResourceIdentity`, `Lineage`, `CorrelationId`, `CausationId`, `SchemaVersion`, error types
- `theligi-schema` crate: spec-derived type definitions

**Dependencies**: Tranche 1 complete and verified.

### Tranche 3 — Data Access Layer (theDAF)

**Goal**: Implement repository, cache, authorizer, algorithm, factory, DataAccess.

**Deliverables**:
- `theligi-repository` (CAS, generation tracking, typed errors)
- `theligi-cache` (L0–L4 tiers, canonical keys, invalidation)
- `theligi-authorization` (fail-closed, context isolation)
- `theligi-algorithm` (typed algorithm trait)
- `theligi-factory` (DI composition)
- `theligi-data-access` (orchestration pipeline)

**Dependencies**: Tranche 2.

### Tranche 4 — Knowledge Stores

**Goal**: Integrate Oxigraph (RDF/SPARQL) and HelixDB (operational graph + vector).

**Deliverables**:
- `theligi-knowledge` (dual-store projection boundary)
- `theligi-graph` (graph node/edge types, traversal)
- `theligi-sparql` (deterministic query wrapper)

**Dependencies**: Tranche 3.

### Tranche 5 — Content Graph: Discrete Model

**Goal**: Implement the bounded `ContentSeries` model with explicit edges.

**Deliverables**:
- `theligi-content` (discrete graph: `ContentSeries`, `ContentArtifact`, `Claim`, `Evidence` nodes; `HAS_POST`, `NEXT_IN_SERIES`, `SUPPORTS`, `CAUSES` edges)
- `theligi-validation` (series invariants: 6 posts, shared topic/thesis, causal consistency, lineage)
- Series lifecycle: `create → publish → complete`
- Tests: series creation, series completeness, invariant enforcement

**Dependencies**: Tranches 3, 4.

### Tranche 6 — Content Graph: Continuous Model

**Goal**: Implement the unbounded `Topic`-rooted temporal stream.

**Deliverables**:
- Extend `theligi-content` (continuous graph: append-only stream, chronological adjacency, similarity edges)
- Windowed query support (no unbounded scans)
- Cross-model references (discrete series → continuous topic; continuous stream → discrete series references)
- Tests: append-only, window query bounds, cross-model reference integrity

**Dependencies**: Tranche 5.

### Tranche 7 — Topic, Inference, and Content Generation

**Goal**: Topic discovery, ranking, GNN inference, content artifact generation.

**Deliverables**:
- `theligi-topic` (discover, rank, detect saturation)
- `theligi-inference` (GNN: node embedding, link prediction, content affinity)
- `theligi-validation` (content-level checks)
- Deterministic SPARQL + learned GNN boundary enforced

**Dependencies**: Tranches 4, 5, 6.

### Tranche 8 — Acquisition, Platform Adapters, and Feedback Loop

**Goal**: Close the observe→adapt loop.

**Deliverables**:
- `theligi-acquisition` / `theligi-http` / `theligi-browser` / `theligi-document`
- `theligi-platform` / `theligi-linkedin` / `theligi-x` / `theligi-instagram`
- `theligi-scheduler`
- `theligi-telemetry` / `theligi-feedback`
- End-to-end feedback pipeline tests

**Dependencies**: Tranche 7.

---

## 6. Open Questions

None remaining. The discrete-vs-continuous distinction is now formally defined as bounded-series vs unbounded-temporal-stream, both rooted, both owned by `theligi-content`, with disjoint edge types and explicit cross-model reference rules.
