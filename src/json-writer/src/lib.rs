use proc_macro::TokenStream;
use syn::parse_macro_input;

mod generate;
mod parse;

use parse::JsonArgs;
use parse::JsonValue;

#[proc_macro]
pub fn json_write(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as JsonArgs);
    let res = generate::gen_write(&args.buf, &args.value);
    TokenStream::from(res)
}

#[proc_macro]
pub fn json_value(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as JsonValue);
    let res = generate::gen_value_pure(&args);
    TokenStream::from(res)
}
