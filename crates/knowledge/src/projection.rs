//! Dual-store projection logic for theligi-knowledge.
use crate::knowledge::{KnowledgeError, KnowledgeId, ProjectedFact, KnowledgeStore, Result};

/// Composed projection that fans-out writes and fans-in reads.
pub struct DualStoreProjection<O, H>
where
    O: KnowledgeStore,
    H: KnowledgeStore,
{
    pub oxigraph: O,
    pub helixdb: H,
}

impl<O, H> DualStoreProjection<O, H>
where
    O: KnowledgeStore,
    H: KnowledgeStore,
{
    pub fn new(oxigraph: O, helixdb: H) -> Self {
        Self { oxigraph, helixdb }
    }

    pub async fn put_fact(&self, fact: &ProjectedFact) -> Result<()> {
        self.oxigraph.put_fact(fact).await?;
        self.helixdb.put_fact(fact).await?;
        Ok(())
    }

    pub async fn get_fact(&self, id: &KnowledgeId) -> Result<Option<ProjectedFact>> {
        self.oxigraph.get_fact(id).await.or_else(|_| self.helixdb.get_fact(id).await)
    }
}
