use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ApiKeys;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Tokens {
    pub discord_token: String,
    pub github_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ChannelIds {
    pub log_channel: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub api_keys: ApiKeys,
    pub tokens: Tokens,
    pub prefix: String,
    pub channels: ChannelIds,
    pub guild_id: u64,
    pub developer_role: u64,
}
