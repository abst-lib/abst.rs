use uuid::Uuid;

pub enum RealmPacket {
    /// Device ID
    Hello(Uuid),
    /// Device ID of the device you would like to connect to
    PairRequest(Uuid),
}
