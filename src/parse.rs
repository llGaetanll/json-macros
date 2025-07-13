use syn::{braced, bracketed, parse::ParseStream, Expr, Ident, Lit, Token};

pub struct JsonArgs {
    pub buf: Expr,
    pub value: JsonValue,
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
pub enum JsonValue {
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
            parse_object(input)
        } else if input.peek(syn::token::Bracket) {
            parse_array(input)
        } else if input.peek(Ident) {
            parse_ident(input)
        } else {
            parse_lit(input)
        }
    }
}

fn parse_object(input: ParseStream) -> syn::Result<JsonValue> {
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
}

fn parse_array(input: ParseStream) -> syn::Result<JsonValue> {
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
}

// TODO: Just null for now, but soon this will be used for non-literal values too
fn parse_ident(input: ParseStream) -> syn::Result<JsonValue> {
    let ident: Ident = input.parse()?;

    if ident == "null" {
        Ok(JsonValue::Null)
    } else {
        Err(syn::Error::new_spanned(ident, "unknown identifier"))
    }
}

fn parse_lit(input: ParseStream) -> syn::Result<JsonValue> {
    let lit: Lit = input.parse()?;

    match lit {
        Lit::Bool(b) => Ok(JsonValue::Bool(b.value)),
        Lit::Str(s) => Ok(JsonValue::String(s.value())),
        Lit::Int(i) => Ok(JsonValue::Number(i.base10_digits().to_string())),
        Lit::Float(f) => Ok(JsonValue::Number(f.base10_digits().to_string())),
        _ => Err(syn::Error::new_spanned(lit, "unsupported literal")),
    }
}
