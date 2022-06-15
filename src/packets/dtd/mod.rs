use bytes::Bytes;
use packet::Packet;
use uuid::Uuid;
use crate::packets::ErrorPacket;

/// Device to Device Packets
/// Packet ID's 0-3 are not encrypted and can only be used before the session is marked as secure
#[derive(Clone, Packet)]
pub enum DeviceToDevicePackets {
    #[packet(packet_id = 0)]
    Heartbeat,
    #[packet(packet_id = 1)]
    Error(ErrorPacket),
    /// Device ID and are they paired or not
    #[packet(packet_id = 2)]
    Hello {
        /// Your device ID
        device_id: Uuid,
        /// If the device is paired or not
        paired: bool,
    },
    /// Device ID and the Byte Array containing the Public Key
    #[packet(packet_id = 3)]
    PairRequest {
        /// Your Device Name
        device_name: String,
        /// Allowed for Implementer Details if they want to
        details: Option<Bytes>,
    },
    /// Send to the other device. They will use this key to send data to you.
    ///
    /// The Optional Test.
    /// If you want to ensure that your key is the one sent to the other side.
    /// Pass a String for the test. This must be encrypted with the same key.
    /// On the other side they will need to know this string. They will use the public key provided to encrypt the known string
    /// If they are test the encrypted test and the known string(encrypted) are the same. The key has not been compromised.
    ///
    /// Once the other side has a received the key they will use the public key to send you their key. And at this point both devices are paired
    /// They can move onto the Key Check to mark the session as secure.
    #[packet(packet_id = 4)]
    SendKey {
        public_key: Bytes,
        test: Option<Bytes>,
    },
    /// Send to the other device. Only encrypt the data inside the packets. leave the packet id and protocol id alone.
    /// The other side will make sure they can decrypt the data. and do the same result back to you. At this point your session is secure.
    #[packet(packet_id = 5)]
    KeyCheck(Bytes),
    #[packet(packet_id = 6)]
    KeyCheckResponse(bool),
}
