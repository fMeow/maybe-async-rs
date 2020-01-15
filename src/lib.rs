//!
//! # Maybe-Async Procedure Macro
//!
//! [![Build Status](https://travis-ci.com/guoli-lyu/maybe-async-rs.svg?token=WSHqSm6F65Fza985QMqn&branch=master)](https://travis-ci.com/guoli-lyu/maybe-async-rs)
//!
//! Unifying async and sync library implementation.
//!
//! Users write async code with normal `async`, `await`, but `maybe_async`
//! delete those `async` and `await`.
//!
//!
//! ## Examples
//!
//! ```rust
//! # use maybe_async::maybe_async;
//! #[maybe_async]
//! trait A {
//!     async fn async_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//!     fn sync_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//! }
//!
//! struct Foo;
//!
//! #[maybe_async]
//! impl A for Foo {
//!     async fn async_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//!     fn sync_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//! }
//!
//! #[maybe_async]
//! async fn maybe_async_fn() -> Result<(), ()> {
//!     let a = Foo::async_fn_name().await?;
//!
//!     let b = Foo::sync_fn_name()?;
//!     Ok(())
//! }
//! ```
//!
//! When `maybe-async` feature gate `is_sync` is **NOT** set, the generated code
//! is async code:
//!
//! ```rust
//! # use async_trait::async_trait;
//! #[async_trait]
//! trait A {
//!     async fn maybe_async_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//!     fn sync_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//! }
//!
//! struct Foo;
//!
//! #[async_trait]
//! impl A for Foo {
//!     async fn maybe_async_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//!     fn sync_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//! }
//!
//! async fn maybe_async_fn() -> Result<(), ()> {
//!     let a = Foo::maybe_async_fn_name().await?;
//!     let b = Foo::sync_fn_name()?;
//!     Ok(())
//! }
//! ```
//!
//! When `maybe-async` feature gate `is_sync` is set, all async keyword is
//! ignored and yields a sync version code:
//!
//! ```rust
//! trait A {
//!     fn maybe_async_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//!     fn sync_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//! }
//!
//! struct Foo;
//!
//! impl A for Foo {
//!     fn maybe_async_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//!     fn sync_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//! }
//!
//! fn maybe_async_fn() -> Result<(), ()> {
//!     let a = Foo::maybe_async_fn_name()?;
//!     let b = Foo::sync_fn_name()?;
//!     Ok(())
//! }
//! ```
//!
//! ## Idea
//!
//! Can be applied to trait item, trait impl, function and struct impl.
//!
//! - Async
//!
//!     if trait declaration or implementation, just add `async_trait` attribute
//!
//! - Sync
//!
//!     remove all `async` and `await` keyword

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, visit_mut::VisitMut, File, ImplItem, TraitItem};

mod parse;
mod visit;

use crate::{parse::Item, visit::AsyncAwaitRemoval};

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

#[proc_macro_attribute]
pub fn sync_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let token = if cfg!(is_sync) {
        quote!(#input)
    } else {
        quote!()
    };
    token.into()
}

#[proc_macro_attribute]
pub fn async_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let token = if cfg!(is_sync) {
        quote!()
    } else {
        quote!(#input)
    };
    token.into()
}
