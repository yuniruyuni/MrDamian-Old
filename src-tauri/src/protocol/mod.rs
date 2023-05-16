use serde::{Deserialize, Serialize};
use specta::Type;
#[derive(Type, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Type, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "type")]
    pub node_type: String,
    pub id: String,
    pub data: NodeData,
    pub position: Position,
}

#[derive(Type, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Type, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NodeData {
    pub label: String,
    pub inputs: Vec<InputPort>,
    pub outputs: Vec<OutputPort>,
}

pub type Assign = std::collections::HashMap<String, String>;

#[derive(Type, Debug, Default, Clone, Serialize, Deserialize)]
pub struct InputPort {
    pub name: String,
    pub assign: Assign,
}

#[derive(Type, Debug, Default, Clone, Serialize, Deserialize)]
pub struct OutputPort {
    pub name: String,
    pub assign: Assign,
}

#[derive(Type, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub label: Option<String>,
    pub source: String,
    pub target: String,
    #[serde(rename = "sourceHandle")]
    pub source_handle: String,
    #[serde(rename = "targetHandle")]
    pub target_handle: String,
}

#[derive(Type, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: String,
    pub label: String,
    pub inputs: Vec<InputPort>,
    pub outputs: Vec<OutputPort>,
}
