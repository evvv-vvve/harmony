use crate::{http::prelude::HttpClient, cache::Cache, prelude::{User, Channel, Server, Message}};

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
    pub async fn user(&mut self, user_id: String) -> Result<User, HarmonyError> {
        if let Some(user) = self.cache.get_user(user_id.clone()) {
            Ok(user)
        } else {
            let http_res = self.http.get_user(&user_id).await;

            match http_res {
                Ok(user) => {
                    self.updated_cache.users.insert(user_id.clone(), user.clone());
                    self.cache.users.insert(user_id, user.clone());
                    
                    Ok(user)
                },
                Err(http_err) => {
                    Err(HarmonyError::HttpError(http_err))
                }
            }
        }
    }

    pub async fn channel(&mut self, channel_id: &str) -> Result<Channel, HarmonyError> {
        if let Some(channel) = self.cache.get_channel(channel_id.to_string()) {
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
        if let Some(server) = self.cache.get_server(server_id.to_string()) {
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