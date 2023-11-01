use serde::Deserialize;

use crate::models::server::PermissionOverride;

#[derive(Debug, Deserialize, Clone)]
pub struct PartialChannel {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub nsfw: Option<bool>,
    #[serde(default)]
    pub default_permissions: Option<PermissionOverride>,
}