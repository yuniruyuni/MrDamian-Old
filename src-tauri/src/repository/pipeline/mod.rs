pub mod running;
pub mod editing;

pub struct PipelineRepositories {
    pub running: Box<dyn running::Pipeline + Send>,
    pub editing: Box<dyn editing::Pipeline + Send>,
}

impl PipelineRepositories {
    pub fn new() -> Self {
        Self {
            running: Box::new(running::Impl::new()),
            editing: Box::new(editing::Impl::new()),
        }
    }
}
