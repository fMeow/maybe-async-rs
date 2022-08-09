<!-- cargo-sync-readme start -->


# Maybe-Async Procedure Macro

**Why bother writing similar code twice for blocking and async code?**

[![Build Status](https://github.com/fMeow/maybe-async-rs/workflows/CI%20%28Linux%29/badge.svg?branch=master)](https://github.com/fMeow/maybe-async-rs/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Latest Version](https://img.shields.io/crates/v/maybe-async.svg)](https://crates.io/crates/maybe-async)
[![maybe-async](https://docs.rs/maybe-async/badge.svg)](https://docs.rs/maybe-async)

When implementing both sync and async versions of API in a crate, most API
of the two version are almost the same except for some async/await keyword.

`maybe-async` help unifying async and sync implementation by **procedural
macro**.
- Write async code with normal `async`, `await`, and let `maybe_async`
  handles
those `async` and `await` when you need a blocking code.
- Switch between sync and async by toggling `is_sync` feature gate in
  `Cargo.toml`.
- use `must_be_async` and `must_be_sync` to keep code in specified version
- use `impl_async` and `impl_sync` to only compile code block on specified
  version
- A handy macro to unify unit test code is also provided.

These procedural macros can be applied to the following codes:
- trait item declaration
- trait implmentation
- function definition
- struct definition

**RECOMMENDATION**: Enable **resolver ver2** in your crate, which is
introduced in Rust 1.51. If not, two crates in dependency with conflict
version (one async and another blocking) can fail complilation.


## Motivation

The async/await language feature alters the async world of rust.
Comparing with the map/and_then style, now the async code really resembles
sync version code.

In many crates, the async and sync version of crates shares the same API,
but the minor difference that all async code must be awaited prevent the
unification of async and sync code. In other words, we are forced to write
an async and an sync implementation repectively.

## Macros in Detail

`maybe-async` offers 4 set of attribute macros: `maybe_async`,
`sync_impl`/`async_impl`, `must_be_sync`/`must_be_async`,  and `test`.

To use `maybe-async`, we must know which block of codes is only used on
blocking implementation, and which on async. These two implementation should
share the same function signatures except for async/await keywords, and use
`sync_impl` and `async_impl` to mark these implementation.

Use `maybe_async` macro on codes that share the same API on both async and
blocking code except for async/await keywords. And use feature gate
`is_sync` in `Cargo.toml` to toggle between async and blocking code.

- `maybe_async`

    Offers a unified feature gate to provide sync and async conversion on
    demand by feature gate `is_sync`, with **async first** policy.

    Want to keep async code? add `maybe_async` in dependencies with default
    features, which means `maybe_async` is the same as `must_be_async`:

    ```toml
    [dependencies]
    maybe_async = "0.2"
    ```

    Wanna convert async code to sync? Add `maybe_async` to dependencies with
    an `is_sync` feature gate. In this way, `maybe_async` is the same as
    `must_be_sync`:

    ```toml
    [dependencies]
    maybe_async = { version = "0.2", features = ["is_sync"] }
    ```

    Not all async traits need futures that are `dyn Future + Send`.
    To avoid having "Send" and "Sync" bounds placed on the async trait
    methods, invoke the maybe_async macro as `#[maybe_async(?Send)]` on both
    the trait and the impl blocks.


- `must_be_async`

    **Keep async**. Add `async_trait` attribute macro for trait declaration
    or implementation to bring async fn support in traits.

    To avoid having "Send" and "Sync" bounds placed on the async trait
    methods, invoke the maybe_async macro as `#[must_be_async(?Send)]`.

- `must_be_sync`

    **Convert to sync code**. Convert the async code into sync code by
    removing all `async move`, `async` and `await` keyword


- `sync_impl`

    An sync implementation should on compile on blocking implementation and
must     simply disappear when we want async version.

    Although most of the API are almost the same, there definitely come to a
    point when the async and sync version should differ greatly. For
    example, a MongoDB client may use the same API for async and sync
    verison, but the code to actually send reqeust are quite different.

    Here, we can use `sync_impl` to mark a synchronous implementation, and a
    sync implementation shoule disappear when we want async version.

- `async_impl`

    An async implementation should on compile on async implementation and
must     simply disappear when we want sync version.

    To avoid having "Send" and "Sync" bounds placed on the async trait
    methods, invoke the maybe_async macro as `#[async_impl(?Send)]`.


- `test`

    Handy macro to unify async and sync **unit and e2e test** code.

    You can specify the condition to compile to sync test code
    and also the conditions to compile to async test code with given test
    macro, e.x. `tokio::test`, `async_std::test` and etc. When only sync
    condition is specified,the test code only compiles when sync condition
    is met.

    ```rust
    # #[maybe_async::maybe_async]
    # async fn async_fn() -> bool {
    #    true
    # }

    #[maybe_async::test(
        feature="is_sync",
        async(all(not(feature="is_sync"), feature="async_std"), async_std::test),
        async(all(not(feature="is_sync"), feature="tokio"), tokio::test)
    )]
    async fn test_async_fn() {
        let res = async_fn().await;
        assert_eq!(res, true);
    }
    ```

## What's Under the Hook

`maybe-async` compiles your code in different way with the `is_sync` feature
gate. It removes all `await` and `async` keywords in your code under
`maybe_async` macro and conditionally compiles codes under `async_impl` and
`sync_impl`.

Here is an detailed example on what's going on whe the `is_sync` feature
gate set or not.

```rust
#[maybe_async::maybe_async(?Send)]
trait A {
    async fn async_fn_name() -> Result<(), ()> {
        Ok(())
    }
    fn sync_fn_name() -> Result<(), ()> {
        Ok(())
    }
}

struct Foo;

#[maybe_async::maybe_async(?Send)]
impl A for Foo {
    async fn async_fn_name() -> Result<(), ()> {
        Ok(())
    }
    fn sync_fn_name() -> Result<(), ()> {
        Ok(())
    }
}

#[maybe_async::maybe_async]
async fn maybe_async_fn() -> Result<(), ()> {
    let a = Foo::async_fn_name().await?;

    let b = Foo::sync_fn_name()?;
    Ok(())
}
```

When `maybe-async` feature gate `is_sync` is **NOT** set, the generated code
is async code:

```rust
// Compiled code when `is_sync` is toggled off.
#[async_trait::async_trait(?Send)]
trait A {
    async fn maybe_async_fn_name() -> Result<(), ()> {
        Ok(())
    }
    fn sync_fn_name() -> Result<(), ()> {
        Ok(())
    }
}

struct Foo;

#[async_trait::async_trait(?Send)]
impl A for Foo {
    async fn maybe_async_fn_name() -> Result<(), ()> {
        Ok(())
    }
    fn sync_fn_name() -> Result<(), ()> {
        Ok(())
    }
}

async fn maybe_async_fn() -> Result<(), ()> {
    let a = Foo::maybe_async_fn_name().await?;
    let b = Foo::sync_fn_name()?;
    Ok(())
}
```

When `maybe-async` feature gate `is_sync` is set, all async keyword is
ignored and yields a sync version code:

```rust
// Compiled code when `is_sync` is toggled on.
trait A {
    fn maybe_async_fn_name() -> Result<(), ()> {
        Ok(())
    }
    fn sync_fn_name() -> Result<(), ()> {
        Ok(())
    }
}

struct Foo;

impl A for Foo {
    fn maybe_async_fn_name() -> Result<(), ()> {
        Ok(())
    }
    fn sync_fn_name() -> Result<(), ()> {
        Ok(())
    }
}

fn maybe_async_fn() -> Result<(), ()> {
    let a = Foo::maybe_async_fn_name()?;
    let b = Foo::sync_fn_name()?;
    Ok(())
}
```

## Examples

### rust client for services

When implementing rust client for any services, like awz3. The higher level
API of async and sync version is almost the same, such as creating or
deleting a bucket, retrieving an object and etc.

The example `service_client` is a proof of concept that `maybe_async` can
actually free us from writing almost the same code for sync and async. We
can toggle between a sync AWZ3 client and async one by `is_sync` feature
gate when we add `maybe-async` to dependency.


# License
MIT

<!-- cargo-sync-readme end -->
