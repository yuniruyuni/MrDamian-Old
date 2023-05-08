// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;

use client::Client;
use miette::{Diagnostic, IntoDiagnostic, Result, WrapErr};
use std::env;
use thiserror::Error;

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

impl From<SystemTrayMenuMode> for CustomMenuItem {
    fn from(mode: SystemTrayMenuMode) -> Self {
        use SystemTrayMenuMode::*;
        match mode {
            Hide => CustomMenuItem::new("hide".to_string(), "Hide"),
            Open => CustomMenuItem::new("open".to_string(), "Open"),
        }
    }
}

fn create_tray_menu(mode: SystemTrayMenuMode) -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(mode.into())
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
}

#[derive(Error, Debug, Diagnostic)]
enum MrDamianError {
    #[error("window not found")]
    WindowNotFound,
}

fn hide_window(app: &AppHandle) -> Result<()> {
    let window = app
        .get_window("main")
        .ok_or(MrDamianError::WindowNotFound)
        .into_diagnostic()?;
    window.hide().into_diagnostic()?;
    app.tray_handle()
        .set_menu(create_tray_menu(SystemTrayMenuMode::Hide))
        .into_diagnostic()
        .context("failed to change hide window system tray menu")
}

fn show_window(app: &AppHandle) -> Result<()> {
    let window = app
        .get_window("main")
        .ok_or(MrDamianError::WindowNotFound)?;
    window.show().into_diagnostic()?;
    app.tray_handle()
        .set_menu(create_tray_menu(SystemTrayMenuMode::Open))
        .into_diagnostic()
        .context("failed to change open window system tray menu")
}

fn flip_window_visibility(app: &AppHandle) -> Result<()> {
    let window = app
        .get_window("main")
        .ok_or(MrDamianError::WindowNotFound)?;
    if window.is_visible().unwrap_or(false) {
        hide_window(app)
    } else {
        show_window(app)
    }
}

fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    use SystemTrayEvent::*;
    match event {
        DoubleClick { .. } => {
            flip_window_visibility(app).expect("failed to flip main window visibility.")
        }
        MenuItemClick { id, .. } => match id.as_str() {
            "quit" => std::process::exit(0),
            "hide" => hide_window(app).expect("failed to hide main window."),
            "open" => show_window(app).expect("failed to show main window."),
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
