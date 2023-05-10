use crate::pipeline::message::Message;

#[derive(Debug)]
pub struct Packet {
    pub port: String,
    pub message: Message,
}
