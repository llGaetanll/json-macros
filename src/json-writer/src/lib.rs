use proc_macro::TokenStream;
use syn::parse_macro_input;

mod generate;
mod ir;
mod parse;

use parse::JsonArgs;

#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as JsonArgs);
    let chunks = ir::ast_merge(&args.value);
    let res = generate::from_chunks(&args.buf, &chunks);
    TokenStream::from(res)
}
