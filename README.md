<!-- cargo-sync-readme start -->


# Maybe-Async Procedure Macro

[![Build Status](https://travis-ci.com/guoli-lyu/maybe-async-rs.svg?token=WSHqSm6F65Fza985QMqn&branch=master)](https://travis-ci.com/guoli-lyu/maybe-async-rs)
[![Latest Version](https://img.shields.io/crates/v/maybe-async.svg)](https://crates.io/crates/maybe-async)

When implementing both sync and async versions of API in a crate, most API
of the two version are almost the same except for some async/await keyword.

`maybe-async` help unifying async and sync implementation.
Write async code with normal `async`, `await`, and let `maybe_async` handles
those `async` and `await` when you need a synchronized code. Switch between
sync and async by toggling `is_sync` feature gate.

## Key features

`maybe-async` offers three attribute macros: `maybe_async`, `must_be_sync`
and `must_be_async`.

These macros can be applied to trait item, trait impl, function and struct
impl.

- `must_be_async`

    add `async_trait` attribute macro for trait declaration or
implementation to bring async fn support in traits

- `must_be_sync`

    convert the async code into sync code by removing all `async move`,
`async` and `await` keyword

- `maybe_async`

    offers a unified feature gate to provide sync and async conversion on
demand by feature gate `is_sync`.

    `maybe_async` adopts async first policy.

    Add `maybe_async` in dependencies with default features means `maybe_async` is the same as `must_be_async`:

    ```toml
    [dependencies]
    maybe_async = "0.1"
    ```

    When specify a `is_sync` feature gate, `maybe_async` is the same as `must_be_sync`:

    ```toml
    [dependencies]
    maybe_async = { version = "0.1", features = ["is_sync"] }
    ```

## Motivation


The async/await language feature alters the async world of rust.
Comparing with the map/and_then style, now the async code really resembles
sync version code.

## Examples

```rust
#[maybe_async]
trait A {
    async fn async_fn_name() -> Result<(), ()> {
        Ok(())
    }
    fn sync_fn_name() -> Result<(), ()> {
        Ok(())
    }
}

struct Foo;

#[maybe_async]
impl A for Foo {
    async fn async_fn_name() -> Result<(), ()> {
        Ok(())
    }
    fn sync_fn_name() -> Result<(), ()> {
        Ok(())
    }
}

#[maybe_async]
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

<!-- cargo-sync-readme end -->
