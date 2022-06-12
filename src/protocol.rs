use uuid::Uuid;
use crate::encryption::{DynamicEncryptionManager, ThemisEncryptionManager};

#[derive(Clone)]
pub enum ConnectionStatus {
    /// The connection is established however needs to be encrypted or paired
    Entry,
    /// Current Pairing
    /// Variables are used by Themis for pairing and securely sharing the keys
    Pairing {

    },
    /// The connection is still needing to be encrypted
    PendingEncryption,
    /// The connection is ready to use.
    Connected,
}

pub trait ConnectionType {}

#[derive(Clone)]
pub struct DirectConnection {
    pub device_id: Uuid,
}

impl ConnectionType for DirectConnection {}


#[derive(Clone)]
pub struct DTDViaRealm {
    pub device_id: Uuid,

}

impl ConnectionType for DTDViaRealm {}
