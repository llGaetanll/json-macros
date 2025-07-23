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
fn test_true() {
    let mut buf = Vec::new();

    let value = true;
    json!(buf, value);

    assert_eq!(buf, b"true");
}

#[test]
fn test_false() {
    let mut buf = Vec::new();

    let value = false;
    json!(buf, value);

    assert_eq!(buf, b"false");
}

#[test]
fn test_number_whole() {
    let mut buf = Vec::new();

    let value = 1;
    json!(buf, value);

    assert_eq!(buf, b"1");
}

#[test]
fn test_number_negative() {
    let mut buf = Vec::new();

    let value = -1;
    json!(buf, value);

    assert_eq!(buf, b"-1");
}

#[test]
#[allow(clippy::approx_constant)]
#[allow(clippy::excessive_precision)]
fn test_number_float() {
    let mut buf = Vec::new();

    let value: f32 = 3.1415927;
    json!(buf, value);

    assert_eq!(buf, b"3.1415927");
}

#[test]
fn test_string_empty() {
    let mut buf = Vec::new();

    let value = "";
    json!(buf, value);

    assert_eq!(buf, b"\"\"");
}

#[test]
fn test_string_nonempty() {
    let mut buf = Vec::new();

    let value = "Hello, world";
    json!(buf, value);

    assert_eq!(buf, b"\"Hello, world\"");
}
