use serde::{Deserialize, Serialize};

use crate::prelude::{PermissionOverride, File};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct VoiceChannel {
    #[serde(rename = "server")]
    server_id: String,
    #[serde(rename = "_id")]
    id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    icon: Option<File>,
    #[serde(default)]
    default_permissions: PermissionOverride,
    #[serde(default)]
    role_permissions: PermissionOverride,
    #[serde(default)]
    nsfw: bool,
}

impl VoiceChannel {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}