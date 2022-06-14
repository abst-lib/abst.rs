use std::net::IpAddr;
use uuid::Uuid;
use crate::device_manager::PairedDevice;
use crate::encryption::{EncryptionManager, EncryptionSet};
use crate::packets::realm::LoginDetails;


pub trait Realm {
    /// Error that can be returned
    type Error;
    ///  Encryption Manager
    type EH: EncryptionManager;
    type PD: PairedDevice<Self::EH>;
    fn login(&self, device_id: &Uuid, login: LoginDetails) -> Result<bool, Self::Error>;

    fn is_paired(&self, uuid: &Uuid) -> bool;
    /// Gets the paired devices
    fn get_paired_device<'device>(&self, uuid: &Uuid) -> Result<Vec<&'device Self::PD>, Self::Error>;

}

/// Represents a device that is paired with the realm. On the local side
pub trait DeviceRealmConnection{
    type EH: EncryptionManager;


    fn get_ip(&self) -> &IpAddr;
    /// An Encryption Manager.
    /// This value is owned by the caller
    fn get_encryption_manager(&self) -> Self::EH;
}