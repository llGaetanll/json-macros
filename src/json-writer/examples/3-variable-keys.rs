use std::io::Write;

use json_traits::SerializeStr;
use json_writer::json;

fn main() -> std::io::Result<()> {
    let mut buf = Vec::new();

    let key = "name";

    json!(&mut buf, {
        [key]: "Michael",
        age: 42,
        car: null
    });

    println!("{}", String::from_utf8_lossy(&buf));

    let exp = br#"{"name":"Michael","age":42,"car":null}"#;
    assert_eq!(
        buf,
        exp,
        "\nl: {}\nr: {}",
        String::from_utf8_lossy(&buf),
        String::from_utf8_lossy(exp)
    );

    Ok(())
}
