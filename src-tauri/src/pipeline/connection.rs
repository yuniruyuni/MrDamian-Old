use miette::Result;
use crate::pipeline::port::{OutputPorts, InputPort, OutputPort};
use crate::pipeline::packet::Packet;

#[derive(Debug)]
pub struct Connection {
    pub name: String,
    pub input: InputPort,
    pub outputs: OutputPorts,
}

impl Connection {
    pub fn new(name: &str) -> Connection {
        Self {
            name: name.to_string(),
            input: InputPort::new(),
            outputs: OutputPorts::default(),
        }
    }

    pub fn receive(&mut self) -> Result<Packet> {
        self.input.receive()
    }

    pub fn send(&mut self, packet: Packet) -> Result<()> {
        self.outputs.send(packet)
    }

    pub fn connect(src: &mut Connection, dest: &mut Connection, src_port: &str, dst_port: &str) {
        src.attach(src_port, dest.accquire(dst_port))
    }

    fn attach(&mut self, src: &str, port: OutputPort) {
        self.outputs.attach(src, port);
    }

    fn accquire(&mut self, port: &str) -> OutputPort {
        self.input.accquire(port)
    }
}
