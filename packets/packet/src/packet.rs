use std::error::Error;
use std::io::Read;


pub trait Packet {
    type Error: Error;
    fn build_or_none<Reader: Read>(id: u8, reader: &mut Reader) -> Option<Result<Self, Self::Error>> where Self: Sized;
}

pub mod test {
    use std::io::Read;
    use crate::packet::Packet;
    use crate::protocol::test::Error;

    pub enum PacketVariantTest {
        PacketTest(PacketTest),
        PacketTestTwo(u64, u64),
    }

    pub struct PacketTest {
        pub data: String,
    }

    impl Packet for PacketVariantTest {
        type Error = Error;
        fn build_or_none<Reader: Read>(id: u8, reader: &mut Reader) -> Option<Result<Self, Self::Error>> where Self: Sized {
            if id == 0 {
                let mut data = String::new();
                reader.read_to_string(&mut data).unwrap();
                let test = PacketVariantTest::PacketTest(PacketTest {
                    data,
                });
                Some(Ok(test))
            } else {
                None
            }
        }
    }
}