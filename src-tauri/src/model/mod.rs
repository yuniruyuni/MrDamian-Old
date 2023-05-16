pub mod error;

#[derive(Debug, Default, Clone)]
pub struct Kind(pub String);

pub type Argument = String;
pub type PropertyName = String;
pub type Assignment = std::collections::HashMap<Argument, PropertyName>;
pub type PropertyNames = Vec<PropertyName>;

#[derive(Debug, Default, Clone)]
pub struct Pipeline {
    pub components: Vec<Component>,
    pub connections: Vec<Connection>,
}

#[derive(Debug, Default, Clone)]
pub struct Component {
    pub kind: Kind,
    pub id: String,
    pub outputs: Vec<OutputPort>,
    pub inputs: Vec<InputPort>,
}

#[derive(Debug, Default, Clone)]
pub struct Connection {
    pub id: String,
    pub source: InputPortID,
    pub target: OutputPortID,
    pub assignment: Assignment,
}

#[derive(Debug, Default, Clone)]
pub struct InputPortID {
    pub parent: String,
    pub name: String,
}

#[derive(Debug, Default, Clone)]
pub struct InputPort {
    pub id: InputPortID,
    pub property_names: PropertyNames,
}

#[derive(Debug, Default, Clone)]
pub struct OutputPortID {
    pub parent: String,
    pub name: String,
}
#[derive(Debug, Default, Clone)]
pub struct OutputPort {
    pub id: OutputPortID,
    pub property_names: PropertyNames,
}

#[derive(Debug, Default, Clone)]
pub struct Candidate {
    pub kind: Kind,
    pub label: String,
}

pub const PIPELINE_UPDATED: &str = "pipeline-updated";
