/// The functions found within this module are just for sending and receiving packets.
/// Any handling will need to be done in the handlers module.
/// This will handle encryption and decryption.
pub mod a_sync;

pub mod device_manager;
/// Tools for handling the encryption provided via Themis
pub mod encryption;
/// Errors that can occur when sending or receiving packets.
pub mod error;
/// The standard Packet and Protocols established in the ABST Standard
pub mod packets;
/// Tools for Handling different packet paths
/// Such as Device to Device. DTDViaRealm.
pub mod protocol;
pub mod realm;

use bytes::Bytes;
pub use error::Error;
use themis::keys::EcdsaPublicKey;
pub trait ToBytes {
    fn to_bytes(self) -> Bytes;
}
impl ToBytes for EcdsaPublicKey {
    fn to_bytes(self) -> Bytes {
self.as_ref().to_vec().into()
    }
}
