use std::borrow::Cow;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::net::{IpAddr, SocketAddr, TcpStream};
use crate::encryption::{Encryption};
use crate::protocol::ConnectionType;

pub mod tokio_abst;

#[derive(Debug)]
pub struct Client<'ct, InnerStream: Debug, CT: ConnectionType, Enc: Encryption> {
    pub(crate) connection: InnerStream,
    pub(crate) connection_type: Cow<'ct, CT>,
    pub(crate) encryption: Cow<'ct, Enc>,
}

impl<'ct, InnerStream:Debug, CT: ConnectionType, Enc: Encryption> Client<'ct, InnerStream, CT, Enc> {
    pub fn change_encryption<NewEnc: Encryption>(self, encryption: NewEnc) -> Client <'ct, InnerStream, CT, NewEnc> {
        Client {
            connection: self.connection,
            connection_type: self.connection_type,
            encryption: Cow::Owned(encryption),
        }
    }
}



/// Represents a Server. That can be connected to.
#[derive(Debug)]
pub struct Server<Listener, Stream> {
    pub(crate) connection: Listener,
    pub(crate) connections: Vec<(Stream, SocketAddr)>,
    pub(crate) _phantom: PhantomData<Stream>,
}

/// Servers have optional fall back to communicate via a Realm. In this scenario. The server is actually a client.
#[derive(Debug)]
pub enum ConnectedDeviceType<'connection> {
    /// ConnectionType is going to be marked as Realm at this time
    Realm,
    /// The server is acting a server.
    DeviceToDevice(&'connection SocketAddr),
}
#[derive(Debug)]
pub struct ConnectedDevice<'connection, Stream, CT: ConnectionType, Enc: Encryption> {
    pub connection: &'connection mut Stream,
    pub connected_type: ConnectedDeviceType<'connection>,
    pub connection_type: Cow<'connection, CT>,
    pub encryption: Cow<'connection, Enc>,
}

impl<'ct, InnerStream, CT: ConnectionType, Enc: Encryption> ConnectedDevice<'ct, InnerStream, CT, Enc> {
    pub fn change_encryption<NewEnc: Encryption>(self, encryption: NewEnc) -> ConnectedDevice<'ct, InnerStream, CT, NewEnc> {
        ConnectedDevice {
            connection: self.connection,
            connected_type: self.connected_type,
            connection_type: self.connection_type,
            encryption: Cow::Owned(encryption),
        }
    }
}
