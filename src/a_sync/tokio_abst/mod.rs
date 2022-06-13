use crate::encryption::EncryptionManager;
use crate::error::Error;
use crate::protocol::{ConnectionType, DTDViaRealm, DirectConnection};
use bytes::{Buf, Bytes, BytesMut};
use packet::{read_packet_type, IntoPacket, PacketContent};
use rmp::decode::read_bin_len;
use rmp::sync;
use rmp::tokio::encode::write_uint;
use std::io::Read;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

pub mod tmp;

/// Reads the Packet and decrypts it from the
pub async fn read_packet_raw<Reader: AsyncReadExt + Unpin>(
    reader: &mut Reader,
) -> Result<Bytes, Error> {
    // Binary Header
    let result = tmp::read_binary_header(reader).await?;
    let mut contents = BytesMut::with_capacity(result);
    // Check for data that could need to be read
    while contents.len() < result {
        reader.read(&mut contents).await.map_err(Error::from)?;
    }

    Ok(contents.freeze())
}

pub async fn read_packet<Reader: AsyncReadExt + Unpin, EM: EncryptionManager>(
    reader: &mut Reader,
    em: &EM,
) -> Result<(u8, u8, Bytes), Error> {
    let mut result = read_packet_raw(reader).await?.reader();
    //TODO decrypt
    let (protocol, packet) = read_packet_type(&mut result)?;
    let size = read_bin_len(&mut result)? as usize;
    let mut content = BytesMut::with_capacity(size);
    reader
        .take(size as u64)
        .read_exact(&mut content)
        .await
        .map_err(Error::from)?;
    Ok((protocol, packet, content.freeze()))
}

/// Writes a Packet to the given Writer
/// Supports any Connection Type
pub async fn send_packet<
    Writer: AsyncWriteExt + Unpin,
    Content: IntoPacket,
    EM: EncryptionManager,
>(
    writer: &mut Writer,
    em: &EM,
    content: Content,
) -> Result<(), Error> {
    let mut payload = Vec::new();
    content.into_packet(&mut payload)?;
    write_uint(writer, payload.len() as u64).await?;
    // TODO encrypt
    writer.write_all(&payload).await.map_err(Error::from)
}

/// Writes a Packet to the Given Writer. This is for the Device to Realm Connection Type
pub async fn send_packet_to_device_via_realm<
    Writer: AsyncWriteExt + Unpin,
    Content: IntoPacket,
    EM: EncryptionManager,
>(
    writer: &mut Writer,
    em: &EM,
    connection_type: &DTDViaRealm,
    content: Content,
) -> Result<(), crate::Error> {
    todo!() // TODO take in a device setting reference
}

/// Reads the Packet and decrypts it from the
pub async fn read_packet_from_realm<Reader: AsyncReadExt + Unpin, EM: EncryptionManager>(
    reader: &mut Reader,
    em: &EM,
) -> Result<(u8, u8, Uuid, Bytes), Error> {
    let mut reader = read_packet_raw(reader).await?.reader();
    //TODO decrypt
    let (protocol, packet) = read_packet_type(&mut reader)?;
    let uuid = PacketContent::read(&mut reader)?;
    read_bin_len(&mut reader)?; //Drop this data
    let bytes = reader.into_inner();

    Ok((protocol, packet, uuid, bytes))
}
