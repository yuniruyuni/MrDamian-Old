// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod error;
mod pipeline;
mod protocol;
mod tray;
mod twitch;

use hashbrown::HashMap;
use miette::{IntoDiagnostic, Result, WrapErr};

use tauri::{async_runtime, generate_context, generate_handler, Builder, SystemTray, WindowEvent};

use pipeline::{Component, Connection};
use protocol::{Edge, InputPort, Node, NodeData, OutputPort, Pipeline, Position};
use twitch::{Publisher, Subscriber};

use error::MrDamianError;

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

fn create_component(name: &str) -> Result<Box<dyn Component + Send>> {
    let config = config::Config::load_envs()?;
    match name {
        "TwitchSubscriber" => Ok(Box::new(Subscriber::new(
            &config.bot,
            &config.channel,
            &config.token,
        ))),
        "TwitchPublisher" => Ok(Box::new(Publisher::new(
            &config.bot,
            &config.channel,
            &config.token,
        ))),
        _ => Err(MrDamianError::InvalidComponent).into_diagnostic(),
    }
}

fn create_pipeline() {
    let pipeline = Pipeline {
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
    };

    let mut components = HashMap::new();
    for node in pipeline.nodes {
        if let Ok(component) = create_component(node.node_type.as_str()) {
            components.insert(node.id, component);
        }
    }

    for edge in pipeline.edges {
        let res = components.get_many_mut([edge.source.as_str(), edge.target.as_str()]);
        if let Some([source, target]) = res {
            Connection::connect(
                source.as_mut(),
                target.as_mut(),
                edge.source_handle.as_str(),
                edge.target_handle.as_str(),
            );
        }
    }

    for (_, mut component) in components {
        async_runtime::spawn(async move {
            let res = component.run().await;
            eprintln!("Component {} exited with {:?}", component.name(), res);
            res
        });
    }
}

fn main() -> Result<()> {
    let system_tray = SystemTray::new().with_menu(tray::menu_from(tray::MenuMode::Hide));

    create_pipeline();

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
