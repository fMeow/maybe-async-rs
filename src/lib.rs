extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, visit_mut::VisitMut, File, ImplItem, TraitItem};

mod parse;
mod visit;

use crate::parse::Item;
use crate::visit::AsyncAwaitRemoval;

fn convert_async(input: &mut Item) -> TokenStream2 {
    match input {
        Item::Impl(item) => quote!(#[async_trait::async_trait]#item),
        Item::Trait(item) => quote!(#[async_trait::async_trait]#item),
        Item::Fn(item) => quote!(#item),
    }
    .into()
}

fn convert_sync(input: &mut Item) -> TokenStream2 {
    match input {
        Item::Impl(item) => {
            for inner in &mut item.items {
                if let ImplItem::Method(ref mut method) = inner {
                    let sig = &mut method.sig;
                    if sig.asyncness.is_some() {
                        sig.asyncness = None;
                    }
                }
            }
            let mut syntax_tree: File = syn::parse(quote!(#item).into()).unwrap();
            AsyncAwaitRemoval.visit_file_mut(&mut syntax_tree);
            quote!(#syntax_tree)
        }
        Item::Trait(item) => {
            for inner in &mut item.items {
                if let TraitItem::Method(ref mut method) = inner {
                    let sig = &mut method.sig;
                    if sig.asyncness.is_some() {
                        sig.asyncness = None;
                    }
                }
            }
            let mut syntax_tree: File = syn::parse(quote!(#item).into()).unwrap();
            AsyncAwaitRemoval.visit_file_mut(&mut syntax_tree);
            quote!(#syntax_tree)
        }
        Item::Fn(item) => quote!(#item),
    }
    .into()
}

/// maybe_async attribute macro
///
/// Can be applied to trait item, trait impl, functions and struct impls.
#[proc_macro_attribute]
pub fn maybe_async(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as Item);

    let token = if cfg!(is_sync) {
        convert_sync(&mut item)
    } else {
        convert_async(&mut item)
    };
    token.into()
}

#[proc_macro_attribute]
pub fn must_be_async(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as Item);
    convert_async(&mut item).into()
}

#[proc_macro_attribute]
pub fn must_be_sync(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as Item);
    convert_sync(&mut item).into()
}
