use serde::Deserialize;

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
    pub video_dirs: Vec<String>
}

impl Config {
    pub fn load(toml_content: &str) ->  Config {
        toml::from_str(toml_content).unwrap()
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
