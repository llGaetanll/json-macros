use json_traits::Serialize;
use json_writer::json;

fn gen_profile(first: &str, last: &str) -> impl Serialize {
    move |buf: &mut Vec<u8>| {
        json!(buf, {
            first,
            last,
        });
    }
}

fn main() {
    let mut buf = Vec::new();

    let profile = gen_profile("Michael", "Scott");

    // Works like this
    json!(&mut buf, {
        profile,
    });

    // Or like this
    json!(&mut buf, [profile]);

    println!("{}", String::from_utf8_lossy(&buf));
}
