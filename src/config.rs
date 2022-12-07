use serde::Deserialize;
use chrono::prelude::*;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_debug")]
    pub debug: bool,
    #[serde(default = "default_server_addr")]
    pub server_addr: String,
    #[serde(default = "default_cert")]
    pub cert: String,
    #[serde(default = "default_key")]
    pub key: String,
    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub video_dirs: Vec<String>
}

impl Config {
    pub fn load(toml_content: &str) ->  Config {
        let mut conf: Config = toml::from_str(toml_content).unwrap();
        conf.jwt_secret = format!("{}{}", conf.jwt_secret, Utc::now());
        conf
    }
}

// Default values for Config fields

fn default_debug() -> bool {
    false
}

fn default_server_addr() -> String {
    ":8443".to_string()
}

fn default_cert() -> String {
    "server.crt".to_string()
}

fn default_key() -> String {
    "server.key".to_string()
}

fn default_jwt_secret() -> String {
    "secret".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}
