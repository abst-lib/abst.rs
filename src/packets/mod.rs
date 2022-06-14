pub mod dtd;
/// Default Handlers for the packets established here
pub mod handlers;
pub mod realm;

use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, Write};
use crate::packets::dtd::DeviceToDevicePackets;
use crate::packets::realm::RealmPacket;
use packet::{PacketContent, PacketReadError, PacketWriteError, Protocol};
use rmp::Marker;

#[derive(Protocol)]
pub enum Protocol {
    #[protocol(protocol_id = 0x00)]
    DeviceToDevice(DeviceToDevicePackets),
    //#[protocol(protocol_id = 1)]
    //DeviceToPeer (),
    #[protocol(protocol_id = 0x02)]
    DeviceToRealm(RealmPacket),
}

pub trait PacketType {
    fn packet_id(&self) -> u8;
}

/// An Error Packet Response
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorPacket {
    /// Error that contains a reference to the error causing packet
    ErrorWithReference {
        reference_protocol: u8,
        reference_packet: u8,
        error_code: u8,
        error_message: Option<Cow<'static, str>>,
    },
    /// No Reference to what caused the error
    ErrorNoReference {
        error_code: u8,
        error_message: Option<Cow<'static, str>>,
    },
}
impl ErrorPacket{
    pub fn invalid_state(protocol: u8, packet: u8) -> Self {
        ErrorPacket::ErrorWithReference {
            reference_protocol: protocol,
            reference_packet: packet,
            error_code: 0,
            error_message: Some("Invalid State".into()),
        }
    }
}
impl From<(u8, u8,u8)> for ErrorPacket{
    fn from((protocol, reference, error): (u8, u8, u8)) -> Self {
        Self::ErrorWithReference {
            reference_protocol: protocol,
            reference_packet: reference,
            error_code: error,
            error_message: None
        }
    }
}impl From<(u8, u8,u8, &'static str)> for ErrorPacket{
    fn from((protocol, reference, error, debug): (u8, u8, u8, &'static str)) -> Self {
        Self::ErrorWithReference {
            reference_protocol: protocol,
            reference_packet: reference,
            error_code: error,
            error_message: Some(Cow::Borrowed(debug))
        }
    }
}

impl From<u8> for ErrorPacket{
    fn from(error: u8) -> Self {
        Self::ErrorNoReference {
            error_code: error,
            error_message: None
        }
    }
}impl From<(u8,  &'static str)> for ErrorPacket{
    fn from((error, debug): (u8, &'static str)) -> Self {
        Self::ErrorNoReference {
            error_code: error,
            error_message: Some(Cow::Borrowed(debug))
        }
    }
}
impl Display for ErrorPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorPacket::ErrorWithReference {
                reference_protocol,
                reference_packet,
                error_code,
                error_message,
            } => write!(
                f,
                "Error with reference: {} {} {} {:?}",
                reference_protocol,
                reference_packet,
                error_code,
                error_message.as_ref()
            ),
            ErrorPacket::ErrorNoReference {
                error_code,
                error_message,
            } => write!(
                f,
                "Error without reference: {} {:?}",
                error_code,
                error_message.as_ref()
            ),
        }
    }
}

impl Error for ErrorPacket {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl PacketContent for ErrorPacket {
    fn read<Reader: BufRead>(reader: &mut Reader) -> Result<Self, PacketReadError> where Self: Sized {
        let buf = reader.fill_buf()?;
        if let Marker::True = Marker::from_u8(buf[0]) {
            Ok(Self::ErrorWithReference {
                reference_protocol: rmp::decode::read_u8(reader)?,
                reference_packet: rmp::decode::read_u8(reader)?,
                error_code: rmp::decode::read_u8(reader)?,
                error_message: PacketContent::read(reader)?,
            })
        } else {
            Ok(Self::ErrorNoReference {
                error_code: rmp::decode::read_u8(reader)?,
                error_message: PacketContent::read(reader)?,
            })
        }
    }

    fn write<Writer: Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError> where Self: Sized {
        rmp::encode::write_u8(writer, 1)?;
        match self {
            Self::ErrorWithReference {
                reference_protocol,
                reference_packet,
                error_code,
                error_message,
            } => {
                rmp::encode::write_bool(writer, false)?;
                rmp::encode::write_u8(writer, *reference_protocol)?;
                rmp::encode::write_u8(writer, *reference_packet)?;
                rmp::encode::write_u8(writer, *error_code)?;
                PacketContent::write(error_message, writer)?;
            }
            Self::ErrorNoReference {
                error_code,
                error_message,
            } => {
                rmp::encode::write_bool(writer, false)?;
                rmp::encode::write_u8(writer, *error_code)?;
                PacketContent::write(error_message, writer)?;
            }
        }
        Ok(())
    }
}