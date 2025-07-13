use json_macro_writer::json;

#[test]
fn test_null() {
    let mut buf = Vec::new();
    json!(buf, null);
    assert_eq!(buf, b"null");
}

#[test]
fn test_true() {
    let mut buf = Vec::new();
    json!(buf, true);
    assert_eq!(buf, b"true");
}

#[test]
fn test_false() {
    let mut buf = Vec::new();
    json!(buf, false);
    assert_eq!(buf, b"false");
}

#[test]
fn test_string_empty() {
    let mut buf = Vec::new();
    json!(buf, "");
    assert_eq!(buf, b"\"\"");
}

#[test]
fn test_string_nonempty() {
    let mut buf = Vec::new();
    json!(buf, "Hello, world");
    assert_eq!(buf, b"\"Hello, world\"");
}

// TODO
//
// for empty array, the generated macro looks like
// ```
// buf.push(b'[');
// buf.push(b']');
// ```
// which causes the clippy lint to trigger. We should probably have better generation for empty
// arrays anyway.
#[test]
#[allow(clippy::vec_init_then_push)]
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
    json!(buf, [null, true, false]);
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
    json!(buf, {
        name: "Michael",
        age: 42
    });
    assert_eq!(buf, br#"{"name":"Michael","age":42}"#);
}

#[test]
fn test_object_deep() {
    let mut buf = Vec::new();
    json!(buf, {
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
    // assert_eq!(buf, ...); TODO: complete this
}
