use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use crate::model::{Component, Position, PIPELINE_UPDATED};
use crate::operation::factories; // TODO: encapsulate by repository layer.
use crate::repository::Repositories;

#[tauri::command]
#[specta::specta]
pub fn components() -> Vec<Component> {
    factories().components()
}

#[tauri::command]
#[specta::specta]
#[allow(unused_variables)]
pub fn create_component(
    app: AppHandle,
    repos: State<'_, Mutex<Repositories>>,
    component: String,
    position: Position,
) {
    let mut repos = repos.lock().expect("Failed to lock pipeline repository");
    repos.pipeline.editing.create_component(component, position);
    app.emit_all(PIPELINE_UPDATED, "create_component").unwrap();
}
