use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ClientEvent {
    Authenticate {
        token: String
    },
    BeginTyping {
        #[serde(rename = "channel")]
        channel_id: String
    },
    EndTyping {
        #[serde(rename = "channel")]
        channel_id: String
    },
    Ping {
        data: i32
    }
}