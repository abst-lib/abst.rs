use std::fmt::Debug;
use rmpv::Value;
use uuid::Uuid;

pub struct PacketBuildError();

pub trait PacketData: Debug + Send + Sync {
    fn into_value(self) -> Result<Value, PacketBuildError>;
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
}

#[derive(Debug)]
pub struct RealmPacketContent<PD: PacketData> {
    pub target_device: Uuid,
    pub content: ABSTPacket<PD>, // Encrypted
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
}
impl PacketData for Value{
    fn into_value(self) -> Result<Value, PacketBuildError> {
        Ok(self)
    }
}