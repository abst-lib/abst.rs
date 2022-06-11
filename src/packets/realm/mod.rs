use uuid::Uuid;
use packet::Packet;
#[derive(Packet)]
pub enum RealmPacket {
    /// Device ID
    #[packet(packet_id = 0)]
    Hello(Uuid),
    /// Device ID of the device you would like to connect to
    #[packet(packet_id = 1)]
    PairRequest(Uuid),
}
