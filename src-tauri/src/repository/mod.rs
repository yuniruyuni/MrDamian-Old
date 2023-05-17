mod editor;
mod pipeline;

pub struct Repositories {
    pub editor: Box<dyn editor::Repository + Send>,
    pub pipeline: Box<dyn pipeline::Repository + Send>,
}

impl Repositories {
    pub fn new() -> Self {
        Self {
            editor: Box::new(editor::Impl::new()),
            pipeline: Box::new(pipeline::Impl::new()),
        }
    }

    // TODO:
    // pub fn mock(&self) -> Self { .. }
}
