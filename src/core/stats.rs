use chrono::{DateTime, Utc};
use std::time::SystemTime;

use crate::core::typing_errors::TypingErrors;
use crate::core::typing_speed::TypingSpeed;

pub struct TrainingStatistics {
    pub errors: TypingErrors,
    pub typing_speed: TypingSpeed,
}

pub struct TrainingRecord {
    pub timestamp: DateTime<Utc>,
    pub stats: TrainingStatistics,
}
