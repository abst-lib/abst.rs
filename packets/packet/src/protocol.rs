use std::error::Error;
use std::io::Read;
use crate::packet::{Packet};

/// Exists for when you create a bunch of Protocols on a Variant
pub trait Protocol {
    type Error: Error;
    fn supports_protocol_id(id: u8) -> bool;

    fn build_if_supported<Reader: Read>(protocol_id: u8, packet_id: u8, reader: &mut Reader) -> Option<Result<Self, Self::Error>> where Self: Sized;
}

pub mod test {
    pub struct Error;

    impl Debug for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl std::error::Error for Error {}

    use std::fmt::{Debug, Display, Formatter};
    use std::io::Read;
    use crate::packet::Packet;
    use crate::packet::test::PacketVariantTest;
    use crate::protocol::Protocol;

    pub enum ProtocolTest {
        PacketTest(PacketVariantTest),
    }


    impl Protocol for ProtocolTest {
        type Error = Error;

        fn supports_protocol_id(id: u8) -> bool {
            match id {
                0 => true,
                _ => false
            }
        }

        fn build_if_supported<Reader: Read>(protocol_id: u8, packet_id: u8, reader: &mut Reader) -> Option<Result<Self, Self::Error>> where Self: Sized {
            match protocol_id {
                0 => {
                    let value = PacketVariantTest::build_or_none(packet_id, reader);
                    if let Some(build) = value {
                        Some(build.map(|value| ProtocolTest::PacketTest(value)))
                    } else {
                        Some(Err(Error {}))
                    }
                }
                _ => {
                    None
                }
            }
        }
    }
}