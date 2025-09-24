use proc_macro::TokenStream;
use syn::parse_macro_input;

mod generate;
mod ir;
mod parse;

use parse::JsonArgs;
use parse::JsonValue;

#[proc_macro]
/// Generate a `json` object which is eagerly written to the buffer.
pub fn json(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as JsonArgs);
    let chunks = ir::ast_merge(&args.value);
    let res = generate::from_chunks(&args.buf, &chunks);
    TokenStream::from(res)
}

#[proc_macro]
/// Generate a `json` object which is lazily written to the buffer. Useful for composition
pub fn lazy(input: TokenStream) -> TokenStream {
    let value = parse_macro_input!(input as JsonValue);
    let chunks = ir::ast_merge(&value);
    let res = generate::from_chunks_lazy(&chunks);
    TokenStream::from(res)
}

#[proc_macro]
/// Like `lazy` but generates `move` closures that take ownership of their captured environment.
///
/// This allows using the macro as the return value of a function.
pub fn lazy_move(input: TokenStream) -> TokenStream {
    let value = parse_macro_input!(input as JsonValue);
    let chunks = ir::ast_merge(&value);
    let res = generate::from_chunks_lazy_move(&chunks);
    TokenStream::from(res)
}
