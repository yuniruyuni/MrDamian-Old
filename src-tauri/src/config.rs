use miette::{IntoDiagnostic, Result, WrapErr};
use std::env;

pub struct Config {
    pub bot: String,
    pub channel: String,
    pub token: String,
}

impl Config {
    pub fn load_envs() -> Result<Self> {
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
