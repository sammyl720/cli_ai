use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub model: String,
    #[serde(rename = "choices")]
    pub choices: Vec<Choice>,
    pub usage: Usage,
}
