extern crate core;

pub mod a_sync;
pub mod packet;
pub mod protocol;
pub mod error;
pub mod encryption;
pub mod packets;

use std::io::Read;
use bytes::Bytes;
use rmp::{decode, encode, Marker};
pub use error::Error;
use crate::packet::{ABSTPacket, PacketBuildError, PacketData};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
