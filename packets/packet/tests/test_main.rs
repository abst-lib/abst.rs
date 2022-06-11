use packet::{PacketContent, PacketReadError, PacketWriteError};
use packet_derive::{Packet, Protocol};
use std::io::{Read, Write};

#[derive(Debug, Protocol)]
pub enum Protocols {
    #[protocol(protocol_id = 0x00)]
    Standard(Packets),
}

#[derive(Debug)]
pub struct ComplexPacket {}

#[derive(Debug, Packet)]
pub enum Packets {
    #[packet(packet_id = 0)]
    Ping(u8),
    #[packet(packet_id = 1)]
    ComplexPacket(ComplexPacket, u8),
}

impl PacketContent for ComplexPacket {
    fn read<Reader: Read>(_reader: &mut Reader) -> Result<Self, PacketReadError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn write<Writer: Write>(&self, _writer: &mut Writer) -> Result<(), PacketWriteError>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[test]
pub fn test() {
    let packet = Packets::Ping(1);
    println!("{:?}", packet);
}
