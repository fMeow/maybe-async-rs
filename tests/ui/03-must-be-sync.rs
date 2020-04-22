#![allow(dead_code)]

use maybe_async::{maybe_async, must_be_sync};

#[maybe_async::maybe_async]
trait Trait {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[maybe_async::maybe_async]
pub trait PubTrait {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[maybe_async::maybe_async]
async fn async_fn() {}

#[maybe_async::maybe_async]
pub async fn pub_async_fn() {}

struct Struct;

#[cfg(feature = "is_sync")]
#[maybe_async::must_be_sync]
impl Trait for Struct {
    fn sync_fn() {}

    async fn declare_async(&self) {}

    async fn async_fn(&self) {
        async { self.declare_async().await }.await
    }
}

#[cfg(feature = "is_sync")]
fn main() -> std::result::Result<(), ()> {
    let s = Struct;
    s.declare_async();
    s.async_fn();
    async_fn();
    pub_async_fn();
    Ok(())
}


#[cfg(not(feature = "is_sync"))]
#[async_std::main]
async fn main() {}
