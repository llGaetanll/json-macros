use syn::Expr;
use syn::Ident;
use syn::Lit;
use syn::Token;
use syn::braced;
use syn::bracketed;
use syn::parse::ParseStream;

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
pub enum JsonKey {
    Lit(Ident),
    Dyn(Ident),
}

impl syn::parse::Parse for JsonKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = if input.peek(syn::token::Bracket) {
            let content;
            bracketed!(content in input);
            let ident: Ident = content.parse()?;

            JsonKey::Dyn(ident)
        } else {
            let ident: Ident = input.parse()?;

            JsonKey::Lit(ident)
        };

        Ok(key)
    }
}

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    String(String),
    Number(String),
    Array(Vec<JsonValue>),
    Object(Vec<(JsonKey, JsonValue)>),
    Dyn(Ident),
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

    let mut fields: Vec<(JsonKey, JsonValue)> = Vec::new();
    while !content.is_empty() {
        let key: JsonKey = content.parse()?;

        let value: JsonValue = match &key {
            JsonKey::Dyn(_) => {
                // A dynamic key must be followed by a value
                content.parse::<Token![:]>()?;
                content.parse()?
            }
            JsonKey::Lit(key) => {
                if content.peek(Token![:]) {
                    content.parse::<Token![:]>()?;
                    content.parse()?
                } else {
                    JsonValue::Dyn(key.clone())
                }
            }
        };

        fields.push((key, value));

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

fn parse_ident(input: ParseStream) -> syn::Result<JsonValue> {
    let ident: Ident = input.parse()?;

    if ident == "null" {
        return Ok(JsonValue::Null);
    }

    Ok(JsonValue::Dyn(ident))
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
