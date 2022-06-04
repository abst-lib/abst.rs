use std::io::Cursor;
use tokio::net::{TcpSocket, TcpStream};
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use rmp::{decode, encode, Marker};
use rmpv::encode::write_value;
use rmpv::Value;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use uuid::Uuid;
use crate::a_sync::Client;
use crate::encryption::Encryption;
use crate::{frame_data, get_header};
use crate::packet::{ABSTPacket, PacketBuildError, PacketData};
use crate::protocol::{ConnectionType, DeviceToDevice, DeviceToRealm, Protocol};

impl<'ct, CT: ConnectionType, Enc: Encryption> Client<'ct, TcpStream, CT, Enc> {
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

       todo!("Rebuild Packet")
    }
}

impl<'ct,CT:ConnectionType,  Enc: Encryption> Client<'ct, TcpStream, CT, Enc> {
    pub async fn send_packet<PD: PacketData>(&mut self, data: ABSTPacket<PD>) -> Result<(), crate::Error> {
        let mut content = Vec::new();
        write_value(&mut content, &data.into_value()?).map_err(|_| PacketBuildError())?;
        let content = frame_data(content)?;
        self.connection.write_all(&content).await.map_err(crate::Error::from)
    }
}

impl<'ct, Enc: Encryption> Client<'ct, TcpStream, DeviceToRealm, Enc> {


    pub async fn send_packet_to_device<PD: PacketData>(&mut self, target: Uuid, data: ABSTPacket<PD>) -> Result<(), crate::Error> {
        todo!()
    }
    pub async fn send_packet_to_peer<PD: PacketData>(&mut self, target: Uuid, data: ABSTPacket<PD>) -> Result<(), crate::Error> {
        todo!()
    }
}
