Yes. This time we're specifying the fusion, not a generic social-media application wearing a TOML hat.

The architecture is:

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

And everything is Rust-native. No Python subprocesses, no “just use Selenium over there”, no second runtime quietly growing a beard in the basement.

Below are the three authoritative artifacts.


---

1. SPEC.toml

# ============================================================
# theLiGI
# Linked Intelligence / Graph Intelligence
#
# Main Architectural Specification
# ============================================================

[spec]
name = "theLiGI"
version = "0.1.0"
status = "experimental"
authority = "highest"

[project]
name = "theLiGI"
full_name = "Linked Intelligence Graph Intelligence"
language = "rust"
license = "MIT"

description = """
Native Rust adaptive commercial and social graph intelligence system.

theLiGI observes external information and platform telemetry, normalizes
that information into typed messages and graph entities, performs
deterministic and learned inference, generates structured content,
deploys through platform adapters, and feeds resulting telemetry back
into the knowledge system.

The architecture fuses the message/query/runtime model of theMQL with
the data-access, caching, authorization, repository, algorithm and
factory semantics of theDAF.
"""

[mission]
primary = "Adaptive commercial intelligence."
secondary = "Graph-native social intelligence."
operating_model = "observe-represent-query-infer-generate-deploy-measure-adapt"
design_lens = "metis"

[architecture]
style = "modular_rust_workspace"
runtime = "single_rust_runtime"
async = true
native_rust = true
platform_agnostic_core = true
transport_agnostic_core = true
storage_agnostic_core = true
provider_agnostic_core = true
dependency_composition = true
specification_first = true

[architecture.authority]
spec = "highest"
subsystem_specs = "second"
tests = "third"
public_interfaces = "fourth"
implementation = "lowest"

[architecture.rules]
semantic_ownership = true
explicit_boundaries = true
typed_contracts = true
dependency_first = true
composition_over_reimplementation = true
no_cross_runtime = true
no_hidden_sidecars = true
no_untracked_state = true
no_silent_fallback = true
no_silent_spec_override = true
fail_closed_security = true
deterministic_core = true
async_first = true

# ============================================================
# Architecture inheritance
# ============================================================

[heritage.themql]
repository = "RAliane-REBORN/theMQL"
role = "runtime-message-query-architecture"

[heritage.themql.inherit]
message_model = true
query_model = true
async_execution = true
transport_separation = true
workspace_structure = true
artifact_versioning = true
specification_first = true
telemetry = true
living_documentation = true
typed_errors = true

[heritage.thedaf]
repository = "RAliane-REBORN/theDAF"
role = "data-access-semantics"

[heritage.thedaf.inherit]
repository_contract = true
cache_contract = true
algorithm_contract = true
authorizer_contract = true
data_access_contract = true
factory_contract = true
cas_mutation = true
generation_tracking = true
canonical_cache_keys = true
authorization_on_cache_hits = true
cache_invalidation = true
typed_error_envelopes = true

# ============================================================
# Runtime
# ============================================================

[runtime]
model = "single_process_rust"
async_runtime = "tokio"

[ runtime.boundaries ]
message = true
query = true
data_access = true
knowledge = true
inference = true
content = true
acquisition = true
platform = true
scheduler = true
telemetry = true
feedback = true

[runtime.forbidden]
python_runtime = true
node_runtime = true
shell_based_worker_architecture = true
cross_runtime_rpc = true
language_specific_sidecar = true

# ============================================================
# Workspace
# ============================================================

[workspace]
resolver = "2"

[workspace.crates]
core = "theligi-core"
schema = "theligi-schema"
context = "theligi-context"

message = "theligi-message"
query = "theligi-query"
runtime = "theligi-runtime"

data_access = "theligi-data-access"
repository = "theligi-repository"
cache = "theligi-cache"
authorization = "theligi-authorization"
algorithm = "theligi-algorithm"
factory = "theligi-factory"

knowledge = "theligi-knowledge"
graph = "theligi-graph"
sparql = "theligi-sparql"

topic = "theligi-topic"
inference = "theligi-inference"

content = "theligi-content"
validation = "theligi-validation"
evidence = "theligi-evidence"

acquisition = "theligi-acquisition"
http = "theligi-http"
browser = "theligi-browser"
document = "theligi-document"

platform = "theligi-platform"
linkedin = "theligi-linkedin"
x = "theligi-x"
instagram = "theligi-instagram"

scheduler = "theligi-scheduler"

telemetry = "theligi-telemetry"
feedback = "theligi-feedback"

artifact = "theligi-artifact"
security = "theligi-security"
observability = "theligi-observability"

server = "theligi-server"
cli = "theligi-cli"
desktop = "theligi-desktop"

# ============================================================
# Core semantic model
# ============================================================

[semantic]
owner = "theligi-core"

[semantic.entities]
Identity = true
User = true
Account = true
Platform = true
Resource = true
Topic = true
Concept = true
Audience = true
ContentArtifact = true
ContentSeries = true
Claim = true
Evidence = true
Interaction = true
Signal = true
Campaign = true
CommercialOutcome = true
Source = true
Document = true
ModelVersion = true
Prediction = true
GraphNode = true
GraphEdge = true

[semantic.metadata]
schema_version = true
resource_id = true
created_at = true
updated_at = true
source = true
correlation_id = true
causation_id = true
lineage = true

# ============================================================
# Message architecture
# ============================================================

[message]
owner = "theligi-message"
transport_independent = true
typed_payload = true
schema_versioned = true
correlation_supported = true
causation_supported = true
idempotency_supported = true

[message.operations]
observe = true
query = true
generate = true
schedule = true
publish = true
measure = true
adapt = true

[message.lineage]
source_required = true
timestamp_required = true
producer_required = true
schema_version_required = true

# ============================================================
# Query architecture
# ============================================================

[query]
owner = "theligi-query"
transport_independent = true
side_effect_free = true
typed = true
cache_aware = true
authorization_aware = true

[query.pipeline]
parse = true
validate = true
authorize = true
cache_lookup = true
repository_lookup = true
algorithm = true
response = true
cache_population = true

# ============================================================
# theDAF data access architecture
# ============================================================

[data_access]
owner = "theligi-data-access"

[data_access.contracts]
Repository = true
Cache = true
Algorithm = true
Authorizer = true
DataAccess = true
Factory = true

[data_access.repository]
create = true
get = true
save = true
delete = true
try_update = true
try_delete = true

[data_access.repository.consistency]
compare_and_swap = true
generation_tracking = true
toctou_protection = true
mutation_result_typed = true

[data_access.cache]
get = true
set = true
delete = true
delete_prefix = true
clear = true

[data_access.cache.key]
canonicalization = "canonical_json"
hash = "sha256"
resource_identity = true
query_identity = true
filter_identity = true
algorithm_identity = true
user_identity_when_required = true

[data_access.authorization]
fail_closed = true
authorization_before_repository = true
authorization_before_mutation = true
authorization_on_cache_hit = true
authorization_context_explicit = true

[data_access.coherence]
mutation_invalidates_cache = true
generation_must_match = true
stale_entries_rejected = true
cache_context_isolated = true

[data_access.execution]
single_repository_read_per_cache_miss = true
algorithm_after_repository = true
cache_population_after_success = true
typed_errors = true

# ============================================================
# Cache hierarchy
# ============================================================

[cache]
hierarchical = true
authority = "authoritative_store"

[cache.tiers]
L0 = "request_local"
L1 = "lru"
L2 = "moka"
L3 = "redis"
L4 = "sled"
L5 = "authoritative"

[cache.L1]
implementation = "lru"
purpose = "ultra_hot_request_reuse"
scope = "request_or_local"

[cache.L2]
implementation = "moka"
purpose = "concurrent_process_cache"
scope = "process"

[cache.L3]
implementation = "redis"
purpose = "distributed_shared_cache"
scope = "distributed"

[cache.L4]
implementation = "sled"
purpose = "durable_local_materialized_state"
scope = "node"

[cache.invariants]
canonical_keys = true
generation_validation = true
prefix_invalidation = true
typed_values = true
serialization_versioned = true
user_context_isolated = true
authorization_on_hit = true

# ============================================================
# Knowledge architecture
# ============================================================

[knowledge]
owner = "theligi-knowledge"

[knowledge.backends]
oxigraph = true
helixdb = true

[knowledge.oxigraph]
role = "rdf_semantic_query"
authority = "semantic_graph"
query_language = "sparql"

[knowledge.helixdb]
role = "operational_graph_vector_intelligence"
authority = "operational_knowledge"

[knowledge.consistency]
dual_store = true
explicit_projection = true
projection_versioned = true
provenance_required = true
no_implicit_cross_store_mutation = true

[knowledge.graph]
nodes = [
    "identity",
    "account",
    "platform",
    "topic",
    "concept",
    "technology",
    "audience",
    "content",
    "interaction",
    "signal",
    "campaign",
    "commercial_outcome",
    "source",
    "evidence",
    "model"
]

[knowledge.relationships]
related_to = true
derived_from = true
similar_to = true
supports = true
contradicts = true
published_on = true
targeted_at = true
generated = true
received = true
performed = true
influenced = true
caused = true
associated_with = true
supported_by = true

# ============================================================
# Acquisition architecture
# ============================================================

[acquisition]
owner = "theligi-acquisition"
rust_native = true
observable = true
rate_limited = true
provenance_first = true

[acquisition.http]
implementation = "reqwest"

[acquisition.html]
parser = "scraper"
standards_parser = "html5ever"
streaming_processor = "lol_html"

[acquisition.browser]
webdriver = true
implementation_candidates = [
    "thirtyfour",
    "fantoccini"
]

[acquisition.browser.abstraction]
trait = "BrowserProvider"
provider_hidden = true
platform_semantics_hidden = true

[acquisition.document]
trait = "DocumentParser"
normalized_output = true
selector_based = true
text_extraction = true
link_extraction = true
metadata_extraction = true

[acquisition.provenance]
url = true
retrieved_at = true
content_hash = true
acquisition_method = true
parser_version = true
extraction_rule_version = true
source_artifact = true

[acquisition.output]
message = true
repository = true
knowledge_graph = true
direct_backend_access_forbidden = true

# ============================================================
# Topic intelligence
# ============================================================

[topic]
seed = "keyword_dictionary"
annual_target = 52
weekly_target = 1

[topic.discovery]
sparql = true
graph_traversal = true
semantic_similarity = true
coverage_analysis = true
historical_performance = true
emerging_relationships = true
duplicate_detection = true
saturation_detection = true

[topic.ranking]
novelty = true
relevance = true
commercial_value = true
audience_affinity = true
platform_affinity = true
evidence_availability = true
historical_performance = true
saturation_penalty = true
duplication_penalty = true

# ============================================================
# Content
# ============================================================

[content]
posts_per_series = 6
weekly_series = 1
annual_series = 52
annual_posts = 312

[content.sequence]
order = [
    "hot",
    "impact",
    "explanation",
    "problem",
    "mechanism",
    "evidence"
]

[content.invariants]
shared_topic = true
shared_thesis = true
causal_consistency = true
no_topic_drift = true
evidence_required = true
lineage_required = true
artifact_hash = true

# ============================================================
# Inference
# ============================================================

[inference]
owner = "theligi-inference"
type = "gnn"

[inference.tasks]
node_embedding = true
link_prediction = true
topic_ranking = true
audience_affinity = true
platform_affinity = true
content_affinity = true
performance_prediction = true
emerging_topic_detection = true

[inference.provenance]
model_version = true
dataset_version = true
feature_schema_version = true
confidence = true
trace = true

[inference.authority]
recommendation_only = true
automatic_publish = false

# ============================================================
# Platform adapters
# ============================================================

[platform]
adapter_architecture = true
core_platform_agnostic = true

[platform.contract]
authenticate = true
resolve_identity = true
publish = true
schedule = true
delete = true
fetch_content = true
fetch_interactions = true
fetch_metrics = true

[platform.linkedin]
enabled = true

[platform.x]
enabled = true

[platform.instagram]
enabled = true

[platform.future]
dynamic_registration = true

# ============================================================
# Authentication
# ============================================================

[authentication]
implementation = "betterauth-rs"
identity_first = true
session_based = true
authorization_separate = true

[authentication.security]
credentials_never_graph_persisted = true
tokens_never_graph_persisted = true
least_privilege = true
audit_log = true

# ============================================================
# Scheduler
# ============================================================

[scheduler]
async = true
timezone_aware = true
idempotent = true
retryable = true
dead_letter = true
human_approval = true

[scheduler.content]
posts_per_week = 6
rest_day = "sunday"

# ============================================================
# Telemetry
# ============================================================

[telemetry]
first_class = true
immutable_observations = true
schema_versioned = true
lineage_required = true

[telemetry.metrics]
impressions = true
engagement = true
comments = true
shares = true
saves = true
followers = true
profile_views = true
clicks = true
conversions = true

# ============================================================
# Feedback
# ============================================================

[feedback]
closed_loop = true

[feedback.pipeline]
observe = true
normalize = true
persist = true
query = true
infer = true
rank = true
generate = true
deploy = true
measure = true
adapt = true

[feedback.adaptation]
topic_scores = true
platform_affinity = true
audience_affinity = true
content_affinity = true
topic_saturation = true
emerging_topics = true

[feedback.history]
observations_immutable = true
policy_versioned = true
reproducible = true

# ============================================================
# Artifact/version model
# ============================================================

[artifact]
versioned = true
hashed = true
schema_versioned = true
lineage_required = true
rollback_supported = true

[artifact.hash]
algorithm = "blake3"

# ============================================================
# Security
# ============================================================

[security]
fail_closed = true
least_privilege = true
credential_isolation = true
audit_logging = true
authorization_required = true

[security.forbidden]
credential_in_graph = true
token_in_graph = true
secret_in_logs = true
unauthorized_cache_access = true

# ============================================================
# Observability
# ============================================================

[observability]
structured_logging = true
metrics = true
tracing = true
health_checks = true
correlation_ids = true
causation_ids = true
decision_trace = true
content_lineage = true
acquisition_lineage = true
prediction_trace = true

# ============================================================
# Development
# ============================================================

[development]
spec_first = true
tests_first_for_contracts = true
adr_for_architecture = true
public_api_documentation = true
dependency_audit = true
license_audit = true
clippy = true
fmt = true
integration_tests = true
contract_tests = true
property_tests = true
end_to_end_tests = true

[development.rules]
inspect_before_edit = true
preserve_existing_contracts = true
minimal_change = true
no_speculative_abstraction = true
no_dependency_reimplementation = true
no_cross_runtime = true


---

2. specs/SUBSYSTEM.toml

This is the composition contract. Individual subsystem TOMLs can then refine each section without contradicting this.

# ============================================================
# theLiGI Subsystem Composition Specification
# ============================================================

[spec]
name = "theLiGI-subsystems"
version = "0.1.0"
parent = "../SPEC.toml"

[authority]
parent_spec = true
subsystem_specs = true
implementation = false

# ============================================================
# Core
# ============================================================

[core]
crate = "theligi-core"
owns = [
    "domain_types",
    "resource_identity",
    "error_types",
    "lineage",
    "correlation",
    "causation",
    "schema_version"
]

forbidden = [
    "database_logic",
    "cache_logic",
    "platform_logic",
    "browser_logic",
    "provider_logic"
]

# ============================================================
# Context
# ============================================================

[context]
crate = "theligi-context"

owns = [
    "request_context",
    "identity_context",
    "authorization_context",
    "correlation_context",
    "execution_context"
]

required = [
    "user_id",
    "correlation_id",
    "schema_version"
]

# ============================================================
# Message
# ============================================================

[message]
crate = "theligi-message"
based_on = "theMQL"

owns = [
    "message",
    "event",
    "command",
    "notification",
    "message_envelope"
]

requires = [
    "schema_version",
    "message_id",
    "timestamp",
    "producer",
    "correlation_id"
]

supports = [
    "causation_id",
    "idempotency_key",
    "lineage"
]

# ============================================================
# Query
# ============================================================

[query]
crate = "theligi-query"
based_on = "theMQL"

owns = [
    "query",
    "query_parser",
    "query_validator",
    "query_execution"
]

must_integrate_with = [
    "data_access",
    "authorization",
    "cache",
    "algorithm"
]

# ============================================================
# Data Access
# ============================================================

[data_access]
crate = "theligi-data-access"
based_on = "theDAF"

contracts = [
    "Repository",
    "Cache",
    "Algorithm",
    "Authorizer",
    "DataAccess",
    "Factory"
]

execution_order = [
    "validate",
    "authorize",
    "cache_lookup",
    "repository_lookup",
    "algorithm",
    "cache_population"
]

mutation_order = [
    "validate",
    "authorize",
    "generation_check",
    "repository_mutation",
    "generation_increment",
    "cache_invalidation"
]

# ============================================================
# Repository
# ============================================================

[repository]
crate = "theligi-repository"
based_on = "theDAF"

operations = [
    "create",
    "get",
    "save",
    "delete",
    "try_update",
    "try_delete"
]

invariants = [
    "CAS",
    "generation_tracking",
    "typed_errors",
    "no_hidden_mutation"
]

# ============================================================
# Cache
# ============================================================

[cache]
crate = "theligi-cache"
based_on = "theDAF"

tiers = [
    "request_local",
    "LRU",
    "Moka",
    "Redis",
    "Sled"
]

rules = [
    "canonical_keys",
    "generation_validation",
    "prefix_invalidation",
    "authorization_on_hit",
    "user_context_isolation"
]

# ============================================================
# Authorization
# ============================================================

[authorization]
crate = "theligi-authorization"

authentication_provider = "BetterAuth.rs"

rules = [
    "fail_closed",
    "authorize_before_mutation",
    "authorize_cache_hits",
    "explicit_context"
]

# ============================================================
# Algorithm
# ============================================================

[algorithm]
crate = "theligi-algorithm"
based_on = "theDAF"

contract = "typed_algorithm"

implementations = [
    "topic_ranker",
    "topic_generator",
    "similarity_detector",
    "saturation_detector",
    "audience_affinity",
    "platform_affinity",
    "evidence_scorer",
    "gnn_inference"
]

# ============================================================
# Factory
# ============================================================

[factory]
crate = "theligi-factory"
based_on = "theDAF"

purpose = "Compose Repository Cache Algorithm Authorizer DataAccess instances."

requirements = [
    "typed_composition",
    "dependency_injection",
    "configuration_validation",
    "no_global_mutable_state"
]

# ============================================================
# Knowledge
# ============================================================

[knowledge]
crate = "theligi-knowledge"

stores = [
    "Oxigraph",
    "HelixDB"
]

[knowledge.oxigraph]
purpose = "RDF/SPARQL semantic graph"

[knowledge.helixdb]
purpose = "operational graph/vector intelligence"

[knowledge.boundary]
cross_store_projection_explicit = true
projection_versioned = true
provenance_required = true

# ============================================================
# Acquisition
# ============================================================

[acquisition]
crate = "theligi-acquisition"

methods = [
    "http",
    "browser",
    "document"
]

[acquisition.http]
crate = "theligi-http"
implementation = "reqwest"

[acquisition.browser]
crate = "theligi-browser"
webdriver = true
candidate_providers = [
    "thirtyfour",
    "fantoccini"
]

[acquisition.document]
crate = "theligi-document"

parsing = [
    "scraper",
    "html5ever",
    "lol_html"
]

[acquisition.provenance]
required = [
    "url",
    "retrieved_at",
    "content_hash",
    "acquisition_method",
    "parser_version",
    "extraction_rule_version"
]

[acquisition.output]
must_emit_message = true
direct_database_mutation = false

# ============================================================
# Topic
# ============================================================

[topic]
crate = "theligi-topic"

seed = "keyword_dictionary"

operations = [
    "discover",
    "rank",
    "deduplicate",
    "detect_saturation",
    "detect_gaps",
    "approve",
    "schedule"
]

[topic.query]
sparql = true

[topic.inference]
gnn = true

# ============================================================
# Inference
# ============================================================

[inference]
crate = "theligi-inference"

model = "GNN"

requires = [
    "versioned_dataset",
    "versioned_features",
    "versioned_model",
    "confidence",
    "trace"
]

authority = "recommendation"

# ============================================================
# Content
# ============================================================

[content]
crate = "theligi-content"

transformations = [
    "hot",
    "impact",
    "explanation",
    "problem",
    "mechanism",
    "evidence"
]

invariants = [
    "shared_topic",
    "shared_thesis",
    "causal_consistency",
    "no_topic_drift",
    "evidence_required",
    "lineage_required"
]

# ============================================================
# Validation
# ============================================================

[validation]
crate = "theligi-validation"

checks = [
    "schema",
    "claims",
    "evidence",
    "lineage",
    "platform_constraints",
    "duplicate_content",
    "topic_consistency"
]

# ============================================================
# Platforms
# ============================================================

[platform]
crate = "theligi-platform"

contract = [
    "authenticate",
    "resolve_identity",
    "publish",
    "schedule",
    "delete",
    "fetch_content",
    "fetch_interactions",
    "fetch_metrics"
]

implementations = [
    "LinkedIn",
    "X",
    "Instagram"
]

platform_logic_must_not_escape_adapter = true

# ============================================================
# Scheduler
# ============================================================

[scheduler]
crate = "theligi-scheduler"

requires = [
    "idempotency",
    "timezone",
    "retry",
    "dead_letter",
    "human_approval"
]

# ============================================================
# Telemetry
# ============================================================

[telemetry]
crate = "theligi-telemetry"

first_class = true
immutable = true
versioned = true

metrics = [
    "impressions",
    "engagement",
    "comments",
    "shares",
    "saves",
    "followers",
    "profile_views",
    "clicks",
    "conversions"
]

# ============================================================
# Feedback
# ============================================================

[feedback]
crate = "theligi-feedback"

pipeline = [
    "observe",
    "normalize",
    "persist",
    "query",
    "infer",
    "rank",
    "generate",
    "deploy",
    "measure",
    "adapt"
]

updates = [
    "topic_rankings",
    "topic_saturation",
    "audience_affinity",
    "platform_affinity",
    "content_affinity",
    "emerging_topics"
]

# ============================================================
# Artifact
# ============================================================

[artifact]
crate = "theligi-artifact"

requirements = [
    "version",
    "schema",
    "hash",
    "lineage",
    "rollback"
]

hash = "blake3"

# ============================================================
# Security
# ============================================================

[security]
crate = "theligi-security"

requirements = [
    "least_privilege",
    "fail_closed",
    "credential_isolation",
    "audit_logging",
    "secret_manager"
]

forbidden = [
    "credentials_in_graph",
    "tokens_in_graph",
    "secrets_in_logs"
]

# ============================================================
# Observability
# ============================================================

[observability]
crate = "theligi-observability"

requirements = [
    "structured_logs",
    "metrics",
    "tracing",
    "health",
    "correlation",
    "causation",
    "decision_trace"
]

# ============================================================
# Dependency boundary
# ============================================================

[dependency_boundary]

allowed_flow = [
    "platform -> message",
    "acquisition -> message",
    "message -> runtime",
    "query -> data_access",
    "data_access -> repository",
    "data_access -> cache",
    "data_access -> authorization",
    "data_access -> algorithm",
    "knowledge -> repository",
    "inference -> algorithm",
    "telemetry -> message",
    "feedback -> knowledge",
    "content -> artifact",
    "scheduler -> platform"
]

forbidden_flow = [
    "platform -> helixdb",
    "platform -> redis",
    "content -> redis",
    "inference -> redis",
    "acquisition -> helixdb",
    "acquisition -> oxigraph",
    "query -> helixdb_direct",
    "query -> redis_direct",
    "adapter -> repository_implementation",
    "adapter -> cache_implementation"
]

# ============================================================
# Cross-cutting invariants
# ============================================================

[invariants]

no_cross_runtime = true

no_credentials_in_graph = true

no_unauthorized_cache_hit = true

no_mutation_without_cas = true

no_stale_generation = true

no_direct_backend_access = true

no_unversioned_model_prediction = true

no_untracked_content = true

no_unprovenanced_external_data = true

no_publish_without_validation = true

no_evidence_without_source = true

no_platform_logic_in_core = true

no_silent_data_loss = true

no_silent_spec_override = true


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