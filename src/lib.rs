pub mod a_sync;
pub mod packet;
mod protocol;
pub mod error;
mod encryption;

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

fn frame_data<B:AsRef<[u8]>>(data: B)-> Result<Vec<u8>, PacketBuildError> {
    let data = data.as_ref();
    let mut result = Vec::new();
    encode::write_u64(&mut result, data.len() as u64).map_err(|_| PacketBuildError())?;
    encode::write_bin(&mut result, data).map_err(|_| PacketBuildError())?;
    Ok(result)
}