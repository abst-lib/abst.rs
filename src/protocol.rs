use uuid::Uuid;
use crate::encryption::ThemisEncryptionManager;
use crate::Error;
#[derive(Clone)]
pub enum ConnectionStatus {
    /// The connection is established however needs to be encrypted or paired
    Entry,
    /// Current Pairing
    Pairing,
    /// The connection is still needing to be encrypted
    PendingEncryption,
    /// The connection is ready to use.
    Connected,
}

pub trait ConnectionType: Clone {}

#[derive(Clone)]
pub struct DirectConnection {
    pub encryption: ThemisEncryptionManager,
    pub device_id: Uuid,
    pub status: ConnectionStatus,
}

impl ConnectionType for DirectConnection {}


#[derive(Clone)]
pub struct DTDViaRealm {
    pub device_id: Uuid,
    pub encryption: ThemisEncryptionManager,

}

impl ConnectionType for DTDViaRealm {}
