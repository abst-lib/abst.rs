
use uuid::Uuid;

#[derive(Clone)]
pub struct ThemisEncryptionManager;


pub struct ThemisEncryptionSession{
    server_id: Uuid,
}
