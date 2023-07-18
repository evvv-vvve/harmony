use serde::{Deserialize, Serialize};

pub mod error;
pub mod user;
pub mod file;
pub mod revolt;
pub mod events;
pub mod server;
pub mod channel;
pub mod message;
pub mod embed;

pub mod prelude {
    pub use super::{
        revolt::*,
        error::*,
        file::*,
        user::*,
        server::*,
        channel::*,
        message::*,
        embed::*,
    };

    pub use super::Permission;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Parent {
    Server {
        id: String
    },
    Detached
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Emoji {
    #[serde(rename = "_id")]
    id: String,
    parent: Parent,
    creator_id: String,
    name: String,
    #[serde(default)]
    animated: bool,
    #[serde(default)]
    nsfw: bool
}

#[derive(Debug, Deserialize)]
pub enum Permission {
    ManageChannel,
    ManageServer,
    ManagePermissions,
    ManageRole,
    ManageCustomisation,
    KickMembers,
    BanMembers,
    TimeoutMembers,
    AssignRoles,
    ChangeNickname,
    ManageNicknames,
    ChangeAvatar,
    RemoveAvatars,
    ViewChannel,
    ReadMessageHistory,
    SendMessage,
    ManageMessages,
    ManageWebhooks,
    InviteOthers,
    SendEmbeds,
    UploadFiles,
    Masquerade,
    React,
    Connect,
    Speak,
    Video,
    MuteMembers,
    DeafenMembers,
    MoveMembers,
    GrantAllSafe,
    GrantAll
}

