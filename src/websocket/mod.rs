use std::{fmt::Display, time::{Duration, Instant}};

use async_channel::{Receiver, Sender};
use futures_util::{StreamExt, SinkExt};
use tokio::{net::TcpStream, select, time::sleep};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream, tungstenite::Message};

use crate::models::events::{server::ServerEvent, client::ClientEvent};

/// the default Revolt websocket url
pub const REVOLT_WEBSOCKET_URL: &str = "wss://ws.revolt.chat";

pub enum PacketFormat {
    Json,
    MsgPack
}

impl Display for PacketFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketFormat::Json => write!(f, "json"),
            PacketFormat::MsgPack => write!(f, "msgpack"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SocketClient {
    pub ws: String,
    pub heartbeat_interval: u64,
    pub last_heartbeat: Instant,
    
    pub client_sender: Sender<ClientEvent>,
    pub client_receiver: Receiver<ClientEvent>,
    
    pub server_sender: Sender<SocketResult<ServerEvent>>,
    pub server_receiver: Receiver<SocketResult<ServerEvent>>,

    //pub socket: WebSocketStream<MaybeTlsStream<TcpStream>>
}

type SocketResult<T> = Result<T, SocketError>;

#[derive(Debug)]
pub enum SocketError {
    JsonSerializationError(serde_json::error::Error),
    TungsteniteError(tokio_tungstenite::tungstenite::error::Error),
    SendError,
}

impl SocketClient {
    async fn handle_events(
        mut client_receiver: Receiver<ClientEvent>,
        server_sender: Sender<SocketResult<ServerEvent>>,
        mut socket: WebSocketStream<MaybeTlsStream<TcpStream>>
    ) {
        loop {
            select! {
                Some(event) = client_receiver.next() => {
                    match serde_json::to_string(&event) {
                        Ok(data) => {
                            let msg = Message::Text(data);
                            socket.send(msg).await.unwrap();
                        },
                        Err(json_err) => {
                            if let Err(e) = server_sender.send(Err(SocketError::JsonSerializationError(json_err))).await {
                                panic!("An error occurred while sending result: {e:#?}")
                            }
                        }
                    };
                },
                Some(msg) = socket.next() => {
                    match msg {
                        Ok(msg) => {
                            let event = match msg {
                                Message::Text(text) => {
                                    let event = match serde_json::from_str::<ServerEvent>(&text) {
                                        Ok(event) => {
                                            Some(event)
                                        },
                                        Err(json_err) => {
                                            let res = server_sender.send(Err(SocketError::JsonSerializationError(json_err))).await;
                                            
                                            if let Err(e) = res {
                                                panic!("An error occurred while sending result: {e:#?}")
                                            };

                                            None
                                        }
                                    };

                                    event
                                },
                                _ => {
                                    println!("Unhandled Event");
                                    None
                                }
                            };
        
                            if let Some(event) = event {
                                server_sender.send(Ok(event)).await.unwrap();
                            } else {

                            }
                        },
                        Err(msg_err) => {
                            if let Err(e) = server_sender.send(Err(SocketError::TungsteniteError(msg_err))).await {
                                panic!("An error occurred while sending result: {e:#?}")
                            }
                        }
                    }
                },
                else => break
            };
        }
    }

    pub async fn connect(config: ClientConfig) -> Self {
        let (stream, _) = connect_async(config.ws.clone())
            .await
            .expect("cant connect");

        let (client_sender, client_receiver) = async_channel::unbounded();
        let (server_sender, server_receiver) = async_channel::unbounded();

        //let (server_sender, server_receiver) = stream.split();

        let dummy = Instant::now() - Duration::from_secs(config.heartbeat_interval);

        tokio::spawn(SocketClient::heartbeat(client_sender.clone(), config.heartbeat_interval));
        tokio::spawn(SocketClient::handle_events(client_receiver.clone(), server_sender.clone(), stream));

        Self {
            ws: config.ws,
            heartbeat_interval: config.heartbeat_interval,
            last_heartbeat: dummy,
            
            client_sender: client_sender,
            client_receiver: client_receiver,
            
            server_sender: server_sender,
            server_receiver: server_receiver,

            //socket
        }
    }

    pub async fn send(&mut self, event: ClientEvent) -> Result<(), SocketError> {
        if let Err(_send_err) = self.client_sender.send(event).await {
            return Err(SocketError::SendError);
        }

        Ok(())
    }

    pub async fn authenticate(&mut self, token: &str) -> Result<(), SocketError> {
        self.send(ClientEvent::Authenticate { token: token.to_string() }).await
    }

    pub async fn start_typing(&mut self, channel_id: &str) -> Result<(), SocketError> {
        self.send(ClientEvent::BeginTyping { channel_id: channel_id.to_string() }).await
    }

    pub async fn stop_typing(&mut self, channel_id: &str) -> Result<(), SocketError> {
        self.send(ClientEvent::EndTyping { channel_id: channel_id.to_string() }).await
    }

    async fn heartbeat(client_sender: Sender<ClientEvent>, heartbeat_interval: u64) {
        loop {
            client_sender.send(ClientEvent::Ping { data: 0 }).await.unwrap();
            sleep(Duration::from_secs(heartbeat_interval)).await;
        }
    }
}

pub struct ClientConfig {
    ws: String,
    heartbeat_interval: u64,
}

pub struct ClientConfigBuilder {
    ws: String,
    version: usize,
    packet_format: PacketFormat,
    heartbeat_interval: u64,
}

impl ClientConfigBuilder {
    pub fn new() -> Self {
        Self {
            ws: REVOLT_WEBSOCKET_URL.to_string(),
            version: 1,
            heartbeat_interval: 15,
            packet_format: PacketFormat::Json,
        }
    }

    pub fn with_websocket_url(mut self, ws: &str) -> Self {
        self.ws = ws.to_string();

        self
    }

    pub fn packet_format(mut self, packet_format: PacketFormat) -> Self {
        self.packet_format = packet_format;

        self
    }

    pub fn heartbeat_interval(mut self, heartbeat_interval: u64) -> Self {
        self.heartbeat_interval = heartbeat_interval;

        self
    }

    pub fn build(&self) -> ClientConfig {
        let ws = format!("{}/?version={}&format={}", self.ws, self.version, self.packet_format);

        ClientConfig {
            ws,
            heartbeat_interval: self.heartbeat_interval
        }
    }
}