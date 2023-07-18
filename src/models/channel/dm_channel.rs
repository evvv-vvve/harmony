use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct DMChannel {
    #[serde(rename = "_id")]
    id: String,
    active: bool,
    recipients: Vec<String>,
    #[serde(default)]
    last_message_id: Option<String>
}