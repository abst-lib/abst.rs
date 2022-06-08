use std::borrow::Cow::Owned;
use tokio::net::TcpStream;
use tokio::time::sleep;
use abst_rs::a_sync::tokio_abst::client::new_tokio_client;
use abst_rs::packet::ABSTPacket;
use abst_rs::protocol::DeviceToDevice;
#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:3695").await.unwrap();
    let mut client = new_tokio_client::<DeviceToDevice>(stream, Owned(DeviceToDevice {}));
    loop {

        sleep(std::time::Duration::from_secs(5));

    }
}