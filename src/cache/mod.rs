use std::collections::BTreeMap;

use crate::models::{user::User, channel::Channel, server::Server, message::{Message, PartialMessage}};

/*type Result<T> = std::result::Result<T, CacheError>;

pub enum CacheError {
    ReadError,
}*/

// TODO: add limits so these dont balloon in size ?
#[derive(Debug, Clone, PartialEq)]
pub struct Cache {
    pub users: BTreeMap<String, User>,
    pub channels: BTreeMap<String, Channel>,
    pub servers: BTreeMap<String, Server>,
    pub messages: BTreeMap<String, Message>,

    pub max_messages: usize,
}

impl Cache {
    pub fn new(max_messages: usize) -> Self {
        Self {
            users: BTreeMap::new(),
            channels: BTreeMap::new(),
            servers: BTreeMap::new(),
            messages: BTreeMap::new(),
            max_messages,
        }
    }

    pub fn get_user(&self, id: &str) -> Option<User> {
        if let Some(user) = self.users.get(id) {
            Some(user.clone())
        } else {
            None
        }
    }

    pub fn get_channel(&self, id: &str) -> Option<Channel> {
        if let Some(channel) = self.channels.get(id) {
            Some(channel.clone())
        } else {
            None
        }
    }

    pub fn get_server(&self, id: &str) -> Option<Server> {
        if let Some(server) = self.servers.get(id) {
            Some(server.clone())
        } else {
            None
        }
    }

    pub fn get_message(&self, id: &str) -> Option<Message> {
        if let Some(message) = self.messages.get(id) {
            Some(message.clone())
        } else {
            None
        }
    }

    pub fn add_message(&mut self, msg: Message) {
        if self.messages.len() >= self.max_messages {
            let _ = self.messages.pop_first();
        }
    
        self.messages.insert(msg.clone().id, msg);
    }

    pub fn update_message(&mut self, msg_id: &str, partial_msg: PartialMessage) {
        if self.messages.contains_key(msg_id) {
            self.messages.entry(msg_id.to_string())
                .and_modify(|msg| {
                    let mut new_msg = msg.clone();

                    if let Some(content) = partial_msg.content {
                        new_msg.content = Some(content);
                    }

                    if let Some(embeds) = partial_msg.embeds {
                        new_msg.embeds = embeds;
                    }
                    
                    if let Some(attachments) = partial_msg.attachments {
                        new_msg.attachments = attachments;
                    };

                    if let Some(replies) = partial_msg.replies {
                        new_msg.replies = replies;
                    }

                    if let Some(masquerade) = partial_msg.masquerade {
                        new_msg.masquerade = Some(masquerade);
                    }

                    *msg = new_msg
                });
        }
    }
}