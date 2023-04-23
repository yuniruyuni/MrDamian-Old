mod client;
use client::Client;
use miette::{IntoDiagnostic, Result, WrapErr};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = env::var("TWITCH_BOT_USERNAME")
        .into_diagnostic()
        .wrap_err("TWITCH_BOT_USERNAME must be set.")?;
    let channel = env::var("TWITCH_CHANNEL")
        .into_diagnostic()
        .wrap_err("TWITCH_CHANNEL must be set.")?;
    let token = env::var("TWITCH_OAUTH_TOKEN")
        .into_diagnostic()
        .wrap_err("TWITCH_OAUTH_TOKEN must be set.")?;

    let mut wsclient = Client::new(&bot, &channel, &token).await?;
    wsclient.run().await?;

    Ok(())
}
