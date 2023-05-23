use crate::presentation::protocol::{Assignment, Editor, Node};

pub trait Repository {
    fn get(&self) -> Editor;
    fn set(&mut self, updated: Editor);

    fn insert_node(&mut self, node: Node);

    fn add_edge(
        &mut self,
        source: String,
        target: String,
        source_handle: String,
        target_handle: String,
    );
    fn remove_edge(
        &mut self,
        source: String,
        target: String,
        source_handle: String,
        target_handle: String,
    );
    fn set_assignment(&mut self, id: String, assignment: Assignment);
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

    fn add_edge(
        &mut self,
        source: String,
        target: String,
        source_handle: String,
        target_handle: String,
    ) {
        self.editor
            .add_edge(source, target, source_handle, target_handle);
    }

    fn remove_edge(
        &mut self,
        source: String,
        target: String,
        source_handle: String,
        target_handle: String,
    ) {
        self.editor
            .remove_edge(source, target, source_handle, target_handle);
    }

    fn set_assignment(&mut self, id: String, assignment: Assignment) {
        self.editor.set_assignment(id, assignment);
    }
}
