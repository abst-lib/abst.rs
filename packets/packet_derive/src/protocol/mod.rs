use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use quote::__private::ext::RepToTokensExt;
use syn::{DataEnum, Field, Fields, Ident, LitInt, Variant};
use syn::parse::{Parse, ParseStream};
use syn::Result;
use syn::spanned::Spanned;
use crate::protocol::packet_attrs::protocol_id;

mod packet_attrs {
    syn::custom_keyword!(protocol_id);
}

enum ProtocolAttrs {
    ProtocolId {
        value_token: packet_attrs::protocol_id,
        eq_token: syn::Token![=],
        value: LitInt,
    },
}

impl Parse for ProtocolAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(packet_attrs::protocol_id) {
            Ok(ProtocolAttrs::ProtocolId {
                value_token: input.parse()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

pub(crate) fn parse_enum(type_ident: Ident, data: DataEnum) -> Result<TokenStream> {
    let mut handlers = Vec::new();
    for protocol_variant in data.variants {
        let protocol_id = protocol_variant.attrs.iter().find(|attr| attr.path.is_ident("protocol"));
        if protocol_id.is_none() {
            return Err(syn::Error::new(protocol_id.span(), "Packet must have a protocol_id attribute"));
        }
        let protocol_id = protocol_id.unwrap();
        let value = protocol_id.parse_args::<ProtocolAttrs>()?;
        match value {
            ProtocolAttrs::ProtocolId { value, .. } => {
                let protocol_id = value.base10_parse::<u8>()?;
                let mod_name = format_ident!("{}_handler", protocol_variant.ident);

                let value = match &protocol_variant.fields {
                    Fields::Unnamed(value) => {
                        if value.unnamed.len() != 1 {
                            return Err(syn::Error::new(protocol_variant.ident.span(), "Protocol must have a single field"));
                        } else {
                            value.unnamed.first().cloned().unwrap()
                        }
                    }
                    _ => return Err(syn::Error::new(protocol_variant.ident.span(), "Protocol must have a single field")),
                };
                let read_method = create_reader(&value, &protocol_variant, &type_ident)?;
                let write_method = create_writer(&value, protocol_id)?;
                let handler = quote! {
                    mod #mod_name {
                        use super::*;
                        use packet::PacketContent;
                        #read_method
                        #write_method
                    }
                };
                handlers.push(handler);
            }
        }
    }

    Ok(quote! {
        #(#handlers)*
        impl ::packet::protocol::Protocol for #type_ident {
            type ReadError = ::packet::PacketReadError;
            type WriteError = ::packet::PacketWriteError;
            fn get_protocol_id(&self) -> u8{
                                unimplemented!()

            }

            fn write_payload<Writer: Write>(self, writer: &mut Writer) -> Result<(), Self::WriteError>{
                                unimplemented!()

            }

            fn supports_protocol_id(id: u8) -> bool{
                                unimplemented!()

            }

            fn build_if_supported<Reader: Read>(protocol_id: u8, packet_id: u8, reader: &mut Reader) -> Option<Result<Self, Self::ReadError>> where Self: Sized{
                unimplemented!()
            }
        }
    })
}


/// Returns a A method name. That takes a reader and Token Stream for the method
fn create_reader(packet_type: &Field, variant: &Variant, value: &syn::Ident) -> Result<TokenStream> {
    let variant_ident = &variant.ident;
    let read_method = quote! {
        pub fn read<Reader: ::std::io::Read>(packet_id: u8, reader: &mut Reader) -> Option<Result<#value, ::packet::PacketReadError>>{
           let packet = <#packet_type as ::packet::packet::Packet>::build_or_none(packet_id, reader);
            if let Some(packet) = packet {
                Ok(#value::#variant_ident(packet))
            } else {
                None
            }
        }
    };
    Ok(read_method)
}

/// Returns a A method name. That takes a reader and Token Stream for the method
fn create_writer(packet_type: &Field, protocol_id: u8) -> Result<TokenStream> {
    let write_method = quote! {
        pub fn write<Writer: ::std::io::Write>(data: &#packet_type, writer: &mut Writer) -> Result<(), ::packet::PacketWriteError>{
            ::packet::PacketContent::write(#protocol_id, writer)?;
            data.write_payload(writer)?;
            Ok(())
        }
    };
    Ok(write_method)
}





