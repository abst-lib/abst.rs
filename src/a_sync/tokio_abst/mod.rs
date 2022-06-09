use std::io::Read;
use bytes::{Buf, Bytes, BytesMut};
use rmp::decode::read_bin_len;
use rmp::sync;
use rmp::tokio::encode::write_uint;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use crate::{ABSTPacket, Error, PacketData};
use crate::packet::{IntoPacketIdentifier, read_packet_type};
use crate::protocol::{ConnectionType,  DTDViaRealm, DirectConnection};

pub mod tmp;

/// Reads the Packet and decrypts it from the
pub async fn read_packet_raw<Reader: AsyncReadExt + Unpin>(reader: &mut Reader) -> Result<Bytes, Error> {
    // Binary Header
    let result = tmp::read_binary_header(reader).await?;
    let mut contents = BytesMut::with_capacity(result);
    // Check for data that could need to be read
    while contents.len() < result {
        reader.read(&mut contents).await.map_err(crate::Error::from)?;
    }

    Ok(contents.freeze())
}

pub async fn read_packet<Reader: AsyncReadExt + Unpin, CT: ConnectionType>(reader: &mut Reader, connection_type: &DirectConnection) -> Result<(u8, u8, Vec<u8>), Error> {
    let mut result = read_packet_raw(reader).await?.reader();
    //TODO decrypt
    let (protocol, packet) = read_packet_type(&mut result)?;
    let size = read_bin_len(&mut result)? as usize;
    let mut content = Vec::with_capacity(size);
    reader.take(size as u64).read_to_end(&mut content).await.map_err(crate::Error::from)?;
    Ok((protocol, packet, content))
}

/// Writes a Packet to the given Writer
/// Supports any Connection Type
pub async fn send_packet<Writer: AsyncWriteExt + Unpin, PI: IntoPacketIdentifier, PD: PacketData>( writer:&mut  Writer, connection_type: &DirectConnection, pi: PI, data: PD) -> Result<(), Error> {
    let (protocol, packet) = pi.into_packet_identifier();
    let mut content = Vec::new();
    sync::encode::write_u8(&mut content, protocol)?;
    sync::encode::write_u8(&mut content, packet)?;
    data.append_bytes(&mut content)?;

    write_uint(writer, content.len() as u64).await?;
    writer.write_all(&content).await.map_err(crate::Error::from)
}

/// Writes a Packet to the Given Writer. This is for the Device to Realm Connection Type
pub async fn send_packet_to_device<Writer: AsyncWriteExt + Unpin, PI: IntoPacketIdentifier, PD: PacketData>(writer:&mut  Writer, connection_type: &DTDViaRealm, pi: PI, data: PD) -> Result<(), crate::Error> {
    todo!() // TODO take in a device setting reference
}

/// Reads the Packet and decrypts it from the
pub async fn read_packet_from_realm<Reader: AsyncReadExt + Unpin, CT: ConnectionType>(reader: &mut Reader, connection_type: &DTDViaRealm) -> Result<(u8, u8, Uuid, Vec<u8>), Error> {
    let mut reader = read_packet_raw(reader).await?.reader();
    //TODO decrypt
    let (protocol, packet) = read_packet_type(&mut reader)?;
    let (uuid, content)= <(Uuid, Vec<u8>) as PacketData>::from_bytes(&mut reader)?;
    Ok((protocol, packet, uuid, content))

}