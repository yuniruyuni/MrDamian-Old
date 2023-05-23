use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use crate::model::{PIPELINE_UPDATED};
use crate::operation::factory; // TODO: encapsulate by repository layer.
use crate::presentation::protocol::{Node, NodeData, Candidate, Position};
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

    let id = ulid::Ulid::new().to_string();

    let Ok(comp) = factory().create_component(&kind, id.as_str()) else {
        return;
    };

    let node = Node {
        id,
        kind,
        position,
        data: NodeData{
            label: comp.label().to_string(),
            inputs: comp.inputs().into_iter().map(|i| i.into()).collect(),
            outputs: comp.outputs().into_iter().map(|o| o.into()).collect(),
        },
    };

    repos.editor.insert_node(node);
    app.emit_all(PIPELINE_UPDATED, "create_component").unwrap();
}
