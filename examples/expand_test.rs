use json_macro_writer::json;

fn main() {
    let mut buf = Vec::new();
    json!(buf, [null, true, false]);
}
