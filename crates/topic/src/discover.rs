//! Topic discovery logic for theligi-topic.
use crate::{ContentArtifactRef, DiscoveryConfig, DiscoveryResult, TopicResult};

pub async fn discover(
    _artifacts: &[ContentArtifactRef],
    _cfg: &DiscoveryConfig,
) -> TopicResult<DiscoveryResult> {
    Ok(DiscoveryResult::default())
}
