use std::{collections::HashMap, io, time};

use chrono::Local;

use crate::{
    file,
    message::Message,
    tool::{
        CreateFileArgs, Function, FunctionParameters, ParameterProperty, RegisteredTools, ToolCall,
    },
};

pub const CREATE_README_FILE: &str = "create_readme_file";
pub const GET_TIME: &str = "get_time";

pub struct FunctionHandler {
    pub registered_functions: Vec<Function>,
}

#[derive(Debug)]
pub enum FunctionHandleError {
    NonToolCallMessage,
    NotRegisteredFunction(String),
    IOError(io::Error),
    SerdeError(serde_json::Error),
}

impl FunctionHandler {
    pub fn new() -> Self {
        Self {
            registered_functions: vec![create_save_to_file_tool(), create_get_current_time_tool()],
        }
    }

    pub fn from_message(message: &Message) -> Result<String, FunctionHandleError> {
        if !message.is_function_call() {
            return Err(FunctionHandleError::NonToolCallMessage);
        }

        if let Some(tool_calls) = &message.tool_calls {
            if let Some(tool_call) = tool_calls.first() {
                let message = Self::handle_call(tool_call)?;
                println!("{}", message);
                return Ok(message);
            } else {
                return Err(FunctionHandleError::NonToolCallMessage);
            }
        } else {
            return Err(FunctionHandleError::NonToolCallMessage);
        }
    }

    fn handle_call(tool_call: &ToolCall) -> Result<String, FunctionHandleError> {
        let function = &tool_call.function;
        if let Some(tool) = RegisteredTools::from(&function.name) {
            return match tool {
                RegisteredTools::CreateReadMeFile => Self::create_readme_file(&function.arguments),
                RegisteredTools::GetTime => Ok(Self::get_time()),
            };
        } else {
            return Err(FunctionHandleError::NotRegisteredFunction(String::from(
                &function.name,
            )));
        }
    }

    fn create_readme_file(arguments: &str) -> Result<String, FunctionHandleError> {
        let args: CreateFileArgs =
            serde_json::from_str(arguments).map_err(|err| FunctionHandleError::SerdeError(err))?;

        file::write_readme(&args.file_name, &args.content)
            .map_err(|err| FunctionHandleError::IOError(err))?;
        Ok(format!(""))
    }

    fn get_time() -> String {
        let now = Local::now();
        now.to_string()
    }
}

fn create_save_to_file_tool() -> Function {
    let mut props: HashMap<String, ParameterProperty> = HashMap::new();
    let file_name = String::from("file_name");
    let content = String::from("content");
    props.insert(
        file_name.clone(),
        ParameterProperty::new(
            String::from("string"),
            String::from("The name of the readme file excluding the extension type."),
        ),
    );
    props.insert(
        content.clone(),
        ParameterProperty::new(
            String::from("string"),
            String::from("The content of the readme file."),
        ),
    );

    let required_fields = vec![file_name, content];
    let parameters = FunctionParameters::new(props, required_fields);

    let func_name = String::from(CREATE_README_FILE);
    let func_description = String::from("Create and save a readme file with the provided markdown content to the the specified file in the current working directory. The file name should be based on the content of the readme");

    Function::new(func_name, func_description, parameters)
}

fn create_get_current_time_tool() -> Function {
    let func_name = String::from(GET_TIME);
    let func_description = String::from("Get the current time");

    Function::from(func_name, func_description)
}
