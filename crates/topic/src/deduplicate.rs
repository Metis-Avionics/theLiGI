//! Topic deduplication logic for theligi-topic.
use crate::{Topic, TopicResult};

pub async fn deduplicate(_topics: &mut [Topic]) -> TopicResult<usize> {
    Ok(0)
}
