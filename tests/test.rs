#![allow(dead_code)]

use async_trait::async_trait;
use maybe_async::{maybe_async, must_be_async, must_be_sync};

#[maybe_async]
trait PrivateTrait {
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
async fn maybe_async_fn() {}

#[maybe_async]
pub async fn pub_maybe_async_fn() {}

struct MaybeAsyncStruct;

#[maybe_async]
impl PrivateTrait for MaybeAsyncStruct {
    fn sync_fn() {}

    async fn declare_async(&self) {}

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

struct MustBeSyncStruct;

#[must_be_sync]
trait PrivateTraitMustBeSync {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[must_be_sync]
pub trait PubTraitMustBeSync {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[must_be_sync]
async fn must_be_sync_fn() {}

#[must_be_sync]
pub async fn pub_must_be_sync_fn() {}

#[must_be_sync]
impl PrivateTraitMustBeSync for MustBeSyncStruct {
    fn sync_fn() {}

    async fn declare_async(&self) {}

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

struct MustBeAsyncStruct;

#[must_be_async]
trait PrivateTraitMustBeAsync {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[must_be_async]
pub trait PubTraitMustBeAsync {
    fn sync_fn() {}

    async fn declare_async(&self);

    async fn async_fn(&self) {
        self.declare_async().await
    }
}

#[must_be_async]
async fn must_be_async_fn() {}

#[must_be_async]
pub async fn pub_must_be_async_fn() {}

#[must_be_async]
impl PrivateTraitMustBeAsync for MustBeAsyncStruct {
    fn sync_fn() {}

    async fn declare_async(&self) {}

    async fn async_fn(&self) {
        self.declare_async().await
    }
}
