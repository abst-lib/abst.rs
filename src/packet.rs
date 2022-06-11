use std::fmt::{Debug};


use uuid::Uuid;







#[derive(Debug)]
pub struct RealmPacketEncrypted {
    pub target_device: Uuid,
    pub content: Vec<u8>, // Encrypted
}

