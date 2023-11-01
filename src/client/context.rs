use crate::{http::prelude::HttpClient, cache::Cache, models::{user::User, server::Server, channel::Channel, message::Message}, };

use super::{harmony_error::HarmonyError, RevoltClient};


pub struct Context {
    pub client: RevoltClient,
    pub user: User,
    pub server: Option<Server>,
    pub channel: Option<Channel>,
    pub message: Option<Message>,

    pub cache: Cache,
    pub http: HttpClient,

    pub(crate) updated_cache: Cache,
}

impl Context {
    pub async fn user(&mut self, user_id: &str) -> Result<User, HarmonyError> {
        if let Some(user) = self.cache.get_user(user_id) {
            Ok(user)
        } else {
            let http_res = self.http.get_user(&user_id).await;

            match http_res {
                Ok(user) => {
                    self.updated_cache.users.insert(user_id.to_string(), user.clone());
                    self.cache.users.insert(user_id.to_string(), user.clone());
                    
                    Ok(user)
                },
                Err(http_err) => {
                    Err(HarmonyError::HttpError(http_err))
                }
            }
        }
    }

    pub async fn message(&mut self, channel_id: &str, message_id: &str) -> Result<Message, HarmonyError> {
        if let Some(message) = self.cache.get_message(message_id) {
            Ok(message)
        } else {
            let http_res = self.http.get_message(&channel_id, &message_id).await;

            match http_res {
                Ok(message) => {
                    if self.cache.messages.len() >= self.cache.max_messages {
                        // is this even the right way to do this
                        let _ = self.cache.messages.pop_first();
                        self.cache.messages.insert(message_id.to_string(), message.clone());
                    }

                    if self.updated_cache.messages.len() >= self.cache.max_messages {
                        let _ = self.updated_cache.messages.pop_first();
                        self.updated_cache.messages.insert(message_id.to_string(), message.clone());
                    }
                    
                    Ok(message)
                },
                Err(http_err) => {
                    Err(HarmonyError::HttpError(http_err))
                }
            }
        }
    }

    pub async fn channel(&mut self, channel_id: &str) -> Result<Channel, HarmonyError> {
        if let Some(channel) = self.cache.get_channel(channel_id) {
            Ok(channel)
        } else {
            let http_res = self.http.get_channel(&channel_id).await;

            match http_res {
                Ok(channel) => {
                    self.updated_cache.channels.insert(channel_id.to_string(), channel.clone());
                    self.cache.channels.insert(channel_id.to_string(), channel.clone());
                    
                    Ok(channel)
                },
                Err(http_err) => {
                    Err(HarmonyError::HttpError(http_err))
                }
            }
        }
    }

    pub async fn server(&mut self, server_id: &str) -> Result<Server, HarmonyError> {
        if let Some(server) = self.cache.get_server(server_id) {
            Ok(server)
        } else {
            let http_res = self.http.get_server(&server_id).await;

            match http_res {
                Ok(server) => {
                    self.updated_cache.servers.insert(server_id.to_string(), server.clone());
                    self.cache.servers.insert(server_id.to_string(), server.clone());
                    
                    Ok(server)
                },
                Err(http_err) => {
                    Err(HarmonyError::HttpError(http_err))
                }
            }
        }
    }
}