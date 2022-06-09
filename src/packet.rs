use std::fmt::{Debug, Formatter};
use std::io::{Cursor, Error, Read, Write};
use bytes::Bytes;
use light_mp_serde::serializer::{SerializeError, Serializer};
use rmp::decode::ValueReadError;
use rmp::{decode, encode};
use rmp::encode::ValueWriteError;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::DeserializeOwned;

use uuid::Uuid;
use crate::packets::{PacketType, Protocol};

#[derive(Debug)]
pub enum PacketBuildError {
    Serialization(light_mp_serde::serializer::SerializeError),
    ValueWriteError(ValueWriteError<std::io::Error>),
}

impl From<ValueWriteError<std::io::Error>> for PacketBuildError {
    fn from(value: ValueWriteError<std::io::Error>) -> Self {
        PacketBuildError::ValueWriteError(value)
    }
}
impl From<SerializeError> for PacketBuildError {
    fn from(value:SerializeError) -> Self {
        PacketBuildError::Serialization(value)
    }
}

#[derive(Debug)]
pub enum PacketReadError {
    Deserialization(light_mp_serde::deserializer::DeserializeError),
    ValueReadError(ValueReadError<std::io::Error>),
    IOError(std::io::Error),
}

impl From<ValueReadError<std::io::Error>> for PacketReadError {
    fn from(value: ValueReadError<std::io::Error>) -> Self {
        PacketReadError::ValueReadError(value)
    }
}

impl From<std::io::Error> for PacketReadError {
    fn from(value: std::io::Error) -> Self {
        PacketReadError::IOError(value)
    }
}


pub trait PacketData: Send + Sync {
    /// To read the slice from the packet
    fn from_bytes<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized;
    fn into_bytes(self) -> Result<Vec<u8>, PacketBuildError> where Self: Sized {
        let mut vec = Vec::new();
        self.append_bytes(&mut vec)?;
        Ok(vec)
    }
    /// Append the packet data to the given buffer.
    /// # Errors
    /// Returns an error if the packet data could not be appended.
    /// # Returns
    /// The number of bytes appended. *Not Implemented*
    fn append_bytes<Writer: Write>(self, write: &mut Writer) -> Result<usize, PacketBuildError> where Self: Sized;
}

pub trait SerdePacketData: PacketData + Serialize + DeserializeOwned {}

pub fn read_target_from_realm_packet(reader: &mut impl Read) -> Result<Uuid, PacketReadError> {
    let most = decode::read_u64(reader)?;
    let least = decode::read_u64(reader)?;
    Ok(Uuid::from_u64_pair(most, least))
}

impl PacketData for (Uuid, Vec<u8>) {
    fn from_bytes<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        let uuid = read_target_from_realm_packet(reader)?;
        let mut vec = Vec::new();
        reader.read_to_end(&mut vec)?;
        Ok((uuid, vec))
    }

    fn append_bytes<Writer: Write>(self, write: &mut Writer) -> Result<usize, PacketBuildError> where Self: Sized {
        let (uuid, data) = self;
        let (most, least) = uuid.as_u64_pair();
        encode::write_u64(write, most)?;
        encode::write_u64(write, least)?;
        encode::write_bin(write, data.as_ref())?;
        Ok(0) //TODO return the number of bytes written
    }
}

/// Uses the Serde Serializer to serialize the packet data.
impl<Builtin: SerdePacketData> PacketData for Builtin {
    fn from_bytes<Reader: Read>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        todo!("light_mp_serde is not implemented yet")
    }

    fn append_bytes<Writer: Write>(self, write: &mut Writer) -> Result<usize, PacketBuildError> where Self: Sized {
        self.serialize(&mut Serializer::new(write))?;
        Ok(0) //TODO return the number of bytes written
    }
}


#[derive(Debug)]
pub struct ABSTPacket<PD: PacketData> {
    pub protocol: u8,
    // 0 device to device. 1 device to realm. 2 device to value
    pub packet_id: u8,
    pub content: PD,
}


#[derive(Debug)]
pub struct RealmPacketEncrypted {
    pub target_device: Uuid,
    pub content: Vec<u8>, // Encrypted
}


pub fn read_packet_type<Reader: Read>(value: &mut Reader) -> Result<(u8, u8),PacketReadError> {
    let protocol = decode::read_u8(value)?;
    let packet = decode::read_u8(value)?;
    Ok((protocol, packet))
}

pub trait IntoPacketIdentifier {
    fn into_packet_identifier(self) -> (u8, u8);
}

impl IntoPacketIdentifier for (u8, u8) {
    fn into_packet_identifier(self) -> (u8, u8) {
        self
    }
}

impl<P: PacketType> IntoPacketIdentifier for (Protocol, &P){
    fn into_packet_identifier(self) -> (u8, u8) {
        (self.0 as u8, self.1.packet_id())
    }
}