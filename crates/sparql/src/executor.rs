//! Deterministic SPARQL executor stub for theligi-sparql.
use crate::{SparqlQuery, SparqlResult, SparqlResultType, SparqlSolution};

#[async_trait::async_trait]
pub trait SparqlExecutor: Send + Sync {
    async fn execute(&self, query: &SparqlQuery) -> SparqlResultType<SparqlResult>;
}

/// Stub executor that echoes the query text as a single solution row.
pub struct StubSparqlExecutor {
    _private: (),
}

impl StubSparqlExecutor {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

#[async_trait::async_trait]
impl SparqlExecutor for StubSparqlExecutor {
    async fn execute(&self, query: &SparqlQuery) -> SparqlResultType<SparqlResult> {
        let solution = SparqlSolution {
            values: {
                let mut map = std::collections::HashMap::new();
                map.insert("query".to_string(), serde_json::json!(query.text));
                map
            },
        };
        Ok(SparqlResult {
            solutions: vec![solution],
        })
    }
}
