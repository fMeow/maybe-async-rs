#![allow(dead_code)]

use maybe_async::{must_be_sync};

#[must_be_sync]
trait Trait {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[must_be_sync]
pub trait PubTrait {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[must_be_sync]
async fn async_fn() {}

#[must_be_sync]
pub async fn pub_async_fn() {}

struct Struct;

#[must_be_sync]
impl Trait for Struct {
    fn sync_fn() {}

    async fn declare_async(&self) {}

    async fn async_fn(&self) {
        self.declare_async().await
    }
}
fn main()->std::result::Result<(),()>{
    let s = Struct;
    s.declare_async();
    s.async_fn();
    Ok(())
}
