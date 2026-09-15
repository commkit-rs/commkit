use crate::{ByteTransfer, packet::Packet};

pub trait Message: ByteTransfer {}

// L4, describes how messages are encoded onto and decoded off of a bus in packet form
pub trait Transport {
    type Packet: Packet;
    type Message: Message;
    type Error;
    type Instant: Copy;
    type State;

    fn packet_rx(&mut self, packet: Self::Packet) -> Result<(), Self::Error>;

    fn packet_tx(&mut self) -> Option<Self::Packet>;

    fn write(&mut self, message: Self::Message) -> Result<(), Self::Error>;

    fn read(&mut self) -> Option<Self::Message>;

    fn poll(&mut self, now: Self::Instant) -> Self::State;
}