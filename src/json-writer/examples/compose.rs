use json_traits::Serialize;
use json_writer::json;
use json_writer::lazy;
use json_writer::lazy_move;

fn gen_profile(first: &str, last: &str) -> impl Serialize {
    lazy_move!({ first, last })
}

// See generated code with
//
//     cargo expand --package json-writer --example compose
//
fn main() {
    let mut buf = Vec::new();

    let michael = gen_profile("Michael", "Scott");

    // `lazy` defers the writing behind a closure
    let dwight = lazy!({
        first: "Dwight",
        last: "Schrute"
    });

    let jim = lazy!({
        first: "Jim",
        last: "Halpert"
    });

    // Works like this
    json!(&mut buf, {
        boss: michael,
        employees: [
            dwight,
            jim
        ]
    });

    // We can also just call serialize directly!
    // This is what our macros do under the hood
    dwight.serialize(&mut buf);

    println!("{}", String::from_utf8_lossy(&buf));
}
