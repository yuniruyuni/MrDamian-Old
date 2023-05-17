use std::sync::Mutex;

use crate::model::{Node, Pipeline, Position};
use crate::operation::{factories, pipeline::Handles};

pub struct PipelineState {
    pub pipeline: Mutex<Pipeline>,
    pub handles: Mutex<Handles>,
}

impl PipelineState {
    pub fn new(pipeline: Pipeline, handles: Handles) -> Self {
        Self {
            pipeline: Mutex::new(pipeline),
            handles: Mutex::new(handles),
        }
    }

    pub fn get(&self) -> Pipeline {
        let Ok(val) = self.pipeline.lock() else {
            return Pipeline::default()
        };
        val.clone()
    }

    pub fn set(&self, updated: Pipeline) {
        let Ok(mut handles) = self.handles.lock() else { return };
        let Ok(mut pipeline) = self.pipeline.lock() else { return };
        *handles = factories().create_pipeline(&updated);
        *pipeline = updated;
    }

    pub fn create_component(&self, component: String, position: Position) {
        let mut pipeline = self.get();
        let id = pipeline.next_id();
        pipeline.nodes.push(Node {
            id,
            node_type: component,
            position,
            data: Default::default(),
        });
        self.set(pipeline);
    }
}
