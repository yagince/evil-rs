use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DataStruct, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(Omit, attributes(omit))]
pub fn derive_omit(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    match gen_omitted_type(item) {
        Ok(x) => x,
        Err(err) => err.to_compile_error().into(),
    }
}

fn gen_omitted_type(item: DeriveInput) -> Result<TokenStream, syn::Error> {
    let new_types = extract_new_type(&item.attrs)?;

    let fields = if let syn::Data::Struct(DataStruct {
        fields: Fields::Named(ref fields),
        ..
    }) = item.data
    {
        fields.named.clone()
    } else {
        return Err(syn::Error::new_spanned(
            item.ident,
            "`Omit` supports only `struct`",
        ));
    };

    let type_tokens = new_types
        .into_iter()
        .map(|(name, omits)| {
            let fields = fields
                .iter()
                .filter(|x| match x {
                    Field { ident: Some(x), .. } => !omits.contains(x),
                    _ => true,
                })
                .map(|field| quote!(#field))
                .collect::<Vec<_>>();

            quote! {
                struct #name {
                    #(#fields,)*
                }
            }
        })
        .collect::<Vec<_>>();

    let gen = quote! {
        #(#type_tokens)*
    };

    dbg!(gen.to_string());

    Ok(gen.into())
}

fn extract_new_type(attrs: &[Attribute]) -> Result<Vec<(Ident, Vec<Ident>)>, syn::Error> {
    dbg!(attrs)
        .iter()
        .filter_map(|x| {
            if x.path.is_ident("omit") {
                Some(x)
            } else {
                None
            }
        })
        .map(|x| {
            let values: Vec<Ident> = x
                .tokens
                .clone()
                .into_iter()
                .flat_map(|x| match x {
                    quote::__private::TokenTree::Ident(x) => vec![x],
                    quote::__private::TokenTree::Group(x) => x
                        .stream()
                        .into_iter()
                        .filter_map(|y| match y {
                            quote::__private::TokenTree::Ident(x) => Some(x),
                            _ => None,
                        })
                        .collect(),
                    _ => vec![],
                })
                .collect();

            match &values[..] {
                [first, omits @ ..] => Ok((first.to_owned(), omits.to_vec())),
                _ => Err(syn::Error::new_spanned(x.path.clone(), "invalid syntax")),
            }
        })
        .collect::<Result<Vec<_>, syn::Error>>()
}
