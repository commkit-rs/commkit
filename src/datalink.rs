use crate::packet::Packet;

// L2, defines a way for us to get data onto and off of a bus
pub trait DataLink {
    type Packet: Packet;
    type PhysicalConfig;
    type Error;

    /// Emit a packet onto the medium.
    fn send(&mut self, packet: Self::Packet) -> Result<(), Self::Error>;

    /// Take the next received packet, if any.
    fn recv(&mut self) -> Result<Option<Self::Packet>, Self::Error>;
}