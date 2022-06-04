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

fn get_header<R: Read>(read: &mut R) -> Result<usize, PacketBuildError> {
    let length: usize = match decode::read_marker(read).map_err(|_| PacketBuildError())? {
        Marker::U8 => { decode::read_u8(read).map_err(|_| PacketBuildError())? as usize }
        Marker::U16 => { decode::read_u16(read).map_err(|_| PacketBuildError())? as usize }
        Marker::U32 => { decode::read_u32(read).map_err(|_| PacketBuildError())? as usize }
        Marker::U64 => { decode::read_u64(read).map_err(|_| PacketBuildError())? as usize }
        Marker::I8 => { decode::read_i8(read).map_err(|_| PacketBuildError())? as usize }
        Marker::I16 => { decode::read_i16(read).map_err(|_| PacketBuildError())? as usize }
        Marker::I32 => { decode::read_i32(read).map_err(|_| PacketBuildError())? as usize }
        Marker::I64 => { decode::read_i64(read).map_err(|_| PacketBuildError())? as usize }
        _ => { 0 }
    };
    Ok(length)
}
fn frame_data<B:AsRef<[u8]>>(data: B)-> Result<Vec<u8>, PacketBuildError> {
    let data = data.as_ref();
    let mut result = Vec::new();
    encode::write_u64(&mut result, data.len() as u64).map_err(|_| PacketBuildError())?;
    encode::write_bin(&mut result, data).map_err(|_| PacketBuildError())?;
    Ok(result)
}