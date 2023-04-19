mod client;
use std::env;
use client::Client;
use miette::{Result, WrapErr, IntoDiagnostic};

#[tokio::main]
async fn main() -> Result<()> {
    let channel = env::var("TWITCH_CHANNEL")
        .into_diagnostic()
        .wrap_err("TWITCH_CHANNEL must be set.")?;
    // TODO: rename envvar name.
    let oauth = env::var("TWITCH_OAUTH")
        .into_diagnostic()
        .wrap_err("TWITCH_OAUTH must be set.")?;

    let mut wsclient = Client::new(channel.as_str(), oauth.as_str()).await?;
    wsclient.run().await?;

    Ok(())
}
