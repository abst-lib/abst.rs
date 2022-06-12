use std::error::Error;

use rmp::decode::{NumValueReadError, ValueReadError};
use rmp::encode::ValueWriteError;
use std::io::{Read, Write};

mod content;
pub mod packet;
pub mod protocol;

pub use packet_derive::{Packet, Protocol, PacketContent};
use crate::packet::Packet;
use crate::protocol::Protocol;
pub use content::PacketContent;

/// A Write Error for a Packet
#[derive(Debug, thiserror::Error)]
pub enum PacketWriteError {
    /// IO Error
    #[error("Failed to write value: {0}")]
    IOError(std::io::Error),
    /// Any internal error
    #[error("Failed to write value: {0}")]
    ContentError(Box<dyn Error + Send + Sync + 'static>),
}

impl From<ValueWriteError<std::io::Error>> for PacketWriteError {
    fn from(value: ValueWriteError<std::io::Error>) -> Self {
        match value {
            ValueWriteError::InvalidDataWrite(error) => PacketWriteError::IOError(error),
            ValueWriteError::InvalidMarkerWrite(error) => PacketWriteError::IOError(error),
        }
    }
}

impl From<std::io::Error> for PacketWriteError {
    fn from(value: std::io::Error) -> Self {
        PacketWriteError::IOError(value)
    }
}

/// A Read Error for a Packet
#[derive(Debug, thiserror::Error)]
pub enum PacketReadError {
    /// IO Error
    #[error("Failed to write value: {0}")]
    IOError(std::io::Error),
    /// Could be any internal Error
    #[error("Failed to write value: {0}")]
    ContentError(Box<dyn Error + Send + Sync + 'static>),
}

impl From<ValueReadError<std::io::Error>> for PacketReadError {
    fn from(value: ValueReadError<std::io::Error>) -> Self {
        match value {
            ValueReadError::InvalidMarkerRead(value) => PacketReadError::IOError(value),
            ValueReadError::InvalidDataRead(value) => PacketReadError::IOError(value),
            v => PacketReadError::ContentError(Box::new(v)),
        }
    }
}

impl From<NumValueReadError<std::io::Error>> for PacketReadError {
    fn from(value: NumValueReadError<std::io::Error>) -> Self {
        match value {
            NumValueReadError::InvalidMarkerRead(value) => PacketReadError::IOError(value),
            NumValueReadError::InvalidDataRead(value) => PacketReadError::IOError(value),
            v => PacketReadError::ContentError(Box::new(v)),
        }
    }
}

impl From<std::io::Error> for PacketReadError {
    fn from(value: std::io::Error) -> Self {
        PacketReadError::IOError(value)
    }
}

/// A Type that can be turned into an Packet
pub trait IntoPacket {
    fn into_packet<Writer: Write>(self, writer: &mut Writer) -> Result<(), PacketWriteError>;
}

impl<Pr: Protocol<ReadError=PacketReadError, WriteError=PacketWriteError>> IntoPacket for Pr {
    fn into_packet<Writer: Write>(self, writer: &mut Writer) -> Result<(), PacketWriteError> {
        self.write_payload(writer)
    }
}

impl IntoPacket for (u8, u8, Vec<u8>) {
    fn into_packet<Writer: Write>(self, writer: &mut Writer) -> Result<(), PacketWriteError> {
        rmp::encode::write_u8(writer, self.0)?;
        rmp::encode::write_u8(writer, self.1)?;
        rmp::encode::write_bin(writer, &self.2)?;
        Ok(())
    }
}

impl<Pk: Packet<ReadError=PacketReadError, WriteError=PacketWriteError>> IntoPacket
for (u8, Pk)
{
    fn into_packet<Writer: Write>(self, writer: &mut Writer) -> Result<(), PacketWriteError> {
        rmp::encode::write_u8(writer, self.0)?;
        self.1.write_payload(writer)
    }
}

pub fn read_packet_type<Reader: Read>(value: &mut Reader) -> Result<(u8, u8), PacketReadError> {
    let protocol = rmp::decode::read_u8(value)?;
    let packet = rmp::decode::read_u8(value)?;
    Ok((protocol, packet))
}

