use std::io::{BufRead, Write};
use std::process::id;
use bytes::{Bytes, BytesMut};
use packet::{Packet, PacketReadError, PacketWriteError};
use uuid::Uuid;
use crate::packets::ErrorPacket;
use packet::{PacketContent};

#[derive(Packet)]
pub enum RealmPacket {
    #[packet(packet_id = 0)]
    Heartbeat,
    #[packet(packet_id = 1)]
    Error(ErrorPacket),

    /// Sent to the Realm After connection
    ///
    /// If `public_key_hash` is `None`, the Realm move to a a key exchange state.
    /// else, will verify the key and mov eto Key Check
    ///
    #[packet(packet_id = 2)]
    Hello {
        device_id: Uuid,
        public_key_hash: Option<Bytes>, // Hash of the public key that the realm should have. None if the realm is not paired
    },
    /// Sent from the Realm to the client if the login was successful
    #[packet(packet_id = 4)]
    SendKey {
        public_key: Bytes,
    },
    /// Send to the other device. Only encrypt the data inside the packets. leave the packet id and protocol id alone.
    /// The other side will make sure they can decrypt the data. and do the same result back to you. At this point your session is secure.
    #[packet(packet_id = 5)]
    KeyCheck(Bytes),
    #[packet(packet_id = 6)]
    KeyCheckResponse(bool),
    #[packet(packet_id = 7)]
    DeviceLogin(LoginDetails),
}

/// The login details for the Realm
#[derive(Debug, Clone)]
pub enum LoginDetails {
    /// No Login Details
    None,
    /// Login Details up for implementation details
    Other {
        id: u8,
        details: Bytes,
    },
}

impl PacketContent for LoginDetails {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        match rmp::decode::read_u8(reader)? {
            0 => {
                Ok(LoginDetails::None)
            }
            id => {
                let details = rmp::decode::read_bin_len(reader).map_err(PacketReadError::from)?;
                let mut bytes = BytesMut::with_capacity(details as usize);
                reader.read_exact(&mut bytes).map_err(PacketReadError::from)?;
                let other = LoginDetails::Other {
                    id,
                    details: bytes.freeze(),
                };
                Ok(other)
            }
        }
    }

    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError> where Self: Sized {
        match self {
            LoginDetails::None => {
                rmp::encode::write_u8(writer, 0).map_err(PacketWriteError::from)?;
            }
            LoginDetails::Other { id, details } => {
                rmp::encode::write_u8(writer, *id).map_err(PacketWriteError::from)?;
                rmp::encode::write_bin(writer, details.as_ref()).map_err(PacketWriteError::from)?;
            }
        }
        Ok(())
    }
}