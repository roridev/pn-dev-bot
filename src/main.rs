use crate::config::Config;

use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Config, Error>;

mod commands;
mod config;
mod utils;

use crate::prefix::register::register;

use crate::slash::*;
use commands::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = octocrab::initialise(octocrab::OctocrabBuilder::new());
    let config: Config;

    if !std::path::Path::new("./config.json").exists() {
        config = Config::default();
        std::fs::write(
            "./config.json",
            serde_json::to_string_pretty(&config).unwrap(),
        )?;
    } else {
        let read = std::fs::read_to_string("./config.json")?;
        config = serde_json::from_str::<Config>(&read).unwrap_or(Config::default());
    }

    let cfg = config.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![waiting::waiting(), issue::issue(), register(), bug::bug()],
            ..Default::default()
        })
        .token(cfg.tokens.discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(config) }));

    framework.run().await.unwrap();

    Ok(())
}
