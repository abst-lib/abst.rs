
use packet::{PacketReadError, PacketWriteError};
use crate::encryption::EncryptionError;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    PacketBuild(PacketWriteError),
    PacketRead(PacketReadError),
    Encryption(EncryptionError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<PacketWriteError> for Error {
    fn from(value: PacketWriteError) -> Self {
        Error::PacketBuild(value)
    }
}

impl From<PacketReadError> for Error {
    fn from(value: PacketReadError) -> Self {
        Error::PacketRead(value)
    }
}

impl From<rmp::decode::ValueReadError<std::io::Error>> for Error {
    fn from(value: rmp::decode::ValueReadError<std::io::Error>) -> Self {
        Error::PacketRead(PacketReadError::from(value))
    }
}

impl From<rmp::encode::ValueWriteError<std::io::Error>> for Error {
    fn from(value: rmp::encode::ValueWriteError<std::io::Error>) -> Self {
        Error::PacketBuild(PacketWriteError::from(value))

    }
}
impl From<EncryptionError> for Error {
    fn from(value: EncryptionError) -> Self {
        Error::Encryption(value)
    }
}