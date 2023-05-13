mod component;
mod connection;
mod message;
mod packet;
mod port;
mod handle;

pub use component::*;
pub use connection::*;
pub use message::*;
pub use packet::*;
pub use handle::*;

use miette::{IntoDiagnostic, Result};
use hashbrown::HashMap;
use crate::protocol::Pipeline;
use crate::error::MrDamianError;

fn create_component(name: &str) -> Result<Box<dyn Component + Send>> {
    let config = crate::config::Config::load_envs()?;
    match name {
        "TwitchSubscriber" => Ok(Box::new(crate::twitch::Subscriber::new(
            &config.bot,
            &config.channel,
            &config.token,
        ))),
        "TwitchPublisher" => Ok(Box::new(crate::twitch::Publisher::new(
            &config.bot,
            &config.channel,
            &config.token,
        ))),
        _ => Err(MrDamianError::InvalidComponent).into_diagnostic(),
    }
}

pub fn create_pipeline(pipeline: &Pipeline) -> Handles {
    let mut components = HashMap::new();
    for node in &pipeline.nodes {
        if let Ok(component) = create_component(node.node_type.as_str()) {
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

