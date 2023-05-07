// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() -> Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|_app| {
            tauri::async_runtime::spawn(async move {
                let config = Config::load_envs()?;
                let mut wsclient = Client::new(&config.bot, &config.channel, &config.token).await?;
                wsclient.run().await
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .into_diagnostic()
        .context("error while running tauri application")?;
    Ok(())
}
