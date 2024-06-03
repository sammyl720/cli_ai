use serde::{Deserialize, Serialize};

use crate::{
    function_handler::FunctionHandler,
    message::{Message, Role},
    tool::Tool,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub messages: Vec<Message>,
    pub model: String,
    pub tools: Vec<Tool>,
    pub tool_choice: String,
}

impl Chat {
    pub fn new(model: String) -> Self {
        let tools = FunctionHandler::new()
            .registered_functions
            .into_iter()
            .map(|function| Tool {
                r#type: String::from("function"),
                function,
            })
            .collect();
        Chat {
            messages: Vec::new(),
            tools,
            model,
            tool_choice: String::from("auto"),
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
