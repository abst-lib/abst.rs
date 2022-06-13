use bytes::Bytes;
use themis::keys::{EcdsaPrivateKey, EcdsaPublicKey};
use uuid::Uuid;

#[derive(Clone)]
pub enum ConnectionStatus {
    /// The connection is established however needs to be encrypted or paired
    Entry,
    /// You send a pair request and waiting for a response
    PendingPairRequest {
        test: Option<Bytes>,
    },
    /// Current Pairing
    Pairing {
        public_key: EcdsaPublicKey,
        private_key: EcdsaPrivateKey,

        key_b: Option<EcdsaPublicKey>,
        /// The Test String. If None do not test. If it is some It needs to be verified
        test: Option<Bytes>,
    },
    /// The connection is still needing to be encrypted
    PendingEncryption,

    CheckingKeys {
        random_bytes: Bytes,
    },
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
