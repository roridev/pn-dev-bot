pub mod commands;
pub mod config;
pub mod utils;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Config, Error>;

use commands::prefix::register;
use commands::slash::{bug, issue, pr, waiting};
use config::Config;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = config::read_from_file("./config.json")?;

    let github_settings = octocrab::OctocrabBuilder::new();

    let _ = octocrab::initialise(github_settings);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                register::register(),
                waiting::waiting(),
                pr::pr(),
                issue::issue(),
                bug::bug(),
            ],
            ..Default::default()
        })
        .token(config.clone().tokens.discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(config) }));

    framework.run().await.unwrap();

    Ok(())
}
