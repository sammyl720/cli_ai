use crate::chat::Chat;
use crate::response::Response;
use reqwest::{
    header::{self, HeaderValue, AUTHORIZATION},
    Client, Error,
};

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct OpenAI {
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum RequestError {
    HttpError(Error),
    ParseError(Error),
    JsonError(serde_json::Error),
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

    pub async fn complete(&self, chat: &Chat) -> Result<Response, RequestError> {
        let response = self
            .client
            .post(OPENAI_URL)
            .json(chat)
            .send()
            .await
            .map_err(|err| RequestError::HttpError(err))?;

        let text = response
            .text()
            .await
            .map_err(|err| RequestError::ParseError(err))?;

        let result: Response =
            serde_json::from_str(&text).map_err(|err| RequestError::JsonError(err))?;

        Ok(result)
    }
}
