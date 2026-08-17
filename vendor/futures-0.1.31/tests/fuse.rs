extern crate futures;

use futures::prelude::*;
use futures::future::ok;
use futures::executor;

mod support;
use support::*;

#[test]
fn fuse() {
    let mut future = executor::spawn(ok::<i32, u32>(2).fuse());
    assert!(future.poll_future_notify(&notify_panic(), 0).unwrap().is_ready());
    assert!(future.poll_future_notify(&notify_panic(), 0).unwrap().is_not_ready());
}

#[test]
fn fuse_is_done() {
    use futures::future::{Fuse, FutureResult};

    struct Wrapped(Fuse<FutureResult<i32, u32>>);

    impl Future for Wrapped {
        type Item = ();
        type Error = ();

        fn poll(&mut self) -> Poll<(), ()> {
            assert!(!self.0.is_done());
            assert_eq!(self.0.poll().unwrap(), Async::Ready(2));
            assert!(self.0.is_done());
            assert_eq!(self.0.poll().unwrap(), Async::NotReady);
            assert!(self.0.is_done());
            
            Ok(Async::Ready(()))            
        }
    }

    assert!(Wrapped(ok::<i32, u32>(2).fuse()).wait().is_ok());
}