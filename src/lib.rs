

/// The functions found within this module are just for sending and receiving packets.
/// Any handling will need to be done in the handlers module.
/// This will handle encryption and decryption.
pub mod a_sync;

/// The standard Packet and Protocols established in the ABST Standard
pub mod packets;
/// Tools for Handling different packet paths
/// Such as Device to Device. DTDViaRealm.
pub mod protocol;
/// Errors that can occur when sending or receiving packets.
pub mod error;
/// Tools for handling the encryption provided via Themis
pub mod encryption;
pub mod device_manager;


use bytes::Bytes;
use themis::keys::EcdsaPublicKey;
pub use error::Error;
pub trait ToBytes {
    fn to_bytes(self) -> Bytes;
}
impl ToBytes for EcdsaPublicKey{
    fn to_bytes(self) -> Bytes {
        let vec = self.to_bytes();
        Bytes::from(vec)
    }
}
