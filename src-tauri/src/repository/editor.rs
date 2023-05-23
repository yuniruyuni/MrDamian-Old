use crate::presentation::protocol::{Editor, Node};

pub trait Repository {
    fn get(&self) -> Editor;
    fn set(&mut self, updated: Editor);

    fn insert_node(&mut self, node: Node);
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

    fn insert_node(&mut self, node: Node) {
        self.editor.nodes.push(node);
    }
}
