pub trait Encryption: Clone {

}
#[derive(Clone)]
pub struct NoEncryption;
impl Encryption for NoEncryption {

}
pub trait EncryptionManager {

}