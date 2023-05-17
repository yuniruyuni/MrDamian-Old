use async_trait::async_trait;
use miette::Result;

use super::{Connection, Packet};
use crate::model::{InputPort, OutputPort};

pub type Generator = dyn Fn(&crate::config::Config) -> Box<dyn Component + Send>;

pub struct Constructor {
    pub kind: &'static str,
    pub label: &'static str,
    pub gen: Box<Generator>,
}

#[async_trait]
pub trait Component {
    fn kind(&self) -> &'static str;
    fn label(&self) -> String;
    fn inputs(&self) -> Vec<InputPort>;
    fn outputs(&self) -> Vec<OutputPort>;

    fn connection(&mut self) -> &mut Connection;
    async fn run(&mut self) -> Result<()>;
}

#[async_trait]
pub trait DefaultComponent: Component {
    async fn setup(&mut self) -> Result<()> {
        Ok(())
    }

    async fn default_run(&mut self) -> Result<()> {
        // TODO: implement better error handling.
        self.setup().await?;

        loop {
            let Some(packet) = self.connection().receive().await else {
                return Ok(());
            };
            let packets = self.handler(packet).await?;

            for packet in packets {
                self.connection().send(packet).await?;
            }
        }
    }

    async fn handler(&mut self, packet: Packet) -> Result<Vec<Packet>>;
}

#[async_trait]
pub trait PassiveComponent: Component {
    async fn setup(&mut self) -> Result<()> {
        Ok(())
    }
    async fn default_run(&mut self) -> Result<()> {
        // TODO: implement better error handling.
        self.setup().await?;

        loop {
            let packets = self.handler().await?;
            for packet in packets {
                self.connection().send(packet).await?;
            }
        }
    }

    async fn handler(&mut self) -> Result<Vec<Packet>>;
}
