use iso8601_timestamp::Timestamp;
use serde::Deserialize;

use crate::models::{message::{Message, PartialMessage}, channel::{Channel, partial_channel::PartialChannel}, server::{Server, PartialServer}, Emoji, user::{RelationshipStatus, User}, embed::Embed, file::File};

#[derive(Debug, Deserialize, thiserror::Error, Default, Clone)]
pub enum ServerError {
    #[default]
    #[error("Uncategorized error")]
    LabelMe,
    #[error("The server has encountered an error")]
    InternalError,
    #[error("Authentication details are incorrect")]
    InvalidSession,
    #[error("User has not chosen a username")]
    OnboardingNotFinished,
    #[error("This connection is already authenticated")]
    AlreadyAuthenticated
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ServerEvent {
    Error { error: ServerError },
    Authenticated,
    Bulk(BulkEvent),
    Pong(PongEvent),
    Ready(ReadyEvent),
    Message(Message),
    MessageUpdate(MessageUpdateEvent),
    MessageAppend(EmbedAppendEvent),
    MessageDelete(MessageDeleteEvent),
    MessageReact(MessageReactEvent),
    MessageUnreact(MessageReactEvent),
    MessageRemoveReaction(RemoveReactionEvent),
    ChannelCreate(Channel),
    ChannelUpdate(ChannelUpdateEvent),
    ChannelDelete { id: String },
    ChannelGroupJoin(Event),
    ChannelGroupLeave(Event),
    ChannelStartTyping(Event),
    ChannelStopTyping(Event),
    ChannelAck(ChannelMessageAcknowledgeEvent),
    ServerCreate(Server),
    ServerUpdate(ServerUpdateEvent),
    ServerDelete { id: String },
    ServerMemberUpdate(ServerMemberUpdate),
    ServerMemberJoin(Event),
    ServerMemberLeave(Event),
    ServerRoleUpdate(ServerRoleUpdate),
    ServerRoleDelete(RoleEvent),
    UserUpdate(UserUpdate),
    UserRelationship(UserRelationshipEvent),
    UserPlatformWipe(UserPlatformWipeEvent),
    EmojiCreate(Emoji),
    EmojiDelete { id: String },
    Auth(AuthEvent),

    /// Represents a received ServerEvent that doesn't have an enum representation
    #[serde(other)]
    Unknown
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteSession {
    pub user_id: String,
    pub session_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteAllSessions {
    pub user_id: String,
    pub exclude_session_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum AuthEvent {
    DeleteSession(DeleteSession),
    DeleteAllSessions(DeleteAllSessions),
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserPlatformWipeEvent {
    pub user_id: String,
    pub flags: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserRelationshipEvent {
    pub id: String,
    user: User,
    status: RelationshipStatus
}

#[derive(Debug, Deserialize, Clone)]
pub enum UserClear {
    ProfileContent,
    ProfileBackground,
    StatusText,
    Avatar
}

#[derive(Debug, Deserialize, Clone)]
pub struct PartialUser {

}

#[derive(Debug, Deserialize, Clone)]
pub struct UserUpdate {
    pub id: String,
    pub data: PartialUser,
    pub clear: Option<Vec<UserClear>>
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoleEvent {

}

#[derive(Debug, Deserialize, Clone)]
pub struct PartialRole {
    #[serde(rename = "role_id")]
    pub id: String,
    #[serde(rename = "id")]
    pub server: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum RoleClear {
    Colour
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerRoleUpdate {
    #[serde(rename = "role_id")]
    pub id: String,
    #[serde(rename = "id")]
    pub server: String,
    pub data: PartialRole,
    pub clear: Option<Vec<RoleClear>>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerMemberId {
    pub server: String,
    pub user: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PartialServerMember {

}

#[derive(Debug, Deserialize, Clone)]
pub enum ServerMemberClear {
    Nickname,
    Avatar
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerMemberUpdate {
    pub id: ServerMemberId,
    pub data: PartialServerMember,
    pub clear: Option<Vec<ServerMemberClear>>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerUpdateEvent {
    pub id: String,
    pub data: PartialServer,
    pub clear: Option<Vec<ClearField>>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelMessageAcknowledgeEvent {
    pub id: String,
    pub user: String,
    pub message_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Event {
    pub id: String,
    pub user: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum ClearField {
    Icon,
    Banner,
    Description,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelUpdateEvent {
    pub id: String,
    pub data: PartialChannel,
    pub clear: Vec<ClearField>
}

#[derive(Debug, Deserialize, Clone)]
pub struct RemoveReactionEvent {
    pub id: String,
    pub channel_id: String,
    pub emoji_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageReactEvent {
    pub id: String,
    pub channel_id: String,
    pub user_id: String,
    pub emoji_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageDeleteEvent {
    pub id: String,
    pub channel: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbedAppendEvent {
    #[serde(rename = "id")]
    pub message: String,
    pub channel: String,
    pub append: EmbedAppends,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbedAppends {
    pub embeds: Option<Vec<Embed>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageUpdateEvent {
    pub id: String,
    pub channel: String,
    #[serde(default)]
    pub data: PartialMessage,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReadyEvent {
    #[serde(default)]
    users: Vec<User>,
    #[serde(default)]
    servers: Vec<Server>,
    #[serde(default)]
    channels: Vec<Channel>,
    #[serde(default)]
    members: Vec<Member>,
    #[serde(default)]
    emojis: Option<Vec<Emoji>>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Member {
    #[serde(rename = "_id")]
    id: MemberId,
    joined_at: Timestamp,
    pub nickname: Option<String>,
    #[serde(default)]
    pub avatar: Option<File>,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default)]
    pub timeout: Option<Timestamp>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MemberId {
    server: String,
    user: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BulkEvent {
    #[serde(rename = "v")]
    pub events: Vec<ServerEvent>
}

#[derive(Debug, Deserialize, Clone)]
pub struct PongEvent {
    pub data: i32
}