use std::io::Write;

use json_traits::Serialize;
use json_writer::json;

#[test]
fn contract() -> std::io::Result<()> {
    let mut buf = Vec::new();

    let value = true;
    json!(&mut buf, { value });

    assert_eq!(buf, br#"{"value":true}"#);

    Ok(())
}

#[test]
fn write_array() -> std::io::Result<()> {
    let mut buf = [0u8; 128];

    fn inner_write(mut buf: &mut [u8]) -> std::io::Result<()> {
        json!(&mut buf, { foo: "bar" })
    }

    inner_write(&mut buf)?;

    assert_eq!(&buf[..13], br#"{"foo":"bar"}"#);

    Ok(())
}

#[test]
fn write_array_too_small() -> std::io::Result<()> {
    let mut buf = [0u8; 10];

    fn inner_write(mut buf: &mut [u8]) -> std::io::Result<()> {
        json!(&mut buf, { foo: "bar" })
    }

    let res = inner_write(&mut buf);

    assert!(res.is_err());

    Ok(())
}
