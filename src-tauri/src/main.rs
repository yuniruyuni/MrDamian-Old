// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;

use client::Client;
use miette::{IntoDiagnostic, Result, WrapErr};
use std::env;

use tauri::{
    async_runtime, generate_context, generate_handler, AppHandle, Builder, CustomMenuItem, Manager,
    SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowEvent,
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

enum SystemTrayMenuMode {
    Hide,
    Open,
}

fn create_tray_menu(mode: SystemTrayMenuMode) -> SystemTrayMenu {
    let visibility = match mode {
        SystemTrayMenuMode::Hide => CustomMenuItem::new("open".to_string(), "Open"),
        SystemTrayMenuMode::Open => CustomMenuItem::new("hide".to_string(), "Hide"),
    };

    SystemTrayMenu::new()
        .add_item(visibility)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
}

fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::DoubleClick { .. } => {
            let window = app.get_window("main").unwrap();
            if window.is_visible().unwrap_or(false) {
                window.hide().unwrap();
                app.tray_handle()
                    .set_menu(create_tray_menu(SystemTrayMenuMode::Hide))
                    .expect("failed to set menu");
            } else {
                window.show().unwrap();
                app.tray_handle()
                    .set_menu(create_tray_menu(SystemTrayMenuMode::Open))
                    .expect("failed to set menu");
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
                app.tray_handle()
                    .set_menu(create_tray_menu(SystemTrayMenuMode::Hide))
                    .expect("failed to set menu");
            }
            "open" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                app.tray_handle()
                    .set_menu(create_tray_menu(SystemTrayMenuMode::Open))
                    .expect("failed to set menu");
            }
            _ => {}
        },
        _ => {}
    }
}

fn main() -> Result<()> {
    let system_tray = SystemTray::new().with_menu(create_tray_menu(SystemTrayMenuMode::Hide));

    async_runtime::spawn(async move {
        let config = Config::load_envs()?;
        let mut wsclient = Client::new(&config.bot, &config.channel, &config.token).await?;
        wsclient.run().await
    });

    Builder::default()
        .invoke_handler(generate_handler![greet])
        .system_tray(system_tray)
        .on_system_tray_event(on_system_tray_event)
        .on_window_event(|event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().expect("failed to hide window");
                api.prevent_close();
            }
        })
        .run(generate_context!())
        .into_diagnostic()
        .context("error while running tauri application")?;
    Ok(())
}
