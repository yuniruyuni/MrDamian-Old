use crate::model::{Node, Pipeline as PipelineModel, Position};

pub trait Pipeline {
    fn get(&self) -> PipelineModel;
    fn set(&mut self, updated: PipelineModel);

    fn create_component(&mut self, component: String, position: Position);
}

pub struct Impl {
    pipeline: PipelineModel,
}

impl Impl {
    pub fn new() -> Self {
        Self { pipeline: PipelineModel::default() }
    }
}

impl Pipeline for Impl {
    fn get(&self) -> PipelineModel { self.pipeline.clone() }

    fn set(&mut self, updated: PipelineModel) {
        self.pipeline = updated;
    }

    fn create_component(&mut self, component: String, position: Position) {
        let id = self.pipeline.next_id();
        self.pipeline.nodes.push(Node {
            id,
            node_type: component,
            position,
            data: Default::default(),
        });
    }
}
