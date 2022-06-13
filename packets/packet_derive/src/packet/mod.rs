use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{DataEnum, LitInt, Variant};
use syn::{Fields, FieldsNamed, FieldsUnnamed, Result};

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

pub(crate) fn parse_enum(packet_ident: syn::Ident, data: DataEnum) -> Result<TokenStream> {
    let mut packet_types = HashMap::new();
    let mut packet_handlers = Vec::new();
    let mut get_packet_id_arms = Vec::new();
    let mut write_arms = Vec::new();

    for packet_variant in data.variants {
        let packet_id = packet_variant
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("packet"));
        if packet_id.is_none() {
            return Err(syn::Error::new(
                packet_variant.span(),
                "Packet must have a packet_id attribute",
            ));
        }
        let packet_id = packet_id.unwrap();
        let value = packet_id.parse_args::<PacketAttrs>()?;
        match value {
            PacketAttrs::PacketId { value, .. } => {
                let packet_id_id = value.base10_parse::<u8>()?;
                let variant_name = packet_variant.ident.clone();
                let packet_id_arm = match packet_variant.fields {
                    Fields::Named(_) => {
                        quote! {
                            #packet_ident::#variant_name{..} => {
                                return #packet_id_id;
                            }
                        }
                    }
                    Fields::Unnamed(_) => {
                        quote! {
                            #packet_ident::#variant_name(..) => {
                                return #packet_id_id;
                            }
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            #packet_ident::#variant_name => {
                                return #packet_id_id;
                            }
                        }
                    }
                };
                get_packet_id_arms.push(packet_id_arm);
                let mod_name = format_ident!("{}_handler", variant_name);

                let read_method = match &packet_variant.fields {
                    Fields::Named(named) => named_parser(&packet_variant, &packet_ident, named),
                    Fields::Unnamed(unamed) => {
                        unamed_parser(&packet_variant, &packet_ident, unamed)
                    }
                    Fields::Unit => unit_parser(&packet_variant, &packet_ident),
                }?;
                let (write_method, arm) = match &packet_variant.fields {
                    Fields::Named(named) => named_writer(
                        &packet_variant,
                        packet_id_id,
                        &packet_ident,
                        &mod_name,
                        named,
                    ),
                    Fields::Unnamed(unamed) => unamed_writer(
                        &packet_variant,
                        packet_id_id,
                        &packet_ident,
                        &mod_name,
                        unamed,
                    ),
                    Fields::Unit => {
                        unit_writer(&packet_variant, packet_id_id, &packet_ident, &mod_name)
                    }
                }?;
                write_arms.push(arm);
                let handler = quote! {
                    mod #mod_name {
                        use super::*;
                        use packet::PacketContent;
                        use packet::packet::Packet;
                        use packet::protocol::Protocol;

                        #read_method
                        #write_method
                    }
                };
                packet_types.insert(packet_id_id, packet_variant);
                packet_handlers.push(handler);
            }
            PacketAttrs::Default => {
                //TODO crate a default handler
            }
        }
    }
    let mut read_arms = Vec::new();
    for (packet_id, variant) in packet_types.iter() {
        let variant_name = &variant.ident;
        let mod_name = format_ident!("{}_handler", variant_name);

        read_arms.push(quote! {
            #packet_id => {
                let packet = #mod_name::read(reader);
                Some(packet)
            }
        });
    }
    Ok(quote! {
        #(#packet_handlers)*
        impl ::packet::packet::Packet for #packet_ident{
            type ReadError = ::packet::PacketReadError;
            type WriteError = ::packet::PacketWriteError;
            fn get_packet_id(&self) -> u8 {
                match self {
                    #(#get_packet_id_arms)*
                }
            }
            fn write_payload<Writer: ::std::io::Write>(self, writer: &mut Writer) -> Result<(), Self::WriteError> {
                match self{
                    #(#write_arms)*
                }
                Ok(())
            }
            fn build_or_none<Reader: ::std::io::BufRead>(id: u8, reader: &mut Reader) -> Option<Result<Self, Self::ReadError>> where Self: ::std::marker::Sized{
                match id {
                    #(#read_arms)*
                    _ => None
                }
            }

        }
    })
}

/// Returns a A method name. That takes a reader and Token Stream for the method
fn unamed_parser(
    variant: &Variant,
    value: &syn::Ident,
    fields: &FieldsUnnamed,
) -> Result<TokenStream> {
    let variant_name = &variant.ident;
    let mut fields_parsers = Vec::new();
    let mut field_names = Vec::new();
    for (key, field) in fields.unnamed.iter().enumerate() {
        let field_name = format_ident!("field_{}", key);
        fields_parsers.push(quote! {
            let #field_name: #field = PacketContent::read(reader)?;
        });
        field_names.push(field_name);
    }
    let token_stream = quote! {
        pub fn read<Reader: ::std::io::BufRead>(reader: &mut Reader) -> Result<#value, ::packet::PacketReadError>{
            #(#fields_parsers)*
            Ok(#value::#variant_name(#(#field_names),*))
        }
    };
    Ok(token_stream)
}

fn named_parser(
    variant: &Variant,
    value: &syn::Ident,
    fields: &FieldsNamed,
) -> Result<TokenStream> {
    let variant_name = &variant.ident;
    let mut fields_parsers = Vec::new();
    for (key, field) in fields.named.iter().enumerate() {
        let field_name = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new(field.span(), "Field must have a name"))?;
        let type_name = &field.ty;
        fields_parsers.push(quote! {
            #field_name:  PacketContent::read(reader)?
        });
    }
    let token_stream = quote! {
        pub fn read<Reader: ::std::io::BufRead>(reader: &mut Reader) -> Result<#value, ::packet::PacketReadError>{
            Ok(#value::#variant_name{
             #(#fields_parsers),*
            })
        }
    };
    Ok(token_stream)
}

fn unit_parser(variant: &Variant, value: &syn::Ident) -> Result<TokenStream> {
    let variant_name = &variant.ident;
    let token_stream = quote! {
        pub fn read<Reader: ::std::io::BufRead>(reader: &mut Reader) -> Result<#value, ::packet::PacketReadError>{
            Ok(#value::#variant_name)
        }
    };
    Ok(token_stream)
}

fn unamed_writer(
    variant: &Variant,
    packet_id: u8,
    type_ident: &syn::Ident,
    mod_name: &syn::Ident,
    fields: &FieldsUnnamed,
) -> Result<(TokenStream, TokenStream)> {
    let variant_name = &variant.ident;

    let mut field_writers = Vec::new();
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    for (key, field) in fields.unnamed.iter().enumerate() {
        let field_name = format_ident!("field_{}", key);
        field_types.push(&field.ty);
        field_writers.push(quote! {
                #field_name.write(writer)?;
        });
        field_names.push(field_name);
    }
    let arm = quote! {
            #type_ident::#variant_name(#(#field_names),*) => {
                #mod_name::write(writer,(#(#field_names),*))?;
            }
    };
    let token_stream = quote! {
        pub fn write<Writer: ::std::io::Write>(writer: &mut Writer,(#(#field_names),*):(#(#field_types),*)) -> Result<(), ::packet::PacketWriteError>{
            PacketContent::write(&#packet_id,writer)?;
            #(#field_writers)*
            Ok(())
        }
    };

    Ok((token_stream, arm))
}

fn unit_writer(
    variant: &Variant,
    packet_id: u8,
    type_ident: &syn::Ident,
    mod_name: &syn::Ident,
) -> Result<(TokenStream, TokenStream)> {
    let variant_name = &variant.ident;

    let token_stream = quote! {
    pub fn write<Writer: ::std::io::Write>(writer: &mut Writer) -> Result<(), ::packet::PacketWriteError> {
        PacketContent::write(&#packet_id,writer)?;
        Ok(())
    }
    };
    let arm = quote! {
        #type_ident::#variant_name => {
            #mod_name::write(writer)?;
        }
    };
    return Ok((token_stream, arm));
}

fn named_writer(
    variant: &Variant,
    packet_id: u8,
    type_ident: &syn::Ident,
    mod_name: &syn::Ident,
    fields: &FieldsNamed,
) -> Result<(TokenStream, TokenStream)> {
    let variant_name = &variant.ident;

    let mut field_writers = Vec::new();
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    for field in fields.named.iter() {
        let field_name = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new(field.span(), "Field must have a name"))?;
        field_writers.push(quote! {
            #field_name.write(writer)?;
        });
        field_names.push(field_name);
        field_types.push(&field.ty);
    }
    let arm = quote! {
        #type_ident::#variant_name{
            #(#field_names),*
        } => {
            #mod_name::write(writer,(#(#field_names),*))?;
        }
    };

    let token_stream = quote! {
             pub fn write<Writer: ::std::io::Write>(writer: &mut Writer,(#(#field_names),*):(#(#field_types),*)) -> Result<(), ::packet::PacketWriteError>{
        PacketContent::write(&#packet_id,writer)?;
        #(#field_writers)*
        Ok(())
    }

    };

    Ok((token_stream, arm))
}
