use std::io::Write;

use json_writer::json;

// `json!` can write to any type `W: std::io::Write`.
fn look_ma_no_allocs(mut buf: &mut [u8]) -> std::io::Result<()> {
    json!(&mut buf, {
        first: "Michael",
        last: "Scott",
        age: 42,
    })
}

fn main() -> std::io::Result<()> {
    let mut buf = [0u8; 128];

    look_ma_no_allocs(&mut buf)?;

    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
}
