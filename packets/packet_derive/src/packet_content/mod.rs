use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{DataStruct, Result};
use syn::{Field, Fields, Ident, LitInt, Variant};

pub(crate) fn parse_struct(type_ident: Ident, data: DataStruct) -> Result<TokenStream> {
    let (write_func, read_func) = match data.fields {
        Fields::Named(fields) => {
            let mut field_writes = Vec::new();
            let mut field_reads = Vec::new();
            for field in fields.named.iter() {
                let field_name = &field.ident;
                let field_type = &field.ty;
                field_writes.push(quote! {
                    self.#field_name.write(writer)?;
                });
                field_reads.push(quote! {
                    #field_name: #field_type::read(reader)?,
                });
            }
            let read_func = quote! {
                let value = Self {
                     #(#field_reads)*
                };
                Ok(value)
            };
            let write_func = quote! {
                #(#field_writes)*
                Ok(())
            };
            (write_func, read_func)
        }
        Fields::Unnamed(fields) => {
            let mut field_writes = Vec::new();
            let mut field_reads = Vec::new();
            let mut values = Vec::new();
            for (key, field) in fields.unnamed.iter().enumerate() {
                let field_type = &field.ty;
                let value = format_ident!("field_{}", key);
                field_writes.push(quote! {
                    self.#key.write(writer)?;
                });
                field_reads.push((quote! {
                    let #value = #field_type::read(reader)?;
                }));
                values.push(value);
            }
            let read_func = quote! {
                #(#field_reads)*
                Ok(Self(#(#values),*))
            };
            let write_func = quote! {
                #(#field_writes)*
                Ok(())
            };
            (write_func, read_func)
        }
        Fields::Unit => {
            return Err(syn::Error::new(
                type_ident.span(),
                "Unit Structs are not supported",
            ));
        }
    };

    Ok(quote! {
        impl ::packet::PacketContent for #type_ident {
            fn read<Reader: ::std::io::Read>(reader: &mut Reader) -> Result<Self, PacketReadError>
                where Self: Sized,
            {
                #read_func
            }
            fn write<Writer: ::std::io::Write>(&self, writer: &mut Writer) -> Result<(), PacketWriteError>
            where Self: Sized,
            {
                #write_func
            }
        }
    })
}
