use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    #[serde(rename = "choices")]
    pub choices: Vec<Choice>,
}
