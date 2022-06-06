// MIT License
// https://github.com/3Hren/msgpack-rust/blob/master/rmp/src/decode/mod.rs#L52
use paste::paste;
macro_rules! read_byteorder_utils {
    ($($name:ident => $tp:ident),* $(,)?) => {
        $(
            #[inline]
            #[doc(hidden)]
            async fn $name<Reader: tokio::io::AsyncRead+ std::marker::Unpin>(reader: &mut Reader) -> Result<$tp, ::rmp::decode::ValueReadError<std::io::Error>> {
                const SIZE: usize = core::mem::size_of::<$tp>();
                let mut buf: [u8; SIZE] = [0u8; SIZE];
                reader.read_exact(&mut buf).await.map_err(::rmp::decode::ValueReadError::InvalidDataRead)?;
                Ok(paste::paste! {
                    <byteorder::BigEndian as byteorder::ByteOrder>::[<read_ $tp>](&mut buf)
                })
            }
        )*
    };
}