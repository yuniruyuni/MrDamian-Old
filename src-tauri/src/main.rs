// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod error;
mod pipeline;
mod tray;
mod twitch;

use std::sync::mpsc::channel;

use miette::{IntoDiagnostic, Result, WrapErr};

use tauri::{async_runtime, generate_context, generate_handler, Builder, SystemTray, WindowEvent};

use crate::twitch::{Publisher, Subscriber};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> Result<()> {
    let system_tray = SystemTray::new().with_menu(tray::menu_from(tray::MenuMode::Hide));

    let (sender, receiver) = channel::<pipeline::Message>();
    async_runtime::spawn(async move {
        let config = config::Config::load_envs()?;
        let mut subscriber =
            Subscriber::new(sender, &config.bot, &config.channel, &config.token).await?;
        let res = subscriber.run().await;
        eprintln!("Subscriber exited with {:?}", res);
        res
    });

    async_runtime::spawn(async move {
        let config = config::Config::load_envs()?;
        let mut publisher =
            Publisher::new(receiver, &config.bot, &config.channel, &config.token).await?;
        let res = publisher.run().await;
        eprintln!("Publisher exited with {:?}", res);
        res
    });

    Builder::default()
        .invoke_handler(generate_handler![greet])
        .system_tray(system_tray)
        .on_system_tray_event(tray::on_system_tray_event)
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
