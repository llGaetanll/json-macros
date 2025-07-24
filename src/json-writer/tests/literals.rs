use json_traits::Serialize;
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
fn test_null() {
    let mut buf = Vec::new();
    json!(buf, null);
    assert_eq!(buf, b"null");
}

#[test]
fn test_true() {
    let mut buf = Vec::new();
    json!(&mut buf, true);
    assert_eq!(buf, b"true");
}

#[test]
fn test_false() {
    let mut buf = Vec::new();
    json!(&mut buf, false);
    assert_eq!(buf, b"false");
}

#[test]
fn test_number_whole() {
    let mut buf = Vec::new();
    json!(buf, 1);
    assert_eq!(buf, b"1");
}

#[test]
fn test_number_negative() {
    let mut buf = Vec::new();
    json!(buf, -1);
    assert_eq!(buf, b"-1");
}

#[test]
fn test_number_float() {
    let mut buf = Vec::new();
    json!(buf, 3.141592653);
    assert_eq!(buf, b"3.141592653");
}

#[test]
fn test_string_empty() {
    let mut buf = Vec::new();
    json!(&mut buf, "");
    assert_eq!(buf, b"\"\"");
}

#[test]
fn test_string_nonempty() {
    let mut buf = Vec::new();
    json!(&mut buf, "Hello, world");
    assert_eq!(buf, b"\"Hello, world\"");
}

#[test]
fn test_array_empty() {
    let mut buf = Vec::new();
    json!(buf, []);
    assert_eq!(buf, b"[]");
}

#[test]
fn test_array_one() {
    let mut buf = Vec::new();
    json!(buf, [null]);
    assert_eq!(buf, b"[null]");
}

#[test]
fn test_array_many() {
    let mut buf = Vec::new();
    json!(&mut buf, [null, true, false]);
    assert_eq!(buf, b"[null,true,false]");
}

#[test]
fn test_object_empty() {
    let mut buf = Vec::new();
    json!(buf, {});
    assert_eq!(buf, b"{}");
}

#[test]
fn test_object_shallow() {
    let mut buf = Vec::new();
    json!(&mut buf, {
        name: "Michael",
        age: 42
    });
    assert_eq!(buf, br#"{"name":"Michael","age":42}"#);
}

#[test]
fn test_object_deep() {
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
}
