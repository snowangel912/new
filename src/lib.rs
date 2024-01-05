#![no_std]
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod structs;

#[proc_macro]
pub fn structs_new(input: TokenStream) -> TokenStream {
    use structs::NewItemStruct;
    let new_structs = parse_macro_input!(input with NewItemStruct::parse_multi);
    let expand = new_structs.into_iter()
                            .map(NewItemStruct::split)
                            .map(|(item_struct, item_impl)| quote! {#item_struct #item_impl});
    TokenStream::from(quote! { #(#expand)* })
}