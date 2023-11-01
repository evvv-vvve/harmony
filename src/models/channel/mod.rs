use serde::{Deserialize, Serialize};

use crate::{client::context::Context, http::prelude::HttpError};

use self::{text_channel::TextChannel, notes_channel::NotesChannel, dm_channel::DMChannel, group_channel::GroupChannel, voice_channel::VoiceChannel};

use super::message::{PartialMessage, Message};

pub mod partial_channel;
pub mod text_channel;
pub mod notes_channel;
pub mod dm_channel;
pub mod group_channel;
pub mod voice_channel;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(tag = "channel_type")]
pub enum Channel {
    SavedMessages(NotesChannel),
    DirectMessage(DMChannel),
    Group(GroupChannel),
    TextChannel(TextChannel),
    VoiceChannel(VoiceChannel),
}

impl Channel {
    pub fn get_notes_channel(&self) -> Option<NotesChannel> {
        if let Self::SavedMessages(channel) = self.clone() {
            Some(channel)
        } else {
            None
        }
    }

    pub fn get_dm_channel(&self) -> Option<DMChannel> {
        if let Self::DirectMessage(channel) = self.clone() {
            Some(channel)
        } else {
            None
        }
    }

    pub fn get_group_channel(&self) -> Option<GroupChannel> {
        if let Self::Group(channel) = self.clone() {
            Some(channel)
        } else {
            None
        }
    }

    pub fn get_text_channel(&self) -> Option<TextChannel> {
        if let Self::TextChannel(channel) = self.clone() {
            Some(channel)
        } else {
            None
        }
    }

    pub fn get_voice_channel(&self) -> Option<VoiceChannel> {
        if let Self::VoiceChannel(channel) = self.clone() {
            Some(channel)
        } else {
            None
        }
    }
}

impl Channel {
    pub fn get_name(&self) -> Option<String> {
        match &self {
            Channel::SavedMessages(_notes) => None,
            Channel::DirectMessage(_dm) => None,
            Channel::Group(group) => Some(group.get_name()),
            Channel::TextChannel(text) => Some(text.get_name()),
            Channel::VoiceChannel(voice) => Some(voice.get_name()),
        }
    }

    pub fn get_id(&self) -> String {
        match &self {
            Channel::SavedMessages(notes) => notes.get_id(),
            Channel::DirectMessage(dm) => dm.get_id(),
            Channel::Group(group) => group.get_id(),
            Channel::TextChannel(text) => text.get_id(),
            Channel::VoiceChannel(voice) => voice.get_id(),
        }
    }
}

impl Channel {
    pub async fn say(&self, ctx: &mut Context, message: &str) -> Result<Message, HttpError> {
        ctx.http.say(&self.get_id(), message).await
    }

    pub async fn send_message(&self, ctx: &mut Context, message: PartialMessage) -> Result<Message, HttpError> {
        ctx.http.send_msg_in_channel(&self.get_id(), message).await
    }
}

impl Channel {
    pub fn is_notes(&self) -> bool {
        if let Self::SavedMessages(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_dm(&self) -> bool {
        if let Self::DirectMessage(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_group(&self) -> bool {
        if let Self::Group(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_text(&self) -> bool {
        if let Self::TextChannel(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_voice(&self) -> bool {
        if let Self::VoiceChannel(_) = self {
            true
        } else {
            false
        }
    }
}