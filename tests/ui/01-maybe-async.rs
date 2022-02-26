#![allow(dead_code)]

use maybe_async::maybe_async;

#[maybe_async(Send)]
trait Trait {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[maybe_async(?Send)]
pub trait PubTrait {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[maybe_async]
pub(crate) trait PubCrateTrait {
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

#[maybe_async]
pub(crate) async fn pub_crate_async_fn() {}

#[maybe_async]
unsafe fn unsafe_fn() {}

struct Struct;

#[maybe_async]
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
async fn main() {
    let s = Struct;
    s.declare_async().await;
    s.async_fn().await;
    async_fn().await;
    pub_async_fn().await;
}
