// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;

use client::Client;
use miette::{IntoDiagnostic, Result, WrapErr};
use std::env;

use tauri::{
    async_runtime, generate_context, generate_handler, Builder, CustomMenuItem, SystemTray,
    SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent, Manager
};

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
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    Builder::default()
        .invoke_handler(generate_handler![greet])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick { .. } => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap_or(false) {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                }
            },
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    if window.is_visible().unwrap_or(false) {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                    }
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|_app| {
            async_runtime::spawn(async move {
                let config = Config::load_envs()?;
                let mut wsclient = Client::new(&config.bot, &config.channel, &config.token).await?;
                wsclient.run().await
            });
            Ok(())
        })
        .run(generate_context!())
        .into_diagnostic()
        .context("error while running tauri application")?;
    Ok(())
}