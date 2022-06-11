use std::error::Error;
use std::io::Read;
use std::io::Write;

/// Exists for when you create a bunch of Protocols on a Variant
pub trait Protocol {
    type ReadError: Error;
    type WriteError: Error;
    fn get_protocol_id(&self) -> u8;

    /// Writes the Protocol Id then passes it along to the inner value. That has the type of Packet
    fn write_payload<Writer: Write>(self, writer: &mut Writer) -> Result<(), Self::WriteError>;

    fn supports_protocol_id(id: u8) -> bool;

    fn build_if_supported<Reader: Read>(
        protocol_id: u8,
        packet_id: u8,
        reader: &mut Reader,
    ) -> Option<Result<Self, Self::ReadError>>
    where
        Self: Sized;
}
