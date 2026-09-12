//! Topic gap detection logic for theligi-topic.
use crate::{ContentArtifactRef, Topic, TopicGap, TopicResult};

pub async fn detect_gaps(
    _topics: &[Topic],
    _artifacts: &[ContentArtifactRef],
) -> TopicResult<Vec<TopicGap>> {
    Ok(Vec::new())
}
