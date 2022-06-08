use std::borrow::Cow;
use std::io::Cursor;
use tokio::net::{TcpSocket, TcpStream};
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use rmp::{decode, encode, Marker, sync};
use rmp::tokio::encode::write_uint;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use uuid::Uuid;
use crate::a_sync::Client;
use crate::encryption::{Encryption, NoEncryption};
use crate::a_sync::tokio_abst::tmp;
use crate::packet::{ABSTPacket, IntoPacketIdentifier, PacketBuildError, PacketData};
use crate::protocol::{ConnectionType, DeviceToDevice, DeviceToRealm, Protocol};

pub fn new_tokio_client<CT: ConnectionType>(socket: TcpStream, connection_type: Cow<CT>) -> Client<TcpStream, CT, NoEncryption> {
    Client {
        connection: socket,
        connection_type,
        encryption: Cow::Owned(NoEncryption {}),
    }
}

impl<'ct, CT: ConnectionType, Enc: Encryption> Client<'ct, TcpStream, CT, Enc> {
    pub async fn receive_packet(&mut self) -> Result<Bytes, crate::Error> {
        let read = super::read_packet(&mut self.connection).await?;
        //TODO decrypt this value
        Ok(read)
    }
}


impl<'ct, CT: ConnectionType, Enc: Encryption> Client<'ct, TcpStream, CT, Enc> {
    pub async fn send_packet<PI: IntoPacketIdentifier, PD: PacketData>(&mut self,pi: PI, data: PD) -> Result<(), crate::Error> {
        let (protocol,packet) = pi.into_packet_identifier();
        let mut content = Vec::new();
        sync::encode::write_u8(&mut content, protocol)?;
        sync::encode::write_u8(&mut content, packet)?;
        data.append_bytes(&mut content)?;

        write_uint(&mut self.connection, content.len() as u64).await?;
        self.connection.write_all(&content).await.map_err(crate::Error::from)
    }
}

impl<'ct, Enc: Encryption> Client<'ct, TcpStream, DeviceToRealm, Enc> {
    pub async fn send_packet_to_device<PI: IntoPacketIdentifier, PD: PacketData>(&mut self,pi: PI, data: PD) -> Result<(), crate::Error> {
        todo!()
    }
    pub async fn send_packet_to_peer<PI: IntoPacketIdentifier, PD: PacketData>(&mut self,pi: PI, data: PD) -> Result<(), crate::Error> {
        todo!()
    }
}
