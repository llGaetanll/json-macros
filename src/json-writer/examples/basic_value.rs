use json_traits::Serialize;
use json_writer::json_value;

fn main() {
    let value = json_value!(true);
    let verdict = json_value!({ value });

    let value = true;
    let verdict = json_value!({ value });
}
