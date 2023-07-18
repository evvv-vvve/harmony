use std::sync::Arc;

use async_trait::async_trait;
use harmony::{prelude::{PartialMessage, Message}, client::{RevoltClient, event_handler::{EventHandler, self}, context::Context}};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: &mut Context, msg: Message) {
        if let Some(content) = msg.content {
            if content.starts_with("!ping") {
                ctx.http.say(&msg.channel, "Pong!").await.unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("REVOLT_TOKEN").expect("Expected Revolt token in environment");

    let mut client = RevoltClient::builder()
        .with_token(&token)
        .with_packet_format(harmony::websocket::PacketFormat::Json)
        .with_event_handler(Arc::new(Handler))
        .build()
        .await
        .unwrap();

    client.login().await.unwrap();

    if let Err(e) = client.listen().await {
        println!("Error: {e:#?}");
    }
}