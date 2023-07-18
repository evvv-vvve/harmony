use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct File {
    #[serde(rename = "_id")]
    pub id: String,
    pub tag: String,
    pub filename: String,
    pub metadata: FileMetadata,
    pub content_type: String,
    pub size: i32,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub reported: bool,
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub server_id: Option<String>,
    #[serde(default)]
    pub object_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum FileMetadata {
    File,
    Text,
    Image {
        width: i32,
        height: i32,
    },
    Video {
        width: i32,
        height: i32
    },
    Audio
}