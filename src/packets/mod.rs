pub mod dtd;
pub mod realm;

pub enum Protocol {
    DeviceToDevice = 0,
    DeviceToPeer = 1,
    DeviceToRealm = 2,
}

pub trait PacketType {
    fn packet_id(&self) -> u8;
}