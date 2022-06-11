
use packet::{PacketReadError, PacketWriteError};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    PacketBuild(PacketWriteError),
    PacketRead(PacketReadError),
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