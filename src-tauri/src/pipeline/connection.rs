use crate::pipeline::packet::Packet;
use crate::pipeline::port::{InputPort, OutputPort, OutputPorts};
use miette::Result;

use super::Component;

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
        src: &mut dyn Component,
        dst: &mut dyn Component,
        src_port: &str,
        dst_port: &str,
    ) {
        let src = src.connection();
        let dst = dst.connection();
        src.attach(src_port, dst.accquire(dst_port))
    }

    fn attach(&mut self, src: &str, port: OutputPort) {
        self.outputs.attach(src, port);
    }

    fn accquire(&mut self, port: &str) -> OutputPort {
        self.input.accquire(port)
    }
}
