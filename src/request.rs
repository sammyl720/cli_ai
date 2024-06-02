use crate::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub messages: Vec<Message>,
    pub model: String,
}
