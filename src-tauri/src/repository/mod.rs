mod pipeline;

pub struct Repositories where {
    pub pipeline: pipeline::PipelineRepositories,
}

impl Repositories {
    pub fn new() -> Self {
        Self {
            pipeline: pipeline::PipelineRepositories::new(),
        }
    }

    // TODO:
    // pub fn mock(&self) -> Self { .. }
}
