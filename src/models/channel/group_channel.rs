use serde::{Deserialize, Serialize};

use crate::prelude::File;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
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
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}