use reqwest::Response;
use serde::{de::DeserializeOwned, Serialize};

use crate::{models::{user::User, prelude::{RevoltInfo, PartialMessage, Message, RevoltError}, server::Server}, prelude::Channel};

use super::prelude::HttpClientBuilder;

/// the default Revolt api url
pub const REVOLT_API_URL: &str = "https://api.revolt.chat";

type HttpResult<T> = std::result::Result<T, HttpError>;

#[derive(Default, Debug)]
pub enum ClientSessionType {
    #[default]
    Bot,
    User,
}

/// Represents an error that occurred within Harmony's HTTP Client
#[derive(Debug)]
pub enum HttpError {
    RequestUnsuccessful(String),
    RevoltError(RevoltError),
}

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub api_url: String,
    pub client: reqwest::Client,
}

impl HttpClient {
    pub fn builder() -> HttpClientBuilder {
        HttpClientBuilder::new()
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> HttpResult<T> {
        let response = self.client.get(self.as_url(path)).send().await;
        
        match response {
            Ok(response) => Self::parse_response(response).await,
            Err(e) => Err(HttpError::RequestUnsuccessful(format!("{e:#?}")))
        }
    }

    pub async fn post<T: DeserializeOwned, U: Serialize>(&self, path: &str, body: U) -> HttpResult<T> {
        let result = self.client.post(self.as_url(path)).json(&body).send().await;

        match result {
            Ok(response) => Self::parse_response(response).await,
            Err(e) => Err(HttpError::RequestUnsuccessful(format!("{e:#?}")))
        }
    }

    async fn parse_response<T: DeserializeOwned>(response: Response) -> HttpResult<T> {
        let text = match response.text().await {
            Ok(text) => text,
            Err(e) => return Err(HttpError::RequestUnsuccessful(format!("{e:#?}")))
        };

        // check if there was an error
        if let Ok(revolt_error) = serde_json::from_str::<RevoltError>(&text) {
            return Err(HttpError::RevoltError(revolt_error));
        };

        // error parse failed, try to parse into our desired type
        match serde_json::from_str::<T>(&text) {
            Ok(data) => Ok(data),
            Err(e) => Err(HttpError::RequestUnsuccessful(format!("{e:#?}")))
        }
    }

    pub async fn query_node(&self) -> HttpResult<RevoltInfo> {
        self.get::<RevoltInfo>("/").await
    }

    fn as_url(&self, path: &str) -> String {
        format!("{}{path}", self.api_url)
    }
}

// === users
impl HttpClient {
    pub async fn get_self(&self) -> HttpResult<User> {
        self.get::<User>("/users/@me").await
    }

    pub async fn get_user(&self, user_id: &str) -> HttpResult<User> {
        self.get::<User>(&format!("/users/{user_id}")).await
    }
}

impl HttpClient {
    pub async fn get_server(&self, server_id: &str) -> HttpResult<Server> {
        self.get::<Server>(&format!("/servers/{server_id}")).await
    }
}

impl HttpClient {
    pub async fn get_channel(&self, channel_id: &str) -> HttpResult<Channel> {
        self.get::<Channel>(&format!("/channels/{channel_id}")).await
    }

    pub async fn get_message(&self, channel_id: &str, message_id: &str) -> HttpResult<Message> {
        self.get::<Message>(&format!("/channels/{channel_id}/messages/{message_id}")).await
    }
}

impl HttpClient {
    pub async fn say(&self, channel_id: &str, msg: &str) -> HttpResult<Message> {
        let msg = PartialMessage {
            content: Some(msg.to_string()),
            ..Default::default()
        };

        self.send_msg_in_channel(channel_id, msg).await
    }
    pub async fn send_msg_in_channel(&self, channel_id: &str, msg: PartialMessage) -> HttpResult<Message> {
        self.post::<Message, PartialMessage>(&format!("/channels/{channel_id}/messages"), msg).await
    }
}