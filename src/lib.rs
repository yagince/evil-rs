use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized, parse::Parse, parse_macro_input, token::Paren, Attribute, DataStruct,
    DeriveInput, Field, Fields, Ident, Token,
};

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

mod kw {
    syn::custom_keyword!(derive);
}

#[derive(Debug, Clone, PartialEq)]
struct OmitOption {
    pub name: Ident,
    pub ignores: Vec<Ident>,
    pub derives: Vec<Ident>,
}

impl Parse for OmitOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let _: Paren = parenthesized!(content in input);
        let name = content.parse()?;
        let _: syn::Result<Token![,]> = content.parse();
        let mut ignores = vec![];
        let mut derives = vec![];
        loop {
            if content.is_empty() {
                break;
            }
            if content.peek(kw::derive) {
                let derive_content;
                let _: kw::derive = content.parse()?;
                let _: Paren = parenthesized!(derive_content in content);

                loop {
                    if derive_content.is_empty() {
                        break;
                    }
                    if derive_content.peek(Ident) {
                        derives.push(derive_content.parse()?);
                        if derive_content.peek(Token![,]) {
                            let _: Token![,] = derive_content.parse()?;
                        }
                    }
                }
                break;
            } else if content.peek(Ident) {
                ignores.push(content.parse()?);
                if content.peek(Token![,]) {
                    let _: Token![,] = content.parse()?;
                }
            } else {
                break;
            }
        }

        // dbg!(&ignores);
        Ok(OmitOption {
            name,
            ignores,
            derives,
        })
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse_omit_option_success() {
        let token = quote::quote! {
            (NewHoge, id, derive(Debug, Clone))
        };
        dbg!(&token);
        let ret = syn::parse2::<OmitOption>(token);

        assert_matches!(ret, Ok(x) => {
            assert_eq!(x.name, "NewHoge");
            assert_eq!(x.ignores, vec!["id"]);
            assert_eq!(x.derives, vec!["Debug", "Clone"]);
        });
    }

    #[test]
    fn test_parse_omit_option_no_derives() {
        let token = quote::quote! {
            (NewHoge, id)
        };
        dbg!(&token);
        let ret = syn::parse2::<OmitOption>(token);

        assert_matches!(ret, Ok(x) => {
            assert_eq!(x.name, "NewHoge");
            assert_eq!(x.ignores, vec!["id"]);
            assert_matches!(&x.derives[..], []);
        });
    }

    #[test]
    fn test_parse_omit_option_no_ignores() {
        let token = quote::quote! {
            (NewHoge,)
        };
        dbg!(&token);
        let ret = syn::parse2::<OmitOption>(token);

        assert_matches!(ret, Ok(x) => {
            assert_eq!(x.name, "NewHoge");
            assert_matches!(&x.ignores[..], []);
            assert_matches!(&x.derives[..], []);
        });
    }

    #[test]
    fn test_parse_omit_option_no_ignores_with_derives() {
        let token = quote::quote! {
            (NewHoge, derive(Debug))
        };
        dbg!(&token);
        let ret = syn::parse2::<OmitOption>(token);

        assert_matches!(ret, Ok(x) => {
            assert_eq!(x.name, "NewHoge");
            assert_matches!(&x.ignores[..], []);
            assert_eq!(x.derives, vec!["Debug"]);
        });
    }
}
