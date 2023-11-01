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

#[repr(u64)]
#[derive(Debug, Deserialize)]
pub enum Permission {
    ManageChannel = 1 << 0,
    ManageServer = 1 << 1,
    ManagePermissions = 1 << 2,
    ManageRole = 1 << 3,
    ManageCustomisation = 1 << 4,

    // % 1 bit reserved

    KickMembers = 1 << 6,
    BanMembers = 1 << 7,
    TimeoutMembers = 1 << 8,
    AssignRoles = 1 << 9,
    ChangeNickname = 1 << 10,
    ManageNicknames = 1 << 11,
    ChangeAvatar = 1 << 12,
    RemoveAvatars = 1 << 13,

    // % 7 bits reserved

    ViewChannel = 1 << 20,
    ReadMessageHistory = 1 << 21,
    SendMessage = 1 << 22,
    ManageMessages = 1 << 23,
    ManageWebhooks = 1 << 24,
    InviteOthers = 1 << 25,
    SendEmbeds = 1 << 26,
    UploadFiles = 1 << 27,
    Masquerade = 1 << 28,
    React = 1 << 29,

    Connect = 1 << 30,
    Speak = 1 << 31,
    Video = 1 << 32,
    MuteMembers = 1 << 33,
    DeafenMembers = 1 << 34,
    MoveMembers = 1 << 35,
    
    // bits 36-52: free to use
    // bits 53-64: don't use

    GrantAllSafe = 0x000F_FFFF_FFFF_FFFF,
    GrantAll = u64::MAX
}

#[repr(i32)]
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Badge {
    Developer = 1,
    Translator = 2,
    Supporter = 4,
    ResponsibleDisclosure = 8,
    Founder = 16,
    PlatformModeration = 32,
    ActiveSupporter = 64,
    Paw = 128,
    EarlyAdopter = 256,
    ReservedRelevantJokeBadge1 = 512,
    ReservedRelevantJokeBadge2 = 1024,
}

pub enum Flag {
    Suspended = 1,
    Deleted = 2,
    Banned = 4,
    Spam = 8,
}