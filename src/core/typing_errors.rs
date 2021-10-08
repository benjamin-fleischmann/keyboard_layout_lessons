use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TypingErrors {
    pub total_error_count: u16,
}
