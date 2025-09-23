use std::fmt::Debug;

use json_traits::Serialize;
use proc_macro2::Ident;

use crate::parse::JsonValue;

pub enum JsonChunk {
    Dyn(Ident),
    Static(Vec<u8>),
}

impl Debug for JsonChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonChunk::Dyn(ident) => f.debug_tuple("Dyn").field(ident).finish(),
            JsonChunk::Static(bytes) => {
                let utf8_str = String::from_utf8_lossy(bytes);
                f.debug_tuple("Static").field(&utf8_str.as_ref()).finish()
            }
        }
    }
}

pub fn ast_merge(value: &JsonValue) -> Vec<JsonChunk> {
    let mut lir = Vec::new();
    let mut buf = Vec::new();

    merge(&mut buf, &mut lir, value);

    if !buf.is_empty() {
        lir.push(JsonChunk::Static(buf.clone()));
    }

    lir
}

fn merge(buf: &mut Vec<u8>, lir: &mut Vec<JsonChunk>, value: &JsonValue) {
    match value {
        JsonValue::Dyn(ident) => {
            if !buf.is_empty() {
                lir.push(JsonChunk::Static(buf.clone()));
                buf.clear();
            }

            lir.push(JsonChunk::Dyn(ident.clone()));
        }
        JsonValue::Null => buf.extend_from_slice(b"null"),
        JsonValue::Bool(b) => b
            .serialize(buf)
            .expect("Failed to serialize JsonValue::bool"),
        JsonValue::String(s) => s
            .serialize(buf)
            .expect("Failed to serialize JsonValue::String"),
        JsonValue::Number(n) => buf.extend_from_slice(n.as_bytes()),
        JsonValue::Array(arr) => {
            buf.push(b'[');

            for (i, val) in arr.iter().enumerate() {
                if i > 0 {
                    buf.push(b',');
                }

                merge(buf, lir, val);
            }

            buf.push(b']');
        }
        JsonValue::Object(obj) => {
            buf.push(b'{');

            for (i, (k, v)) in obj.iter().enumerate() {
                if i > 0 {
                    buf.push(b',');
                }

                buf.push(b'"');
                buf.extend_from_slice(k.as_bytes());
                buf.push(b'"');
                buf.push(b':');

                merge(buf, lir, v);
            }

            buf.push(b'}');
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;
    use proc_macro2::Ident;
    use proc_macro2::Span;

    use super::ast_merge;
    use crate::parse::JsonValue;

    #[test]
    fn gen_dyn() {
        let value = JsonValue::Dyn(Ident::new("dummy", Span::call_site()));

        let lir = ast_merge(&value);

        assert_debug_snapshot!(lir, @r"
        [
            Dyn(
                Ident(
                    dummy,
                ),
            ),
        ]
        ");
    }

    #[test]
    fn gen_static() {
        let value = JsonValue::Object(vec![
            (
                "first".to_string(),
                JsonValue::String("Michael".to_string()),
            ),
            ("last".to_string(), JsonValue::String("Scott".to_string())),
            (
                "friends".to_string(),
                JsonValue::Array(vec![
                    JsonValue::String("Pam".to_string()),
                    JsonValue::String("Jim".to_string()),
                ]),
            ),
        ]);

        let lir = ast_merge(&value);

        assert_debug_snapshot!(lir, @r#"
        [
            Static(
                "{\"first\":\"Michael\",\"last\":\"Scott\",\"friends\":[\"Pam\",\"Jim\"]}",
            ),
        ]
        "#);
    }

    #[test]
    fn gen_dyn_simple() {
        let value = JsonValue::Object(vec![
            (
                "first".to_string(),
                JsonValue::String("Michael".to_string()),
            ),
            ("last".to_string(), JsonValue::String("Scott".to_string())),
            (
                "age".to_string(),
                JsonValue::Dyn(Ident::new("dummy", Span::call_site())),
            ),
            (
                "friends".to_string(),
                JsonValue::Array(vec![
                    JsonValue::String("Pam".to_string()),
                    JsonValue::String("Jim".to_string()),
                ]),
            ),
        ]);

        let lir = ast_merge(&value);

        assert_debug_snapshot!(lir, @r#"
        [
            Static(
                "{\"first\":\"Michael\",\"last\":\"Scott\",\"age\":",
            ),
            Dyn(
                Ident(
                    dummy,
                ),
            ),
            Static(
                ",\"friends\":[\"Pam\",\"Jim\"]}",
            ),
        ]
        "#);
    }
}
