use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type Name = String;

pub enum Property {
    Text(String),
    I64(i64),
}

pub type Message = HashMap<Name, Property>;

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub data: NodeData,
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
