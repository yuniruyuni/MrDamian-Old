use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Name = String;

pub enum Property {
    Text(String),
    I64(i64),
}

pub type Message = HashMap<Name, Property>;

#[derive(Serialize, Deserialize)]
pub struct Pipeline {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "type")]
    pub node_type: String,
    pub id: String,
    pub data: NodeData,
    pub position: Position,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize)]
pub struct NodeData {
    pub label: String,
    pub inputs: Vec<InputPort>,
    pub outputs: Vec<OutputPort>,
}

#[derive(Serialize, Deserialize)]
pub struct InputPort {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct OutputPort {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub label: String,
    pub source: String,
    pub target: String,
    pub source_handle: String,
    pub target_handle: String,
}
