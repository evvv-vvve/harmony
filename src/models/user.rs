use serde::{Deserialize, Serialize};

use super::file::File;

#[derive(Debug, Deserialize, Clone, PartialEq, Default)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub discriminator: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub avatar: Option<File>,
    #[serde(default)]
    pub relations: Vec<Relationship>,
    #[serde(default)]
    pub badges: i32,
    #[serde(default)]
    pub status: Status,
    #[serde(default)]
    pub profile: UserProfile,
    #[serde(default)]
    pub flags: i32,
    #[serde(default)]
    pub privileged: bool,
    #[serde(default)]
    pub bot: Option<BotInfo>,
    #[serde(default)]
    pub relationship: RelationshipStatus,
    #[serde(default)]
    pub online: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Relationship {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(default)]
    pub status: RelationshipStatus
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BotInfo {
    pub owner: String
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq)]
pub struct UserProfile {
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub background: Option<File>
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq)]
pub struct Status {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub presence: Presence,
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq)]
pub enum Presence {
    Online,
    Idle,
    Focus,
    Busy,
    #[default]
    Invisible
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq)]
pub enum RelationshipStatus {
    #[default]
    None,
    User,
    Friend,
    Outgoing,
    Incoming,
    Blocked,
    BlockedOther
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserPermission {
    Access,
    ViewProfile,
    SendMessage,
    Invite,
}