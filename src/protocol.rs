use crate::Error;
use crate::packet::{ABSTPacket, PacketData};

pub enum Protocol {
    DeviceToDevice,
    DeviceToPeer,
    DeviceToRealm,
    Other(u8),
}

pub trait ConnectionType: Clone {

}

#[derive(Clone)]
pub struct DeviceToDevice {}

impl ConnectionType for DeviceToDevice {

}

#[derive(Clone)]
pub struct DeviceToRealm {

}

impl ConnectionType for DeviceToRealm {

}