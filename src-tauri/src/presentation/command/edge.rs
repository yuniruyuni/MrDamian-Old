use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use crate::model::PIPELINE_UPDATED;
use crate::presentation::protocol::Assignment;
use crate::repository::Repositories;

#[tauri::command]
#[specta::specta]
pub fn add_edge(
    app: AppHandle,
    repos: State<'_, Mutex<Repositories>>,
    source: String,
    target: String,
    source_handle: String,
    target_handle: String,
) {
    let mut repos = repos.lock().expect("Failed to lock pipeline repository");

    repos
        .editor
        .add_edge(source, target, source_handle, target_handle);

    app.emit_all(PIPELINE_UPDATED, "ad_edge").unwrap();
}

#[tauri::command]
#[specta::specta]
pub fn remove_edge(
    app: AppHandle,
    repos: State<'_, Mutex<Repositories>>,
    source: String,
    target: String,
    source_handle: String,
    target_handle: String,
) {
    let mut repos = repos.lock().expect("Failed to lock pipeline repository");

    repos
        .editor
        .remove_edge(source, target, source_handle, target_handle);

    app.emit_all(PIPELINE_UPDATED, "remove_edge").unwrap();
}

#[tauri::command]
#[specta::specta]
pub fn set_assignment(
    app: AppHandle,
    repos: State<'_, Mutex<Repositories>>,
    id: String,
    assignment: Assignment,
) {
    let mut repos = repos.lock().expect("Failed to lock pipeline repository");

    repos.editor.set_assignment(id, assignment);

    app.emit_all(PIPELINE_UPDATED, "set_assignment").unwrap();
}
