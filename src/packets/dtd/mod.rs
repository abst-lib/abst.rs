use bytes::Bytes;
use uuid::Uuid;

#[derive(Clone)]
pub enum DeviceToDevicePackets{
    /// Device ID and are they paired or not
    Hello(Uuid, bool),
    /// Device ID and the Byte Array containing the Public Key
    PairRequest(Uuid, Bytes),
    KeyCheck
}