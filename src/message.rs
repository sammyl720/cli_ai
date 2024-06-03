use core::fmt;
use serde::{Deserialize, Serialize};

use crate::tool::ToolCall;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Message {
    pub role: Role,
    pub content: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,

    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[default]
    System,
    User,
    Assistant,
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Message {
            role,
            content: Some(content),
            ..Default::default()
        }
    }

    pub fn is_function_call(&self) -> bool {
        self.tool_calls.is_some()
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role = match self {
            Role::Assistant => "assistant",
            Role::System => "system",
            Role::User => "user",
        };
        write!(f, "{}", role)
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_function_call() {
            let function_calls = &self.tool_calls.as_ref();
            let first_call = function_calls.unwrap().first().unwrap();
            return write!(f, "function {}()", first_call.function.name);
        }
        write!(
            f,
            "{}: {}",
            self.role,
            self.content
                .as_ref()
                .expect("No function message should have content")
        )
    }
}
