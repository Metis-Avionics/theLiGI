//! # theligi-sparql
//!
//! Deterministic SPARQL query wrapper.  Serialises all
//! non-deterministic SPARQL behaviour (result ordering, `LIMIT`/`OFFSET`,
//! solution modifiers) so that callers always receive reproducible
//! results regardless of backend internals.

use serde::{Deserialize, Serialize};

/// A fully-specified, deterministic SPARQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparqlQuery {
    /// The raw SPARQL text.
    pub text: String,
    /// Deterministic ordering clause appended before LIMIT/OFFSET.
    pub order_by: Vec<SparqlOrderTerm>,
    /// Maximum number of results to return.
    pub limit: Option<u64>,
    /// Number of results to skip.
    pub offset: Option<u64>,
}

impl SparqlQuery {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn with_order_by(mut self, terms: Vec<SparqlOrderTerm>) -> Self {
        self.order_by = terms;
        self
    }

    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Produce the deterministic SPARQL text with ORDER BY injected.
    pub fn to_deterministic_string(&self) -> String {
        let mut result = self.text.clone();
        if !self.order_by.is_empty() {
            let order_clause: Vec<String> =
                self.order_by.iter().map(|t| t.to_sparql_string()).collect();
            result.push(' ');
            result.push_str(&format!("ORDER BY {} ", order_clause.join(" ")));
        }
        if let Some(limit) = self.limit {
            result.push_str(&format!("LIMIT {} ", limit));
        }
        if let Some(offset) = self.offset {
            result.push_str(&format!("OFFSET {} ", offset));
        }
        result.trim().to_string()
    }
}

/// One term in a deterministic ORDER BY clause.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SparqlOrderTerm {
    AscTerm,
    DescTerm,
    AscLang,
    DescLang,
    AscDatatype,
    DescDatatype,
}

impl SparqlOrderTerm {
    pub fn to_sparql_string(&self) -> String {
        match self {
            Self::AscTerm => "ASC(?term)".to_string(),
            Self::DescTerm => "DESC(?term)".to_string(),
            Self::AscLang => "ASC(LANG(?term))".to_string(),
            Self::DescLang => "DESC(LANG(?term))".to_string(),
            Self::AscDatatype => "ASC(DATATYPE(?term))".to_string(),
            Self::DescDatatype => "DESC(DATATYPE(?term))".to_string(),
        }
    }
}

/// A single row in a SPARQL result set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparqlSolution {
    pub values: std::collections::HashMap<String, serde_json::Value>,
}

/// Complete result of a deterministic SPARQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparqlResult {
    pub solutions: Vec<SparqlSolution>,
}

impl SparqlResult {
    pub fn is_empty(&self) -> bool {
        self.solutions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.solutions.len()
    }
}

/// Errors produced by SPARQL query execution.
#[derive(Debug, thiserror::Error)]
pub enum SparqlError {
    #[error("query parse error: {0}")]
    Parse(String),

    #[error("execution error: {0}")]
    Execution(String),

    #[error("serialization error: {0}")]
    Serialization(String),
}

pub type SparqlResultType<T> = std::result::Result<T, SparqlError>;

/// Deterministic SPARQL query executor trait.
#[async_trait::async_trait]
pub trait SparqlExecutor: Send + Sync {
    async fn execute(&self, query: &SparqlQuery) -> SparqlResultType<SparqlResult>;
}

pub mod executor;
