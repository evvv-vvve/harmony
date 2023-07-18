use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct RevoltInfo {
    #[serde(rename = "revolt")]
    pub api_version: String,
    pub features: RevoltFeatures,
    pub ws: String,
    pub app: String,
    pub vapid: String,
    pub build: RevoltBuild
}

#[derive(Debug, Deserialize, Default)]
pub struct RevoltFeatures {
    pub captcha: CaptchaConfig,
    #[serde(rename = "email")]
    pub uses_email_verification: bool,
    pub invite_only: bool,
    pub autumn: ServiceConfig,
    pub january: ServiceConfig,
    pub voso: ServerConfig
}

#[derive(Debug, Deserialize, Default)]
pub struct CaptchaConfig {
    pub enabled: bool,
    pub key: String
}

#[derive(Debug, Deserialize, Default)]
pub struct ServiceConfig {
    pub enabled: bool,
    pub url: String
}

#[derive(Debug, Deserialize, Default)]
pub struct ServerConfig {
    pub enabled: bool,
    pub url: String,
    pub ws: String
}

#[derive(Debug, Deserialize, Default)]
pub struct RevoltBuild {
    #[serde(default)]
    pub commit_sha: String,
    #[serde(default)]
    pub commit_timestamp: String,
    #[serde(default)]
    pub semver: String,
    #[serde(default)]
    pub origin_url: String,
    #[serde(default)]
    pub timestamp: String,
}