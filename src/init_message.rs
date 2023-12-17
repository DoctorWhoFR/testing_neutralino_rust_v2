use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitMessageData {
    pub state: i32,
    pub message: String
}