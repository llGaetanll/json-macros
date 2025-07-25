use quote::quote;
use syn::Expr;

use crate::ir::JsonChunk;

pub fn from_chunks(buf: &Expr, chunks: &[JsonChunk]) -> proc_macro2::TokenStream {
    let statements: Vec<proc_macro2::TokenStream> = chunks
        .iter()
        .map(|chunk| match chunk {
            JsonChunk::Dyn(ident) => quote! { #ident.serialize(#buf); },
            JsonChunk::Static(bytes) => {
                if bytes.len() == 1 {
                    let bchr = proc_macro2::Literal::byte_character(bytes[0]);
                    quote! { #buf.push(#bchr); }
                } else {
                    let bstr = proc_macro2::Literal::byte_string(bytes);
                    quote! { #buf.extend_from_slice(#bstr); }
                }
            }
        })
        .collect();

    quote! {
        #[allow(unused_must_use)]
        {
            #(#statements)*
        }
    }
}
