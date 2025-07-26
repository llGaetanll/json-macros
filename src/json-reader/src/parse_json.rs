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
    InvalidUtf8(::std::str::Utf8Error),
    InvalidToken,
}

const fn take_1(input: &[u8]) -> Option<(u8, &[u8])> {
    let [c, input @ ..] = input else { return None };

    Some((*c, input))
}

const fn peek1(input: &[u8]) -> Option<u8> {
    let [c, ..] = input else { return None };

    Some(*c)
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
            (c, input) = take_1(input).ok_or(JsonError::UnexpectedEndOfInput)?;

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

/// Kind of a cop-out. This doesn't strictly follow the JSON spec for parsing pure numbers. Under
/// the hood, it uses Rust's own `parse()` function for number types.
pub struct JsonNum;

impl Parse for JsonNum {
    type Error = JsonError;

    fn parse(input: &[u8]) -> Result<(Self, &[u8]), Self::Error> {
        let mut i = 0;
        loop {
            match input.get(i) {
                None | Some(b',') => break,
                Some(_) => {}
            };

            i += 1;
        }

        let (num, input) = input.split_at(i); // Safe
        let num = ::std::str::from_utf8(num).map_err(JsonError::InvalidUtf8)?;

        // TODO: f64 only for now
        let _num: f64 = num.parse().map_err(|_| JsonError::InvalidToken)?;

        Ok((Self, input))
    }
}

pub struct JsonNull;

impl Parse for JsonNull {
    type Error = JsonError;

    fn parse(input: &[u8]) -> Result<(Self, &[u8]), Self::Error> {
        let Some((null, input)) = input.split_at_checked(4) else {
            return Err(JsonError::UnexpectedEndOfInput);
        };

        if null == b"null" {
            Ok((Self, input))
        } else {
            Err(JsonError::UnexpectedToken)
        }
    }
}

pub struct JsonBool;

impl Parse for JsonBool {
    type Error = JsonError;

    fn parse(input: &[u8]) -> Result<(Self, &[u8]), Self::Error> {
        match peek1(input) {
            None => Err(JsonError::UnexpectedEndOfInput),
            Some(b't') => {
                let Some((t, input)) = input.split_at_checked(4) else {
                    return Err(JsonError::UnexpectedEndOfInput);
                };

                if t == b"true" {
                    Ok((Self, input))
                } else {
                    Err(JsonError::InvalidToken)
                }
            }
            Some(b'f') => {
                let Some((f, input)) = input.split_at_checked(5) else {
                    return Err(JsonError::UnexpectedEndOfInput);
                };

                if f == b"false" {
                    Ok((Self, input))
                } else {
                    Err(JsonError::InvalidToken)
                }
            }
            Some(_) => Err(JsonError::InvalidToken),
        }
    }
}

pub struct JsonValue;

impl Parse for JsonValue {
    type Error = JsonError;

    fn parse(input: &[u8]) -> Result<(Self, &[u8]), Self::Error> {
        if let Ok((_, input)) = JsonNull::parse(input) {
            return Ok((Self, input));
        };

        if let Ok((_, input)) = JsonBool::parse(input) {
            return Ok((Self, input));
        };

        if let Ok((_, input)) = JsonNum::parse(input) {
            return Ok((Self, input));
        };

        if let Ok((_, input)) = JsonStr::parse(input) {
            return Ok((Self, input));
        };

        if let Ok((_, input)) = JsonArr::parse(input) {
            return Ok((Self, input));
        };

        if let Ok((_, input)) = JsonObj::parse(input) {
            return Ok((Self, input));
        };

        Err(JsonError::InvalidToken)
    }
}

pub struct JsonArr;

impl Parse for JsonArr {
    type Error = JsonError;

    fn parse(input: &[u8]) -> Result<(Self, &[u8]), Self::Error> {
        let (_, mut input) = JsonArrStart::parse(input)?;

        // TODO: Only works for non-empty arrays
        loop {
            (_, input) = JsonValue::parse(input)?;

            match JsonComma::parse(input) {
                Err(_) => break,
                Ok((_, inp)) => input = inp,
            }
        }

        let (_, input) = JsonArrEnd::parse(input)?;

        Ok((Self, input))
    }
}

pub struct JsonObj;

impl Parse for JsonObj {
    type Error = JsonError;

    fn parse(input: &[u8]) -> Result<(Self, &[u8]), Self::Error> {
        let (_, mut input) = JsonObjStart::parse(input)?;

        // TODO: Only works for non-empty objects
        loop {
            (_, input) = JsonKey::parse(input)?;
            (_, input) = JsonColon::parse(input)?;
            (_, input) = JsonValue::parse(input)?;

            match JsonComma::parse(input) {
                Err(_) => break,
                Ok((_, inp)) => input = inp,
            }
        }

        let (_, input) = JsonObjEnd::parse(input)?;

        Ok((Self, input))
    }
}
