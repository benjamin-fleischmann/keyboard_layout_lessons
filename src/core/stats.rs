use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::typing_errors::TypingErrors;
use crate::core::typing_speed::TypingSpeed;

#[derive(Serialize, Deserialize)]
pub struct TrainingStatistics {
    pub errors: TypingErrors,
    pub typing_speed: TypingSpeed,
}
#[derive(Serialize, Deserialize)]
pub struct TrainingRecord {
    pub timestamp: DateTime<Utc>,
    pub stats: TrainingStatistics,
}
