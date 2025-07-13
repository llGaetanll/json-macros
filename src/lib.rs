use proc_macro::TokenStream;
use syn::parse_macro_input;

mod generate;
mod parse;

use parse::JsonArgs;

#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as JsonArgs);
    let res = generate::gen_value(&args.buf, &args.value);
    TokenStream::from(res)
}
