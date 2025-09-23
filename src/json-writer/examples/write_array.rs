use std::io::Write;

use json_writer::json;

fn look_ma_no_allocs(mut buf: &mut [u8]) -> std::io::Result<()> {
    json!(&mut buf, {
        foo: "bar"
    })
}

fn main() -> std::io::Result<()> {
    let mut buf = [0u8; 128];

    look_ma_no_allocs(&mut buf)?;

    println!("message: {}", String::from_utf8_lossy(&buf));

    Ok(())
}
