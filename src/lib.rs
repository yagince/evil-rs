use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, DataStruct, DeriveInput, Field, Fields, Ident,
};

#[proc_macro_derive(Omit, attributes(omit, evil))]
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
                    Field { ident: Some(x), .. } => !omits.contains(&x.to_string()),
                    _ => true,
                })
                .map(|field| quote!(#field))
                .collect::<Vec<_>>();

            let name = Ident::new(&name, item.span());
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

    Ok(gen.into())
}

fn extract_new_type(attrs: &[Attribute]) -> Result<Vec<(String, Vec<String>)>, syn::Error> {
    attrs
        .iter()
        .filter_map(|x| {
            if x.path.is_ident("omit") {
                Some(x)
            } else {
                None
            }
        })
        .map(|x| {
            let values: Vec<String> = x
                .tokens
                .to_string()
                .split(['(', ')', ',', ' '])
                .filter(|x| !x.is_empty())
                .map(ToString::to_string)
                .collect();

            match &values[..] {
                [first, omits @ ..] => Ok((first.to_owned(), omits.to_vec())),
                _ => Err(syn::Error::new_spanned(x.path.clone(), "invalid syntax")),
            }
        })
        .collect::<Result<Vec<_>, syn::Error>>()
}
