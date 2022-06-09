use std::io::Read;
use packet::{PacketContent, PacketReadError};
use packet_derive::Packet;
#[derive(Debug)]
pub struct ComplexPacket {}
#[derive(Debug, Packet)]
pub enum Packets {
    #[packet(packet_id=0)]
    Ping(u8),
    #[packet(packet_id = 1)]
    ComplexPacket(ComplexPacket, u8),
}

impl PacketContent for ComplexPacket{
    fn read<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        todo!()
    }
}
#[test]
pub fn test() {
    let packet = Packets::Ping(1);
    println!("{:?}", packet);
}