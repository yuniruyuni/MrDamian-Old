// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod error;
mod pipeline;
mod protocol;
mod tray;
mod twitch;

use miette::{IntoDiagnostic, Result, WrapErr};
use std::sync::Mutex;

use tauri::{
    generate_context, generate_handler, Builder, Manager, State, SystemTray,
    WindowEvent,
};

use pipeline::{create_pipeline, Handles};
use protocol::{Pipeline, Component, InputPort, OutputPort};


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

#[tauri::command]
#[specta::specta]
fn components() -> Vec<Component> {
    vec![
        Component{
            component_type: "TwitchSubscriber".to_string(),
            label: "Twitch Subscriber".to_string(),
            inputs: vec![],
            outputs: vec![
                OutputPort{name: "raid".to_string()},
            ],
        },
        Component{
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
