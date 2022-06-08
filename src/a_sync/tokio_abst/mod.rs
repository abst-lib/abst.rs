use std::io::Read;
use bytes::{Bytes, BytesMut};
use tokio::io::AsyncReadExt;
use crate::{ABSTPacket, Error, PacketData};

pub mod client;
pub mod server;
pub mod tmp;

pub async fn read_packet<Reader: AsyncReadExt + Unpin>(reader: &mut Reader) -> Result<Bytes, Error> {
    // Binary Header
    let result = tmp::read_binary_header(reader).await?;
    let mut contents = BytesMut::with_capacity(result);
    // Check for data that could need to be read
    while contents.len() < result {
        reader.read(&mut contents).await.map_err(crate::Error::from)?;
    }

    Ok(contents.freeze())
}

