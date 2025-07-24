use json_traits::Serialize;
use json_writer::json;

fn regional_manager(buf: &mut Vec<u8>) {
    json!(buf, {
        first: "Michael",
        last: "Scott",
    });
}

fn main() {
    let mut buf = Vec::new();

    // Should still work with values like this
    let value = true;
    json!(&mut buf, { value });

    // Works like this
    json!(&mut buf, {
        regional_manager,
    });

    json!(&mut buf, [regional_manager]);

    println!("{}", String::from_utf8_lossy(&buf));
}
