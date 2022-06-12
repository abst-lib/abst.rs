use crate::device_manager::{DeviceManager, PairedDevice};
use crate::encryption;
use crate::encryption::{DynamicEncryptionManager, EncryptionError, EncryptionManager, ThemisEncryptionManager};
use crate::packets::dtd::DeviceToDevicePackets;
use crate::packets::Protocol;
use crate::protocol::{ConnectionStatus, DirectConnection, DTDViaRealm};
use crate::protocol::ConnectionStatus::PendingEncryption;

/// Responses the Handlers can return
pub enum Response {
    /// The connection now has a context. Please pass the Protocol back to the other device
    NewContext {
        message: Protocol,
        new_context: ConnectionContext,
    },
    /// Send this message to the other device to continue the connection
    Message(Protocol),
}

/// Connection Type
pub enum ConnectionType {
    DTDViaRealm(DTDViaRealm),
    DirectConnection(DirectConnection),
}

/// Context for the connection
pub struct ConnectionContext {
    /// The standard encryption manager for the connection
    pub encryption: DynamicEncryptionManager,
    pub status: ConnectionStatus,
    pub connection_type: ConnectionType,
}

/// The Default Protocol Handler. For a receiving device.
/// For a Realm Server please use the RealmHandler(hint: it does not exist yet).
pub struct DefaultProtocolHandler<'dm, Error, PD: PairedDevice<DynamicEncryptionManager>, DM: DeviceManager<Error=Error, PD=PD>> {
    device_manager: &'dm mut DM,
    phantom: std::marker::PhantomData<Error>,
    phantom_pd: std::marker::PhantomData<PD>,
}

impl<'dm, Error, PD: PairedDevice<DynamicEncryptionManager>, DM: DeviceManager<Error=Error, PD=PD>> DefaultProtocolHandler<'dm, Error, PD, DM>
    where Error: std::error::Error + std::convert::From<crate::encryption::EncryptionError> {
    pub fn new(device_manager: &'dm mut DM) -> Self {
        DefaultProtocolHandler {
            device_manager,
            phantom: std::marker::PhantomData,
            phantom_pd: std::marker::PhantomData,
        }
    }
    /// Handles the packet that is a for a device.
    ///
    /// # Panics
    /// Panics if the packet is not a DeviceToDevice Packet.
    pub fn handle_packet_direct_communication(&mut self, packet: Protocol, connection_context: Option<&mut ConnectionContext>) -> Result<Response, Error> {
        match packet {
            Protocol::DeviceToDevice(device_to_device) => self.handle_device_to_device_direct_communication(device_to_device, connection_context),
            Protocol::DeviceToRealm(_) => {
                todo!("Currently realms do not exist in the real world. Just  in André's imagination.")
            }
        }
    }
    fn handle_device_to_device_direct_communication(&mut self, packet: DeviceToDevicePackets, connection_context: Option<&mut ConnectionContext>) -> Result<Response, Error> {
        match packet {
            DeviceToDevicePackets::Hello(other_device, _) => {
                if let Some(_) = connection_context {
                    let is_paired = self.device_manager.is_paired(&other_device);

                    Ok(Response::Message(Protocol::DeviceToDevice(DeviceToDevicePackets::Hello(self.device_manager.get_device_id(), is_paired))))
                } else {
                    let is_paired = self.device_manager.is_paired(&other_device);

                    let context = ConnectionContext {
                        encryption: DynamicEncryptionManager::None,
                        status: ConnectionStatus::PendingEncryption,
                        connection_type: ConnectionType::DirectConnection(DirectConnection {
                            device_id: other_device,
                        }),
                    };
                    Ok(Response::NewContext {
                        message: Protocol::DeviceToDevice(DeviceToDevicePackets::Hello(self.device_manager.get_device_id(), is_paired)),
                        new_context: context,
                    })
                }
            }
            DeviceToDevicePackets::PairRequest(_) => {
                todo!("Handle Hello")
            }
            DeviceToDevicePackets::KeyCheck(check_message) => {
                if let Some(context) = connection_context {
                    if let ConnectionType::DirectConnection(direct) = &context.connection_type {
                        let result = self.device_manager.get_paired_device(&direct.device_id)?;
                        let manager = result.get_encryption_manager();
                        let decrypt_message = manager.decrypt_message(check_message)?;
                        let encrypt_message = manager.encrypt_message(decrypt_message)?;
                        context.status = ConnectionStatus::Connected;

                        let context = ConnectionContext {
                            encryption: manager,
                            status: ConnectionStatus::Connected,
                            connection_type: ConnectionType::DirectConnection(direct.clone()),
                        };
                        Ok(Response::NewContext {
                            message: Protocol::DeviceToDevice(DeviceToDevicePackets::KeyCheck(encrypt_message)),
                            new_context: context,
                        })
                    } else {
                        panic!("Direct Connections only!")
                    }
                } else {
                    todo!("Implement Error Responses")
                }
            }
        }
    }
}