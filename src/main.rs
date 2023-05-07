mod client;

use client::Client;
use miette::{IntoDiagnostic, Result, WrapErr};
use std::env;

struct Config {
    bot: String,
    channel: String,
    token: String,
}

impl Config {
    fn load_envs() -> Result<Self> {
        let bot = env::var("TWITCH_BOT_USERNAME")
            .into_diagnostic()
            .wrap_err("TWITCH_BOT_USERNAME must be set.")?;
        let channel = env::var("TWITCH_CHANNEL")
            .into_diagnostic()
            .wrap_err("TWITCH_CHANNEL must be set.")?;
        let token = env::var("TWITCH_OAUTH_TOKEN")
            .into_diagnostic()
            .wrap_err("TWITCH_OAUTH_TOKEN must be set.")?;

        Ok(Self {
            bot,
            channel,
            token,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let websocket_thread = tokio::spawn(async move {
        let config = Config::load_envs()?;
        let mut wsclient = Client::new(&config.bot, &config.channel, &config.token).await?;
        wsclient.run().await?;
        Ok(())
    });

    tokio::try_join!(flatten(websocket_thread),)?;

    Ok(())
}

async fn flatten(h: tokio::task::JoinHandle<Result<()>>) -> Result<()> {
    match h.await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(e).into_diagnostic(),
    }
}
