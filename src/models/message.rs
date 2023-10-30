use std::collections::{HashMap, HashSet};

use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use super::prelude::{File, Embed};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WebhookInfo {
    name: String,
    avatar: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum SystemEventMessage {
    Text {
        content: String
    },
    #[serde(rename = "user_added")]
    UserAdded {
        id: String,
        by: String,
    },
    #[serde(rename = "user_remove")]
    UserRemove {
        id: String,
        by: String,
    },
    #[serde(rename = "user_joined")]
    UserJoined {
        id: String,
    },
    #[serde(rename = "user_left")]
    UserLeft {
        id: String,
    },
    #[serde(rename = "user_kicked")]
    UserKicked {
        id: String,
    },
    #[serde(rename = "user_banned")]
    UserBanned {
        id: String,
    },
    #[serde(rename = "channel_renamed")]
    ChannelRenamed {
        name: String,
        by: String,
    },
    #[serde(rename = "channel_description_changed")]
    ChannelDescriptionChanged {
        by: String,
    },
    #[serde(rename = "channel_icon_changed")]
    ChannelIconChanged {
        by: String,
    },
    #[serde(rename = "channel_ownership_changed")]
    ChannelOwnershipChanged {
        from: String,
        to: String,
    },
}

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(default)]
    pub nonce: Option<String>,
    pub channel: String,
    pub author: String,
    #[serde(default)]
    pub webhook: Option<WebhookInfo>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub system: Option<SystemEventMessage>,
    #[serde(default)]
    pub attachments: Vec<File>,
    #[serde(default)]
    pub edited: Option<Timestamp>,
    #[serde(default)]
    pub embeds: Vec<Embed>,
    #[serde(default)]
    pub replies: Vec<String>,
    #[serde(default)]
    pub reactions: HashMap<String, HashSet<String>>,
    #[serde(default)]
    pub interactions: Interactions,
    #[serde(default)]
    pub masquerade: Option<Masquerade>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartialMessage {
    pub content: Option<String>,
    pub attachments: Option<Vec<File>>,
    pub embeds: Option<Vec<Embed>>,
    pub replies: Option<Vec<String>>,
    /// Name and / or avatar overrides for this message
    pub masquerade: Option<Masquerade>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Masquerade {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub colour: Option<String>
}

#[derive(Debug, Deserialize, Default, PartialEq, Clone)]
pub struct Interactions {
    #[serde(default)]
    pub reactions: Vec<String>,
    #[serde(default)]
    pub restrict_reactions: bool
}