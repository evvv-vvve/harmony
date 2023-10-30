use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct DMChannel {
    #[serde(rename = "_id")]
    id: String,
    active: bool,
    recipients: Vec<String>,
    #[serde(default)]
    last_message_id: Option<String>
}

impl DMChannel {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn is_dm_active(&self) -> bool {
        self.active
    }

    pub fn get_recipients(&self) -> Vec<String> {
        self.recipients.clone()
    }

    pub fn get_last_message_id(&self) -> Option<String> {
        self.last_message_id.clone()
    }
}