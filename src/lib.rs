//!
//! # Maybe-Async Procedure Macro
//!
//! **Why bother writing similar code twice for blocking and async code?**
//!
//! [![Build Status](https://github.com/fMeow/maybe-async-rs/workflows/CI%20%28Linux%29/badge.svg?branch=master)](https://github.com/fMeow/maybe-async-rs/actions)
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
//! sync and async by toggling `is_sync` feature gate. A handy macro to
//! unify unit test code is also provided.
//!
//!
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
//!     **keep async**. Add `async_trait` attribute macro for trait declaration
//!     or implementation to bring async fn support in traits.
//!
//!
//! - `must_be_sync`
//!
//!     **Convert to sync code**. Convert the async code into sync code by
//!     removing all `async move`, `async` and `await` keyword
//!
//! - `maybe_async`
//!
//!     offers a unified feature gate to provide sync and async conversion on
//!     demand by feature gate `is_sync`, with **async first** policy.
//!
//!     Want to keep async code? add `maybe_async` in dependencies with default
//!     features, which means `maybe_async` is the same as `must_be_async`:
//!
//!     ```toml
//!     [dependencies]
//!     maybe_async = "0.1"
//!     ```
//!
//!     Wanna convert async code to sync? Add `maybe_async` to dependencies with
//!     an `is_sync` feature gate. In this way, `maybe_async` is the same as
//!     `must_be_sync`:
//!
//!     ```toml
//!     [dependencies]
//!     maybe_async = { version = "0.1", features = ["is_sync"] }
//!     ```
//!
//! - `sync_impl`
//!
//!     Although most of the API are almost the same, there definitely come to a
//!     point when the async and sync version should differ greatly. For
//! example, a     MongoDB client may use the same API for async and sync
//! verison, but the code     to actually send reqeust are quite different.
//!
//!     Here, we can use `sync_impl` to mark a synchronous implementation, and a
//!     sync implementation shoule disappear when we want async version.
//!
//! - `async_impl`
//!
//!     an async implementation shoule simply disappear when we want sync
//! version.
//!
//! - `test`
//!
//!     handy macro to unify async and sync unit test code
//!
//! ## Motivation
//!
//!
//! The async/await language feature alters the async world of rust.
//! Comparing with the map/and_then style, now the async code really resembles
//! sync version code.
//!
//! In many crates, the async and sync version of crates shares the same API,
//! but the minor difference that all async code must be awaited prevent the
//! unification of async and sync code. In other words, an async and sync
//! implementation must be written repectively.
//!
//! ## Examples
//!
//! ### rust client for services
//!
//! When implementing rust client for any services, like awz3. The higher level
//! API of async and sync version is almost the same, such as creating or
//! deleting a bucket, retrieving an object and etc.
//!
//! Here is a proof of concept that `maybe_async` can actually free us from
//! writing almost the same code for sync and async.
//!
//! ```rust
//! type Response = String;
//! type Url = &'static str;
//! type Method = String;
//!
//! /// InnerClient are used to actually send request,
//! /// which differ a lot between sync and async.
//! #[maybe_async::maybe_async]
//! trait InnerClient {
//!     async fn request(method: Method, url: Url, data: String) -> Response;
//!     #[inline]
//!     async fn post(url: Url, data: String) -> Response {
//!         Self::request(String::from("post"), url, data).await
//!     }
//!     #[inline]
//!     async fn delete(url: Url, data: String) -> Response {
//!         Self::request(String::from("delete"), url, data).await
//!     }
//! }
//!
//! /// The higher level API for end user.
//! pub struct ServiceClient;
//!
//! /// Code of upstream API are almost the same for sync and async,
//! /// except for async/await keyword.
//! impl ServiceClient {
//!     #[maybe_async::maybe_async]
//!     async fn create_bucket(name: String) -> Response {
//!         Self::post("http://correct_url4create", String::from("my_bucket")).await
//!     }
//!     #[maybe_async::maybe_async]
//!     async fn delete_bucket(name: String) -> Response {
//!         Self::delete("http://correct_url4delete", String::from("my_bucket")).await
//!     }
//!     // and another thousands of functions that interact with service side
//! }
//!
//! /// Synchronous implementation, will be deleted when we want an async implementation.
//! /// Else the compiler will complain that *request is defined multiple times* and blabla.
//! #[maybe_async::sync_impl]
//! impl InnerClient for ServiceClient {
//!     fn request(method: Method, url: Url, data: String) -> Response {
//!         // your implementation for sync, like use
//!         // `reqwest::blocking` to send request
//!         String::from("pretend we have a response")
//!     }
//! }
//!
//! /// Asynchronous implementation, will be deleted when we want a sync implementation
//! #[maybe_async::async_impl]
//! impl InnerClient for ServiceClient {
//!     async fn request(method: Method, url: Url, data: String) -> Response {
//!         // your implementation for async, like use `reqwest::client`
//!         // or `async_std` to send request
//!         String::from("pretend we have a response")
//!     }
//! }
//! ```
//!
//! With the code above, we can toggle between a sync AWZ3 client and async one
//! by `is_sync` feature gate when we add `maybe-async` to dependency.
//!
//! ### Example for maybe_async conversion
//!
//! ```rust
//! #[maybe_async::maybe_async]
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
//! #[maybe_async::maybe_async]
//! impl A for Foo {
//!     async fn async_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//!     fn sync_fn_name() -> Result<(), ()> {
//!         Ok(())
//!     }
//! }
//!
//! #[maybe_async::maybe_async]
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
//! # License
//! MIT

extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, ImplItem, Lit, Meta, NestedMeta, TraitItem,
};

use quote::quote;

use crate::{parse::Item, visit::AsyncAwaitRemoval};

mod parse;
mod visit;

fn convert_async(input: &mut Item, send: bool) -> TokenStream2 {
    if send {
        match input {
            Item::Impl(item) => quote!(#[async_trait::async_trait]#item),
            Item::Trait(item) => quote!(#[async_trait::async_trait]#item),
            Item::Fn(item) => quote!(#item),
        }
    } else {
        match input {
            Item::Impl(item) => quote!(#[async_trait::async_trait(?Send)]#item),
            Item::Trait(item) => quote!(#[async_trait::async_trait(?Send)]#item),
            Item::Fn(item) => quote!(#item),
        }
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
pub fn maybe_async(args: TokenStream, input: TokenStream) -> TokenStream {
    let send = match args.to_string().replace(" ", "").as_str() {
        "" | "Send" => true,
        "?Send" => false,
        _ => {
            return syn::Error::new(Span::call_site(), "Only accepts `Send` or `?Send`")
                .to_compile_error()
                .into()
        }
    };

    let mut item = parse_macro_input!(input as Item);

    let token = if cfg!(feature = "is_sync") {
        convert_sync(&mut item)
    } else {
        convert_async(&mut item, send)
    };
    token.into()
}

/// convert marked async code to async code with `async-trait`
#[proc_macro_attribute]
pub fn must_be_async(args: TokenStream, input: TokenStream) -> TokenStream {
    let send = match args.to_string().replace(" ", "").as_str() {
        "" | "Send" => true,
        "?Send" => false,
        _ => {
            return syn::Error::new(Span::call_site(), "Only accepts `Send` or `?Send`")
                .to_compile_error()
                .into()
        }
    };
    let mut item = parse_macro_input!(input as Item);
    convert_async(&mut item, send).into()
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
pub fn async_impl(args: TokenStream, _input: TokenStream) -> TokenStream {
    let send = match args.to_string().replace(" ", "").as_str() {
        "" | "Send" => true,
        "?Send" => false,
        _ => {
            return syn::Error::new(Span::call_site(), "Only accepts `Send` or `?Send`")
                .to_compile_error()
                .into()
        }
    };

    let token = if cfg!(feature = "is_sync") {
        quote!()
    } else {
        let mut item = parse_macro_input!(_input as Item);
        convert_async(&mut item, send)
    };
    token.into()
}

macro_rules! match_nested_meta_to_str_lit {
    ($t:expr) => {
        match $t {
            NestedMeta::Lit(lit) => {
                match lit {
                    Lit::Str(s) => {
                        s.value().parse::<TokenStream2>().unwrap()
                    }
                    _ => {
                        return syn::Error::new(lit.span(), "expected meta or string literal").to_compile_error().into();
                    }
                }
            }
            NestedMeta::Meta(meta) => quote!(#meta)
        }
    };
}

/// Handy macro to unify test code of sync and async code
///
/// Since the API of both sync and async code are the same,
/// with only difference that async functions must be awaited.
/// So it's tedious to write unit sync and async respectively.
///
/// This macro helps unify the sync and async unit test code.
/// Pass the condition to treat test code as sync as the first
/// argument. And specify the condition when to treat test code
/// as async and the lib to run async test, e.x. `async-std::test`,
/// `tokio::test`, or any valid attribute macro.
///
/// **ATTENTION**: do not write await inside a assert macro
///
/// - Examples
///
/// ```rust
/// #[maybe_async::maybe_async]
/// async fn async_fn() -> bool {
///     true
/// }
///
/// #[maybe_async::test(
///     // when to treat the test code as sync version
///     feature="is_sync",
///     // when to run async test
///     async(all(not(feature="is_sync"), feature="async_std"), async_std::test),
///     // you can specify multiple conditions for different async runtime
///     async(all(not(feature="is_sync"), feature="tokio"), tokio::test)
/// )]
/// async fn test_async_fn() {
///     let res = async_fn().await;
///     assert_eq!(res, true);
/// }
///
/// // Only run test in sync version
/// #[maybe_async::test(feature = "is_sync")]
/// async fn test_sync_fn() {
///     let res = async_fn().await;
///     assert_eq!(res, true);
/// }
/// ```
///
/// The above code is transcripted to the following code:
///
/// ```rust
/// # use maybe_async::{must_be_async, must_be_sync, sync_impl};
/// # #[maybe_async::maybe_async]
/// # async fn async_fn() -> bool { true }
///
/// // convert to sync version when sync condition is met, keep in async version when corresponding
/// // condition is met
/// #[cfg_attr(feature = "is_sync", must_be_sync, test)]
/// #[cfg_attr(
///     all(not(feature = "is_sync"), feature = "async_std"),
///     must_be_async,
///     async_std::test
/// )]
/// #[cfg_attr(
///     all(not(feature = "is_sync"), feature = "tokio"),
///     must_be_async,
///     tokio::test
/// )]
/// async fn test_async_fn() {
///     let res = async_fn().await;
///     assert_eq!(res, true);
/// }
///
/// // force converted to sync function, and only compile on sync condition
/// #[cfg(feature = "is_sync")]
/// #[test]
/// fn test_sync_fn() {
///     let res = async_fn();
///     assert_eq!(res, true);
/// }
/// ```
#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let input = TokenStream2::from(input);
    if attr_args.len() < 1 {
        return syn::Error::new(
            Span::call_site(),
            "Must not be empty, at least specify the condition for sync code",
        )
        .to_compile_error()
        .into();
    }

    // The first attributes indicates sync condition
    let sync_cond = match_nested_meta_to_str_lit!(attr_args.first().unwrap());
    let mut ts = if attr_args.len() == 1 {
        quote!(
        #[cfg(#sync_cond)]
        #[cfg_attr(#sync_cond, maybe_async::must_be_sync, test)]
        )
    } else {
        quote!(#[cfg_attr(#sync_cond, maybe_async::must_be_sync, test)])
    };

    // The rest attributes indicates async condition and async test macro
    // only accepts in the forms of `async(cond, test_macro)`, but `cond` and
    // `test_macro` can be either meta attributes or string literal
    let mut async_token = Vec::new();
    for async_meta in attr_args.into_iter().skip(1) {
        match async_meta {
            NestedMeta::Meta(meta) => match meta {
                Meta::List(list) => {
                    let name = list.path.segments[0].ident.to_string();
                    if name.ne("async") {
                        return syn::Error::new(
                            list.path.span(),
                            format!("Unknown path: `{}`, must be `async`", name),
                        )
                        .to_compile_error()
                        .into();
                    }
                    if list.nested.len() == 2 {
                        let async_cond =
                            match_nested_meta_to_str_lit!(list.nested.first().unwrap());
                        let async_test = match_nested_meta_to_str_lit!(list.nested.last().unwrap());
                        let attr = quote!( #[cfg_attr(#async_cond, maybe_async::must_be_async, #async_test)] );
                        async_token.push(attr);
                    } else {
                        let msg = format!(
                            "Must pass two metas or string literals like `async(condition, \
                             async_test_macro)`, you passed {} metas.",
                            list.nested.len()
                        );
                        return syn::Error::new(list.span(), msg).to_compile_error().into();
                    }
                }
                _ => {
                    return syn::Error::new(
                        meta.span(),
                        "Must be list of metas like: `async(condition, async_test_macro)`",
                    )
                    .to_compile_error()
                    .into();
                }
            },
            NestedMeta::Lit(lit) => {
                return syn::Error::new(
                    lit.span(),
                    "Must be list of metas like: `async(condition, async_test_macro)`",
                )
                .to_compile_error()
                .into();
            }
        };
    }

    async_token.into_iter().for_each(|t| ts.extend(t));
    ts.extend(quote!( #input ));
    ts.into()
}
