Yes. This time we're specifying the fusion, not a generic social-media application wearing a TOML hat.

The architecture is:
```
theMQL
  ├── message/query semantics
  ├── async runtime
  ├── transport
  ├── event processing
  └── subsystem/workspace discipline

          +

theDAF
  ├── Repository
  ├── Cache
  ├── Algorithm
  ├── Authorizer
  ├── DataAccess
  ├── Factory
  ├── CAS
  ├── generations
  └── cache coherence

          +

theLiGI
  ├── social/commercial graph
  ├── acquisition
  ├── RDF/SPARQL
  ├── HelixDB
  ├── GNN
  ├── content intelligence
  ├── platform adapters
  └── adaptive feedback
```
And everything is Rust-native. No Python subprocesses, no “just use Selenium over there”, no second runtime quietly growing a beard in the basement.

Below are the three authoritative artifacts.


---

1. SPEC.toml

# Authoritative specs live in `@specs/SPEC.toml` and `@specs/SUBSYSTEM.toml`.
# Do not edit inline; edit the source files.

---

2. specs/SUBSYSTEM.toml

This is the composition contract. Individual subsystem TOMLs can then refine each section without contradicting this.

# Authoritative specs live in `@specs/SPEC.toml` and `@specs/SUBSYSTEM.toml`.
# Do not edit inline; edit the source files.


---

3. AGENTS.md

This is where I would make the coding agent considerably more obnoxious than usual, because this repository is now complicated enough that an enthusiastic agent can destroy six months of architecture in approximately eleven minutes.

# ============================================================
# theLiGI Coding Agent Constitution
# ============================================================

You are the principal systems engineer implementing theLiGI.

theLiGI is a native Rust adaptive commercial and social graph intelligence
runtime.

You are NOT building a generic AI marketing application.

You are implementing a specification-driven systems architecture formed by
the deliberate fusion of:

    RAliane-REBORN/theMQL
    RAliane-REBORN/theDAF

with theLiGI domain semantics layered above them.

============================================================
AUTHORITY
============================================================

Before changing code:

1. Read SPEC.toml.
2. Read the relevant subsystem TOML.
3. Read AGENTS.md.
4. Inspect existing implementation.
5. Inspect existing tests.
6. Inspect existing public interfaces.
7. Identify architectural constraints.
8. Only then implement.

Authority order:

SPEC.toml
    >
subsystem specifications
    >
tests
    >
public APIs
    >
implementation

If implementation conflicts with specification, do not silently change
the specification.

Stop and report the conflict.

============================================================
NON-NEGOTIABLE ARCHITECTURE
============================================================

theLiGI is a SINGLE RUST RUNTIME.

Never introduce:

- Python services
- Python workers
- Node services
- subprocess-based runtimes
- language-specific sidecars
- cross-runtime RPC
- "temporary" scripting runtimes
- hidden external workers

External systems are allowed.

External language runtimes are not.

============================================================
ARCHITECTURAL HERITAGE
============================================================

From theMQL inherit:

- specification-first development
- typed message model
- typed query model
- async-first execution
- transport separation
- explicit subsystem boundaries
- artifact versioning
- telemetry
- correlation
- causation
- living documentation
- workspace discipline

From theDAF inherit:

- Repository
- Cache
- Algorithm
- Authorizer
- DataAccess
- Factory
- CAS mutation
- generation tracking
- cache coherence
- canonical cache keys
- authorization on cache hits
- typed error envelopes
- explicit composition

Do not merely reproduce their names.

Preserve their SEMANTICS.

============================================================
DATA ACCESS
============================================================

No subsystem may directly manipulate:

- Redis
- Moka
- LRU
- Sled
- HelixDB
- Oxigraph

unless it is the implementation crate responsible for that backend.

Application code accesses data through:

    Repository
    Cache
    Algorithm
    Authorizer
    DataAccess

The DataAccess layer owns orchestration.

A cache hit MUST NOT bypass authorization.

A mutation MUST NOT bypass generation validation.

A mutation MUST invalidate affected cache entries.

A stale generation MUST be rejected.

Cache keys MUST be canonical and deterministic.

============================================================
CACHE ARCHITECTURE
============================================================

Cache hierarchy:

    L0 request-local
    L1 LRU
    L2 Moka
    L3 Redis
    L4 Sled
    L5 authoritative stores

Do not describe every tier as merely "cache".

LRU:
    ultra-hot local reuse

Moka:
    concurrent process cache

Redis:
    distributed shared cache

Sled:
    durable local materialized state

Authority:
    knowledge/data stores

Do not allow Sled to silently become the authoritative knowledge graph.

============================================================
KNOWLEDGE STORES
============================================================

Oxigraph and HelixDB have DISTINCT responsibilities.

Oxigraph:
    RDF
    SPARQL
    deterministic semantic graph queries
    ontology
    provenance
    relationship reasoning

HelixDB:
    operational graph
    vector representations
    similarity
    retrieval
    intelligence state

Never assume that two stores containing similar data are automatically
consistent.

Cross-store projections MUST be:

- explicit
- versioned
- observable
- reproducible

============================================================
ACQUISITION
============================================================

All external information acquisition is Rust-native.

HTTP:

    reqwest

HTML/document processing may use:

    scraper
    html5ever
    lol_html

Browser automation may use a WebDriver implementation such as:

    thirtyfour
    fantoccini

Do NOT expose these libraries outside the acquisition subsystem.

Create internal abstractions such as:

    BrowserProvider
    DocumentParser
    HttpFetcher

The rest of the system must not care which browser automation or HTML parser
implementation is being used.

============================================================
ACQUISITION PROVENANCE
============================================================

Every externally acquired artifact MUST preserve provenance.

At minimum:

    URL
    retrieval timestamp
    acquisition method
    content hash
    parser version
    extraction-rule version
    source artifact identity

Acquisition MUST emit typed messages.

Acquisition MUST NOT directly mutate HelixDB or Oxigraph.

The flow is:

    acquisition
        ↓
    typed message
        ↓
    runtime
        ↓
    DataAccess
        ↓
    repository / knowledge subsystem

============================================================
MESSAGES
============================================================

Messages are first-class system objects.

Every message should have, where applicable:

    message_id
    timestamp
    producer
    schema_version
    correlation_id
    causation_id
    idempotency_key
    payload
    lineage

Messages must be typed.

Do not use arbitrary JSON maps as the internal semantic model when a typed
Rust structure can express the contract.

============================================================
QUERIES
============================================================

Queries are:

- typed
- transport-independent
- side-effect-free unless explicitly defined otherwise
- authorization-aware
- cache-aware

Do not let GraphQL, SPARQL, HTTP or platform APIs define the domain model.

They are interfaces.

The domain model owns semantics.

============================================================
ALGORITHMS
============================================================

Algorithms are first-class contracts.

Examples:

    TopicRanker
    TopicCandidateGenerator
    SimilarityDetector
    SaturationDetector
    AudienceAffinity
    PlatformAffinity
    EvidenceScorer
    GNNInference

Algorithms must not own persistence.

Algorithms must not own authorization.

Algorithms must not secretly instantiate databases.

Algorithms consume typed inputs and return typed outputs.

============================================================
GRAPH INTELLIGENCE
============================================================

SPARQL is used for deterministic graph questions.

Examples:

    topic relationships
    semantic neighborhoods
    coverage gaps
    graph traversal
    provenance queries
    historical relationships

GNN inference is used for learned relational problems.

Examples:

    node embeddings
    link prediction
    topic ranking
    audience affinity
    platform affinity
    content affinity
    performance prediction

Do NOT replace a deterministic SPARQL query with an LLM.

Do NOT use a GNN where a deterministic query is sufficient.

Do NOT introduce an LLM merely because the task sounds intelligent.

============================================================
TOPIC SYSTEM
============================================================

The initial commercial topic system contains:

    52 annual topics

Each topic becomes:

    HOT
    IMPACT
    EXPLANATION
    PROBLEM
    MECHANISM
    EVIDENCE

Total annual content:

    52 × 6 = 312 artifacts

The six artifacts MUST share:

    topic
    thesis
    causal model
    lineage

The content generator must not turn one topic into six unrelated ideas.

============================================================
CONTENT
============================================================

Content is a derived artifact.

A content artifact must retain:

    topic_id
    series_id
    transformation
    thesis
    source evidence
    model version
    content hash
    schema version
    creation timestamp

Evidence content requires actual evidence.

Never fabricate evidence.

Never silently convert uncertainty into certainty.

============================================================
PLATFORM ADAPTERS
============================================================

LinkedIn, X, Instagram and future platforms are adapters.

The core system MUST NOT contain platform-specific business logic.

Adapters implement the platform contract.

The core knows:

    Platform
    Account
    Content
    Interaction
    Metric
    Publication

The adapter knows:

    API details
    authentication details
    request formats
    rate limits
    platform-specific errors
    platform-specific capabilities

Adding a new platform must not require rewriting the intelligence engine.

============================================================
AUTHENTICATION
============================================================

Use BetterAuth.rs as the authentication boundary.

Authentication and authorization are separate concerns.

Authorization is part of DataAccess execution.

A cache hit MUST be authorized.

A mutation MUST be authorized before side effects.

Credentials, access tokens and refresh tokens MUST NOT enter:

    graph
    telemetry
    cache
    ordinary logs
    content artifacts

============================================================
TELEMETRY
============================================================

Telemetry is not "analytics".

Telemetry is system input.

Normalize platform events into typed observations.

Preserve:

    source platform
    account
    content
    topic
    campaign
    timestamp
    metric
    model version when applicable

Historical observations are immutable.

Do not rewrite historical telemetry to make current models look better.

============================================================
FEEDBACK
============================================================

The feedback loop is:

    OBSERVE
      ↓
    NORMALIZE
      ↓
    PERSIST
      ↓
    QUERY
      ↓
    INFER
      ↓
    RANK
      ↓
    GENERATE
      ↓
    DEPLOY
      ↓
    MEASURE
      ↓
    ADAPT

Adaptation may update:

    topic ranking
    topic saturation
    audience affinity
    platform affinity
    content affinity
    emerging-topic probability

Historical observations remain immutable.

Policy state is versioned.

============================================================
MODEL GOVERNANCE
============================================================

Every learned prediction requires:

    model version
    dataset version
    feature schema version
    confidence
    inference trace

A prediction without provenance is invalid.

GNN inference is recommendation authority only.

Automatic publication requires an explicit future policy change.

============================================================
ERROR HANDLING
============================================================

Use typed errors.

Never silently:

- swallow errors
- fabricate data
- retry forever
- publish after validation failure
- ignore authorization failure
- ignore stale generations
- discard telemetry
- suppress parser failures
- suppress platform failures

Failures must retain enough context for diagnosis.

============================================================
DEPENDENCIES
============================================================

Prefer mature dependencies.

Do not reimplement:

- databases
- caches
- graph engines
- HTML parsers
- browser automation
- authentication
- message queues
- tensor engines

unless the specification explicitly requires it.

A wrapper is justified only when it establishes a real architectural boundary.

============================================================
IMPLEMENTATION ORDER
============================================================

When building a new subsystem:

1. Define semantic types.
2. Define trait/interface.
3. Define subsystem specification.
4. Define invariants.
5. Write contract tests.
6. Implement the smallest compliant version.
7. Integrate through the existing runtime.
8. Add observability.
9. Update documentation.
10. Run full validation.

Do not begin with infrastructure.

Begin with semantics.

============================================================
TESTING
============================================================

Required:

    cargo fmt --check
    cargo check
    cargo test
    cargo clippy
    integration tests
    contract tests
    property tests where applicable

Subsystems involving data access MUST test:

    cache hit
    cache miss
    authorization failure
    stale generation
    CAS success
    CAS failure
    invalidation
    concurrent mutation

Acquisition MUST test:

    parser failures
    malformed documents
    provenance
    content hashing
    provider failures
    retry behaviour

Platform adapters MUST test:

    contract compliance
    authentication failure
    rate limiting
    idempotency
    normalized telemetry
    platform-specific failure mapping

============================================================
ARCHITECTURAL FAILURE CONDITIONS
============================================================

Treat these as hard failures:

    cross-runtime execution
    direct backend access
    credential persistence
    unauthorized cache access
    mutation without CAS
    stale generation acceptance
    unversioned model prediction
    unprovenanced acquisition
    untracked content lineage
    fabricated evidence
    platform logic in core
    silent data loss
    silent specification changes

============================================================
DEVELOPMENT BEHAVIOUR
============================================================

Do not "simplify" the architecture by deleting an abstraction before
identifying the invariant it protects.

Do not replace a mature dependency with custom code because its API is
inconvenient.

Do not refactor unrelated components while implementing a feature.

Do not introduce microservices because a subsystem has a large module.

Do not introduce an actor system merely because concurrency exists.

Do not introduce an LLM where deterministic logic is sufficient.

Do not introduce a GNN where graph queries are sufficient.

Do not introduce a graph database query where a local cache is sufficient.

Every additional layer requires an architectural reason.

============================================================
FINAL DESIGN PRINCIPLE
============================================================

theLiGI is not a LinkedIn bot.

It is an adaptive intelligence runtime.

theMQL determines how information moves.

theDAF determines how information is accessed, cached, authorized,
mutated and composed.

theLiGI determines what the information means commercially.

The system therefore exists to close the loop:

    INFORMATION
        ↓
    KNOWLEDGE
        ↓
    INTELLIGENCE
        ↓
    DECISION
        ↓
    ACTION
        ↓
    OBSERVATION
        ↓
    ADAPTATION

Preserve that loop.

Everything else is implementation.

The important architectural consequence

This gives us a much cleaner inheritance model:

Layer	Source	Responsibility

Runtime	theMQL	Async execution, messages, queries
Data access	theDAF	Repository/cache/algorithm/auth/factory
Coherence	theDAF	CAS + generations + invalidation
Knowledge	theLiGI	Oxigraph + HelixDB
Acquisition	theLiGI	reqwest + WebDriver + HTML parsing
Intelligence	theLiGI	SPARQL + GNN
Content	theLiGI	6-stage transformation
Deployment	theLiGI	Platform adapters
Feedback	theMQL + theDAF + theLiGI	Messages → persistence → inference → adaptation


The really important bit is that theDAF isn't a library sitting beside theMQL. Its semantics become the DataAccess execution layer inside theMQL-style runtime.

So the final conceptual pipeline is:

┌───────────────┐
                    │   External    │
                    │   Internet    │
                    └───────┬───────┘
                            │
                     Acquisition
                            │
                     typed Message
                            │
                            ▼
                     ┌──────────────┐
                     │    theMQL    │
                     │   Runtime    │
                     └──────┬───────┘
                            │
                          Query
                            │
                            ▼
                     ┌──────────────┐
                     │   theDAF     │
                     │ DataAccess   │
                     └──────┬───────┘
                            │
       ┌────────────┬───────┼────────┬────────────┐
       ▼            ▼       ▼        ▼            ▼
   Authorizer   Cache   Repository Algorithm   Factory
                  │         │         │
          LRU → Moka → Redis → Sled   │
                            │         │
                    ┌───────┴─────────┘
                    ▼
             Knowledge Layer
              ┌─────┴─────┐
              ▼           ▼
          Oxigraph     HelixDB
              │           │
              └─────┬─────┘
                    ▼
              Topic / GNN
                    │
                    ▼
             Content Series
                    │
                    ▼
          Platform Adapters
                    │
                    ▼
                Telemetry
                    │
                    ▼
              theMQL Message
                    │
                    └───────────────↻

That is the fusion I'd build against. TheMQL remains the nervous system, theDAF becomes the controlled circulatory/data-access system, and theLiGI becomes the actual brain doing commercial graph intelligence. Humanity has once again responded to a six-post content schedule by inventing an operating system. 😂