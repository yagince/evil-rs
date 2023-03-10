use syn::{parenthesized, parse::Parse, token::Paren, Attribute, Ident, Path, Token};

mod kw {
    syn::custom_keyword!(derive);
}

pub(crate) fn extract_new_type(
    attr_name: &str,
    attrs: &[Attribute],
) -> Result<Vec<NewTypeOption>, syn::Error> {
    attrs
        .iter()
        .filter_map(|x| {
            if x.path.is_ident(attr_name) {
                Some(x)
            } else {
                None
            }
        })
        .map(|x| match syn::parse2::<NewTypeOption>(x.tokens.clone()) {
            Ok(x) => Ok(x),
            Err(e) => Err(syn::Error::new_spanned(x.path.clone(), e)),
        })
        .collect::<Result<Vec<_>, syn::Error>>()
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct NewTypeOption {
    pub name: Ident,
    pub fields: Vec<Ident>,
    pub derive_option: Option<DeriveOption>,
}

impl Parse for NewTypeOption {
    /// e.g. (NewType, id, created_at, derive(Debug, Clone))
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let _: Paren = parenthesized!(content in input);
        let name = content.parse()?;
        let _: syn::Result<Token![,]> = content.parse();

        let mut ignores = vec![];
        let mut derive_option = None;

        loop {
            if content.is_empty() {
                break;
            }

            if content.peek(kw::derive) {
                derive_option = Some(content.parse()?);
            } else if content.peek(Ident) {
                ignores.push(content.parse()?);
            } else {
                break;
            }

            if content.peek(Token![,]) {
                let _: Token![,] = content.parse()?;
            }
        }

        Ok(NewTypeOption {
            name,
            fields: ignores,
            derive_option,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct DeriveOption {
    pub derives: Vec<Path>,
}

impl Parse for DeriveOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_content;
        let _: kw::derive = input.parse()?;
        let _: Paren = parenthesized!(derive_content in input);

        let mut derives = vec![];

        loop {
            if derive_content.is_empty() {
                break;
            }
            if derive_content.peek(Ident) {
                derives.push(derive_content.call(Path::parse_mod_style)?);
                if derive_content.peek(Token![,]) {
                    let _: Token![,] = derive_content.parse()?;
                }
            } else {
                break;
            }
        }

        Ok(DeriveOption { derives })
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use darling::ToTokens;
    use pretty_assertions::assert_eq;
    use quote::quote;

    use super::*;

    #[test]
    fn test_parse_new_type_option_success() {
        let token = quote! {
            (NewHoge, id, derive(Debug, Clone))
        };
        dbg!(&token);
        let ret = syn::parse2::<NewTypeOption>(token);

        assert_matches!(ret, Ok(x) => {
            assert_eq!(x.name, "NewHoge");
            assert_eq!(x.fields, vec!["id"]);
            assert_matches!(x.derive_option, Some(_));
        });
    }

    #[test]
    fn test_parse_new_type_option_no_derives() {
        let token = quote! {
            (NewHoge, id)
        };
        dbg!(&token);
        let ret = syn::parse2::<NewTypeOption>(token);

        assert_matches!(ret, Ok(x) => {
            assert_eq!(x.name, "NewHoge");
            assert_eq!(x.fields, vec!["id"]);
            assert_matches!(x.derive_option, None);
        });
    }

    #[test]
    fn test_parse_new_type_option_no_ignores() {
        let token = quote! {
            (NewHoge,)
        };
        dbg!(&token);
        let ret = syn::parse2::<NewTypeOption>(token);

        assert_matches!(ret, Ok(x) => {
            assert_eq!(x.name, "NewHoge");
            assert_matches!(&x.fields[..], []);
            assert_matches!(x.derive_option, None);
        });
    }

    #[test]
    fn test_parse_new_type_option_no_ignores_with_derives() {
        let token = quote::quote! {
            (NewHoge, derive(Debug, hoge::Clone))
        };
        dbg!(&token);

        let ret = syn::parse2::<NewTypeOption>(token);

        assert_matches!(ret, Ok(x) => {
            assert_eq!(x.name, "NewHoge");
            assert_matches!(&x.fields[..], []);
            assert_matches!(x.derive_option, Some(DeriveOption { derives }) => {
                assert_matches!(&derives[..], [_, _]);
            });
        });
    }

    #[test]
    fn test_parse_new_type_option_no_ignores_with_derives_2() {
        let token = quote::quote! {
            (NewHoge, id, derive(Debug, hoge::Clone), hoge)
        };
        dbg!(&token);

        let ret = syn::parse2::<NewTypeOption>(token);

        assert_matches!(ret, Ok(x) => {
            assert_eq!(x.name, "NewHoge");
            assert_eq!(x.fields, vec!["id", "hoge"]);
            assert_matches!(x.derive_option, Some(DeriveOption { derives }) => {
                assert_matches!(&derives[..], [_, _]);
            });
        });
    }

    #[test]
    fn test_parse_new_type_option_err() {
        let token = quote! {
            (NewHoge, ,)
        };
        dbg!(&token);
        let ret = syn::parse2::<NewTypeOption>(token);

        assert_matches!(ret, Err(_));
    }

    #[test]
    fn test_parse_derive() {
        let token = dbg!(quote! {
            derive(NewHoge, hoge::Hoge)
        });

        let ret = syn::parse2::<DeriveOption>(token);
        assert_matches!(ret, Ok(x) => {
            assert_matches!(&x.derives[..], [first, second] => {
                assert_eq!(first.segments.to_token_stream().to_string(), "NewHoge");
                assert_eq!(second.segments.to_token_stream().to_string(), "hoge :: Hoge");
            });
        });
    }
}
