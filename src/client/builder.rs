use std::sync::Arc;

use crate::{websocket::{self, ClientConfigBuilder, PacketFormat, SocketClient}, http::prelude::{ClientSessionType, HttpClientBuilderError, HttpClientBuilder}, cache::Cache};

use super::{event_handler::EventHandler, RevoltClient};

#[derive(Debug)]
pub enum RevoltBuilderError {
    HttpClientError(HttpClientBuilderError)
}

pub struct RevoltClientBuilder {
    http: HttpClientBuilder,
    socket_config: websocket::ClientConfigBuilder,
    event_handler: Option<Arc<dyn EventHandler>>,
    token: Option<String>,

    max_messages: usize,
}

impl RevoltClientBuilder {
    pub fn new() -> Self {
        Self {
            http: HttpClientBuilder::new(),
            socket_config: ClientConfigBuilder::new(),
            event_handler: None,
            token: None,
            max_messages: 256,
        }
    }

    pub fn set_max_cache_messages(mut self, max_messages: usize) -> Self {
        self.max_messages = max_messages;

        self
    }

    pub fn with_event_handler(mut self, event_handler: Arc<dyn EventHandler>) -> Self {
        self.event_handler = Some(event_handler);

        self
    }

    pub fn with_api(mut self, api_url: &str) -> Self {
        self.http = self.http.with_api(api_url);

        self
    }

    pub fn session_type(mut self, session_type: ClientSessionType) -> Self {
        self.http = self.http.session_type(session_type);

        self
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self.http = self.http.with_token(token);

        self
    }

    pub fn with_websocket_url(mut self, socket_url: &str) -> Self {
        self.socket_config = self.socket_config.with_websocket_url(socket_url);
        
        self
    }

    pub fn with_packet_format(mut self, packet_format: PacketFormat) -> Self {
        self.socket_config = self.socket_config.packet_format(packet_format);
        
        self
    }

    pub fn with_heartbeat_interval(mut self, heartbeat_interval: u64) -> Self {
        self.socket_config = self.socket_config.heartbeat_interval(heartbeat_interval);
        
        self
    }

    pub async fn build(self) -> Result<RevoltClient, RevoltBuilderError> {
        let http = match self.http.build().await {
            Ok(http) => http,
            Err(e) => {
                return Err(RevoltBuilderError::HttpClientError(e));
            }
        };

        let build = http.query_node().await.unwrap();

        let socket_config = self.socket_config.with_websocket_url(&build.ws).build();

        let socket = SocketClient::connect(socket_config).await;

        Ok(RevoltClient {
            cache: Cache::new(self.max_messages),
            http,
            socket,
            event_handler: self.event_handler,
            token: self.token
        })
    }
}