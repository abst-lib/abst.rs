use rmp::decode::ValueReadError;
use crate::packet::PacketBuildError;

pub enum Error {
    IO(std::io::Error),
    Packet(PacketBuildError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<PacketBuildError> for Error {
    fn from(value: PacketBuildError) -> Self {
        Error::Packet(value)
    }
}

impl From<rmp::decode::ValueReadError<std::io::Error>> for Error {
    fn from(value: rmp::decode::ValueReadError<std::io::Error>) -> Self {
        match value {
            ValueReadError::InvalidDataRead(io) => {
                Error::IO(io)
            }
            _ => {
                Error::Packet(PacketBuildError())
            }
        }
    }
}