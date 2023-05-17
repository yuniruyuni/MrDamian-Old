use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use crate::model::PIPELINE_UPDATED;
use crate::presentation::protocol::Editor;
use crate::repository::Repositories;

#[tauri::command]
#[specta::specta]
pub fn editor(repos: State<'_, Mutex<Repositories>>) -> Editor {
    let repos = repos.lock().expect("Failed to lock pipeline repository");
    repos.editor.get()
}

#[tauri::command]
#[specta::specta]
pub fn update_editor(app: AppHandle, repos: State<'_, Mutex<Repositories>>, updated: Editor) {
    let mut repos = repos.lock().expect("Failed to lock pipeline repository");
    repos.editor.set(updated.clone());
    repos.pipeline.set(updated.into());
    app.emit_all(PIPELINE_UPDATED, "update_pipeline").unwrap();
}
