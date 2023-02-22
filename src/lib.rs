use omit::gen_omitted_type;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod omit;

#[proc_macro_derive(Omit, attributes(omit))]
pub fn derive_omit(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    match gen_omitted_type(item) {
        Ok(x) => x,
        Err(err) => err.to_compile_error().into(),
    }
}
