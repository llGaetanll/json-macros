use quote::quote;
use syn::Expr;
use syn::Ident;

use crate::parse::JsonValue;

fn gen_null(buf: &Expr) -> proc_macro2::TokenStream {
    quote! { #buf.extend_from_slice(b"null"); }
}

fn gen_bool(buf: &Expr, b: bool) -> proc_macro2::TokenStream {
    quote! {
        #b.serialize(&mut #buf);
    }
}

fn gen_string(buf: &Expr, s: &str) -> proc_macro2::TokenStream {
    quote! {
        #s.serialize(&mut #buf);
    }
}

fn gen_number(buf: &Expr, n: &str) -> proc_macro2::TokenStream {
    // TODO: We would use .serialize, but numbers are
    // represented as strs internally and so they get quoted
    quote! {
        #buf.extend_from_slice(#n.as_bytes());
    }
}

fn gen_array(buf: &Expr, arr: &[JsonValue]) -> proc_macro2::TokenStream {
    if arr.is_empty() {
        return quote! { #buf.extend_from_slice(b"[]"); };
    }

    let mut statements = vec![quote! { #buf.push(b'['); }];
    for (i, element) in arr.iter().enumerate() {
        if i > 0 {
            statements.push(quote! { #buf.push(b','); });
        }
        statements.push(gen_value(buf, element));
    }
    statements.push(quote! { #buf.push(b']'); });
    quote! { #(#statements)* }
}

fn gen_object(buf: &Expr, obj: &[(String, JsonValue)]) -> proc_macro2::TokenStream {
    if obj.is_empty() {
        return quote! { #buf.extend_from_slice(b"{}"); };
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
        statements.push(gen_value(buf, value));
    }
    statements.push(quote! { #buf.push(b'}'); });
    quote! { #(#statements)* }
}

fn gen_dyn(buf: &Expr, d: &Ident) -> proc_macro2::TokenStream {
    quote! {
        #d.serialize(&mut #buf);
    }
}

pub fn gen_value(buf: &Expr, value: &JsonValue) -> proc_macro2::TokenStream {
    match value {
        JsonValue::Null => gen_null(buf),
        JsonValue::Bool(b) => gen_bool(buf, *b),
        JsonValue::String(s) => gen_string(buf, s),
        JsonValue::Number(n) => gen_number(buf, n),
        JsonValue::Array(arr) => gen_array(buf, arr),
        JsonValue::Object(obj) => gen_object(buf, obj),
        JsonValue::Dyn(ident) => gen_dyn(buf, ident),
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

                statements.push(gen_value_pure(element));
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

                statements.push(gen_value_pure(value));
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
                #d.serialize(buf);
            }
        }
    }
}
