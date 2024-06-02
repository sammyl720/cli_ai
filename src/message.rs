use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Message { role, content }
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
        write!(f, "{}: {}", self.role, self.content)
    }
}
