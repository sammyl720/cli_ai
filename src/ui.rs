use std::io::{self, Write};

use crate::message::Message;

pub struct UI;

impl UI {
    pub fn prompt_user() -> Option<String> {
        println!("");
        print!("User: ");
        if let Err(_) = io::stdout().flush() {
            return None;
        }

        let mut buffer = String::new();
        if let Ok(_) = io::stdin().read_line(&mut buffer) {
            if UI::is_exit_request(&buffer) {
                return None;
            }
            println!("");
            return Some(buffer);
        }

        None
    }

    pub fn prompt(prompt: &str) -> Option<String> {
        println!("{}", prompt);
        UI::prompt_user()
    }

    pub fn display_message(message: &Message) {
        println!("{}", message);
    }

    fn is_exit_request(message: &str) -> bool {
        let trimmed = message.trim();

        let lower = trimmed.to_lowercase();

        return lower.is_empty() || ["exit", "q", "quit"].contains(&&lower[..]);
    }
}
