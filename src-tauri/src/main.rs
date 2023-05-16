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
    generate_context, generate_handler, AppHandle, Builder, Manager, State, SystemTray, WindowEvent,
};

use pipeline::{Factories, Handles};
use protocol::{Component, Pipeline, PIPELINE_UPDATED};

fn factories() -> Factories {
    Factories::new(vec![
        twitch::Publisher::constructor(),
        twitch::Subscriber::constructor(),
    ])
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
        *handles = factories().create_pipeline(&updated);
        *pipeline = updated;
    }

    fn create_component(&self, component: String, position: protocol::Position) {
        let mut pipeline = self.get();
        let id = pipeline.next_id();
        pipeline.nodes.push(protocol::Node {
            id,
            node_type: component,
            position,
            data: Default::default(),
        });
        self.set(pipeline);
    }
}

#[tauri::command]
#[specta::specta]
fn pipeline(state: State<'_, PipelineState>) -> Pipeline {
    state.get()
}

#[tauri::command]
#[specta::specta]
#[allow(unused_variables)]
fn create_component(
    app: AppHandle,
    state: State<'_, PipelineState>,
    component: String,
    position: protocol::Position,
) {
    state.create_component(component, position);
    app.emit_all(PIPELINE_UPDATED, "create_component").unwrap();
}

#[tauri::command]
#[specta::specta]
fn update_pipeline(app: AppHandle, state: State<'_, PipelineState>, updated: Pipeline) {
    state.set(updated);
    app.emit_all(PIPELINE_UPDATED, "update_pipeline").unwrap();
}

#[tauri::command]
#[specta::specta]
fn components() -> Vec<Component> {
    factories().components()
}

fn gen_bindings() {
    tauri_specta::ts::export(
        specta::collect_types![pipeline, update_pipeline, components, create_component],
        "../src/bindings.ts",
    )
    .unwrap();
}

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    gen_bindings();

    let system_tray = SystemTray::new().with_menu(tray::menu_from(tray::MenuMode::Hide));

    Builder::default()
        .invoke_handler(generate_handler![
            pipeline,
            update_pipeline,
            components,
            create_component
        ])
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
            let handles = factories().create_pipeline(&pipe);

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
