use std::io::Write;

use json_traits::Serialize;
use json_writer::json;
use json_writer::lazy;
use json_writer::lazy_move;

fn gen_employee(first: &str, last: &str) -> impl Serialize<Vec<u8>> {
    // `lazy_move` defers writing while taking ownership of the environment.
    //
    // This allows us to use it as the return values in functions
    lazy_move!({
        first,
        last
    })
}

// See generated code with
//
//     cargo expand --package json-writer --example compose
//
fn main() -> std::io::Result<()> {
    let mut buf = Vec::new();

    let michael = gen_employee("Michael", "Scott");

    let company = String::from("Dunder Mifflin Paper Company");

    // `lazy` defers the writing behind a closure.
    let dwight = lazy!({
        first: "Dwight",
        last: "Schrute",
        company
    });

    // Since `lazy` doesn't move its environment, we can reuse variables here.
    let jim = lazy!({
        first: "Jim",
        last: "Halpert",
        company
    });

    // When we're done, we can write the final version of our object with `json!`.
    json!(&mut buf, {
        boss: michael,
        employees: [
            dwight,
            jim
        ]
    });

    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
}
