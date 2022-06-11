use crate::{PacketReadError, PacketWriteError};
use std::io::{Read, Write};

// Data Types that Implement this trait can be put inside the Packet Content
pub trait PacketContent {
    /// Read the data from the reader
    fn read<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError>
    where
        Self: Sized;
    /// Write the data to the writer
    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError>
    where
        Self: Sized;
}

impl PacketContent for u8 {
    fn read<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError>
    where
        Self: Sized,
    {
        rmp::decode::read_u8(reader).map_err(PacketReadError::from)
    }

    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError>
    where
        Self: Sized,
    {
        rmp::encode::write_u8(writer, *self).map_err(PacketWriteError::from)
    }
}

impl PacketContent for u32 {
    fn read<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError>
    where
        Self: Sized,
    {
        rmp::decode::read_u32(reader).map_err(PacketReadError::from)
    }

    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError>
    where
        Self: Sized,
    {
        rmp::encode::write_u32(writer, *self).map_err(PacketWriteError::from)
    }
}

impl PacketContent for u64 {
    fn read<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError>
    where
        Self: Sized,
    {
        rmp::decode::read_u64(reader).map_err(PacketReadError::from)
    }

    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError>
    where
        Self: Sized,
    {
        rmp::encode::write_u64(writer, *self).map_err(PacketWriteError::from)
    }
}
