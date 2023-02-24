use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Field, Fields};

use crate::new_type_option::extract_new_type;

pub(crate) fn gen_picked_type(item: DeriveInput) -> Result<TokenStream, syn::Error> {
    let new_types = extract_new_type("pick", &item.attrs)?;

    let fields = if let syn::Data::Struct(DataStruct {
        fields: Fields::Named(ref fields),
        ..
    }) = item.data
    {
        fields.named.clone()
    } else {
        return Err(syn::Error::new_spanned(
            item.ident,
            "`Pick` supports only `struct`",
        ));
    };

    let type_tokens = new_types
        .into_iter()
        .map(|opt| {
            let fields = fields
                .iter()
                .filter(|x| match x {
                    Field { ident: Some(x), .. } => opt.fields.contains(x),
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
