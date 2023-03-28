use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod with_flags;

#[proc_macro_derive(DecodeWithFlags, attributes(flags_field, flag))]
pub fn derive_decode_with_flags(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match with_flags::impl_my_derive(&input) {
        Ok(output) => output,
        Err(error) => error.to_compile_error().into(),
    }
}

mod multiple;

#[proc_macro_derive(DecodeMultiple, attributes(many))]
pub fn derive_decode_multiple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match multiple::impl_my_derive(&input) {
        Ok(output) => output,
        Err(error) => error.to_compile_error().into(),
    }
}
