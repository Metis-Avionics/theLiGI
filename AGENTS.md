# AGENTS.md

## Authority

Before changing code:
1. Read `@specs/SPEC.toml`
2. Read the relevant `@specs/SUBSYSTEM.toml` section
3. Read existing tests and public interfaces
4. Only then implement

Authority order: `SPEC.toml` > subsystem TOML > tests > public APIs > implementation.

If implementation conflicts with spec, report the conflict. Do not silently change the spec.

## Non-Negotiable Architecture

Single Rust runtime. Never introduce Python, Node, subprocess-based workers, cross-runtime RPC, or hidden external runtimes.

External systems are allowed. External language runtimes are not.

## Workspace

Rust workspace with 37 crates under `crates/`. Path dependencies to `../../theMQL/` and `../../theDAF/`.

Key boundaries (see `@specs/SUBSYSTEM.toml` for full list):
- `core`, `schema`, `context` - semantic primitives
- `message`, `query`, `runtime` - theMQL inheritance
- `data_access`, `repository`, `cache`, `authorization`, `algorithm`, `factory` - theDAF inheritance
- `knowledge`, `graph`, `sparql` - Oxigraph + HelixDB
- `acquisition`, `http`, `browser`, `document` - Rust-native external info
- `platform`, `linkedin`, `x`, `instagram` - adapters only
- `topic`, `inference`, `content`, `validation`, `evidence` - intelligence
- `scheduler`, `telemetry`, `feedback` - runtime loop
- `artifact`, `security`, `observability` - cross-cutting
- `server`, `cli`, `desktop` - entrypoints

## Commands

```bash
cargo fmt --check
cargo check
cargo clippy
cargo test
```

Tests are inline in `#[cfg(test)]` modules only. No `tests/` directories.

## Data Access Pipeline

Execution order: validate -> authorize -> cache_lookup -> repository_lookup -> algorithm -> cache_population

Mutation order: validate -> authorize -> generation_check -> repository_mutation -> generation_increment -> cache_invalidation

Cache hit MUST be authorized. Mutation MUST invalidate affected cache entries. Stale generation MUST be rejected.

## Cache Hierarchy

L0 request-local, L1 LRU (ultra-hot), L2 Moka (concurrent process), L3 Redis (distributed), L4 Sled (durable local materialized), L5 authoritative stores.

Sled is NOT the authoritative knowledge graph. Oxigraph and HelixDB have distinct responsibilities.

## Knowledge Stores

- Oxigraph: RDF, SPARQL, deterministic semantic graph queries, ontology, provenance
- HelixDB: operational graph, vector representations, similarity, retrieval, intelligence state

Cross-store projections must be explicit, versioned, observable, reproducible.

## Acquisition

All external acquisition is Rust-native (reqwest, thirtyfour/fantoccini, scraper/html5ever/lol_html).

Must emit typed messages. Must NOT directly mutate HelixDB or Oxigraph.

Must preserve provenance: URL, retrieved_at, content_hash, acquisition_method, parser_version, extraction_rule_version.

## Platform Adapters

LinkedIn, X, Instagram are adapters. Core system MUST NOT contain platform-specific business logic.

Adding a new platform must not require rewriting the intelligence engine.

## Dependency Boundaries

Forbidden flows (from SUBSYSTEM.toml):
- platform -> helixdb/redis
- content -> redis
- inference -> redis
- acquisition -> helixdb/oxigraph
- query -> helixdb_direct/redis_direct
- adapter -> repository_implementation/cache_implementation

## Content Invariants

- 6 posts per series, ordered: hot -> impact -> explanation -> problem -> mechanism -> evidence
- Series must share topic, thesis, causal model, lineage
- Evidence requires actual evidence - never fabricate
- Content hash, schema version, model version required

## Error Handling

Use typed errors. Never silently swallow errors, fabricate data, retry forever, publish after validation failure, ignore authorization failure, or discard telemetry.

## GNN vs Deterministic

Do NOT replace deterministic SPARQL with an LLM. Do NOT use GNN where graph queries suffice. Do NOT introduce LLM merely because the task sounds intelligent.
