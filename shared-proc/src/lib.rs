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
    println!("{:#?}", trees[1]);
    trees.into_iter().collect::<TokenStream>()
}

