use tauri::{AppHandle, Manager, State};

use crate::model::{Pipeline, PIPELINE_UPDATED};
use crate::repository::PipelineState;

#[tauri::command]
#[specta::specta]
pub fn pipeline(state: State<'_, PipelineState>) -> Pipeline {
    state.get()
}

#[tauri::command]
#[specta::specta]
pub fn update_pipeline(app: AppHandle, state: State<'_, PipelineState>, updated: Pipeline) {
    state.set(updated);
    app.emit_all(PIPELINE_UPDATED, "update_pipeline").unwrap();
}
