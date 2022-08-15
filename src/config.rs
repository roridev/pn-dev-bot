use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ApiKeys;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Tokens {
    pub discord_token: String,
    pub github_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DiscordIds {
    pub guild: u64,
    pub log_channel: u64,
    pub developer_role: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub api_keys: ApiKeys,
    pub tokens: Tokens,
    pub ids: DiscordIds,
    pub prefix: String,
}

pub fn read_from_file(path: &'static str) -> Result<Config, std::io::Error> {
    let config: Config;

    if !std::path::Path::new(path).exists() {
        config = Config::default();
        std::fs::write(path, serde_json::to_string_pretty(&config).unwrap())?;
    } else {
        let read = std::fs::read_to_string(path)?;
        config = serde_json::from_str::<Config>(&*read).unwrap_or(Config::default());
    }

    Ok(config)
}
