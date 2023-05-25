use std::collections::HashMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use miette::{IntoDiagnostic, Result};

use super::message::Message;
use super::packet::Packet;
use crate::model::error::MrDamianError;
use crate::model::Assignment;

#[derive(Debug)]
pub struct InputPort {
    pub base_sender: Sender<Packet>,
    pub receiver: Receiver<Packet>,
}

impl InputPort {
    pub fn new() -> InputPort {
        let (base_sender, receiver) = channel::<Packet>(32);
        Self {
            base_sender,
            receiver,
        }
    }

    pub fn accquire(&self, dest: &str, assignment: &Assignment) -> OutputPort {
        OutputPort {
            dest: dest.to_string(),
            sender: self.base_sender.clone(),
            assignment: assignment.clone(),
        }
    }

    pub async fn receive(&mut self) -> Option<Packet> {
        self.receiver.recv().await
    }
}

#[derive(Debug)]
pub struct OutputPort {
    pub dest: String,
    pub sender: Sender<Packet>,
    pub assignment: Assignment,
}

impl OutputPort {
    pub async fn send(&self, message: Message) -> Result<()> {
        let mut assigned = Message::default();
        for (arg, prop) in self.assignment.iter() {
            assigned.insert(arg.clone(), message[prop].clone());
        }

        let packet = Packet {
            port: self.dest.clone(),
            message: assigned,
        };
        self.sender.send(packet).await.into_diagnostic()
    }
}

#[derive(Debug, Default)]
pub struct OutputPorts {
    pub ports: HashMap<String, Vec<OutputPort>>,
}

impl OutputPorts {
    pub fn attach(&mut self, src: &str, port: OutputPort) {
        self.ports
            .entry(src.to_string())
            .or_insert(Vec::new())
            .push(port);
    }

    pub async fn send(&self, packet: Packet) -> Result<()> {
        let port = self
            .ports
            .get(&packet.port)
            .ok_or_else(|| MrDamianError::PortNotFound(packet.port.clone()))?;
        for p in port {
            p.send(packet.message.clone()).await?;
        }
        Ok(())
    }
}
