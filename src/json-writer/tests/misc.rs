use json_traits::Serialize;
use json_writer::json_write;

#[test]
fn contract() {
    let mut buf = Vec::new();

    let value = true;
    json_write!(buf, { value });

    assert_eq!(buf, br#"{"value":true}"#);
}
