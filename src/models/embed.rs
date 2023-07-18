use serde::{Deserialize, Serialize};

use super::file::File;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Embed {
    Website {
        #[serde(default)]
        url: Option<String>,
        #[serde(default)]
        original_url: Option<String>,
        #[serde(default)]
        special: Option<RemoteContent>,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        description: Option<String>,
        #[serde(default)]
        image: Option<Image>,
        #[serde(default)]
        video: Option<Video>,
        #[serde(default)]
        site_name: Option<String>,
        #[serde(default)]
        icon_url: Option<String>,
        #[serde(default)]
        colour: Option<String>
    },
    Image {
        url: String,
        width: usize,
        height: usize,
        size: Size
    },
    Video {
        url: String,
        width: usize,
        height: usize
    },
    Text {
        #[serde(default)]
        icon_url: Option<String>,
        #[serde(default)]
        url: Option<String>,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        description: Option<String>,
        #[serde(default)]
        media: Option<File>,
        #[serde(default)]
        colour: Option<String>
    },
    None
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Size {
    Large,
    Preview
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub url: String,
    pub width: usize,
    pub height: usize,
    pub size: Size,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub url: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(tag = "type")]
pub enum RemoteContent {
    #[default]
    None,
    GIF,
    YouTube {
        id: String,
        #[serde(default)]
        timestamp: Option<String>
    },
    Lightspeed {
        id: String,
        content_type: ContentType
    },
    Twitch {
        id: String,
        content_type: ContentType
    },
    Spotify {
        id: String,
        content_type: String
    },
    Soundcloud,
    Bandcamp {
        id: String,
        content_type: ContentType
    },
    Streamable {
        id: String
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum ContentType {
    #[default]
    None,
    Channel,
    Video,
    Clip,
    Album,
    Track
}