use crate::chat::Chat;
use crate::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub messages: Vec<Message>,
    pub model: String,
}

impl Request {
    fn from_chat(value: &Chat) -> Self {
        Request {
            messages: value.messages.to_owned(),
            model: value.model.clone(),
        }
    }
}
