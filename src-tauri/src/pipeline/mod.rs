mod component;
mod connection;
mod handle;
mod message;
mod packet;
mod port;

pub use component::*;
pub use connection::*;
pub use handle::*;
pub use message::*;
pub use packet::*;

use crate::error::MrDamianError;
use crate::protocol::Pipeline;
use hashbrown::HashMap;
use miette::{IntoDiagnostic, Result};

pub struct Factories(HashMap<String, Box<dyn Constructor + Sync>>);

impl Factories {
    pub fn new(cs: Vec<Box<dyn Constructor + Sync>>) -> Self {
        let mut map = HashMap::new();
        for c in cs {
            map.insert(c.component_type(), c);
        }
        Self(map)
    }

    fn create_component(&self, name: &str) -> Result<Box<dyn Component + Send>> {
        if let Some(constructor) = self.0.get(name) {
            Ok(constructor.construct(&crate::config::Config::load_envs()?))
        } else {
            Err(MrDamianError::InvalidComponent).into_diagnostic()
        }
    }

    pub fn create_pipeline(&self, pipeline: &Pipeline) -> Handles {
        let mut components = HashMap::new();
        for node in &pipeline.nodes {
            if let Ok(component) = self.create_component(node.node_type.as_str()) {
                components.insert(node.id.clone(), component);
            }
        }

        for edge in &pipeline.edges {
            let res = components.get_many_mut([edge.source.as_str(), edge.target.as_str()]);
            if let Some([source, target]) = res {
                eprintln!("Connecting {} to {}", edge.source, edge.target);
                Connection::connect(
                    source.as_mut(),
                    target.as_mut(),
                    edge.source_handle.as_str(),
                    edge.target_handle.as_str(),
                );
            }
        }

        let mut handles = Handles::default();
        for (_, mut component) in components {
            eprintln!("Starting {}", component.name());
            let handle = tauri::async_runtime::spawn(async move {
                let res = component.run().await;
                eprintln!("Component {} exited with {:?}", component.name(), res);
                res
            });
            handles.push(handle);
        }
        handles
    }

    pub fn components(&self) -> Vec<crate::protocol::Component> {
        let mut res = vec![];
        for (t, c) in &self.0 {
            res.push(crate::protocol::Component {
                component_type: t.clone(),
                label: c.label(),
                inputs: c.inputs(),
                outputs: c.outputs(),
            });
        }
        res
    }
}
