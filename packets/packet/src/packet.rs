use std::error::Error;
use std::io::{Read, Write};


pub trait Packet {
    type ReadError: Error;
    type WriteError: Error;
    /// Gets the packet id
    fn get_packet_id(&self) -> u8;

    /// Writes the PacketID then the content
    fn write_payload<Writer: Write>(self, writer: &mut Writer) -> Result<(), Self::WriteError>;

    /// Builds itself from the packet ID and the content
    fn build_or_none<Reader: Read>(id: u8, reader: &mut Reader) -> Option<Result<Self, Self::ReadError>> where Self: Sized;
}

