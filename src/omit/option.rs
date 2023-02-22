use syn::{parenthesized, parse::Parse, token::Paren, Ident, Token};

mod kw {
    syn::custom_keyword!(derive);
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct OmitOption {
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
                    } else if derive_content.peek(Token![::]) {
                        dbg!(&derive_content);
                        // if let Some(last) = derives.last_mut() {
                        //     let colon2: Token![::] = derive_content.parse().unwrap();
                        //     let type_name: Ident = derive_content.parse()?;
                        //     // *last = format_ident!("{}::{}", last, type_name);
                        //     // *last = Group::new(quote::__private::Delimiter::None, )
                        //     let mut tokens = TokenStream::new();
                        //     tokens.extend(colon2.into());
                        // }
                        break;
                    } else {
                        break;
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

    // #[test]
    // fn test_parse_omit_option_no_ignores_with_derives() {
    //     let token = quote::quote! {
    //         (NewHoge, derive(Debug, hoge::Clone))
    //     };
    //     dbg!(&token);

    //     let ret = syn::parse2::<OmitOption>(token);

    //     assert_matches!(ret, Ok(x) => {
    //         assert_eq!(x.name, "NewHoge");
    //         assert_matches!(&x.ignores[..], []);
    //         assert_eq!(x.derives, vec!["Debug", "hoge::Clone"]);
    //     });
    //     assert!(false);
    // }

    #[test]
    fn test_parse_omit_option_err() {
        let token = quote::quote! {
            (NewHoge, ,)
        };
        dbg!(&token);
        let ret = syn::parse2::<OmitOption>(token);

        assert_matches!(ret, Err(_));
    }
}
