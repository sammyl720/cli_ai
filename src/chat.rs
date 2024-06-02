use serde::{Deserialize, Serialize};

use crate::message::{Message, Role};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub messages: Vec<Message>,
    pub model: String,
}

impl Chat {
    pub fn new(model: String) -> Self {
        Chat {
            messages: Vec::new(),
            model,
        }
    }

    pub fn add_user_message(&mut self, text: String) {
        let message = Message::new(Role::User, text);
        self.messages.push(message);
    }

    pub fn add_assistant_message(&mut self, text: String) {
        let message = Message::new(Role::Assistant, text);
        self.messages.push(message);
    }

    pub fn add_system_message(&mut self, text: String) {
        let message = Message::new(Role::System, text);
        self.messages.push(message);
    }
}
