#![allow(dead_code)]

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


#[cfg(not(feature = "is_sync"))]
#[maybe_async::must_be_async]
async fn async_fn() {}


#[cfg(not(feature = "is_sync"))]
#[maybe_async::must_be_async]
pub async fn pub_async_fn() {}

struct Struct;


#[cfg(not(feature = "is_sync"))]
#[maybe_async::must_be_async]
impl Trait for Struct {
    fn sync_fn() {}

    async fn declare_async(&self) {}

    async fn async_fn(&self) {
        async { self.declare_async().await }.await
    }
}

#[cfg(feature = "is_sync")]
fn main() {}


#[cfg(not(feature = "is_sync"))]
#[async_std::main]
async fn main() {
    let s = Struct;
    s.declare_async().await;
    s.async_fn().await;
    async_fn().await;
    pub_async_fn().await;
}
