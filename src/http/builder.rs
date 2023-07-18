use reqwest::header::{HeaderMap, HeaderValue};

use super::prelude::{ClientSessionType, REVOLT_API_URL, HttpClient};

#[derive(Debug)]
pub enum HttpClientBuilderError {
    MissingToken
}

#[derive(Default, Debug)]
pub struct HttpClientBuilder {
    token: Option<String>,
    api_url: String,
    client_type: ClientSessionType,
}

impl HttpClientBuilder {
    pub fn new() -> Self {
        Self {
            api_url: REVOLT_API_URL.to_string(),
            ..Default::default()
        }
    }

    pub fn session_type(mut self, session_type: ClientSessionType) -> Self {
        self.client_type = session_type;

        self
    }

    /// Sets the token to use for the session type
    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());

        self
    }

    /// sets the API url.
    pub fn with_api(mut self, api: &str) -> Self {
        self.api_url = api.to_string();

        self
    }

    pub async fn build(self) -> Result<HttpClient, HttpClientBuilderError> {
        // check if user session
        // if so, log in and grab token

        let token = if let Some(token) = self.token {
            token
        } else {
            return Err(HttpClientBuilderError::MissingToken)
        };

        let session_header = if let ClientSessionType::Bot = self.client_type {
            "x-bot-token"
        } else {
            "x-session-token"
        };

        let mut headers = HeaderMap::new();
        headers.insert(session_header, HeaderValue::from_str(&token).unwrap());

        let req_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
 
        Ok(HttpClient {
            api_url: self.api_url,
            client: req_client,
        })
    }
}