mod packet;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(Packet, attributes(packet))]
pub fn derive_from_value(stream: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(stream as DeriveInput);
    match input.data {
        Data::Enum(en) => {
            packet::parse_enum(input.ident, en).unwrap_or_else(|e| e.to_compile_error()).into()
        }
        _ => panic!("Packet can only be derived from an enum"),
    }
}
