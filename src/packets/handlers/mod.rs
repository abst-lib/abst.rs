use std::io::Cursor;
use bytes::Bytes;
use rand::Rng;
use themis::keygen::gen_ec_key_pair;
use themis::keys::{EcdsaKeyPair, EcdsaPublicKey, KeyPair};
use themis::secure_message::SecureMessage;
use crate::device_manager::{DeviceManager, PairedDevice};
use crate::encryption::{DynamicEncryptionManager, EncryptionError, EncryptionManager, EncryptionSet};
use crate::packets::dtd::DeviceToDevicePackets;
use crate::packets::Protocol;
use crate::protocol::{ConnectionStatus, DirectConnection, DTDViaRealm};

/// Responses the Handlers can return
pub enum Response {
    /// The connection now has a context. Please pass the Protocol back to the other device
    NewContext {
        message: Protocol,
        new_context: ConnectionContext,
    },
    /// Send this message to the other device to continue the connection
    Message(Protocol),
    Nothing,
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
            DeviceToDevicePackets::Hello { device_id, .. } => {
                if let Some(_) = connection_context {
                    let is_paired = self.device_manager.is_paired(&device_id);

                    Ok(Response::Message(Protocol::DeviceToDevice(DeviceToDevicePackets::Hello { device_id: self.device_manager.get_device_id(), paired: is_paired })))
                } else {
                    let is_paired = self.device_manager.is_paired(&device_id);

                    let context = ConnectionContext {
                        encryption: DynamicEncryptionManager::None,
                        status: ConnectionStatus::PendingEncryption,
                        connection_type: ConnectionType::DirectConnection(DirectConnection {
                            device_id,
                        }),
                    };
                    Ok(Response::NewContext {
                        message: Protocol::DeviceToDevice(DeviceToDevicePackets::Hello { device_id: self.device_manager.get_device_id(), paired: is_paired }),
                        new_context: context,
                    })
                }
            }
            DeviceToDevicePackets::PairRequest { device_name, details } => {
                if let Some(context) = connection_context {
                    if let ConnectionType::DirectConnection(direct) = &context.connection_type {
                        let details = if let Some(details) = details {
                            Cursor::new(details)
                        } else {
                            Cursor::default()
                        };
                        let (request, test) = self.device_manager.pair_request(&direct.device_id, &device_name, details)?;
                        if request {
                            let pair = gen_ec_key_pair();


                            let encrypted_test = if let Some(test) = &test {
                                let message = SecureMessage::new(pair.clone());
                                let bytes = message.encrypt(test.as_ref()).map_err(|e| EncryptionError::from(e))?;
                                Some(Bytes::from(bytes))
                            } else { None };
                            let (private, public) = pair.split();
                            let public_key = crate::ToBytes::to_bytes(public.clone());
                            context.status = ConnectionStatus::Pairing {
                                public_key: public,
                                private_key: private,
                                key_b: None,
                                test,
                            };

                            Ok(Response::Message(Protocol::DeviceToDevice(DeviceToDevicePackets::SendKey {
                                public_key,
                                test: encrypted_test,
                            })))
                        } else {
                            todo!("Say No and Disconnect")
                        }
                    } else {
                        panic!("Direct Connection Only!!!!")
                    }
                } else {
                    todo!("Implement Error")
                }
            }
            DeviceToDevicePackets::SendKey { public_key, test } => {
                let key_b = if let Ok(key_b) = EcdsaPublicKey::try_from_slice(public_key.as_ref()) {
                    key_b
                } else {
                    return todo!("Implement Error");
                };
                let other_test_string = test;
                if let Some(context) = connection_context {
                    if let ConnectionType::DirectConnection(direct) = &context.connection_type {
                        if let ConnectionStatus::PendingPairRequest { test } = &context.status {
                            let my_key = gen_ec_key_pair();
                            if other_test_string.is_none() != test.is_none() {
                                todo!("Mismatching Test Strings")
                            }
                            let (my_private, my_public) = my_key.split();
                            let encrypted_key_again = if let Some(other_test_string) = other_test_string {
                                let my_test = test.as_ref().unwrap();

                                let mix_message = SecureMessage::new(EcdsaKeyPair::join(my_private.clone(), key_b.clone()));
                                let vec = mix_message.encrypt(my_test.as_ref()).map_err(|e| EncryptionError::from(e))?;
                                if !other_test_string.eq(&vec) {
                                    return todo!("Mismatching Test Strings");// The key has been compromised
                                }
                                let my_message = SecureMessage::new(EcdsaKeyPair::join(my_private.clone(), my_public.clone()));
                                let my_test_encrypted = my_message.encrypt(my_test.as_ref()).map_err(|e| EncryptionError::from(e))?;
                                Some(Bytes::from(my_test_encrypted))
                            } else { None };
                            // As far as this device is concerned, the other device is now paired.
                            let message = Protocol::DeviceToDevice(DeviceToDevicePackets::SendKey {
                                public_key: crate::ToBytes::to_bytes(my_public.clone()),
                                test: encrypted_key_again,
                            });
                            self.device_manager.register_device(&direct.device_id, EncryptionSet {
                                public_key: my_public,
                                private_key: my_private,
                                key_b,
                            })?;
                            Ok(Response::NewContext {
                                message,
                                new_context: ConnectionContext {
                                    encryption: DynamicEncryptionManager::None,
                                    status: ConnectionStatus::PendingEncryption,
                                    connection_type: ConnectionType::DirectConnection(direct.clone()),
                                },
                            })
                        } else if let ConnectionStatus::Pairing { test, public_key, private_key, .. } = &context.status {
                            if let Some(other_test_string) = other_test_string {
                                let my_test = test.as_ref().unwrap();

                                let mix_message = SecureMessage::new(EcdsaKeyPair::join(private_key.clone(), key_b.clone()));
                                let vec = mix_message.encrypt(my_test.as_ref()).map_err(|e| EncryptionError::from(e))?;
                                if !other_test_string.eq(&vec) {
                                    return todo!("Mismatching Test Strings");// The key has been compromised
                                }
                            }
                            let mix_message = SecureMessage::new(EcdsaKeyPair::join(private_key.clone(), key_b.clone()));

                            let mut bytes = [0u8; 256];
                            rand::thread_rng().fill(&mut bytes);
                            let encrypt = mix_message.encrypt(bytes).map_err(EncryptionError::from)?;
                            // As far as this device is concerned, the other device is now paired.

                            let message = Protocol::DeviceToDevice(DeviceToDevicePackets::KeyCheck(Bytes::from(encrypt)));
                            self.device_manager.register_device(&direct.device_id, EncryptionSet {
                                public_key: public_key.clone(),
                                private_key: private_key.clone(),
                                key_b,
                            })?;

                            Ok(Response::NewContext {
                                message,
                                new_context: ConnectionContext {
                                    encryption: DynamicEncryptionManager::None,
                                    status: ConnectionStatus::PendingEncryption,
                                    connection_type: ConnectionType::DirectConnection(direct.clone()),
                                },
                            })
                        } else {
                            // This device is not in pairing mode
                            todo!("Implement Error")
                        }
                    } else {
                        panic!("Direct Connection Only!!!!")
                    }
                } else {
                    todo!("Implement Error")
                }
            }
            DeviceToDevicePackets::KeyCheck(random_check) => {
                if let Some(context) = connection_context {
                    if let ConnectionType::DirectConnection(direct) = &context.connection_type {
                        if let ConnectionStatus::PendingEncryption = &context.status {
                            let result = self.device_manager.get_paired_device(&direct.device_id)?;
                            let manager = result.get_encryption_manager();
                            let decrypt_message = manager.decrypt_message(random_check)?;
                            context.status = ConnectionStatus::CheckingKeys {
                                random_bytes: decrypt_message.clone(),
                            };
                            let encrypt_message = manager.encrypt_message(decrypt_message)?;

                            Ok(Response::Message(Protocol::DeviceToDevice(DeviceToDevicePackets::KeyCheck(encrypt_message))))
                        } else if let ConnectionStatus::CheckingKeys { random_bytes } = &context.status {
                            let result = self.device_manager.get_paired_device(&direct.device_id)?;
                            let manager = result.get_encryption_manager();
                            let bytes = manager.decrypt_message(random_check)?;
                            if !bytes.eq(random_bytes) {
                                Ok(Response::Message(Protocol::DeviceToDevice(DeviceToDevicePackets::KeyCheckResponse(false))))
                            } else {
                                Ok(Response::NewContext {
                                    message: Protocol::DeviceToDevice(DeviceToDevicePackets::KeyCheckResponse(true)),
                                    new_context: ConnectionContext {
                                        encryption: manager,
                                        status: ConnectionStatus::Connected,
                                        connection_type: ConnectionType::DirectConnection(direct.clone()),
                                    },
                                })
                            }
                        } else {
                            todo!("Implement Error")
                        }
                    } else {
                        panic!("Direct Connections only!")
                    }
                } else {
                    todo!("Implement Error Responses")
                }
            }
            DeviceToDevicePackets::KeyCheckResponse(success) => {
                if let Some(context) = connection_context {
                    if let ConnectionType::DirectConnection(direct) = &context.connection_type {
                        if let ConnectionStatus::CheckingKeys { .. } = &context.status {
                            if success {
                                let result = self.device_manager.get_paired_device(&direct.device_id)?;
                                let manager = result.get_encryption_manager();
                                context.encryption = manager;
                                context.status = ConnectionStatus::Connected;
                                Ok(Response::Nothing)
                            } else {
                                todo!("Implement Error")
                            }
                        } else {
                            todo!("Implement Error")
                        }
                    } else {
                        panic!("Direct Connections only!")
                    }
                } else {
                    todo!("Implement Error")
                }
            }
        }
    }
}