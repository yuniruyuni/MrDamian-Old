use tauri::{AppHandle, Manager, State};

use crate::model::{Component, Position, PIPELINE_UPDATED};
use crate::operation::factories; // TODO: encapsulate by repository layer.
use crate::repository::PipelineState;

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
    state: State<'_, PipelineState>,
    component: String,
    position: Position,
) {
    state.create_component(component, position);
    app.emit_all(PIPELINE_UPDATED, "create_component").unwrap();
}
