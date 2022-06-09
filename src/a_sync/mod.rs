use std::borrow::Cow;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::net::{IpAddr, SocketAddr, TcpStream};
use crate::protocol::ConnectionType;

pub mod tokio_abst;