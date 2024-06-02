use chat::Chat;
use open_ai::OpenAI;
use std::env;
use ui::UI;

const OPENAI_API_KEY: &str = "OPENAI_API_KEY";
const MODEL: &str = "gpt-3.5-turbo";
mod chat;
mod message;
mod open_ai;
mod request;
mod response;
mod ui;

#[tokio::main]
async fn main() {
    let args = env::args();
    let initial_prompt = args.skip(1).collect::<Vec<String>>().join(" ");

    let prompt = get_fallback(&initial_prompt);

    let key = env::var(OPENAI_API_KEY).expect("Could not find 'OPENAI_API_KEY ' env variable");

    match prompt {
        Some(message) => match OpenAI::new(&key) {
            Ok(ai) => {
                let mut chat = Chat::new(MODEL.to_string());
                chat.add_user_message(message.into());

                let response = ai.complete(&chat).await;
                match response {
                    Ok(result) => ui::UI::display_message(&result.choices.first().unwrap().message),
                    Err(err) => eprintln!("{:?}", err),
                }
            }
            Err(err) => eprintln!("{:?}", err),
        },
        _ => println!("By bye"),
    }
}

/// Get fallback message from user if there is no initial message
fn get_fallback(initial: &str) -> Option<String> {
    if initial.trim().is_empty() {
        return UI::prompt("Asssistant: How can i help you today?");
    }
    println!("");
    return Some(initial.into());
}
