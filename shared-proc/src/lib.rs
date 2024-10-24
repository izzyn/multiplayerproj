use std::vec;

use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::Ident;
use proc_macro2::Punct;
use proc_macro2::Spacing;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::format_ident;
use quote::quote;

#[proc_macro_attribute]
pub fn expand(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    println!("attr: \"{attr:?}\"");
    println!("item: \"{item}\"");
    let mut trees: Vec<TokenTree> = TokenStream::from(item)
        .into_iter()
        .collect::<Vec<TokenTree>>();
    let TokenTree::Ident(ref mut fn_name) = trees[1] else {
        panic!()
    };
    *fn_name = Ident::new(&format!("aa{fn_name}"), Span::call_site());
    println!("{:#?}", trees);
    println!("{trees:?}");

    trees.into_iter().collect::<TokenStream>().into()
}

//Enables the function to be used via signals using the connect! macro.
//The function can only use input types of any integer size (unsigned or signed), chars, floats and
//&str
#[proc_macro_attribute]
pub fn netfunc(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut trees: Vec<TokenTree> = TokenStream::from(item)
        .into_iter()
        .collect::<Vec<TokenTree>>();
    let mut iterator = trees.iter_mut().skip_while(|x| {
        if let TokenTree::Ident(x) = x {
            *x != "fn"
        } else {
            false
        }
    });

    let Some(TokenTree::Ident(ref mut fn_name)) = iterator.nth(1) else {
        panic!("can't find the function name")
    };

    let Some(TokenTree::Group(ref mut fn_args)) = iterator.next() else {
        panic!("can't find the function args")
    };

    let mut argtypes: Vec<TokenTree> = vec![];
    let mut enumtypes: Vec<TokenTree> = vec![];

    let mut identnr = 0;
    let argtokens: Vec<TokenTree> = fn_args.stream().into_iter().collect();
    for a in 0..argtokens.len() {
        if matches!(argtokens[a], TokenTree::Ident(_)) {
            if identnr % 2 != 0 {
                argtypes.push(argtokens[a].clone());
                enumtypes.push(match argtokens[a].to_string().as_str() {
                    "str" => {
                        if a != 0 {
                            if argtokens[a - 1].to_string() != "&" {
                                panic!(
                                    "Non convertable argument used: {}, did you mean to use a &str",
                                    argtokens[a]
                                )
                            }
                            TokenTree::Ident(Ident::new("STRING", Span::call_site()))
                        } else {
                            panic!("Non convertable argument used: {}", argtokens[a])
                        }
                    }
                    "u8" => TokenTree::Ident(Ident::new("U8", Span::call_site())),
                    "u16" => TokenTree::Ident(Ident::new("U16", Span::call_site())),
                    "u32" => TokenTree::Ident(Ident::new("U32", Span::call_site())),
                    "u64" => TokenTree::Ident(Ident::new("U64", Span::call_site())),
                    "i8" => TokenTree::Ident(Ident::new("I8", Span::call_site())),
                    "i16" => TokenTree::Ident(Ident::new("I16", Span::call_site())),
                    "i32" => TokenTree::Ident(Ident::new("I32", Span::call_site())),
                    "i64" => TokenTree::Ident(Ident::new("I64", Span::call_site())),
                    "char" => TokenTree::Ident(Ident::new("CHAR", Span::call_site())),
                    "f32" => TokenTree::Ident(Ident::new("F32", Span::call_site())),
                    "f64" => TokenTree::Ident(Ident::new("F64", Span::call_site())),
                    _ => panic!("Non convertable argument used: {}", argtokens[a]),
                });
            }
            identnr += 1;
        }
    }

    let mut listtypes: Vec<TokenTree> = vec![];
    let mut varnamelist: Vec<TokenTree> = vec![];

    for i in 0..enumtypes.len() {
        if i != 0 {
            let comma = TokenTree::Punct(Punct::new(',', Spacing::Alone));
            listtypes.push(comma.clone());
            varnamelist.push(comma.clone());
        }
        listtypes.push(TokenTree::Ident(Ident::new(
            "ParsedData",
            Span::call_site(),
        )));
        listtypes.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
        listtypes.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
        listtypes.push(enumtypes[i].clone());

        let varname = format_ident!("t{}", i);
        listtypes.push(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            vec![TokenTree::Ident(varname.clone())]
                .into_iter()
                .collect::<TokenStream>(),
        )));
        if enumtypes[i].to_string() == "STRING" {
            varnamelist.push(TokenTree::Punct(Punct::new('&', Spacing::Alone)));
        } else {
            varnamelist.push(TokenTree::Punct(Punct::new('*', Spacing::Alone)));
        }
        varnamelist.push(varname.into());
    }

    let netfnname = format_ident!("{}____net____", fn_name.to_string());
    let netfunction = quote! {
        use helper::ParsedData;
        use shared::data::DataParseError;
        fn #netfnname(__data: &[ParsedData]) -> Result<(), DataParseError> {
            if let [#(#listtypes)*] = __data {
                let _ = #fn_name(#(#varnamelist)*);
                return Ok(())
            }
            else{
                return Err(DataParseError { message : "Functions had incorrect type signature".to_string() })
            }
        }
    };

    let functiontree = netfunction.into_iter().collect::<Vec<TokenTree>>();
    trees = [trees, functiontree].concat();
    trees.into_iter().collect::<TokenStream>().into()
}
