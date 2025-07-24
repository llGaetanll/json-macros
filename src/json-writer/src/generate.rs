use quote::quote;
use quote::ToTokens;
use syn::Expr;

use crate::parse::JsonValue;

pub fn gen_json(buf: &Expr, value: &JsonValue) -> proc_macro2::TokenStream {
    let value = gen_value(buf, value);

    quote! {
        #[allow(unused_must_use)]
        {
            #value
        }
    }
}

fn gen_serialize<S: ToTokens>(buf: &Expr, s: &S) -> proc_macro2::TokenStream {
    quote! {
        #s.serialize(#buf);
    }
}

fn gen_null(buf: &Expr) -> proc_macro2::TokenStream {
    quote! {
        #buf.extend_from_slice(b"null");
    }
}

fn gen_number(buf: &Expr, n: &str) -> proc_macro2::TokenStream {
    quote! {
        #buf.extend_from_slice(#n.as_bytes());
    }
}

fn gen_array(buf: &Expr, arr: &[JsonValue]) -> proc_macro2::TokenStream {
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

fn gen_object(buf: &Expr, obj: &[(String, JsonValue)]) -> proc_macro2::TokenStream {
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

fn gen_value(buf: &Expr, value: &JsonValue) -> proc_macro2::TokenStream {
    match value {
        JsonValue::Null => gen_null(buf),
        JsonValue::Bool(b) => gen_serialize(buf, b),
        JsonValue::String(s) => gen_serialize(buf, s),
        JsonValue::Number(n) => gen_number(buf, n),
        JsonValue::Array(arr) => gen_array(buf, arr),
        JsonValue::Object(obj) => gen_object(buf, obj),
        JsonValue::Dyn(d) => gen_serialize(buf, d),
    }
}
