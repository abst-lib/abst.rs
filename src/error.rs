use rmp::decode::ValueReadError;
use rmp::encode::ValueWriteError;
use crate::packet::{PacketBuildError, PacketReadError};
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    PacketBuild(PacketBuildError),
    PacketRead(PacketReadError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<PacketBuildError> for Error {
    fn from(value: PacketBuildError) -> Self {
        Error::PacketBuild(value)
    }
}impl From<PacketReadError> for Error {
    fn from(value: PacketReadError) -> Self {
        Error::PacketRead(value)
    }
}

impl From<rmp::decode::ValueReadError<std::io::Error>> for Error {
    fn from(value: rmp::decode::ValueReadError<std::io::Error>) -> Self {
        match value {
            ValueReadError::InvalidDataRead(io) => {
                Error::IO(io)
            }
            _ => {
                Error::PacketRead(PacketReadError::ValueReadError(value))
            }
        }
    }
}impl From<rmp::encode::ValueWriteError<std::io::Error>> for Error {
    fn from(value: rmp::encode::ValueWriteError<std::io::Error>) -> Self {
        match value {
            ValueWriteError::InvalidDataWrite(io) => {
                Error::IO(io)
            }
            _ => {
                Error::PacketBuild(PacketBuildError::ValueWriteError(value))
            }
        }
    }
}