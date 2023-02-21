use evil::Omit;
use syn::{parenthesized, parse::Parse, token, Ident, Token};

#[derive(Omit, Debug)]
#[omit(NewHoge, id)]
#[omit(OldHoge, age)]
struct Hoge {
    pub id: u64,
    pub age: u64,
}

#[test]
fn test_hoge() {
    let hoge = Hoge { id: 1000, age: 0 };
    assert_eq!(hoge.id, 1000);
    assert_eq!(hoge.age, 0);
}

#[test]
fn test_new_hoge() {
    let hoge = NewHoge { age: 0 };
    // dbg!(&hoge);
    assert_eq!(hoge.age, 0);
}

#[test]
fn test_parse() {
    mod kw {
        syn::custom_keyword!(derive);
    }

    let token = quote::quote! {
        (NewHoge, id, derive(Debug, Clone))
    };
    dbg!(&token);

    #[derive(Debug, Clone, PartialEq)]
    struct Opt {
        pub name: Ident,
        pub ignores: Vec<Ident>,
        pub derives: Vec<Ident>,
    }

    impl Parse for Opt {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let content;
            let _: token::Paren = parenthesized!(content in input);
            let name = content.parse()?;
            let _: Token!(,) = content.parse()?;
            let mut ignores = vec![];
            let mut derives = vec![];
            loop {
                if content.is_empty() {
                    break;
                }
                if content.peek(kw::derive) {
                    let derive_content;
                    let _: kw::derive = content.parse()?;
                    let _: token::Paren = parenthesized!(derive_content in content);

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
            Ok(Opt {
                name,
                ignores,
                derives,
            })
        }
    }

    let ret: syn::Result<Opt> = syn::parse2(token);
    assert_eq!(ret.is_ok(), true);
}
// #[test]
// #[should_panic]
// fn test_enum() {
//     #[derive(Omit)]
//     enum Foo {}
// }
