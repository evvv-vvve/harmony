use std::sync::Arc;

use async_channel::RecvError;

use crate::{websocket::{SocketError, SocketClient}, cache::Cache, http::client::HttpClient, models::{user::User, events::server::ServerEvent, message::PartialMessage}};

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
                    updated_cache: Cache::new(self.cache.max_messages),
                };
                
                match socket_result {
                    Ok(server_event) => {
                        if let Err(err) = handle_events(event_handler, &mut ctx, server_event).await {
                            println!("err: {err:#?}");
                        }

                        if ctx.updated_cache != Cache::new(self.cache.max_messages) {
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

                            for (id, message) in ctx.updated_cache.messages {
                                self.cache.messages.entry(id)
                                    .and_modify(|msg| *msg = message.clone())
                                    .or_insert(message);
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
        ServerEvent::Bulk(bulk) => {
            /*for event in bulk.events.clone() {
                handle_events(event_handler, ctx, event).await;
            }*/

            event_handler.bulk(ctx, bulk).await
        },
        ServerEvent::Pong(pong) => event_handler.pong(ctx, pong).await,
        ServerEvent::Ready(ready) => event_handler.ready(ctx, ready).await,
        ServerEvent::Message(message) => {
            if let Err(err) = set_context_data(ctx, &message.channel, Some(&message.id)).await {
                return Err(err);
            }

            ctx.cache.add_message(message.clone());
            ctx.updated_cache.add_message(message.clone());

            event_handler.message_received(ctx, message).await
        },
        ServerEvent::MessageUpdate(msg_update) => {
            if let Err(err) = set_context_data(ctx, &msg_update.channel, Some(&msg_update.id)).await {
                return Err(err);
            }

            // add orig msg to the cache update, then update its content
            ctx.updated_cache.add_message(ctx.message.clone().unwrap());
            ctx.updated_cache.update_message(&msg_update.id.clone(), msg_update.data.clone());

            event_handler.message_updated(ctx, msg_update.data).await
        },
        ServerEvent::MessageAppend(append) => {
            if let Err(err) = set_context_data(ctx, &append.channel, Some(&append.message)).await {
                return Err(err);
            }

            let partial = PartialMessage {
                embeds: append.clone().append.embeds,
                ..Default::default()
            };

            // add orig msg to the cache update, then update its content
            ctx.updated_cache.add_message(ctx.message.clone().unwrap());
            ctx.updated_cache.update_message(&append.message, partial);

            event_handler.embed_append(ctx, append).await
        },
        ServerEvent::MessageDelete(msg_delete) => {
            if let Err(err) = set_context_data(ctx, &msg_delete.channel, Some(&msg_delete.id)).await {
                return Err(err);
            }

            let msg = ctx.message.clone().unwrap();

            event_handler.message_deleted(ctx, msg).await
        },
        ServerEvent::MessageReact(msg_react) => {
            if let Err(err) = set_context_data(ctx, &msg_react.channel_id, Some(&msg_react.id)).await {
                return Err(err);
            }

            event_handler.message_reacted(ctx, msg_react).await
        },
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
        ServerEvent::ServerMemberJoin(member_event) => {
            if member_event.user == ctx.user.id {
                println!("WOW");
            }

            event_handler.member_joined(ctx, member_event).await
        },
        ServerEvent::ServerMemberLeave(member_event) => {
            // Decide if we (the current signed in user) left the server,
            // or someone else

            if member_event.user == ctx.user.id {
                // remove server from the cache
                // fire left_guild event
            } else {
                event_handler.member_left(ctx, member_event).await
            }
        },
        ServerEvent::ServerRoleUpdate(role_update) => event_handler.role_updated(ctx, role_update).await,
        ServerEvent::ServerRoleDelete(role_event) => event_handler.role_deleted(ctx, role_event).await,
        ServerEvent::UserUpdate(user_update) => event_handler.user_updated(ctx, user_update).await,
        ServerEvent::UserRelationship(relationship_update) => event_handler.relationship_update(ctx, relationship_update).await,
        ServerEvent::UserPlatformWipe(wipe_event) => event_handler.user_wiped_from_platform(ctx, wipe_event).await,
        ServerEvent::EmojiCreate(emoji) => event_handler.emoji_created(ctx, emoji).await,
        ServerEvent::EmojiDelete { id } => event_handler.emoji_deleted(ctx, id).await,
        ServerEvent::Auth(auth_event) => {
            // TODO
            /*match auth_event.clone() {
                AuthEvent::DeleteSession(session) => {
                    session.
                },
                AuthEvent::DeleteAllSessions(sessions) => {

                }
            };*/

            event_handler.auth(ctx, auth_event).await
        },
        _ => event_handler.received_unhandled_event().await,
    };

    Ok(())
}

async fn set_context_data(ctx: &mut Context, channel: &str, message: Option<&str>) -> std::result::Result<(), HarmonyError> {
    let channel_res = ctx.channel(&channel).await;

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

    if let Some(msg_id) = message {
        ctx.message = ctx.cache.get_message(msg_id);
    }
    
    Ok(())
}