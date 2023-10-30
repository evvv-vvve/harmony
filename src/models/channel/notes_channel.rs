use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct NotesChannel {
    #[serde(rename = "_id")]
    id: String,
    user: String,
}

impl NotesChannel {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_user_id(&self) -> String {
        self.user.clone()
    }
}