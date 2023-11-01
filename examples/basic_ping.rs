use async_trait::async_trait;
use harmony::{client::{event_handler::EventHandler, context::Context, RevoltClient}, models::message::Message};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message_received(&self, ctx: &mut Context, msg: Message) {
        if let Some(content) = msg.content {
            if content.starts_with("!ping") {
                let channel = ctx.channel.clone().unwrap();

                if let Err(err) = channel.say(ctx, "Pong!").await {
                    println!("Could not send message: {err:#?}");
                }
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
        .with_event_handler(Handler)
        .build()
        .await
        .unwrap();

    client.login().await.unwrap();

    if let Err(e) = client.listen().await {
        println!("Error: {e:#?}");
    }
}