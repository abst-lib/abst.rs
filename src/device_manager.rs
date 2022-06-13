use std::io::Cursor;
use bytes::Bytes;
use themis::keys::EcdsaKeyPair;
use uuid::{Uuid};
use crate::encryption::{EncryptionManager, EncryptionSet, ThemisEncryptionManager};
use crate::Error;

/// The Manger of Paired Devices
pub trait DeviceManager {
    /// Error that can be returned
    type Error;
    ///  Encryption Manager
    type EH: EncryptionManager;
    /// The Paired Device Type
    type PD: PairedDevice<Self::EH>;
    /// Gets the Current Device ID
    fn get_device_id(&self) -> Uuid;
    /// Gets the current device name
    fn get_device_name(&self) -> String;
    /// Rather or not the uuid is paired
    fn is_paired(&self, uuid: &Uuid) -> bool;
    /// Gets the paired devices
    fn get_paired_devices<'device>(&self) -> Result<Vec<&'device Self::PD>, Self::Error>;
    /// Gets the paired device
    fn get_paired_device<'device>(&self, device_id: &Uuid) -> Result<&'device Self::PD, Self::Error>;
    /// Registers a device after successful pairing
    fn register_device(&mut self, device_id: &Uuid, encryption: EncryptionSet) -> Result<(), Self::Error>;
    /// Removes a device from the paired devices
    fn delete_device(&mut self, device_id: &Uuid) -> Result<(), Self::Error>;

    /// This is called by the handler when a pair request happens. It is your job to ask the user if they want to pair.
    ///
    /// # Returns
    /// Returns true if the user accepted the pairing request.
    /// Returns false if the user rejected the pairing request.
    /// Option<Bytes> is the Test value defined in send key.
    fn pair_request(&self, device_id: &Uuid, device_name: &str, cursor: Cursor<Bytes>) -> Result<(bool, Option<Bytes>), Self::Error>;


}

/// The Paired Device
pub trait PairedDevice<EH: EncryptionManager> {
    /// The device UUID
    fn get_device_id(&self) -> &Uuid;
    /// An Encryption Manager.
    /// This value is owned by the caller
    fn get_encryption_manager(&self) -> EH;
}