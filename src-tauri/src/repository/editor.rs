use crate::presentation::protocol::{Editor, Node, Position, NodeData};

pub trait Repository {
    fn get(&self) -> Editor;
    fn set(&mut self, updated: Editor);

    fn create_component(&mut self, comp: &dyn crate::operation::pipeline::Component, component: String, position: Position);
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

    fn create_component(&mut self, comp: &dyn crate::operation::pipeline::Component, kind: String, position: Position) {
        let id = self.editor.next_id();
        self.editor.nodes.push(Node {
            id,
            kind,
            position,
            data: NodeData{
                label: comp.label().to_string(),
                inputs: comp.inputs().into_iter().map(|i| i.into()).collect(),
                outputs: comp.outputs().into_iter().map(|o| o.into()).collect(),
            },
        });
    }
}
