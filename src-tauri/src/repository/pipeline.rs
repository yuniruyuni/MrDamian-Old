use crate::model::Pipeline;
use crate::operation::{factory, pipeline::Handles};

pub trait Repository {
    fn get(&self) -> Pipeline;
    fn set(&mut self, updated: Pipeline);
}

pub struct Impl {
    pub pipeline: Pipeline,
    pub handles: Handles,
}

impl Impl {
    pub fn new() -> Self {
        let pipeline = Pipeline::default();
        let handles = factory().create_pipeline(&pipeline);

        Self { pipeline, handles }
    }
}

impl Repository for Impl {
    fn get(&self) -> Pipeline {
        self.pipeline.clone()
    }

    fn set(&mut self, updated: Pipeline) {
        self.handles = factory().create_pipeline(&updated);
        self.pipeline = updated;
    }
}
