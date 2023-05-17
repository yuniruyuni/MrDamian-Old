use crate::presentation::protocol::{Editor, Node, Position};

pub trait Repository {
    fn get(&self) -> Editor;
    fn set(&mut self, updated: Editor);

    fn create_component(&mut self, component: String, position: Position);
}

pub struct Impl {
    editor: Editor,
}

impl Impl {
    pub fn new() -> Self {
        Self {
            editor: Editor::default(),
        }
    }
}

impl Repository for Impl {
    fn get(&self) -> Editor {
        self.editor.clone()
    }

    fn set(&mut self, updated: Editor) {
        self.editor = updated;
    }

    fn create_component(&mut self, kind: String, position: Position) {
        let id = self.editor.next_id();
        self.editor.nodes.push(Node {
            id,
            kind,
            position,
            data: Default::default(),
        });
    }
}
