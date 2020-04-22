//!
//! # Maybe-Async Procedure Macro
//!
//! **Why bother writing similar code twice for blocking and async code?**
//!
//! [![Build Status](https://travis-ci.com/guoli-lyu/maybe-async-rs.svg?token=WSHqSm6F65Fza985QMqn&branch=master)](https://travis-ci.com/guoli-lyu/maybe-async-rs)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![Latest Version](https://img.shields.io/crates/v/maybe-async.svg)](https://crates.io/crates/maybe-async)
//! [![maybe-async](https://docs.rs/maybe-async/badge.svg)](https://docs.rs/maybe-async)
//!
//! When implementing both sync and async versions of API in a crate, most API
//! of the two version are almost the same except for some async/await keyword.
//!
//! `maybe-async` help unifying async and sync implementation.
//! Write async code with normal `async`, `await`, and let `maybe_async` handles
//! those `async` and `await` when you need a synchronized code. Switch between
//! sync and async by toggling `is_sync` feature gate.
//!
//! ## Key features
//!
//! `maybe-async` offers three attribute macros: `maybe_async`, `must_be_sync`
//! and `must_be_async`.
//!
//! These macros can be applied to trait item, trait impl, function and struct
//! impl.
//!
//! - `must_be_async`
//!
//!     add `async_trait` attribute macro for trait declaration or
//! implementation to bring async fn support in traits
//!
//! - `must_be_sync`
//!
//!     convert the async code into sync code by removing all `async move`,
//! `async` and `await` keyword
//!
//! - `maybe_async`
//!
//!     offers a unified feature gate to provide sync and async conversion on
//! demand by feature gate `is_sync`.
//!
//!     `maybe_async` adopts async first policy.
//!
//!     Add `maybe_async` in dependencies with default features means
//! `maybe_async` is the same as `must_be_async`:
//!
//!     ```toml
//!     [dependencies]
//!     maybe_async = "0.1"
//!     ```
//!
//!     When specify a `is_sync` feature gate, `maybe_async` is the same as
//! `must_be_sync`:
//!
//!     ```toml
//!     [dependencies]
//!     maybe_async = { version = "0.1", features = ["is_sync"] }
//!     ```
//!
//! ## Motivation
//!
//!
//! The async/await language feature alters the async world of rust.
//! Comparing with the map/and_then style, now the async code really resembles
//! sync version code.
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

extern crate proc_macro;

use proc_macro::TokenStream;

use darling::FromMeta;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, AttributeArgs, ImplItem, TraitItem};

use quote::quote;

use crate::{parse::Item, visit::AsyncAwaitRemoval};

mod parse;
mod visit;

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
                    if method.sig.asyncness.is_some() {
                        method.sig.asyncness = None;
                    }
                }
            }
            AsyncAwaitRemoval.remove_async_await(quote!(#item))
        }
        Item::Trait(item) => {
            for inner in &mut item.items {
                if let TraitItem::Method(ref mut method) = inner {
                    if method.sig.asyncness.is_some() {
                        method.sig.asyncness = None;
                    }
                }
            }
            AsyncAwaitRemoval.remove_async_await(quote!(#item))
        }
        Item::Fn(item) => {
            if item.sig.asyncness.is_some() {
                item.sig.asyncness = None;
            }
            AsyncAwaitRemoval.remove_async_await(quote!(#item))
        }
    }
    .into()
}

/// maybe_async attribute macro
///
/// Can be applied to trait item, trait impl, functions and struct impls.
#[proc_macro_attribute]
pub fn maybe_async(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as Item);

    let token = if cfg!(feature = "is_sync") {
        convert_sync(&mut item)
    } else {
        convert_async(&mut item)
    };
    token.into()
}

/// convert marked async code to async code with `async-trait`
#[proc_macro_attribute]
pub fn must_be_async(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as Item);
    convert_async(&mut item).into()
}

/// convert marked async code to sync code
#[proc_macro_attribute]
pub fn must_be_sync(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as Item);
    convert_sync(&mut item).into()
}

/// mark sync implementation
///
/// only compiled when `is_sync` feature gate is set.
/// When `is_sync` is not set, marked code is removed.
#[proc_macro_attribute]
pub fn sync_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let token = if cfg!(feature = "is_sync") {
        quote!(#input)
    } else {
        quote!()
    };
    token.into()
}

/// mark async implementation
///
/// only compiled when `is_sync` feature gate is not set.
/// When `is_sync` is set, marked code is removed.
#[proc_macro_attribute]
pub fn async_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let token = if cfg!(feature = "is_sync") {
        quote!()
    } else {
        quote!(#input)
    };
    token.into()
}

#[derive(Debug, FromMeta)]
struct TestArgs {
    #[darling(rename = "async")]
    r#async: String,
    sync: String,
    #[darling(default)]
    test: String,
}

/// Handy macro to unify test code of sync and async code
///
/// Since the API of both sync and async code are the same,
/// with only difference that async functions must be awaited.
/// So it's tedious to write unit sync and async respectively.
///
/// This macro helps unify the sync and async unit test code.
/// There are three options to customize:
/// 1. `sync`: when to treat as sync code
/// 1. `async`: when to treat as async
/// 1. `test`: what lib to run asyync test: `async-std::test`, `tokio::test`
/// or any valid attribute macro. By default, it's set to `async_std::test`.
///
/// **ATTENTION**: do not write await inside a assert macro
///
/// - Examples
///
/// ```rust
/// #[maybe_async]
/// async fn async_fn() -> bool {
///     true
/// }
///
/// #[maybe_async::test(
///     sync = r#"feature="is_sync""#,
///     async = r#"not(feature="is_sync")"#,
///     test = "async_std::test"
/// )]
/// async fn test_async_fn() {
///     let res = async_fn().await;
///     assert_eq!(res, true);
/// }
/// ```
///
/// The above code is transcripted to the following code:
///
/// ```rust
/// # #[maybe_async]
/// # async fn async_fn() -> bool { true }
///
/// #[cfg_attr(
///     any(not(feature = "is_sync")),
///     maybe_async::must_be_async,
///     async_std::test
/// )]
/// #[cfg_attr(any(feature = "is_sync"), maybe_async::must_be_sync, test)]
/// async fn test_async_fn1() {
///     let res = async_fn().await;
///     assert_eq!(res, true);
/// }
/// ```
#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let input = TokenStream2::from(input);

    let args = match TestArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let async_cond: TokenStream2 = args.r#async.parse().unwrap();
    let sync_cond: TokenStream2 = args.sync.parse().unwrap();
    let async_test_macro: TokenStream2 = if args.test.is_empty() {
        quote!(tokio::test)
    } else {
        args.test.parse().unwrap()
    };

    quote!(
        #[cfg_attr(#async_cond, maybe_async::must_be_async, #async_test_macro)]
        #[cfg_attr(#sync_cond, maybe_async::must_be_sync, test)]
        #input
    )
    .into()
}
