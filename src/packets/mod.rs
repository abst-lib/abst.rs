pub mod dtd;
/// Default Handlers for the packets established here
pub mod handlers;
pub mod realm;

use crate::packets::dtd::DeviceToDevicePackets;
use crate::packets::realm::RealmPacket;
use packet::Protocol;

#[derive(Protocol)]
pub enum Protocol {
    #[protocol(protocol_id = 0x00)]
    DeviceToDevice(DeviceToDevicePackets),
    //#[protocol(protocol_id = 1)]
    //DeviceToPeer (),
    #[protocol(protocol_id = 0x02)]
    DeviceToRealm(RealmPacket),
}

pub trait PacketType {
    fn packet_id(&self) -> u8;
}
