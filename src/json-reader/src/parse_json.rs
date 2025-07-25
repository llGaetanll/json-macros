use crate::Parse;

// Single byte tokens
pub struct JsonComma;
pub struct JsonColon;
pub struct JsonQuote;
pub struct JsonObjStart;
pub struct JsonObjEnd;
pub struct JsonArrStart;
pub struct JsonArrEnd;

pub type JsonResult<T> = Result<T, JsonError>;

#[derive(Debug)]
pub enum JsonError {
    UnexpectedEndOfInput,
    UnexpectedToken,
}

fn take1(input: &[u8]) -> JsonResult<(u8, &[u8])> {
    let [c, input @ ..] = input else {
        return Err(JsonError::UnexpectedEndOfInput);
    };

    Ok((*c, input))
}

macro_rules! impl_byte_parser {
    ($type:ty, $token:expr) => {
        impl Parse for $type {
            type Error = JsonError;

            fn parse(input: &[u8]) -> Result<(Self, &[u8]), Self::Error> {
                match input {
                    [] => Err(JsonError::UnexpectedEndOfInput),
                    [$token, tail @ ..] => Ok((Self, tail)),
                    [_c, ..] => Err(JsonError::UnexpectedToken),
                }
            }
        }
    };
}

impl_byte_parser!(JsonComma, b',');
impl_byte_parser!(JsonColon, b':');
impl_byte_parser!(JsonQuote, b'"');
impl_byte_parser!(JsonObjStart, b'{');
impl_byte_parser!(JsonObjEnd, b'}');
impl_byte_parser!(JsonArrStart, b'[');
impl_byte_parser!(JsonArrEnd, b']');

pub type JsonKey = JsonStr;
pub struct JsonStr;

impl Parse for JsonStr {
    type Error = JsonError;

    fn parse(input: &[u8]) -> Result<(Self, &[u8]), Self::Error> {
        let (_, mut input) = JsonQuote::parse(input)?;
        let mut c;
        let mut esc = false;

        loop {
            (c, input) = take1(input)?;

            match c {
                b'"' if !esc => break,
                b'\\' => {
                    esc = !esc;
                }
                _ => {
                    esc = false;
                }
            }
        }

        Ok((Self, input))
    }
}
