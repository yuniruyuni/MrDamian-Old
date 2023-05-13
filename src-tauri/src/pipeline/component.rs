use async_trait::async_trait;

use crate::pipeline::{Connection, Packet};
use miette::Result;

use crate::protocol::{InputPort, OutputPort};

pub trait Constructor {
    fn construct(&self, config: &crate::config::Config) -> Box<dyn Component + Send>;

    fn component_type(&self) -> String;
    fn label(&self) -> String;
    fn inputs(&self) -> Vec<InputPort>;
    fn outputs(&self) -> Vec<OutputPort>;
}

#[async_trait]
pub trait Component {
    fn name(&self) -> String;
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
