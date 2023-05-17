use crate::model::{Pipeline as PipelineModel};
use crate::operation::{factories, pipeline::Handles};

pub trait Pipeline {
    fn get(&self) -> PipelineModel;
    fn set(&mut self, updated: PipelineModel);
}

pub struct Impl {
    pub pipeline: PipelineModel,
    pub handles: Handles,
}

impl Impl {
    pub fn new() -> Self {
        let pipeline = PipelineModel::default();
        let handles = factories().create_pipeline(&pipeline);

        Self {
            pipeline,
            handles,
        }
    }
}

impl Pipeline for Impl {
    fn get(&self) -> PipelineModel { self.pipeline.clone() }

    fn set(&mut self, updated: PipelineModel) {
        self.handles = factories().create_pipeline(&updated);
        self.pipeline = updated;
    }
}
