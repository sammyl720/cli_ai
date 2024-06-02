use crate::chat::Chat;
use crate::response::Response;
use reqwest::{
    header::{self, HeaderValue, AUTHORIZATION},
    Client,
};

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct OpenAI {
    client: reqwest::Client,
}

impl OpenAI {
    pub fn new(api_key: &str) -> Result<OpenAI, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );

        let client = Client::builder().default_headers(headers).build()?;

        Ok(OpenAI { client })
    }

    pub async fn complete(&self, chat: &Chat) -> Result<Response, reqwest::Error> {
        let response: Response = self
            .client
            .post(OPENAI_URL)
            .json(chat)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}
