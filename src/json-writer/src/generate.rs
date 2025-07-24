use quote::quote;
use syn::Expr;

use crate::parse::JsonValue;

pub fn gen_write(buf: &Expr, value: &JsonValue) -> proc_macro2::TokenStream {
    let value = gen_value(buf, value);

    quote! {
        #[allow(unused_must_use)]
        {
            #value
        }
    }
}

fn gen_value(buf: &Expr, value: &JsonValue) -> proc_macro2::TokenStream {
    match value {
        JsonValue::Null => {
            quote! {
                buf.extend_from_slice(b"null");
            }
        }
        JsonValue::Bool(b) => {
            quote! {
                #b.serialize(#buf);
            }
        }
        JsonValue::String(s) => {
            quote! {
                #s.serialize(#buf);
            }
        }
        JsonValue::Number(n) => {
            quote! {
                buf.extend_from_slice(#n.as_bytes());
            }
        }
        JsonValue::Array(arr) => {
            if arr.is_empty() {
                return quote! {
                    #buf.extend_from_slice(b"[]");
                };
            }

            let mut statements = vec![quote! { #buf.push(b'['); }];
            for (i, element) in arr.iter().enumerate() {
                if i > 0 {
                    statements.push(quote! { #buf.push(b','); });
                }

                let value = gen_value(buf, element);
                statements.push(value);
            }
            statements.push(quote! { #buf.push(b']'); });

            quote! {
                #(#statements)*
            }
        }
        JsonValue::Object(obj) => {
            if obj.is_empty() {
                return quote! {
                    #buf.extend_from_slice(b"{}");
                };
            }

            let mut statements = vec![quote! { #buf.push(b'{'); }];
            for (i, (key, value)) in obj.iter().enumerate() {
                if i > 0 {
                    statements.push(quote! { #buf.push(b','); });
                }

                statements.push(quote! {
                    #buf.push(b'"');
                    #buf.extend_from_slice(#key.as_bytes());
                    #buf.push(b'"');
                    #buf.push(b':');
                });

                let value = gen_value(buf, value);
                statements.push(value);
            }
            statements.push(quote! { #buf.push(b'}'); });

            quote! {
                #(#statements)*
            }
        }
        JsonValue::Dyn(d) => {
            quote! {
                #d.serialize(#buf);
            }
        }
    }
}
