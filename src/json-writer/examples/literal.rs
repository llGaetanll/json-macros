use json_traits::Serialize;
use json_writer::json;

fn main() {
    let mut buf = Vec::new();

    json!(&mut buf, {
        name: "John",
        age: 30,
        car: null
    });
}
