use json_traits::Serialize;
use json_writer::json_value;
use json_writer::json_write;

fn gen_inner() -> impl Serialize {
    json_value!(true)
}

fn gen_outer() -> impl Serialize {
    // The problem is that inner is owned by `get_outer`
    let inner = gen_inner();

    // But json_value produces a closure, which is returned by `get_outer`.
    // Naturally `inner` is dropped at the end of the function even though it
    // is needed outside of it.
    json_value!({ inner })
}

fn main() {
    let mut buf = Vec::new();

    let outer = gen_outer();
    json_write!(buf, outer);
}
