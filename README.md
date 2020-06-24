<!-- cargo-sync-readme start -->


# Maybe-Async Procedure Macro

**Why bother writing similar code twice for blocking and async code?**

[![Build Status](https://github.com/fMeow/maybe-async-rs/workflows/CI%20%28Linux%29/badge.svg?branch=master)](https://github.com/fMeow/maybe-async-rs/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Latest Version](https://img.shields.io/crates/v/maybe-async.svg)](https://crates.io/crates/maybe-async)
[![maybe-async](https://docs.rs/maybe-async/badge.svg)](https://docs.rs/maybe-async)

When implementing both sync and async versions of API in a crate, most API
of the two version are almost the same except for some async/await keyword.

`maybe-async` help unifying async and sync implementation.
Write async code with normal `async`, `await`, and let `maybe_async` handles
those `async` and `await` when you need a synchronized code. Switch between
sync and async by toggling `is_sync` feature gate. A handy macro to
unify unit test code is also provided.



## Key features

`maybe-async` offers three attribute macros: `maybe_async`, `must_be_sync`
and `must_be_async`.

These macros can be applied to trait item, trait impl, function and struct
impl.

- `must_be_async`

    **keep async**. Add `async_trait` attribute macro for trait declaration
    or implementation to bring async fn support in traits.


- `must_be_sync`

    **Convert to sync code**. Convert the async code into sync code by
    removing all `async move`, `async` and `await` keyword

- `maybe_async`

    offers a unified feature gate to provide sync and async conversion on
    demand by feature gate `is_sync`, with **async first** policy.

    Want to keep async code? add `maybe_async` in dependencies with default
    features, which means `maybe_async` is the same as `must_be_async`:

    ```toml
    [dependencies]
    maybe_async = "0.1"
    ```

    Wanna convert async code to sync? Add `maybe_async` to dependencies with
    an `is_sync` feature gate. In this way, `maybe_async` is the same as
    `must_be_sync`:

    ```toml
    [dependencies]
    maybe_async = { version = "0.1", features = ["is_sync"] }
    ```

- `sync_impl`

    Although most of the API are almost the same, there definitely come to a
    point when the async and sync version should differ greatly. For
example, a     MongoDB client may use the same API for async and sync
verison, but the code     to actually send reqeust are quite different.

    Here, we can use `sync_impl` to mark a synchronous implementation, and a
    sync implementation shoule disappear when we want async version.

- `async_impl`

    an async implementation shoule simply disappear when we want sync
version.

- `test`

    handy macro to unify async and sync unit test code

## Motivation


The async/await language feature alters the async world of rust.
Comparing with the map/and_then style, now the async code really resembles
sync version code.

In many crates, the async and sync version of crates shares the same API,
but the minor difference that all async code must be awaited prevent the
unification of async and sync code. In other words, an async and sync
implementation must be written repectively.

## Examples

### rust client for services

When implementing rust client for any services, like awz3. The higher level
API of async and sync version is almost the same, such as creating or
deleting a bucket, retrieving an object and etc.

Here is a proof of concept that `maybe_async` can actually free us from
writing almost the same code for sync and async.

```rust
type Response = String;
type Url = &'static str;
type Method = String;

/// InnerClient are used to actually send request,
/// which differ a lot between sync and async.
#[maybe_async::maybe_async]
trait InnerClient {
    async fn request(method: Method, url: Url, data: String) -> Response;
    #[inline]
    async fn post(url: Url, data: String) -> Response {
        Self::request(String::from("post"), url, data).await
    }
    #[inline]
    async fn delete(url: Url, data: String) -> Response {
        Self::request(String::from("delete"), url, data).await
    }
}

/// The higher level API for end user.
pub struct ServiceClient;

/// Code of upstream API are almost the same for sync and async,
/// except for async/await keyword.
impl ServiceClient {
    #[maybe_async::maybe_async]
    async fn create_bucket(name: String) -> Response {
        Self::post("http://correct_url4create", String::from("my_bucket")).await
    }
    #[maybe_async::maybe_async]
    async fn delete_bucket(name: String) -> Response {
        Self::delete("http://correct_url4delete", String::from("my_bucket")).await
    }
    // and another thousands of functions that interact with service side
}

/// Synchronous implementation, will be deleted when we want an async implementation.
/// Else the compiler will complain that *request is defined multiple times* and blabla.
#[maybe_async::sync_impl]
impl InnerClient for ServiceClient {
    fn request(method: Method, url: Url, data: String) -> Response {
        // your implementation for sync, like use
        // `reqwest::blocking` to send request
        String::from("pretent we have a response")
    }
}

/// Asynchronous implementation, will be deleted when we want a sync implementation
#[maybe_async::async_impl]
impl InnerClient for ServiceClient {
    async fn request(method: Method, url: Url, data: String) -> Response {
        // your implementation for async, like use `reqwest::client`
        // or `async_std` to send request
        String::from("pretent we have a response")
    }
}
```

With the code above, we can toggle between a sync AWZ3 client and async one
by `is_sync` feature gate when we add `maybe-async` to dependency.

### Example for maybe_async conversion

```rust
#[maybe_async::maybe_async]
trait A {
    async fn async_fn_name() -> Result<(), ()> {
        Ok(())
    }
    fn sync_fn_name() -> Result<(), ()> {
        Ok(())
    }
}

struct Foo;

#[maybe_async::maybe_async]
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
#[async_trait]
trait A {
    async fn maybe_async_fn_name() -> Result<(), ()> {
        Ok(())
    }
    fn sync_fn_name() -> Result<(), ()> {
        Ok(())
    }
}

struct Foo;

#[async_trait]
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

# License
MIT

<!-- cargo-sync-readme end -->
