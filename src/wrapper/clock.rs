use chrono::{DateTime, Utc};
pub struct Clock {}

impl Clock {
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
}
