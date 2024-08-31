use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Card {
    pub japanese: String,
    pub french: String,
    pub box_number: usize,
    pub next_review: DateTime<Utc>,
    pub theme: String,
}
