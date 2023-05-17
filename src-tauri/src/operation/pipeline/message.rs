use std::collections::HashMap;

pub type Name = String;

#[derive(Debug, Clone)]
pub enum Property {
    Text(String),
    I64(i64),
}

pub type Message = HashMap<Name, Property>;
