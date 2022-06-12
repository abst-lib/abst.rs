use uuid::Uuid;
use crate::encryption::{EncryptionManager, ThemisEncryptionManager};
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
    fn register_device(&mut self, device_id: &Uuid, encryption: Self::EH) -> Result<(), Self::Error>;
    /// Removes a device from the paired devices
    fn delete_device(&mut self, device_id: &Uuid) -> Result<(), Self::Error>;
}

/// The Paired Device
pub trait PairedDevice<EH: EncryptionManager> {
    /// The device UUID
    fn get_device_id(&self) -> &Uuid;
    /// An Encryption Manager.
    /// This value is owned by the caller
    fn get_encryption_manager(&self) -> EH;
}