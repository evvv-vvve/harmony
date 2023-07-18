use std::sync::Arc;

use async_channel::RecvError;

use crate::{prelude::{SocketClient, HttpClient, SocketError, Server, Channel, Message, User}, models::events::server::ServerEvent, cache::Cache, http::prelude::HttpError};

use self::{event_handler::EventHandler, builder::RevoltClientBuilder, context::Context, harmony_error::HarmonyError};

pub mod builder;
pub mod context;
pub mod event_handler;
pub mod colors;
pub mod harmony_error;

type Result<T> = std::result::Result<T, RevoltClientError>;

#[derive(Debug)]
pub enum RevoltClientError {
    TokenMissing,
    SocketError(SocketError),
    RecvError(RecvError)
}

#[derive(Clone)]
pub struct RevoltClient {
    pub cache: Cache,
    pub http: HttpClient,
    pub socket: SocketClient,
    pub event_handler: Option<Arc<dyn EventHandler>>,
    
    token: Option<String>,
}

impl RevoltClient {
    pub fn builder() -> RevoltClientBuilder {
        RevoltClientBuilder::new()
    }

    pub async fn login(&mut self) -> Result<()> {
        if let Some(token) = &self.token {
            if let Err(socket_err) = self.socket.authenticate(&token).await {
                Err(RevoltClientError::SocketError(socket_err))
            } else {
                Ok(())
            }
        } else {
            Err(RevoltClientError::TokenMissing)
        }
    }

    pub async fn listen(&mut self) -> Result<()> {
        while let Ok(socket_result) = self.socket.server_receiver.recv().await {
            // event dispatching or something if there's no EventHandler set

            let http_res = self.http.get_self().await;
            
            let mut curr_user = User::default();
    
            match http_res {
                Ok(user) => {
                    curr_user = user.clone()
                },
                Err(http_err) => {
                    println!("http error: {http_err:#?}")
                }
            }

            if let Some(event_handler) = &self.event_handler {
                let mut ctx = Context {
                    client: self.clone(),
                    user: curr_user,
                    server: None,
                    channel: None,
                    message: None,

                    cache: self.cache.clone(),
                    http: self.http.clone(),
                    updated_cache: Cache::new(),
                };
                
                match socket_result {
                    Ok(server_event) => {
                        if let Err(err) = handle_events(event_handler, &mut ctx, server_event).await {
                            println!("err: {err:#?}");
                        }

                        if ctx.updated_cache != Cache::new() {
                            for (id, user) in ctx.updated_cache.users {
                                self.cache.users.entry(id)
                                    .and_modify(|usr| *usr = user.clone())
                                    .or_insert(user);
                            }

                            for (id, channel) in ctx.updated_cache.channels {
                                self.cache.channels.entry(id)
                                    .and_modify(|chnl| *chnl = channel.clone())
                                    .or_insert(channel);
                            }

                            for (id, server) in ctx.updated_cache.servers {
                                self.cache.servers.entry(id)
                                    .and_modify(|srvr| *srvr = server.clone())
                                    .or_insert(server);
                            }
                        }
                    },
                    Err(socket_err) => {
                        event_handler.socket_error(&mut ctx, socket_err).await
                    }
                }
            }
        }

        Ok(())
    }
}

async fn handle_events(event_handler: &Arc<dyn EventHandler>, ctx: &mut Context, server_event: ServerEvent) -> std::result::Result<(), HarmonyError> {
    match server_event {
        ServerEvent::Error { error } => event_handler.server_error(ctx, error).await,
        ServerEvent::Authenticated => event_handler.authenticated(ctx).await,
        ServerEvent::Bulk(bulk) => event_handler.bulk(ctx, bulk).await,
        ServerEvent::Pong(pong) => event_handler.pong(ctx, pong).await,
        ServerEvent::Ready(ready) => event_handler.ready(ctx, ready).await,
        ServerEvent::Message(message) => {
            let channel_res = ctx.channel(&message.channel).await;

            if let Err(harmony_err) = channel_res {
                return Err(harmony_err);
            }

            let channel = channel_res.unwrap();

            ctx.channel = Some(channel.clone());

            if let Some(channel) = channel.get_text_channel() {
                let server_res = ctx.server(&channel.get_server_id()).await;

                if let Err(harmony_err) = server_res {
                    return Err(harmony_err);
                }

                let server = server_res.unwrap();

                ctx.server = Some(server.clone());
            }

            ctx.message = Some(message.clone());

            event_handler.message(ctx, message).await
        },
        ServerEvent::MessageUpdate(msg_update) => event_handler.message_updated(ctx, msg_update).await,
        ServerEvent::MessageAppend(append) => event_handler.message_append(ctx, append).await,
        ServerEvent::MessageDelete(msg_delete) => event_handler.message_deleted(ctx, msg_delete).await,
        ServerEvent::MessageReact(msg_react) => event_handler.message_reacted(ctx, msg_react).await,
        ServerEvent::MessageUnreact(msg_react) => event_handler.message_unreacted(ctx, msg_react).await,
        ServerEvent::MessageRemoveReaction(react_remove) => event_handler.message_react_removed(ctx, react_remove).await,
        ServerEvent::ChannelCreate(channel) => event_handler.channel_created(ctx, channel).await,
        ServerEvent::ChannelUpdate(channel_event) => event_handler.channel_updated(ctx, channel_event).await,
        ServerEvent::ChannelDelete { id } => event_handler.channel_deleted(ctx, id).await,
        ServerEvent::ChannelGroupJoin(channel_event) => event_handler.user_joined_group(ctx, channel_event).await,
        ServerEvent::ChannelGroupLeave(channel_event) => event_handler.user_left_group(ctx, channel_event).await,
        ServerEvent::ChannelStartTyping(channel_event) => event_handler.user_started_typing(ctx, channel_event).await,
        ServerEvent::ChannelStopTyping(channel_event) => event_handler.user_stopped_typing(ctx, channel_event).await,
        ServerEvent::ChannelAck(acknowlege_event) => event_handler.messages_acknowledged(ctx, acknowlege_event).await,
        ServerEvent::ServerCreate(server) => event_handler.server_created(ctx, server).await,
        ServerEvent::ServerUpdate(server_event) => event_handler.server_updated(ctx, server_event).await,
        ServerEvent::ServerDelete { id } => event_handler.server_deleted(ctx, id).await,
        ServerEvent::ServerMemberUpdate(member_update) => event_handler.member_updated(ctx, member_update).await,
        ServerEvent::ServerMemberJoined(member_event) => event_handler.member_joined(ctx, member_event).await,
        ServerEvent::ServerMemberLeft(member_event) => event_handler.member_left(ctx, member_event).await,
        ServerEvent::ServerRoleUpdate(role_update) => event_handler.role_updated(ctx, role_update).await,
        ServerEvent::ServerRoleDelete(role_event) => event_handler.role_deleted(ctx, role_event).await,
        ServerEvent::UserUpdate(user_update) => event_handler.user_updated(ctx, user_update).await,
        ServerEvent::UserRelationship(relationship_update) => event_handler.relationship_update(ctx, relationship_update).await,
        ServerEvent::UserPlatformWipe(wipe_event) => event_handler.user_wiped_from_platform(ctx, wipe_event).await,
        ServerEvent::EmojiCreate(emoji) => event_handler.emoji_created(ctx, emoji).await,
        ServerEvent::EmojiDelete { id } => event_handler.emoji_deleted(ctx, id).await,
        ServerEvent::Auth(auth_event) => event_handler.auth(ctx, auth_event).await,
        _ => event_handler.received_unhandled_event().await,
    };

    Ok(())
}