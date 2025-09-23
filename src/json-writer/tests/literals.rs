use std::io::Write;

use json_writer::json;

macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        ::core::assert_eq!(
            String::from_utf8_lossy(&$left),
            String::from_utf8_lossy($right)
        )
    };
}

#[test]
fn test_null() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, null);
    assert_eq!(buf, b"null");

    Ok(())
}

#[test]
fn test_true() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, true);
    assert_eq!(buf, b"true");

    Ok(())
}

#[test]
fn test_false() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, false);
    assert_eq!(buf, b"false");

    Ok(())
}

#[test]
fn test_number_whole() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, 1);
    assert_eq!(buf, b"1");

    Ok(())
}

#[test]
fn test_number_negative() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, -1);
    assert_eq!(buf, b"-1");

    Ok(())
}

#[test]
fn test_number_float() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, 3.141592653);
    assert_eq!(buf, b"3.141592653");

    Ok(())
}

#[test]
fn test_string_empty() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, "");
    assert_eq!(buf, b"\"\"");

    Ok(())
}

#[test]
fn test_string_nonempty() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, "Hello, world");
    assert_eq!(buf, b"\"Hello, world\"");

    Ok(())
}

#[test]
fn test_array_empty() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, []);
    assert_eq!(buf, b"[]");

    Ok(())
}

#[test]
fn test_array_one() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, [null]);
    assert_eq!(buf, b"[null]");

    Ok(())
}

#[test]
fn test_array_many() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, [null, true, false]);
    assert_eq!(buf, b"[null,true,false]");

    Ok(())
}

#[test]
fn test_object_empty() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, {});
    assert_eq!(buf, b"{}");

    Ok(())
}

#[test]
fn test_object_shallow() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, {
        name: "Michael",
        age: 42
    });
    assert_eq!(buf, br#"{"name":"Michael","age":42}"#);

    Ok(())
}

#[test]
fn test_object_deep() -> std::io::Result<()> {
    let mut buf = Vec::new();
    json!(&mut buf, {
        name: "Michael",
        age: 42,
        friends: [
            {
                name: "Emily",
                age: 29
            },
            {
                name: "Jim",
                age: 25
            },
        ]
    });
    assert_eq!(buf, br#"{"name":"Michael","age":42,"friends":[{"name":"Emily","age":29},{"name":"Jim","age":25}]}"#);

    Ok(())
}
