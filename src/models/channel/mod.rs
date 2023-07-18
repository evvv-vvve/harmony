use serde::{Deserialize, Serialize};

use self::{text_channel::TextChannel, notes_channel::NotesChannel, dm_channel::DMChannel, group_channel::GroupChannel, voice_channel::VoiceChannel};

pub mod partial_channel;
pub mod text_channel;
pub mod notes_channel;
pub mod dm_channel;
pub mod group_channel;
pub mod voice_channel;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
#[serde(tag = "channel_type")]
pub enum Channel {
    SavedMessages(NotesChannel),
    DirectMessage(DMChannel),
    Group(GroupChannel),
    TextChannel(TextChannel),
    VoiceChannel(VoiceChannel),
    #[default]
    Unknown
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
            Channel::Unknown => None,
        }
    }
}