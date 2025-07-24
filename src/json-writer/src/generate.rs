use quote::quote;
use syn::Expr;

use crate::parse::JsonValue;

pub fn gen_write(buf: &Expr, value: &JsonValue) -> proc_macro2::TokenStream {
    let exprs = gen_value_pure(value);

    quote! {
        (#exprs)(&mut #buf);
    }
}

pub fn gen_value_pure(value: &JsonValue) -> proc_macro2::TokenStream {
    match value {
        JsonValue::Null => {
            quote! {
                |buf: &mut Vec<u8>| {
                    buf.extend_from_slice(b"null");
                }
            }
        }
        JsonValue::Bool(b) => {
            quote! {
                |buf: &mut Vec<u8>| {
                    #b.serialize(buf);
                }
            }
        }
        JsonValue::String(s) => {
            quote! {
                |buf: &mut Vec<u8>| {
                    #s.serialize(buf);
                }
            }
        }
        JsonValue::Number(n) => {
            quote! {
                |buf: &mut Vec<u8>| {
                    buf.extend_from_slice(#n.as_bytes());
                }
            }
        }
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                return quote! {
                    |buf: &mut Vec<u8>| {
                        buf.extend_from_slice(b"[]");
                    }
                };
            }

            let mut statements = vec![quote! { buf.push(b'['); }];
            for (i, element) in arr.iter().enumerate() {
                if i > 0 {
                    statements.push(quote! { buf.push(b','); });
                }

                let value = gen_value_pure(element);
                statements.push(quote! { (#value)(buf); });
            }
            statements.push(quote! { buf.push(b']'); });

            quote! {
                |buf: &mut Vec<u8>| {
                    #(#statements)*
                }
            }
        }
        JsonValue::Object(obj) => {
            if obj.is_empty() {
                return quote! {
                    |buf: &mut Vec<u8>| {
                        buf.extend_from_slice(b"{}");
                    }
                };
            }

            let mut statements = vec![quote! { buf.push(b'{'); }];
            for (i, (key, value)) in obj.iter().enumerate() {
                if i > 0 {
                    statements.push(quote! { buf.push(b','); });
                }

                statements.push(quote! {
                    buf.push(b'"');
                    buf.extend_from_slice(#key.as_bytes());
                    buf.push(b'"');
                    buf.push(b':');
                });

                let value = gen_value_pure(value);
                statements.push(quote! { (#value)(buf); });
            }
            statements.push(quote! { buf.push(b'}'); });

            quote! {
                |buf: &mut Vec<u8>| {
                    #(#statements)*
                }
            }
        }
        JsonValue::Dyn(d) => {
            quote! {
                |buf: &mut Vec<u8>| {
                    #d.serialize(buf);
                }
            }
        }
    }
}
