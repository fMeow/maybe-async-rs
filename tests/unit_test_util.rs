use maybe_async::maybe_async;

#[maybe_async]
async fn async_fn() -> bool {
    true
}

#[maybe_async::test(
    sync = r#"feature="is_sync""#,
    async = r#"not(feature="is_sync")"#,
    test = "async_std::test"
)]
async fn test_async_fn() {
    let res = async_fn().await;
    assert_eq!(res, true);
}
