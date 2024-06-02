use chat::Chat;
use open_ai::OpenAI;
use std::env;
use ui::UI;

mod chat;
mod message;
mod open_ai;
mod request;
mod response;
mod ui;

const OPENAI_API_KEY: &str = "OPENAI_API_KEY";
const MODEL: &str = "gpt-3.5-turbo";

const SYSTEM_INSTRUCTIONS: &str = "
You are an easy-going and intelligent AI Assistant that strives to give succinct and clarifying responses. You are also a skilled software engineer with extensive knowledge in various aspects of the field, including computer science, design architecture, and operating systems, to name a few. You have the unique ability to explain complex concepts in a way that a less knowledgeable person can understand.

You will be interacting with the user through a command line interface, so avoid using markdown text. Ensure that all responses are optimized and safe for CLI output. If the user ever needs help exiting the CLI program, they can press 'Enter', or type 'q', 'quit', or 'exit'.
";

#[tokio::main]
async fn main() {
    let args = env::args();
    let initial_prompt = args.skip(1).collect::<Vec<String>>().join(" ");

    let mut prompt = get_fallback(&initial_prompt);

    let key = env::var(OPENAI_API_KEY).expect("Could not find 'OPENAI_API_KEY ' env variable");
    let mut chat = Chat::new(MODEL.to_string());

    chat.add_system_message(SYSTEM_INSTRUCTIONS.to_string());

    while let Some(message) = &prompt {
        match OpenAI::new(&key) {
            Ok(ai) => {
                chat.add_user_message(message.clone().into());

                let response = ai.complete(&chat).await;
                match response {
                    Ok(result) => {
                        if let Some(choice) = result.choices.first() {
                            ui::UI::display_message(&choice.message);
                            chat.add_assistant_message(choice.message.content.to_string());
                            prompt = UI::prompt_user();
                        } else {
                            println!("No response..");
                            prompt = None;
                        }
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                        prompt = None;
                    }
                }
            }
            Err(err) => {
                eprintln!("{:?}", err);
                prompt = None;
            }
        }
    }
    println!("");
    println!("Goodbye!");
}

/// Get fallback message from user if there is no initial message
fn get_fallback(initial: &str) -> Option<String> {
    if initial.trim().is_empty() {
        return UI::prompt("Asssistant: How can i help you today?");
    }
    println!("");
    return Some(initial.into());
}
