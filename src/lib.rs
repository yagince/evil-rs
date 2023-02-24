mod new_type_option;
mod omit;
mod pick;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Omit, attributes(omit))]
pub fn derive_omit(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    match omit::gen_omitted_type(item) {
        Ok(x) => x,
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(Pick, attributes(pick))]
pub fn derive_pick(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    match pick::gen_picked_type(item) {
        Ok(x) => x,
        Err(err) => err.to_compile_error().into(),
    }
}
