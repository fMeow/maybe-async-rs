# Maybe-Async Procedure Macro

[![Build Status](https://travis-ci.com/guoli-lyu/maybe-async-rs.svg?token=WSHqSm6F65Fza985QMqn&branch=master)](https://travis-ci.com/guoli-lyu/maybe-async-rs)

Unifying async and sync library implementation.

Users write async code with normal `async`, `await`, but `maybe_async` delete those `async` and `await`.


<a id="org3483821"></a>

## Examples

```rust
#[maybe_async]
trait A{
    async fn async_fn_name()->Result<(),()> {}
    fn sync_fn_name() ->Result<(),()>{}
}

struct Foo;

#[maybe_async]
impl A for Foo{
    async fn async_fn_name()->Result<(),()>{
    }
    fn sync_fn_name() ->Result<(),()>{}
}

#[maybe_async]
async fn maybe_async_fn()->Result<(),()>{
    #[maybe_async(async_fn_name)]
    let a = Foo::async_fn_name()?;

    let b = Foo::sync_fn_name()?;
    Ok(())
}
```

When `maybe-async` feature gate `is_sync` is **NOT** set, the generated code is async code:

```rust
#[async_trait]
trait A{
    async fn maybe_async_fn_name() ->Result<(),()>{}
    fn sync_fn_name() ->Result<(),()>{}
}

struct Foo;

#[async_trait]
impl A for Foo{
    async fn maybe_async_fn_name() ->Result<(),()>{
    }
    fn sync_fn_name() ->Result<(),()>{}
}

async fn maybe_async_fn()->Result<(),()>{
    let a = Foo::maybe_async_fn_name().await?;
    let b = Foo::sync_fn_name()?;
    Ok(())
}
```

When `maybe-async` feature gate `is_sync` is set, all async keyword is ignored and yields a sync version code:

```rust
trait A {
    fn maybe_async_fn_name() ->Result<(),()>{}
    fn sync_fn_name()->Result<(),()> {}
}

struct Foo;

impl A for Foo {
   fn maybe_async_fn_name()->Result<(),()>{}
   fn sync_fn_name() ->Result<(),()>{}
}

fn maybe_async_fn()->Result<(),()>{
    let a = Foo::maybe_async_fn_name()?;
    let b = Foo::sync_fn_name()?;
    Ok(())
}
```


<a id="org6fd57c9"></a>

## Idea

Can be applied to trait item, trait impl, function and struct impl.

-   Async
    
    if trait declaration or implementation, just add `async_trait` attribute

-   Sync
    
    remove all `async` and `await` keyword
