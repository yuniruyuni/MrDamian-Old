pub mod pipeline;
mod twitch;

use hashbrown::HashMap;
use miette::{IntoDiagnostic, Result};

use crate::model::error::MrDamianError;
use crate::model::{Candidate, Kind, Pipeline};
use crate::operation::pipeline::{Component, Connection, Constructor, Handles};

pub struct Factory(HashMap<&'static str, Constructor>);

impl Factory {
    pub fn new(cs: Vec<Constructor>) -> Self {
        let mut map = HashMap::new();
        for c in cs {
            map.insert(c.kind, c);
        }
        Self(map)
    }

    pub fn create_component(&self, kind: &Kind, id: &str) -> Result<Box<dyn Component + Send>> {
        if let Some(c) = self.0.get(kind.0.as_str()) {
            Ok((c.gen)(id, &crate::config::Config::load_envs()?))
        } else {
            Err(MrDamianError::InvalidComponent).into_diagnostic()
        }
    }

    pub fn create_pipeline(&self, pipeline: &Pipeline) -> Handles {
        let mut processes = HashMap::new();
        for mcomp in &pipeline.components {
            if let Ok(ocomp) = self.create_component(&mcomp.kind, mcomp.id.as_str()) {
                let conn = Connection::new();
                let proc = ocomp.spawn();
                processes.insert(mcomp.id.clone(), (conn, proc));
            }
        }

        for conn in &pipeline.connections {
            let res =
                processes.get_many_mut([conn.source.parent.as_str(), conn.target.parent.as_str()]);
            if let Some([source, target]) = res {
                Connection::connect(
                    &mut source.0,
                    &mut target.0,
                    conn.source.name.as_str(),
                    conn.target.name.as_str(),
                    &conn.assignment,
                );
            }
        }

        let mut handles = Handles::default();
        for (_, mut proc) in processes {
            let handle = tauri::async_runtime::spawn(async move {
                let mut inst = proc.1.await?;
                inst.run(&mut proc.0).await
            });
            handles.push(handle);
        }
        handles
    }

    pub fn candidates(&self) -> Vec<Candidate> {
        let mut res = vec![];
        for (_, c) in &self.0 {
            res.push(Candidate {
                kind: Kind(c.kind.to_string()),
                label: c.label.to_string(),
            });
        }
        res
    }
}

pub fn factory() -> Factory {
    Factory::new(vec![
        twitch::PublisherComponent::constructor(),
        twitch::SubscriberComponent::constructor(),
    ])
}
