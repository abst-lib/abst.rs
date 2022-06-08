use std::borrow::{Borrow, Cow};
use std::io::Cursor;
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use rmp::decode::read_marker;
use rmp::{encode, sync};
use rmp::tokio::encode::write_uint;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use crate::packet::{ABSTPacket, IntoPacketIdentifier, PacketData};
use crate::{PacketBuildError};
use crate::a_sync::{ConnectedDevice, ConnectedDeviceType, Server};
use crate::a_sync::tokio_abst::tmp;
use crate::encryption::{Encryption, NoEncryption};
use crate::protocol::{ConnectionType, DeviceToDevice, DeviceToRealm};

pub fn new_tokio_server(listener: TcpListener) -> Server<TcpListener, TcpStream> {
    Server {
        connection: listener,
        connections: vec![],
        _phantom: Default::default(),
    }
}

impl Server<TcpListener, TcpStream> {
    pub async fn accept<'connection>(&'connection mut self) -> Result<ConnectedDevice<'connection, TcpStream, DeviceToDevice, NoEncryption>, crate::Error> {
        let (stream, addr) = self.connection.accept().await?;
        self.connections.push((stream, addr));
        //TODO find a better way to do this
        let (stream, addr) = self.connections.last_mut().unwrap();
        Ok(ConnectedDevice {
            connection: stream,
            connection_type: Cow::Owned(DeviceToDevice {}),
            connected_type: ConnectedDeviceType::DeviceToDevice(addr),
            encryption: Cow::Owned(NoEncryption),
        })
    }
}

impl<CT: ConnectionType, Enc: Encryption> ConnectedDevice<'_, TcpStream, CT, Enc> {
    pub async fn receive_packet(&mut self) -> Result<Bytes, crate::Error> {
        let read = super::read_packet(&mut self.connection).await?;
        //TODO decrypt this value
        Ok(read)
    }
}

impl<'ct, CT: ConnectionType, Enc: Encryption> ConnectedDevice<'ct, TcpStream, CT, Enc> {
    pub async fn send_packet<PI: IntoPacketIdentifier, PD: PacketData>(&mut self, pi: PI, data: PD) -> Result<(), crate::Error> {
        let (protocol, packet) = pi.into_packet_identifier();
        let mut content = Vec::new();
        sync::encode::write_u8(&mut content, protocol)?;
        sync::encode::write_u8(&mut content, packet)?;
        data.append_bytes(&mut content)?;
        //TODO encrypt this value

        // First send the length of the packet
        write_uint(&mut self.connection, content.len() as u64).await?;
        // Then send the packet
        self.connection.write_all(&content).await.map_err(crate::Error::from)
    }
}

impl<'ct, Enc: Encryption> ConnectedDevice<'ct, TcpStream, DeviceToRealm, Enc> {
    pub async fn send_packet_to_device<PI: IntoPacketIdentifier, PD: PacketData>(&mut self, pi: PI, data: PD) -> Result<(), crate::Error> {
        todo!()
    }
    pub async fn send_packet_to_peer<PI: IntoPacketIdentifier, PD: PacketData>(&mut self, pi: PI, data: PD) -> Result<(), crate::Error> {
        todo!()
    }
}
