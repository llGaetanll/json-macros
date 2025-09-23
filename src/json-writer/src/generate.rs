use quote::quote;
use syn::Expr;

use crate::ir::JsonChunk;

pub fn from_chunks(buf: &Expr, chunks: &[JsonChunk]) -> proc_macro2::TokenStream {
    let statements: Vec<proc_macro2::TokenStream> = chunks
        .iter()
        .map(|chunk| match chunk {
            JsonChunk::Dyn(ident) => quote! {
                #ident.serialize(#buf)?;
            },
            JsonChunk::Static(bytes) => {
                let bstr = proc_macro2::Literal::byte_string(bytes);
                quote! {
                    #buf.write_all(#bstr)?;
                }
            }
        })
        .collect();

    quote! {
        #[allow(unused_must_use)]
        {
            #(#statements)*

            ::std::io::Result::Ok(())
        }
    }
}

fn gen_lazy_statements(chunks: &[JsonChunk]) -> Vec<proc_macro2::TokenStream> {
    chunks
        .iter()
        .map(|chunk| match chunk {
            JsonChunk::Dyn(ident) => quote! {
                #ident.serialize(__buf)?;
            },
            JsonChunk::Static(bytes) => {
                let bstr = proc_macro2::Literal::byte_string(bytes);
                quote! {
                    __buf.write_all(#bstr)?;
                }
            }
        })
        .collect()
}

pub fn from_chunks_lazy(chunks: &[JsonChunk]) -> proc_macro2::TokenStream {
    let statements = gen_lazy_statements(chunks);

    quote! {
        |__buf: &mut Vec<u8>| -> ::std::io::Result<()> {
            #(#statements)*

            ::std::io::Result::Ok(())
        }
    }
}

pub fn from_chunks_lazy_move(chunks: &[JsonChunk]) -> proc_macro2::TokenStream {
    let statements = gen_lazy_statements(chunks);

    quote! {
        move |__buf: &mut Vec<u8>| -> ::std::io::Result<()> {
            #(#statements)*

            ::std::io::Result::Ok(())
        }
    }
}
