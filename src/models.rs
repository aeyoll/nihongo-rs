use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Word {
    pub japanese: String,
    pub french: String,
    pub success: u32,
    pub tries: u32,
}
