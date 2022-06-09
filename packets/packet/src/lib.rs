use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Read;
use rmp::decode::ValueReadError;

pub mod protocol;
pub mod packet;

#[derive(Debug)]
pub enum PacketReadError {
    IOError(std::io::Error),
    ContentError(Box<dyn Error + Send + Sync + 'static>),
}

impl Display for PacketReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for PacketReadError {}
impl From<ValueReadError<std::io::Error>> for PacketReadError {
    fn from(value: ValueReadError<std::io::Error>) -> Self {
        match value {
            ValueReadError::InvalidMarkerRead(value) => { PacketReadError::IOError(value) }
            ValueReadError::InvalidDataRead(value) => { PacketReadError::IOError(value) }
            v => PacketReadError::ContentError(Box::new(v))
        }
    }
}

// Data Types that Implement this trait can be put inside the Packet Content
pub trait PacketContent {
    fn read<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized;
}

impl PacketContent for u8 {
    fn read<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {

        rmp::decode::read_u8(reader).map_err(PacketReadError::from)
    }
}

impl PacketContent for u32 {
    fn read<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        rmp::decode::read_u32(reader).map_err(PacketReadError::from)
    }
}

impl PacketContent for u64 {
    fn read<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        rmp::decode::read_u64(reader).map_err(PacketReadError::from)
    }
}