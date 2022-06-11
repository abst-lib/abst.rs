use bytes::Bytes;
use packet::Packet;
use uuid::Uuid;
#[derive(Clone, Packet)]
pub enum DeviceToDevicePackets {
    /// Device ID and are they paired or not
    #[packet(packet_id = 0)]
    Hello(Uuid, bool),
    /// Device ID and the Byte Array containing the Public Key
    #[packet(packet_id = 1)]
    PairRequest(Uuid),
    #[packet(packet_id = 2)]
    KeyCheck,
}
