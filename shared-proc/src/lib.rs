use proc_macro::Ident;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

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
    println!("{}", fn_name);
    println!("{}", fn_args);
    trees.into_iter().collect::<TokenStream>()
}
