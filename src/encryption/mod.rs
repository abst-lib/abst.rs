use bytes::Bytes;
use themis::keys::{EcdsaPrivateKey, EcdsaPublicKey};
use uuid::Uuid;
pub struct EncryptionSet{
    pub public_key: EcdsaPublicKey,
    pub private_key: EcdsaPrivateKey,
    pub key_b: EcdsaPublicKey,
}
#[derive(Debug)]
pub enum EncryptionError{
    ThemisError,
}
impl From<themis::Error> for EncryptionError{
    fn from(_: themis::Error) -> Self{
        EncryptionError::ThemisError
    }
}
pub enum DynamicEncryptionManager {
    Themis(ThemisEncryptionManager),
    None,
}
impl EncryptionManager for DynamicEncryptionManager {
    type Error = EncryptionError;

    fn decrypt_message(&self, message: Bytes) -> Result<Bytes, Self::Error> {
        todo!()
    }

    fn encrypt_message(&self, message: Bytes) -> Result<Bytes, Self::Error> {
        todo!()
    }
}

pub trait EncryptionManager {
    type Error;
    fn decrypt_message(&self, message: Bytes) -> Result<Bytes, Self::Error>;
    fn encrypt_message(&self, message: Bytes) -> Result<Bytes, Self::Error>;

}

#[derive(Clone)]
pub struct ThemisEncryptionManager {
    pub self_private_key: Bytes,
    pub self_public_key: Bytes,
    pub other_public_key: Bytes,
}




pub struct ThemisEncryptionSession {
    server_id: Uuid,
}
