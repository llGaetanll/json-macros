use std::io::Write;

use json_writer::json;

fn main() -> std::io::Result<()> {
    let mut buf = Vec::new();

    json!(&mut buf, {
        name: "John",
        age: 30,
        car: null
    })
}
