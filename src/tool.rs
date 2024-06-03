use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::function_handler::CREATE_README_FILE;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub r#type: String,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: FunctionParameters,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    pub r#type: String,
    pub properties: HashMap<String, ParameterProperty>,
    pub required: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterProperty {
    #[serde(rename = "type")]
    pub r#type: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ToolCall {
    pub id: String,

    #[serde(rename = "type")]
    pub r#type: String,
    pub function: FunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

impl Function {
    pub fn new(name: String, description: String, parameters: FunctionParameters) -> Self {
        Self {
            name,
            description,
            parameters,
        }
    }
}

impl FunctionParameters {
    pub fn new(properties: HashMap<String, ParameterProperty>, required: Vec<String>) -> Self {
        Self {
            r#type: String::from("object"),
            properties,
            required,
        }
    }
}

impl ParameterProperty {
    pub fn new(r#type: String, description: String) -> Self {
        Self {
            r#type,
            description,
        }
    }
}

pub enum RegisteredTools {
    CreateReadMeFile,
}

impl RegisteredTools {
    pub fn from(name: &String) -> Option<RegisteredTools> {
        if name == CREATE_README_FILE {
            return Some(RegisteredTools::CreateReadMeFile);
        }

        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateFileArgs {
    pub content: String,
    pub file_name: String,
}
