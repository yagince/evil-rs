use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DataStruct, DeriveInput, Field, Fields};

mod option;

use option::OmitOption;

pub(crate) fn gen_omitted_type(item: DeriveInput) -> Result<TokenStream, syn::Error> {
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
        .map(|opt| {
            let fields = fields
                .iter()
                .filter(|x| match x {
                    Field { ident: Some(x), .. } => !opt.ignores.contains(x),
                    _ => true,
                })
                .map(|field| quote!(#field))
                .collect::<Vec<_>>();

            let derive = opt
                .derive_option
                .map(|x| {
                    let derives = x.derives;
                    quote! {
                        #[derive(#(#derives),*)]
                    }
                })
                .unwrap_or_default();

            let name = opt.name;
            quote! {
                #derive
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

fn extract_new_type(attrs: &[Attribute]) -> Result<Vec<OmitOption>, syn::Error> {
    attrs
        .iter()
        .filter_map(|x| {
            if x.path.is_ident("omit") {
                Some(x)
            } else {
                None
            }
        })
        .map(|x| match syn::parse2::<OmitOption>(x.tokens.clone()) {
            Ok(x) => Ok(x),
            Err(e) => Err(syn::Error::new_spanned(x.path.clone(), e)),
        })
        .collect::<Result<Vec<_>, syn::Error>>()
}
