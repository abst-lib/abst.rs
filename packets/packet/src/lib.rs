use std::error::Error;

use rmp::decode::ValueReadError;
use rmp::encode::ValueWriteError;
use std::io::Write;

mod content;
pub mod packet;
pub mod protocol;

use crate::packet::Packet;
use crate::protocol::Protocol;
pub use content::PacketContent;

#[derive(Debug, thiserror::Error)]
pub enum PacketWriteError {
    #[error("Failed to write value: {0}")]
    IOError(std::io::Error),
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

#[derive(Debug, thiserror::Error)]
pub enum PacketReadError {
    #[error("Failed to write value: {0}")]
    IOError(std::io::Error),
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

pub trait IntoPacket {
    fn into_packet<Writer: Write>(self, writer: &mut Writer) -> Result<(), PacketWriteError>;
}

impl<Pr: Protocol<ReadError = PacketReadError, WriteError = PacketWriteError>> IntoPacket for Pr {
    fn into_packet<Writer: Write>(self, writer: &mut Writer) -> Result<(), PacketWriteError> {
        self.write_payload(writer)
    }
}

impl IntoPacket for (u8, u8, Vec<u8>) {
    fn into_packet<Writer: Write>(self, _writer: &mut Writer) -> Result<(), PacketWriteError> {
        todo!()
    }
}
impl<Pk: Packet<ReadError = PacketReadError, WriteError = PacketWriteError>> IntoPacket
    for (u8, Pk)
{
    fn into_packet<Writer: Write>(self, _writer: &mut Writer) -> Result<(), PacketWriteError> {
        todo!()
    }
}
