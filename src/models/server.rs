use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::prelude::File;

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: String,
    pub owner: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub channels: Vec<String>,
    #[serde(default)]
    pub categories: Vec<ChannelCategory>,
    #[serde(default)]
    pub system_messages: Option<SystemMessages>,
    #[serde(default)]
    pub roles: HashMap<String, Role>,
    pub default_permissions: i64,
    #[serde(default)]
    pub icon: Option<File>,
    #[serde(default)]
    pub banner: Option<File>,
    #[serde(default)]
    pub flags: i32,
    #[serde(default)]
    pub nsfw: bool,
    #[serde(default)]
    pub analytics: bool,
    #[serde(default)]
    pub discoverable: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PartialServer {
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub categories: Option<Vec<ChannelCategory>>,
    #[serde(default)]
    pub system_messages: Option<SystemMessages>,
    #[serde(default)]
    pub flags: Option<i32>,
    #[serde(default)]
    pub nsfw: Option<bool>,
    #[serde(default)]
    pub analytics: Option<bool>,
    #[serde(default)]
    pub discoverable: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Role {
    pub name: String,
    pub permissions: PermissionOverride,
    #[serde(default)]
    pub colour: Option<String>,
    #[serde(default)]
    pub hoist: bool,
    #[serde(default)]
    pub rank: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq)]
pub struct PermissionOverride {
    #[serde(rename = "a")]
    #[serde(default)]
    allow: i64,
    #[serde(default)]
    #[serde(rename = "d")]
    deny: i64
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SystemMessages {
    #[serde(default)]
    pub user_joined: Option<String>,
    #[serde(default)]
    pub user_left: Option<String>,
    #[serde(default)]
    pub user_kicked: Option<String>,
    #[serde(default)]
    pub user_banned: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelCategory {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub channels: Vec<String>,

}