use proc_macro::TokenStream;
use quote::quote;
use syn::{braced, bracketed, parse_macro_input, Expr, Ident, Lit, Token};

struct JsonArgs {
    buf: Expr,
    value: JsonValue,
}

impl syn::parse::Parse for JsonArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let buf = input.parse()?;
        input.parse::<Token![,]>()?;
        let value = input.parse()?;
        Ok(JsonArgs { buf, value })
    }
}

#[derive(Debug)]
enum JsonValue {
    Null,
    Bool(bool),
    String(String),
    Number(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl syn::parse::Parse for JsonValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Brace) {
            // Parse { key: value, key: value }
            let content;
            braced!(content in input);

            let mut fields = Vec::new();
            while !content.is_empty() {
                let key: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                let value: JsonValue = content.parse()?;
                fields.push((key.to_string(), value));

                if !content.is_empty() {
                    content.parse::<Token![,]>()?;
                }
            }
            Ok(JsonValue::Object(fields))
        } else if input.peek(syn::token::Bracket) {
            // Parse [value, value]
            let content;
            bracketed!(content in input);

            let mut elements = Vec::new();
            while !content.is_empty() {
                let value: JsonValue = content.parse()?;
                elements.push(value);

                if !content.is_empty() {
                    content.parse::<Token![,]>()?;
                }
            }
            Ok(JsonValue::Array(elements))
        } else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            if ident == "null" {
                Ok(JsonValue::Null)
            } else {
                Err(syn::Error::new_spanned(ident, "unknown identifier"))
            }
        } else {
            // Parse literals
            let lit: Lit = input.parse()?;
            match lit {
                Lit::Bool(b) => Ok(JsonValue::Bool(b.value)),
                Lit::Str(s) => Ok(JsonValue::String(s.value())),
                Lit::Int(i) => Ok(JsonValue::Number(i.base10_digits().to_string())),
                Lit::Float(f) => Ok(JsonValue::Number(f.base10_digits().to_string())),
                _ => Err(syn::Error::new_spanned(lit, "unsupported literal")),
            }
        }
    }
}

fn gen_null(buf: &Expr) -> proc_macro2::TokenStream {
    quote! { #buf.extend_from_slice(b"null"); }
}

fn gen_bool(buf: &Expr, b: bool) -> proc_macro2::TokenStream {
    if b {
        quote! { #buf.extend_from_slice(b"true"); }
    } else {
        quote! { #buf.extend_from_slice(b"false"); }
    }
}

fn gen_string(buf: &Expr, s: &str) -> proc_macro2::TokenStream {
    quote! {
        #buf.push(b'"');
        #buf.extend_from_slice(#s.as_bytes());
        #buf.push(b'"');
    }
}

fn gen_number(buf: &Expr, n: &str) -> proc_macro2::TokenStream {
    quote! { #buf.extend_from_slice(#n.as_bytes()); }
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

fn gen_value(buf: &Expr, value: &JsonValue) -> proc_macro2::TokenStream {
    match value {
        JsonValue::Null => gen_null(buf),
        JsonValue::Bool(b) => gen_bool(buf, *b),
        JsonValue::String(s) => gen_string(buf, s),
        JsonValue::Number(n) => gen_number(buf, n),
        JsonValue::Array(arr) => gen_array(buf, arr),
        JsonValue::Object(obj) => gen_object(buf, obj),
    }
}

#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as JsonArgs);
    let res = gen_value(&args.buf, &args.value);
    TokenStream::from(res)
}
