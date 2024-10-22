use std::iter::Filter;

use proc_macro::Ident;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use quote::format_ident;
use quote::quote;

#[proc_macro_attribute]
pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr:?}\"");
    println!("item: \"{item}\"");
    let mut trees = item.into_iter().collect::<Vec<TokenTree>>();
    let TokenTree::Ident(ref mut fn_name) = trees[1] else {
        panic!()
    };
    *fn_name = Ident::new(&format!("aa{fn_name}"), Span::call_site());
    println!("{:#?}", trees);
    println!("{trees:?}");

    trees.into_iter().collect::<TokenStream>()
}

#[proc_macro_attribute]
pub fn netfunc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut trees = item.into_iter().collect::<Vec<TokenTree>>();
    let mut iterator = trees.iter_mut().skip_while(|x| {
        if let TokenTree::Ident(x) = x {
            x.to_string() != "fn"
        } else {
            false
        }
    });

    let Some(TokenTree::Ident(ref mut fn_name)) = iterator.nth(1) else {
        panic!("can't find the function name")
    };

    let Some(TokenTree::Group(ref mut fn_args)) = iterator.nth(0) else {
        panic!("can't find the function args")
    };

    let a = iterator.map(|x| println!("{:#?}", x)).skip_while(|x| j);

    println!("{}", fn_name);
    println!("{}", fn_args);
    let argidents: Vec<TokenTree> = fn_args
        .stream()
        .into_iter()
        .filter(|x| match x {
            TokenTree::Ident(_) => true,
            _ => false,
        })
        .collect();
    let mut argtypes: Vec<TokenTree> = vec![];
    let mut enumtypes: Vec<TokenTree> = vec![];
    for a in 0..argidents.len() {
        if a % 2 != 0 {
            argtypes.push(argidents[a].clone());
            enumtypes.push(match argidents[a].to_string().as_str() {
                "String" => TokenTree::Ident(Ident::new("STRING", Span::call_site())),
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
                _ => panic!("Non convertable argument used: {}", a),
            });
            println!("{:#?}", enumtypes);
        }
    }

    let netfnname = format_ident!("{}____net____", fn_name.to_string());

    let netfunction = quote! {
        fn #netfnname(&[Type]) -> Result<_, DataParseError> {

        }
    };

    let final_tree: Vec<TokenStream> = vec![];

    trees.into_iter().collect::<TokenStream>()
}
