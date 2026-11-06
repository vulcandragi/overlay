use std::sync::LazyLock;

use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;

static CONFIG: LazyLock<Config> = LazyLock::new(Config::load);

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bot_username: String,
    pub bot_token: String,
    pub channel_to_join: String,
    pub gateway_url: String,
}

impl Config {
    fn load() -> Self {
        Figment::new()
            .merge(Toml::file("env.toml"))
            .merge(Env::prefixed("APP_"))
            .extract()
            .expect("Fail get config")
    }

    pub fn get() -> &'static Config {
        &CONFIG
    }
}
