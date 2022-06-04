use std::borrow::Cow;
use std::marker::PhantomData;
use std::net::SocketAddr;
use crate::encryption::{Encryption};
use crate::protocol::ConnectionType;

pub mod tokio_abst;


pub struct Client<'ct, InnerStream, CT: ConnectionType, Enc: Encryption> {
    pub(crate) connection: InnerStream,
    pub(crate) connection_type: Cow<'ct, CT>,
    pub(crate) encryption: Cow<'ct, Enc>,
}

impl<'ct, InnerStream, CT: ConnectionType, Enc: Encryption> Client<'ct, InnerStream, CT, Enc> {
    pub fn change_encryption<NewEnc: Encryption>(self, encryption: NewEnc) -> Client <'ct, InnerStream, CT, NewEnc> {
        Client {
            connection: self.connection,
            connection_type: self.connection_type,
            encryption: Cow::Owned(encryption),
        }
    }
}


/// Represents a Server. That can be connected to.
pub struct Server<Listener, Stream> {
    pub(crate) connection: Listener,
    _phantom: PhantomData<Stream>,
}

/// Servers have optional fall back to communicate via a Realm. In this scenario. The server is actually a client.
pub enum ConnectedDeviceType {
    /// ConnectionType is going to be marked as Realm at this time
    Realm,
    /// The server is acting a server.
    DeviceToDevice(SocketAddr),
}
pub struct ConnectedDevice<'ct, Stream, CT: ConnectionType, Enc: Encryption> {
    pub(crate) connection: Stream,
    pub(crate) connected_type: ConnectedDeviceType,
    pub(crate) connection_type: Cow<'ct, CT>,
    pub(crate) encryption: Cow<'ct, Enc>,
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
