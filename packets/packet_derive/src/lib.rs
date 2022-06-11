mod packet;
mod protocol;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Packet, attributes(packet))]
pub fn packet(stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(stream as DeriveInput);
    match input.data {
        Data::Enum(en) => packet::parse_enum(input.ident, en)
            .unwrap_or_else(|e| e.to_compile_error())
            .into(),
        _ => panic!("Packet can only be derived from an enum"),
    }
}
#[proc_macro_derive(Protocol, attributes(protocol))]
pub fn protocol(stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(stream as DeriveInput);
    match input.data {
        Data::Enum(en) => protocol::parse_enum(input.ident, en)
            .unwrap_or_else(|e| e.to_compile_error())
            .into(),
        _ => panic!("Packet can only be derived from an enum"),
    }
}
