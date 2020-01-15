#![allow(dead_code)]

use maybe_async::maybe_async;

#[maybe_async]
trait Trait {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[maybe_async]
pub trait PubTrait {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[maybe_async]
async fn async_fn() {}

#[maybe_async]
pub async fn pub_async_fn() {}

struct Struct;

#[maybe_async]
impl Trait for Struct {
    fn sync_fn() {}

    async fn declare_async(&self) {}

    async fn async_fn(&self) {
        async { self.declare_async().await }.await
    }
}

fn main() -> Result<(), ()> {
    let s = Struct;
    if cfg!(is_sync) {
        s.declare_async();
        s.async_fn();
    }
    Ok(())
}
