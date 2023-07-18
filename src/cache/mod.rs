use std::collections::HashMap;

use crate::prelude::{User, Channel, Server};

/*type Result<T> = std::result::Result<T, CacheError>;

pub enum CacheError {
    ReadError,
}*/

// TODO: add limits so these dont balloon in size ?
#[derive(Debug, Clone, PartialEq)]
pub struct Cache {
    pub users: HashMap<String, User>,
    pub channels: HashMap<String, Channel>,
    pub servers: HashMap<String, Server>,
    //pub messages: RwLock<HashMap<String, Message>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            channels: HashMap::new(),
            servers: HashMap::new(),
            //messages: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_user(&self, id: String) -> Option<User> {
        if let Some(user) = self.users.get(&id) {
            Some(user.clone())
        } else {
            None
        }
    }

    pub fn get_channel(&self, id: String) -> Option<Channel> {
        if let Some(channel) = self.channels.get(&id) {
            Some(channel.clone())
        } else {
            None
        }
    }

    pub fn get_server(&self, id: String) -> Option<Server> {
        if let Some(server) = self.servers.get(&id) {
            Some(server.clone())
        } else {
            None
        }
    }
}