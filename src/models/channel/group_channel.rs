use serde::{Deserialize, Serialize};

use crate::models::file::File;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct GroupChannel {
    #[serde(rename = "_id")]
    id: String,
    name: String,
    #[serde(rename = "owner")]
    owner_id: String,
    #[serde(default)]
    description: Option<String>,
    recipients: Vec<String>,
    #[serde(default)]
    icon: Option<File>,
    #[serde(default)]
    last_message_id: Option<String>,
    #[serde(default)]
    permissions: i64,
    #[serde(default)]
    nsfw: bool,
}

impl GroupChannel {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_owner_id(&self) -> String {
        self.owner_id.clone()
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn get_recipient_ids(&self) -> Vec<String> {
        self.recipients.clone()
    }

    pub fn get_icon(&self) -> Option<File> {
        self.icon.clone()
    }

    pub fn get_last_message_id(&self) -> Option<String> {
        self.last_message_id.clone()
    }

    pub fn get_permissions(&self) -> i64 {
        self.permissions
    }

    pub fn is_nsfw(&self) -> bool {
        self.nsfw
    }
}