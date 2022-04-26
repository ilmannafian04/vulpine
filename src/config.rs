use log::error;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_message")]
    pub message: String,
}

macro_rules! default_value {
    ($name:ident, $type:ty, $val:expr) => {
        fn $name() -> $type {
            $val
        }
    };
}
default_value!(default_host, String, "127.0.0.1".to_owned());
default_value!(default_message, String, "no message configured".to_owned());
default_value!(default_port, u16, 8080);

impl AppConfig {
    pub fn new() -> Self {
        match envy::from_env::<Self>() {
            Ok(app_config) => app_config,
            Err(err) => {
                error!("error reading environment variable: {}", err);
                std::process::exit(1);
            }
        }
    }
}
