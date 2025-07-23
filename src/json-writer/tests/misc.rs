use json_traits::Serialize;
use json_writer::json;

#[test]
fn contract() {
    let mut buf = Vec::new();

    let value = true;
    json!(buf, { value });

    assert_eq!(buf, br#"{"value":true}"#);
}
