extern crate futures;

use futures::future::*;

#[test]
fn ignore_err() {
    let v = vec![
        err(1),
        err(2),
        ok(3),
        ok(4),
    ];

    let (i, v) = select_ok(v).wait().ok().unwrap();
    assert_eq!(i, 3);

    assert_eq!(v.len(), 1);

    let (i, v) = select_ok(v).wait().ok().unwrap();
    assert_eq!(i, 4);

    assert!(v.is_empty());
}

#[test]
fn last_err() {
    let v = vec![
        ok(1),
        err(2),
        err(3),
    ];

    let (i, v) = select_ok(v).wait().ok().unwrap();
    assert_eq!(i, 1);

    assert_eq!(v.len(), 2);

    let i = select_ok(v).wait().err().unwrap();
    assert_eq!(i, 3);
}
