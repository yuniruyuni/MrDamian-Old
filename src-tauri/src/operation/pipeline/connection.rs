use miette::Result;

use super::packet::Packet;
use super::port::{InputPort, OutputPort, OutputPorts};
use crate::model::Assignment;

#[derive(Debug)]
pub struct Connection {
    pub input: InputPort,
    pub outputs: OutputPorts,
}

impl Connection {
    pub fn new() -> Connection {
        Self {
            input: InputPort::new(),
            outputs: OutputPorts::default(),
        }
    }

    pub async fn receive(&mut self) -> Option<Packet> {
        self.input.receive().await
    }

    pub async fn send(&mut self, packet: Packet) -> Result<()> {
        self.outputs.send(packet).await
    }

    pub fn connect(
        src: &mut Connection,
        dst: &mut Connection,
        src_port: &str,
        dst_port: &str,
        assignment: &Assignment,
    ) {
        src.attach(src_port, dst.accquire(dst_port, assignment))
    }

    fn attach(&mut self, src: &str, port: OutputPort) {
        self.outputs.attach(src, port);
    }

    fn accquire(&mut self, port: &str, assigment: &Assignment) -> OutputPort {
        self.input.accquire(port, assigment)
    }
}
