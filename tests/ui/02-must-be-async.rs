#![allow(dead_code)]

use maybe_async::{must_be_async};

#[must_be_async]
trait Trait {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[must_be_async]
pub trait PubTrait {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[must_be_async]
async fn async_fn() {}

#[must_be_async]
pub async fn pub_async_fn() {}

struct Struct;

#[must_be_async]
impl Trait for Struct {
    fn sync_fn() {}

    async fn declare_async(&self) {}

    async fn async_fn(&self) {
        async{ self.declare_async().await}.await
    }
}
fn main()->std::result::Result<(),()>{
    Ok(())
}
