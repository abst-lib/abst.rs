use std::fmt::Debug;
use std::io::{Cursor};
use bytes::Bytes;

use rmpv::decode::read_value;
use rmpv::Value;
use uuid::Uuid;

pub struct PacketBuildError();

pub trait PacketData: Debug + Send + Sync {
    fn into_value(self) -> Result<Value, PacketBuildError>;

    fn from_bytes(bytes: Bytes) -> Result<Self, PacketBuildError>
        where
            Self: Sized;
}


#[derive(Debug)]
pub struct ABSTPacket<PD: PacketData> {
    pub protocol: u8,
    // 0 device to device. 1 device to realm. 2 device to value
    pub packet_id: u8,
    pub content: PD,
}


impl<PD: PacketData> PacketData for ABSTPacket<PD> {
    fn into_value(self) -> Result<Value, PacketBuildError> {
        let mut convert = Vec::new();
        rmp::encode::write_u8(&mut convert, self.protocol).map_err(|_| PacketBuildError())?;
        rmp::encode::write_u8(&mut convert, self.packet_id).map_err(|_| PacketBuildError())?;
        let value = self.content.into_value()?;
        rmpv::encode::write_value(&mut convert, &value).map_err(|_| PacketBuildError())?;
        Ok(Value::Binary(convert))
    }

    fn from_bytes(bytes:Bytes) -> Result<Self, PacketBuildError> where Self: Sized {
        let mut cursor = Cursor::new(bytes);
        let protocol = rmp::decode::read_u8(&mut cursor).map_err(|_| PacketBuildError())?;
        let packet_id = rmp::decode::read_u8(&mut cursor).map_err(|_| PacketBuildError())?;
        let content = rmpv::decode::read_value(&mut cursor).map_err(|_| PacketBuildError()).and_then(to_bytes).map(Bytes::from)?;

        let content = PD::from_bytes(content)?;
        Ok(ABSTPacket {
            protocol,
            packet_id,
            content,
        })
    }
}

#[derive(Debug)]
pub struct RealmPacketContent<PD: PacketData> {
    pub target_device: Uuid,
    pub content: ABSTPacket<PD>, // Encrypted
}

#[derive(Debug)]
pub struct RealmPacketEncrypted {
    pub target_device: Uuid,
    pub content: Vec<u8>, // Encrypted
}

impl<PD: PacketData> PacketData for RealmPacketContent<PD> {
    fn into_value(self) -> Result<Value, PacketBuildError> {
        let (most, least) = self.target_device.as_u64_pair();
        let mut convert = Vec::new();
        rmp::encode::write_u64(&mut convert, most).map_err(|_| PacketBuildError())?;
        rmp::encode::write_u64(&mut convert, least).map_err(|_| PacketBuildError())?;
        let value = self.content.into_value()?;
        rmpv::encode::write_value(&mut convert, &value).map_err(|_| PacketBuildError())?;
        Ok(Value::Binary(convert))
    }

    fn from_bytes(bytes: Bytes) -> Result<Self, PacketBuildError> where Self: Sized {
        panic!("You can not decrypt this here")
    }
}

impl PacketData for RealmPacketEncrypted {
    fn into_value(self) -> Result<Value, PacketBuildError> {
        panic!("You can not encrypt this here")
    }

    fn from_bytes(bytes: Bytes) -> Result<Self, PacketBuildError> where Self: Sized {
        let mut cursor = Cursor::new(bytes);

        let most = rmpv::decode::read_value(&mut cursor).map_err(|_| PacketBuildError()).and_then(to_u64)?;
        let least = rmpv::decode::read_value(&mut cursor).map_err(|_| PacketBuildError()).and_then(to_u64)?;
        let uuid = Uuid::from_u64_pair(most, least);
        let content = rmpv::decode::read_value(&mut cursor).map_err(|_| PacketBuildError()).and_then(to_bytes)?;
        Ok(RealmPacketEncrypted {
            target_device: uuid,
            content,
        })
    }
}

fn to_u64(value: Value) -> Result<u64, PacketBuildError> {
    match value {
        Value::Integer(value) => {
            if value.is_i64() {
                Ok(value.as_i64().unwrap() as u64)
            } else {
                Ok(value.as_u64().unwrap())
            }
        }
        _ => { Err(PacketBuildError()) }
    }
}

fn to_bytes(value: Value) -> Result<Vec<u8>, PacketBuildError> {
    match value {
        Value::Binary(value) => { Ok(value) }
        _ => { return Err(PacketBuildError()); }
    }
}

impl PacketData for Value {
    fn into_value(self) -> Result<Value, PacketBuildError> {
        Ok(self)
    }

    fn from_bytes(bytes: Bytes) -> Result<Self, PacketBuildError> where Self: Sized {
        Self::from_bytes(bytes)
    }
}

