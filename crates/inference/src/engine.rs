//! GNN inference engine stub for theligi-inference.
use crate::{
    AffinityScores, InferenceEngine, InferenceInput, InferenceResult, LinkPrediction,
    NodeEmbedding, PerformancePrediction, TopicRanking,
};

pub struct InferenceEngineImpl {
    _private: (),
}

impl Default for InferenceEngineImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl InferenceEngineImpl {
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }
}

#[async_trait::async_trait]
impl InferenceEngine for InferenceEngineImpl {
    async fn node_embedding(&self, _input: &InferenceInput) -> InferenceResult<Vec<NodeEmbedding>> {
        Ok(Vec::new())
    }

    async fn link_prediction(
        &self,
        _input: &InferenceInput,
    ) -> InferenceResult<Vec<LinkPrediction>> {
        Ok(Vec::new())
    }

    async fn topic_ranking(&self, _input: &InferenceInput) -> InferenceResult<Vec<TopicRanking>> {
        Ok(Vec::new())
    }

    async fn audience_affinity(
        &self,
        _input: &InferenceInput,
    ) -> InferenceResult<Vec<AffinityScores>> {
        Ok(Vec::new())
    }

    async fn platform_affinity(
        &self,
        _input: &InferenceInput,
    ) -> InferenceResult<Vec<AffinityScores>> {
        Ok(Vec::new())
    }

    async fn content_affinity(
        &self,
        _input: &InferenceInput,
    ) -> InferenceResult<Vec<AffinityScores>> {
        Ok(Vec::new())
    }

    async fn performance_prediction(
        &self,
        _input: &InferenceInput,
    ) -> InferenceResult<Vec<PerformancePrediction>> {
        Ok(Vec::new())
    }
}
