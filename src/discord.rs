use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct APIEmbed {
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,

    #[serde(alias = "field")]
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedVideo {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub proxy_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedProvider {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedAuthor {
    pub icon_url: Option<String>,
    pub name: String,
    pub url: Option<String>,
    pub proxy_icon_url: Option<String>,
}