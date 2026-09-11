//! Oxigraph backend stub for theligi-knowledge.
use crate::knowledge::{KnowledgeError, KnowledgeId, ProjectedFact, KnowledgeStore, Result};

pub struct OxigraphBackend {
    _private: (),
}

impl OxigraphBackend {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

#[async_trait::async_trait]
impl KnowledgeStore for OxigraphBackend {
    async fn put_fact(&self, _fact: &ProjectedFact) -> Result<()> {
        Ok(())
    }

    async fn get_fact(&self, _id: &KnowledgeId) -> Result<Option<ProjectedFact>> {
        Ok(None)
    }

    async fn query(&self, _sparql: &str) -> Result<Vec<ProjectedFact>> {
        Ok(Vec::new())
    }
}
