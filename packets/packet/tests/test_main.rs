use packet::{ PacketReadError, PacketWriteError};
use packet_derive::{Packet, Protocol, PacketContent};
use std::io::{Read, Write};

#[derive(Debug, Protocol)]
pub enum Protocols {
    #[protocol(protocol_id = 0x00)]
    Standard(Packets),
}


#[derive(Debug, Packet)]
pub enum Packets {
    #[packet(packet_id = 0)]
    Ping(u8),
    #[packet(packet_id = 1)]
    ComplexPacket(ComplexPacket, u8),
}

#[derive(Debug, PacketContent)]
pub struct ComplexPacket {
    pub a: u8,
    pub b: u8,
    pub c: u8,
}
#[derive(Debug, PacketContent)]
pub struct InnerPacket(pub u8, pub u8, pub u8);
#[test]
pub fn test() {}
