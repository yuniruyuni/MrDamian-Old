use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use crate::model::{Pipeline, PIPELINE_UPDATED};
use crate::repository::Repositories;

#[tauri::command]
#[specta::specta]
pub fn pipeline(
    repos: State<'_, Mutex<Repositories>>,
) -> Pipeline {
    let repos = repos.lock().expect("Failed to lock pipeline repository");
    repos.pipeline.editing.get()
}

#[tauri::command]
#[specta::specta]
pub fn update_pipeline(
    app: AppHandle,
    repos: State<'_, Mutex<Repositories>>,
    updated: Pipeline,
) {
    let mut repos = repos.lock().expect("Failed to lock pipeline repository");
    repos.pipeline.editing.set(updated.clone());
    repos.pipeline.running.set(updated);
    app.emit_all(PIPELINE_UPDATED, "update_pipeline").unwrap();
}
