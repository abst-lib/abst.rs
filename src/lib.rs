extern crate core;

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


pub use error::Error;