use std::borrow::Cow;
use crate::{PacketReadError, PacketWriteError};
use std::io::{BufRead, Read, Write};
use bytes::{Bytes, BytesMut};
use rmp::Marker;
use uuid::Uuid;


// Data Types that Implement this trait can be put inside the Packet Content
pub trait PacketContent {
    /// Read the data from the reader
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError>
        where
            Self: Sized;
    /// Write the data to the writer
    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError>
        where
            Self: Sized;
}

impl PacketContent for u8 {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError>
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
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError>
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
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError>
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

impl PacketContent for Uuid {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        let most = rmp::decode::read_u64(reader).map_err(PacketReadError::from)?;
        let least = rmp::decode::read_u64(reader).map_err(PacketReadError::from)?;
        Ok(Uuid::from_u64_pair(most, least))
    }

    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError> where Self: Sized {
        let (most, least) = self.as_u64_pair();
        rmp::encode::write_u64(writer, most).map_err(PacketWriteError::from)?;
        rmp::encode::write_u64(writer, least).map_err(PacketWriteError::from)?;
        Ok(())
    }
}

impl PacketContent for Vec<u8> {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        let len: usize = rmp::decode::read_int(reader).map_err(PacketReadError::from)?;
        let mut vec = Vec::with_capacity(len);
        reader.read_to_end(&mut vec).map_err(PacketReadError::from)?;
        Ok(vec)
    }

    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError> where Self: Sized {
        rmp::encode::write_bin(writer, self.as_ref()).map_err(PacketWriteError::from)?;
        Ok(())
    }
}

impl PacketContent for Bytes {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        let len: usize = rmp::decode::read_int(reader).map_err(PacketReadError::from)?;
        let mut bytes = BytesMut::with_capacity(len);
        reader.take(len as u64).read_exact(&mut bytes).map_err(PacketReadError::from)?;
        Ok(bytes.freeze())
    }

    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError> where Self: Sized {
        rmp::encode::write_bin(writer, self.as_ref()).map_err(PacketWriteError::from)?;
        Ok(())
    }
}

impl PacketContent for bool {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        rmp::decode::read_bool(reader).map_err(PacketReadError::from)
    }

    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError> where Self: Sized {
        rmp::encode::write_bool(writer, self.clone()).map_err(PacketWriteError::from)?;
        Ok(())
    }
}

impl PacketContent for String {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        let len: u32 = rmp::decode::read_str_len(reader).map_err(PacketReadError::from)?;
        let mut vec = Vec::with_capacity(len as usize);
        reader.read_exact(&mut vec).map_err(PacketReadError::from)?;
        Ok(String::from_utf8(vec).map_err(|e| PacketReadError::ContentError(Box::new(e)))?)
    }
    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError> where Self: Sized {
        rmp::encode::write_str(writer, self.as_ref()).map_err(PacketWriteError::from)?;
        Ok(())
    }
}
/// This implemenation takes the the data via a BufRead::fill_buf() call, and then
/// Checks the marker type. If the type is Null return it consumes 1 byte and returns None.
/// If the type is not Null, then it calls the read method on the contained type.
impl<T> PacketContent for Option<T> where T: PacketContent {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        let borred_data = reader.fill_buf().map_err(PacketReadError::from)?;
        let marker = Marker::from_u8(borred_data[0].clone());
        if let Marker::Null = marker{
            reader.consume(1);
            Ok(None)
        }else{
            drop(borred_data); // Return data to the buffer
            let content = T::read(reader)?;
            Ok(Some(content))
        }
    }
    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError> where Self: Sized {
        if let Some(value) = self {
            T::write(value, writer)?;
        } else {
            rmp::encode::write_nil(writer).map_err(PacketWriteError::from)?;
        }
        Ok(())
    }
}
impl< T> PacketContent for Cow<'_, T>  where T: PacketContent + Clone {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        T::read(reader).map(Cow::Owned)
    }
    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError> where Self: Sized {
        T::write(self.as_ref(), writer)
    }
}
impl PacketContent for Cow<'_, str> {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        let len: u32 = rmp::decode::read_str_len(reader).map_err(PacketReadError::from)?;
        let mut vec = Vec::with_capacity(len as usize);
        reader.read_exact(&mut vec).map_err(PacketReadError::from)?;
        Ok(Cow::Owned(String::from_utf8(vec).map_err(|e| PacketReadError::ContentError(Box::new(e)))?))
    }
    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError> where Self: Sized {
        rmp::encode::write_str(writer, self.as_ref()).map_err(PacketWriteError::from)?;
        Ok(())
    }
}