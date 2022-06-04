use std::borrow::{Borrow, Cow};
use std::io::Cursor;
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use rmp::encode;
use rmpv::encode::write_value;
use rmpv::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use crate::packet::{ABSTPacket, PacketData};
use crate::{frame_data, get_header, PacketBuildError};
use crate::a_sync::{ConnectedDevice, ConnectedDeviceType, Server};
use crate::encryption::{Encryption, NoEncryption};
use crate::protocol::{ConnectionType, DeviceToDevice, DeviceToRealm};


impl<CT: ConnectionType> Server<TcpListener, CT> {
    pub async fn accept(&mut self) -> Result<ConnectedDevice<'_, TcpStream, DeviceToDevice, NoEncryption>, crate::Error> {
        let (stream, addr) = self.connection.accept().await?;
        Ok(ConnectedDevice {
            connection: stream,
            connection_type: Cow::Owned(DeviceToDevice{}),
            connected_type: ConnectedDeviceType::DeviceToDevice(addr),
            encryption: Cow::Owned(NoEncryption)
        })
    }
}

impl<CT: ConnectionType, Enc: Encryption> ConnectedDevice<'_, TcpStream, CT, Enc> {

    pub async fn receive_packet(&mut self) -> Result<ABSTPacket<Value>, crate::Error> {
        // Create the initial reader
        let mut reader = BytesMut::new();
        // Read data
        self.connection.read(&mut reader).await.map_err(crate::Error::from)?;
        // Parse header
        let mut cursor = Cursor::new(&reader);
        let length: usize = get_header(&mut cursor)?;
        // Drops the data that has already been read.
        let mut value = reader.split_off(cursor.position() as usize);
        // Check for data that could need to be read
        while value.len() < length {
            self.connection.read(&mut reader).await.map_err(crate::Error::from)?;
        }
        todo!("Parse the data")
    }
}

impl<'ct,CT:ConnectionType, Enc: Encryption> ConnectedDevice<'ct, TcpStream, CT, Enc> {
    pub async fn send_packet<PD: PacketData>(&mut self, data: ABSTPacket<PD>) -> Result<(), crate::Error> {
        let mut content = Vec::new();
        write_value(&mut content, &data.into_value()?).map_err(|_| PacketBuildError())?;
        let content = frame_data(content)?;
        self.connection.write_all(&content).await.map_err(crate::Error::from)
    }
}

impl<'ct, Enc: Encryption> ConnectedDevice<'ct, TcpStream, DeviceToRealm, Enc> {


    pub async fn send_packet_to_device<PD: PacketData>(&mut self, target: Uuid, data: ABSTPacket<PD>) -> Result<(), crate::Error> {
        todo!()
    }
    pub async fn send_packet_to_peer<PD: PacketData>(&mut self, target: Uuid, data: ABSTPacket<PD>) -> Result<(), crate::Error> {
        todo!()
    }
}
