use rmp::encode::ValueWriteError;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub async fn write_u8<Writer: AsyncWrite + std::marker::Unpin>(writer: &mut Writer, ) -> Result<u8,ValueWriteError<std::io::Error>> {
    writer.write_all()
    std
}