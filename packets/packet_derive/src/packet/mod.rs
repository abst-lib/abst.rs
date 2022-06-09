use proc_macro::{Ident, Span};
use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{DataEnum, Field, LitByte, LitInt, Variant};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Result;
use syn::spanned::Spanned;
use syn::token::Comma;

mod packet_attrs {
    syn::custom_keyword!(packet_id);
    syn::custom_keyword!(default);
}

enum PacketAttrs {
    PacketId {
        value_token: packet_attrs::packet_id,
        eq_token: syn::Token![=],
        value: LitInt,
    },
    Default,
}

impl Parse for PacketAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(packet_attrs::packet_id) {
            Ok(PacketAttrs::PacketId {
                value_token: input.parse()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(packet_attrs::default) {
            Ok(PacketAttrs::Default)
        } else {
            Err(lookahead.error())
        }
    }
}

pub(crate) fn parse_enum(ident: syn::Ident, data: DataEnum) -> Result<TokenStream> {
    let mut packets = HashMap::new();
    let mut packets_handlers = Vec::new();
    for packet_types in data.variants {
        let packet_id = packet_types.attrs.iter().find(|attr| attr.path.is_ident("packet"));
        if packet_id.is_none() {
            return Err(syn::Error::new(packet_types.span(), "Packet must have a packet_id attribute"));
        }
        let packet_id = packet_id.unwrap();
        let value = packet_id.parse_args::<PacketAttrs>()?;
        match value {
            PacketAttrs::PacketId { value, .. } => {
                let value = value.base10_parse::<u8>()?;
                let (method_name, method) = create_parser(packet_types, ident.clone())?;
                packets.insert(value, method_name);
                packets_handlers.push(method);
            }
            PacketAttrs::Default => {
                //TODO crate a default handler
            }
        }
    }

    let packet_parsers = quote! {
            #(#packets_handlers)*

    };
    let mut arms = Vec::new();
    for (value, method_name) in packets {
        arms.push(quote! {
            #value => {
                let packet = #method_name(reader);
                Some(packet)
            }
        });
    }
    Ok(quote! {
            #packet_parsers

        impl ::packet::packet::Packet for #ident{
            type Error = ::packet::PacketReadError;
            fn build_or_none<Reader: ::std::io::Read>(id: u8, reader: &mut Reader) -> Option<Result<Self, Self::Error>> where Self: ::std::marker::Sized{
                match id {
                    #(#arms)*
                    _ => None
                }
            }

        }
    }.into())
}

/// Returns a A method name. That takes a reader and Token Stream for the method
fn create_parser(variant: Variant, value: syn::Ident) -> Result<(proc_macro2::Ident, TokenStream)> {
    let varname = format_ident!("{}_parser", variant.ident);
    let variant_name = variant.ident;
    let fields = variant.fields;
    let mut fields_parsers = Vec::new();
    let mut field_names = Vec::new();
    for (key, field) in fields.iter().enumerate() {
        let field_name = format_ident!("field_{}", key);
        fields_parsers.push(quote!{
            let #field_name: #field = PacketContent::read(reader)?;
        });
        field_names.push(field_name);
    }
    let token_stream = quote! {
        fn #varname<Reader: ::std::io::Read>(reader: &mut Reader) -> Result<#value, ::packet::PacketReadError>{ //TODO get the types to return
                      use packet::PacketContent;

            #(#fields_parsers)*
Ok(#value::#variant_name(#(#field_names),*))
        }
    };
    Ok((varname, token_stream))
}