use std::sync::LazyLock;

static CONFIG: LazyLock<Config> = LazyLock::new(Config::load);

pub struct Config {
    pub core_url: String,
}

impl Config {
    fn load() -> Self {
        Self {
            core_url: option_env!("APP_CORE_URL").unwrap_or("").to_string(),
        }
    }

    pub fn get() -> &'static Config {
        &CONFIG
    }
}
