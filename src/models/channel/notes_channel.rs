use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NotesChannel {
    #[serde(rename = "_id")]
    id: String,
    user: String,
}