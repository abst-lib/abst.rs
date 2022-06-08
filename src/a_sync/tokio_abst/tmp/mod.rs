use ::rmp::Marker;
use ::rmp::decode::{MarkerReadError, ValueReadError};
use rmp::tokio::decode::{read_marker, read_u16, read_u8, read_u32, read_u64, read_i8, read_i16, read_i32, read_i64};
use rmp::tokio::encode::write_uint;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};

pub async fn read_binary_header<Reader: AsyncRead + std::marker::Unpin>(reader: &mut Reader) -> Result<usize, ValueReadError<std::io::Error>> {
    let marker = read_marker(reader).await?;
    let length: usize = match marker {
        Marker::U8 => { read_u8(reader).await? as usize }
        Marker::U16 => { read_u16(reader).await? as usize }
        Marker::U32 => { read_u32(reader).await? as usize }
        Marker::U64 => { read_u64(reader).await? as usize }
        Marker::I8 => { read_i8(reader).await? as usize }
        Marker::I16 => { read_i16(reader).await? as usize }
        Marker::I32 => { read_i32(reader).await? as usize }
        Marker::I64 => { read_i64(reader).await? as usize }
        _ => { 0 }
    };
    Ok(length)
}

