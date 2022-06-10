use std::error::Error;
use std::io::Write;
use std::io::Read;
use crate::packet::{Packet};

/// Exists for when you create a bunch of Protocols on a Variant
pub trait Protocol {
    type Error: Error;

    fn get_protocol_id(&self) -> u8;

    fn get_packet_id(&self) -> u8;

    /// Writes the Protocol Id then passes it along to the inner value. That has the type of Packet
    fn write_payload<Writer: Write>(self, writer: &mut Writer) -> Result<(), Self::Error>;

    fn supports_protocol_id(id: u8) -> bool;

    fn build_if_supported<Reader: Read>(protocol_id: u8, packet_id: u8, reader: &mut Reader) -> Option<Result<Self, Self::Error>> where Self: Sized;
}

