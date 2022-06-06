#[macro_use]
mod rmp;
pub mod encode;

use ::rmp::Marker;
use ::rmp::decode::{MarkerReadError, ValueReadError};
use rmpv::decode;
use tokio::io::{AsyncRead, AsyncReadExt};

pub async fn read_marker<Reader: AsyncRead+ std::marker::Unpin>(reader: &mut Reader) -> Result<Marker,ValueReadError<std::io::Error>> {

    let marker = Marker::from_u8(read_u8(reader).await.map_err( ValueReadError::from)?);
    Ok(marker)
}
pub async fn read_binary_header<Reader: AsyncRead+ std::marker::Unpin>(reader: &mut Reader) -> Result<usize,ValueReadError<std::io::Error>> {
    let marker = read_marker(reader).await?;
    let length: usize = match marker{
        Marker::U8 => { read_u8(reader).await? as usize }
        Marker::U16 => { read_data_u16(reader).await? as usize }
        Marker::U32 => { read_data_u32(reader).await? as usize }
        Marker::U64 => { read_data_u64(reader).await? as usize }
        Marker::I8 => { read_i8(reader).await? as usize }
        Marker::I16 => { read_data_i16(reader).await? as usize }
        Marker::I32 => { read_data_i32(reader).await? as usize }
        Marker::I64 => { read_data_i64(reader).await? as usize }
        _ => { 0 }
    };
    Ok(length)
}

pub async fn read_u8<Reader: AsyncRead + std::marker::Unpin>(reader: &mut Reader) -> Result<u8,ValueReadError<std::io::Error>> {
    let mut buf = [0; 1];
    reader.read_exact(&mut buf).await.map_err(ValueReadError::InvalidDataRead)?;
    Ok(buf[0])
}
pub async fn read_i8<Reader: AsyncRead + std::marker::Unpin>(reader: &mut Reader) -> Result<i8,ValueReadError<std::io::Error>> {
    let mut buf = [0; 1];
    reader.read_exact(&mut buf).await.map_err(ValueReadError::InvalidDataRead)?;
    Ok(buf[0] as i8)
}
// Code from https://github.com/3Hren/msgpack-rust/blob/master/rmp/src/decode/mod.rs#L118
read_byteorder_utils!(
        read_data_u16 => u16,
        read_data_u32 => u32,
        read_data_u64 => u64,
        read_data_i16 => i16,
        read_data_i32 => i32,
        read_data_i64 => i64,
    );
