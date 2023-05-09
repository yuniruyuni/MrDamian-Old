// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod error;
mod pipeline;
mod tray;
mod twitch;

use std::sync::mpsc::channel;

use miette::{IntoDiagnostic, Result, WrapErr};

use pipeline::Position;
use tauri::{async_runtime, generate_context, generate_handler, Builder, SystemTray, WindowEvent};

use crate::pipeline::{Edge, InputPort, Node, NodeData, OutputPort, Pipeline};
use crate::twitch::{Publisher, Subscriber};

#[tauri::command]
fn pipeline() -> Pipeline {
    Pipeline {
        nodes: vec![
            Node {
                node_type: "TwitchSubscriber".to_string(),
                id: "1".to_string(),
                data: NodeData {
                    label: "Twitch Subscriber".to_string(),
                    inputs: vec![],
                    outputs: vec![OutputPort {
                        name: "raid".to_string(),
                    }],
                },
                position: Position { x: 20.0, y: 20.0 },
            },
            Node {
                node_type: "TwitchPublisher".to_string(),
                id: "2".to_string(),
                data: NodeData {
                    label: "Twitch Publisher".to_string(),
                    inputs: vec![InputPort {
                        name: "message".to_string(),
                    }],
                    outputs: vec![],
                },
                position: Position { x: 300.0, y: 120.0 },
            },
        ],
        edges: vec![Edge {
            id: "connect test".to_string(),
            label: "connect".to_string(),
            source: "1".to_string(),
            source_handle: "raid".to_string(),
            target: "2".to_string(),
            target_handle: "message".to_string(),
        }],
    }
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
        .invoke_handler(generate_handler![pipeline])
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
