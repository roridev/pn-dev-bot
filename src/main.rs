pub mod config;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Config, Error>;

use config::Config;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = config::read_from_file("./config.json")?;

    let github_settings = octocrab::OctocrabBuilder::new();

    let _ = octocrab::initialise(github_settings);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![],
            ..Default::default()
        })
        .token(config.clone().tokens.discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(config) }));

    framework.run().await.unwrap();

    Ok(())
}
