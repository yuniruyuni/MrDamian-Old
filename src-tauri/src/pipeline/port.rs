use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

use miette::{IntoDiagnostic, Result};

use crate::error::MrDamianError;
use crate::pipeline::message::Message;
use crate::pipeline::packet::Packet;

#[derive(Debug)]
pub struct InputPort {
    pub base_sender: Sender<Packet>,
    pub receiver: Receiver<Packet>,
}

impl InputPort {
    pub fn new() -> InputPort {
        let (base_sender, receiver) = channel::<Packet>();
        Self {
            base_sender,
            receiver,
        }
    }

    pub fn accquire(&self, dest: &str) -> OutputPort {
        OutputPort {
            dest: dest.to_string(),
            sender: self.base_sender.clone(),
        }
    }

    pub fn receive(&self) -> Result<Packet> {
        self.receiver.recv().into_diagnostic()
    }
}

#[derive(Debug)]
pub struct OutputPort {
    pub dest: String,
    pub sender: Sender<Packet>,
}

impl OutputPort {
    pub fn send(&self, message: Message) -> Result<()> {
        let packet = Packet {
            port: self.dest.clone(),
            message,
        };
        self.sender.send(packet).into_diagnostic()
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

    pub fn send(&self, packet: Packet) -> Result<()> {
        let port = self
            .ports
            .get(&packet.port)
            .ok_or_else(|| MrDamianError::PortNotFound(packet.port.clone()))?;
        for p in port {
            p.send(packet.message.clone())?;
        }
        Ok(())
    }
}
