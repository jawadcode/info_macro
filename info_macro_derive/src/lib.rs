extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Data, DeriveInput, Visibility};

fn impl_info_macro(ast: &DeriveInput) -> TokenStream {
    // Extract info about item from ast
    let name = &ast.ident;
    let vis = match &ast.vis {
        Visibility::Public(_) => "public",
        _ => "private",
    };
    let typ = match &ast.data {
        Data::Enum(_) => "enum",
        Data::Struct(_) => "struct",
        Data::Union(_) => "union",
    };
    // Generate impl block for the item using all the info about it extracted above
    let gen = quote! {
        impl InfoMacro for #name {
            fn info(&self) {
                println!("I'm an instance of a {} {} called {}", #vis, #typ, stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(InfoMacro)]
pub fn info_macro_derive(input: TokenStream) -> TokenStream {
    // Construct Abstract Syntax Tree from source code
    let ast = parse(input).unwrap();
    // Take in ast for definition and get back implementation of `InfoMacro` (in tokens)
    impl_info_macro(&ast)
}
