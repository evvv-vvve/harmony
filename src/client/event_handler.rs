use async_trait::async_trait;

use crate::{prelude::{Message, SocketError, Channel, Server, PartialMessage}, models::{events::server::{MessageDeleteEvent, BulkEvent, ReadyEvent, MessageReactEvent, RemoveReactionEvent, ChannelUpdateEvent, Event, ChannelMessageAcknowledgeEvent, ServerUpdateEvent, ServerMemberUpdate, ServerRoleUpdate, RoleEvent, UserUpdate, UserRelationshipEvent, AuthEvent, ServerError, PongEvent, UserPlatformWipeEvent, EmbedAppendEvent}, Emoji}};

use super::context::Context;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn received_unhandled_event(&self) { }

    async fn socket_error(&self, _ctx: &mut Context, _socket_error: SocketError) { }

    async fn server_error(&self, _ctx: &mut Context, _server_error: ServerError) { }

    async fn authenticated(&self, _ctx: &mut Context) { }

    async fn bulk(&self, _ctx: &mut Context, _bulk: BulkEvent) { }

    async fn pong(&self, _ctx: &mut Context, _pong: PongEvent) { }

    async fn ready(&self, _ctx: &mut Context, _ready: ReadyEvent) { }

    async fn message_received(&self, _ctx: &mut Context, _msg: Message) { }

    async fn message_updated(&self, _ctx: &mut Context, _msg: PartialMessage) { }

    async fn embed_append(&self, _ctx: &mut Context, _append: EmbedAppendEvent) { }

    async fn message_deleted(&self, _ctx: &mut Context, _msg: Message) { }

    async fn message_reacted(&self, _ctx: &mut Context, _msg_react: MessageReactEvent) { }

    async fn message_unreacted(&self, _ctx: &mut Context, _msg_react: MessageReactEvent) { }

    async fn message_react_removed(&self, _ctx: &mut Context, _react_remove: RemoveReactionEvent) { }

    async fn channel_created(&self, _ctx: &mut Context, _channel: Channel) { }

    async fn channel_updated(&self, _ctx: &mut Context, _channel_event: ChannelUpdateEvent) { }

    async fn channel_deleted(&self, _ctx: &mut Context, _channel_id: String) { }

    async fn user_joined_group(&self, _ctx: &mut Context, _channel_event: Event) { }

    async fn user_left_group(&self, _ctx: &mut Context, _channel_event: Event) { }

    async fn user_started_typing(&self, _ctx: &mut Context, _channel_event: Event) { }

    async fn user_stopped_typing(&self, _ctx: &mut Context, _channel_event: Event) { }

    async fn messages_acknowledged(&self, _ctx: &mut Context, _acknowlege_event: ChannelMessageAcknowledgeEvent) { }

    async fn server_created(&self, _ctx: &mut Context, _server: Server) { }

    async fn server_updated(&self, _ctx: &mut Context, _server_event: ServerUpdateEvent) { }

    async fn server_deleted(&self, _ctx: &mut Context, _server_id: String) { }

    async fn member_updated(&self, _ctx: &mut Context, _member_update: ServerMemberUpdate) { }

    async fn member_joined(&self, _ctx: &mut Context, _member_event: Event) { }

    async fn member_left(&self, _ctx: &mut Context, _member_event: Event) { }

    async fn role_updated(&self, _ctx: &mut Context, _role_update: ServerRoleUpdate) { }

    async fn role_deleted(&self, _ctx: &mut Context, _role_event: RoleEvent) { }

    async fn user_updated(&self, _ctx: &mut Context, _user_update: UserUpdate) { }

    async fn relationship_update(&self, _ctx: &mut Context, _relationship_update: UserRelationshipEvent) { }

    async fn user_wiped_from_platform(&self, _ctx: &mut Context, _wipe_event: UserPlatformWipeEvent) { }

    async fn emoji_created(&self, _ctx: &mut Context, _emoji: Emoji) { }

    async fn emoji_deleted(&self, _ctx: &mut Context, _emoji_id: String) { }

    async fn auth(&self, _ctx: &mut Context, _auth_event: AuthEvent) { }
}