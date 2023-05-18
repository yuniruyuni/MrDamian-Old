use async_trait::async_trait;
use miette::Result;

use super::{Connection, Packet};
use crate::model::{InputPort, OutputPort};

pub type Generator = dyn Fn(&str, &crate::config::Config) -> Box<dyn Component + Send>;

pub struct Constructor {
    pub kind: &'static str,
    pub label: &'static str,
    pub gen: Box<Generator>,
}

pub trait Component {
    fn id(&self) -> String;
    fn kind(&self) -> &'static str;
    fn label(&self) -> &'static str;
    fn inputs(&self) -> Vec<InputPort>;
    fn outputs(&self) -> Vec<OutputPort>;

    fn spawn(&self) -> ProcessInit;
}

#[async_trait]
pub trait Process {
    async fn run(&mut self, conn: &mut Connection) -> Result<()>;
}

pub type ProcessInit =
    std::pin::Pin<Box<dyn core::future::Future<Output = Result<Box<dyn Process + Send>>> + Send>>;

#[async_trait]
pub trait DefaultProcess: Process {
    async fn default_run(&mut self, conn: &mut Connection) -> Result<()> {
        loop {
            let Some(packet) = conn.receive().await else {
                return Ok(());
            };
            let packets = self.handler(packet).await?;

            for packet in packets {
                conn.send(packet).await?;
            }
        }
    }

    async fn handler(&mut self, packet: Packet) -> Result<Vec<Packet>>;
}

#[async_trait]
pub trait PassiveProcess: Process {
    async fn passive_run(&mut self, connection: &mut Connection) -> Result<()> {
        loop {
            let packets = self.handler().await?;
            for packet in packets {
                connection.send(packet).await?;
            }
        }
    }

    async fn handler(&mut self) -> Result<Vec<Packet>>;
}
