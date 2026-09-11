//! Topic ranking logic for theligi-topic.
use crate::{Topic, TopicResult};

pub async fn rank(_topics: &[Topic]) -> TopicResult<Vec<Topic>> {
    let mut sorted = _topics.to_vec();
    sorted.sort_by(|a, b| b.relevance_score.total_cmp(&a.relevance_score));
    Ok(sorted)
}
