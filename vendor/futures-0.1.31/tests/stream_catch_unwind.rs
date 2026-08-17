extern crate futures;

use futures::stream;
use futures::prelude::*;

#[test]
fn panic_in_the_middle_of_the_stream() {
    let stream = stream::iter_ok::<_, bool>(vec![Some(10), None, Some(11)]);

    // panic on second element
    let stream_panicking = stream.map(|o| o.unwrap());
    let mut iter = stream_panicking.catch_unwind().wait();

    assert_eq!(Ok(10), iter.next().unwrap().ok().unwrap());
    assert!(iter.next().unwrap().is_err());
    assert!(iter.next().is_none());
}

#[test]
fn no_panic() {
    let stream = stream::iter_ok::<_, bool>(vec![10, 11, 12]);

    let mut iter = stream.catch_unwind().wait();

    assert_eq!(Ok(10), iter.next().unwrap().ok().unwrap());
    assert_eq!(Ok(11), iter.next().unwrap().ok().unwrap());
    assert_eq!(Ok(12), iter.next().unwrap().ok().unwrap());
    assert!(iter.next().is_none());
}
