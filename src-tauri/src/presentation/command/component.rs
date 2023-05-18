use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use crate::model::{PIPELINE_UPDATED};
use crate::operation::factory; // TODO: encapsulate by repository layer.
use crate::presentation::protocol::{Candidate, Position};
use crate::repository::Repositories;

#[tauri::command]
#[specta::specta]
pub fn candidates() -> Vec<Candidate> {
    let mut res = vec![];
    for c in factory().candidates() {
        res.push(Candidate {
            kind: c.kind.0.clone(),
            label: c.label.to_string(),
        });
    }
    res
}

#[tauri::command]
#[specta::specta]
#[allow(unused_variables)]
pub fn create_component(
    app: AppHandle,
    repos: State<'_, Mutex<Repositories>>,
    kind: String,
    position: Position,
) {
    let mut repos = repos.lock().expect("Failed to lock pipeline repository");

    let Ok(comp) = factory().create_component(
        &kind,
        ulid::Ulid::new().to_string().as_str(),
    ) else {
        return;
    };

    repos.editor.create_component(comp.as_ref(), kind, position);
    app.emit_all(PIPELINE_UPDATED, "create_component").unwrap();
}
