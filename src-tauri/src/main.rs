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
use std::sync::Mutex;

use tauri::{
    async_runtime, generate_context, generate_handler, Builder, Manager, State, SystemTray,
    WindowEvent,
};

use pipeline::{Component, Connection};
use protocol::{Pipeline, Component as ProtocolComponent, InputPort, OutputPort};
use twitch::{Publisher, Subscriber};

use error::MrDamianError;

#[derive(Debug, Default)]
pub struct Handles {
    handles: Vec<async_runtime::JoinHandle<Result<()>>>,
}

impl Handles {
    fn push(&mut self, handle: async_runtime::JoinHandle<Result<()>>) {
        self.handles.push(handle);
    }
}

impl Drop for Handles {
    fn drop(&mut self) {
        for handle in self.handles.drain(..) {
            handle.abort();
        }
    }
}

struct PipelineState {
    pipeline: Mutex<Pipeline>,
    handles: Mutex<Handles>,
}

impl PipelineState {
    fn get(&self) -> Pipeline {
        let Ok(val) = self.pipeline.lock() else {
            return Pipeline::default()
        };
        val.clone()
    }

    fn set(&self, updated: Pipeline) {
        let Ok(mut handles) = self.handles.lock() else { return };
        let Ok(mut pipeline) = self.pipeline.lock() else { return };
        *handles = create_pipeline(&updated);
        *pipeline = updated;
    }
}

#[tauri::command]
#[specta::specta]
fn pipeline(state: State<'_, PipelineState>) -> Pipeline {
    state.get()
}

#[tauri::command]
#[specta::specta]
fn update_pipeline(state: State<'_, PipelineState>, updated: Pipeline) {
    state.set(updated);
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

fn create_pipeline(pipeline: &Pipeline) -> Handles {
    let mut components = HashMap::new();
    for node in &pipeline.nodes {
        if let Ok(component) = create_component(node.node_type.as_str()) {
            components.insert(node.id.clone(), component);
        }
    }

    for edge in &pipeline.edges {
        let res = components.get_many_mut([edge.source.as_str(), edge.target.as_str()]);
        if let Some([source, target]) = res {
            eprintln!("Connecting {} to {}", edge.source, edge.target);
            Connection::connect(
                source.as_mut(),
                target.as_mut(),
                edge.source_handle.as_str(),
                edge.target_handle.as_str(),
            );
        }
    }

    let mut handles = Handles::default();
    for (_, mut component) in components {
        eprintln!("Starting {}", component.name());
        let handle = async_runtime::spawn(async move {
            let res = component.run().await;
            eprintln!("Component {} exited with {:?}", component.name(), res);
            res
        });
        handles.push(handle);
    }
    handles
}


#[tauri::command]
#[specta::specta]
fn components() -> Vec<ProtocolComponent> {
    vec![
        ProtocolComponent{
            component_type: "TwitchSubscriber".to_string(),
            label: "Twitch Subscriber".to_string(),
            inputs: vec![],
            outputs: vec![
                OutputPort{name: "raid".to_string()},
            ],
        },
        ProtocolComponent{
            component_type: "TwitchPublisher".to_string(),
            label: "Twitch Publisher".to_string(),
            inputs: vec![
                InputPort{name: "message".to_string()},
            ],
            outputs: vec![],
        }
    ]
}

fn gen_bindings() {
    tauri_specta::ts::export(specta::collect_types![
        pipeline,
        update_pipeline,
        components,
    ], "../src/bindings.ts").unwrap();
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    gen_bindings();

    let system_tray = SystemTray::new().with_menu(tray::menu_from(tray::MenuMode::Hide));

    Builder::default()
        .invoke_handler(generate_handler![pipeline, update_pipeline, components])
        .system_tray(system_tray)
        .on_system_tray_event(tray::on_system_tray_event)
        .on_window_event(|event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().expect("failed to hide window");
                api.prevent_close();
            }
        })
        .setup(|app| {
            let pipe = Pipeline::default();

            let handles = create_pipeline(&pipe);

            let pipeline_state = PipelineState {
                pipeline: Mutex::new(pipe),
                handles: Mutex::new(handles),
            };

            app.manage(pipeline_state);

            Ok(())
        })
        .run(generate_context!())
        .into_diagnostic()
        .context("error while running tauri application")?;
    Ok(())
}


#[cfg(test)]
mod test {
    #[test]
    fn export_bindings() {
        super::gen_bindings();
    }
}
