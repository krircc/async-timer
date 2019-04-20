#![feature(async_await, await_macro, futures_api)]

use async_timer::{Timed, Delay};
use futures::executor::block_on;

use std::time;

#[test]
fn test_timed() {
    let future = Delay::platform_new(time::Duration::from_secs(4));
    let work = Timed::platform_new(future, time::Duration::from_secs(3));

    let before = time::SystemTime::now();

    let expired = block_on(work).unwrap_err();
    let work = block_on(expired);

    assert!(block_on(work).is_ok());
    let after = time::SystemTime::now();
    let diff = after.duration_since(before).unwrap();
    assert_eq!(diff.as_secs(), 4);
}
