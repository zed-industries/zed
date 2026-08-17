extern crate futures;

mod support;
use support::*;

use std::sync::mpsc;
use std::thread;

use futures::prelude::*;
use futures::future::{ok, err};
use futures::sync::oneshot;

#[test]
fn and_then1() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();
    let p1 = ok::<_, i32>("a").then(move |t| { tx2.send("first").unwrap(); t });
    let tx2 = tx.clone();
    let p2 = ok("b").then(move |t| { tx2.send("second").unwrap(); t });
    let f = p1.and_then(|_| p2);

    assert!(rx.try_recv().is_err());
    f.map(move |s| tx.send(s).unwrap()).forget();
    assert_eq!(rx.recv(), Ok("first"));
    assert_eq!(rx.recv(), Ok("second"));
    assert_eq!(rx.recv(), Ok("b"));
    assert!(rx.recv().is_err());
}

#[test]
fn and_then2() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();
    let p1 = err::<i32, _>(2).then(move |t| { tx2.send("first").unwrap(); t });
    let tx2 = tx.clone();
    let p2 = ok("b").then(move |t| { tx2.send("second").unwrap(); t });
    let f = p1.and_then(|_| p2);

    assert!(rx.try_recv().is_err());
    f.map_err(|_| drop(tx)).forget();
    assert_eq!(rx.recv(), Ok("first"));
    assert!(rx.recv().is_err());
}

#[test]
fn oneshot1() {
    let (c, p) = oneshot::channel::<i32>();
    let t = thread::spawn(|| c.send(1).unwrap());

    let (tx, rx) = mpsc::channel();
    p.map(move |e| tx.send(e).unwrap()).forget();
    assert_eq!(rx.recv(), Ok(1));
    t.join().unwrap();
}

#[test]
fn oneshot2() {
    let (c, p) = oneshot::channel::<i32>();
    let t = thread::spawn(|| c.send(1).unwrap());
    t.join().unwrap();

    let (tx, rx) = mpsc::channel();
    p.map(move |e| tx.send(e).unwrap()).forget();
    assert_eq!(rx.recv(), Ok(1));
}

#[test]
fn oneshot3() {
    let (c, p) = oneshot::channel::<i32>();
    let (tx, rx) = mpsc::channel();
    p.map(move |e| tx.send(e).unwrap()).forget();

    let t = thread::spawn(|| c.send(1).unwrap());
    t.join().unwrap();

    assert_eq!(rx.recv(), Ok(1));
}

#[test]
fn oneshot4() {
    let (c, p) = oneshot::channel::<i32>();
    drop(c);

    let (tx, rx) = mpsc::channel();
    p.map(move |e| tx.send(e).unwrap()).forget();
    assert!(rx.recv().is_err());
}

#[test]
fn oneshot5() {
    let (c, p) = oneshot::channel::<i32>();
    let t = thread::spawn(|| drop(c));
    let (tx, rx) = mpsc::channel();
    p.map(move |t| tx.send(t).unwrap()).forget();
    t.join().unwrap();
    assert!(rx.recv().is_err());
}

#[test]
fn oneshot6() {
    let (c, p) = oneshot::channel::<i32>();
    drop(p);
    c.send(2).unwrap_err();
}

#[test]
fn cancel1() {
    let (c, p) = oneshot::channel::<i32>();
    drop(c);
    p.map(|_| panic!()).forget();
}

#[test]
fn map_err1() {
    ok::<i32, i32>(1).map_err(|_| panic!()).forget();
}

#[test]
fn map_err2() {
    let (tx, rx) = mpsc::channel();
    err::<i32, i32>(1).map_err(move |v| tx.send(v).unwrap()).forget();
    assert_eq!(rx.recv(), Ok(1));
    assert!(rx.recv().is_err());
}

#[test]
fn map_err3() {
    let (c, p) = oneshot::channel::<i32>();
    p.map_err(|_| {}).forget();
    drop(c);
}

#[test]
fn or_else1() {
    let (c1, p1) = oneshot::channel::<i32>();
    let (c2, p2) = oneshot::channel::<i32>();

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    let p1 = p1.map_err(move |i| { tx2.send(2).unwrap(); i });
    let tx2 = tx.clone();
    let p2 = p2.map(move |i| { tx2.send(i).unwrap(); i });

    assert!(rx.try_recv().is_err());
    drop(c1);
    c2.send(3).unwrap();
    p1.or_else(|_| p2).map(move |v| tx.send(v).unwrap()).forget();

    assert_eq!(rx.recv(), Ok(2));
    assert_eq!(rx.recv(), Ok(3));
    assert_eq!(rx.recv(), Ok(3));
    assert!(rx.recv().is_err());
}

#[test]
fn or_else2() {
    let (c1, p1) = oneshot::channel::<i32>();

    let (tx, rx) = mpsc::channel();

    p1.or_else(move |_| {
        tx.send(()).unwrap();
        ok::<i32, i32>(1)
    }).forget();

    c1.send(2).unwrap();
    assert!(rx.recv().is_err());
}

#[test]
fn join1() {
    let (tx, rx) = mpsc::channel();
    ok::<i32, i32>(1).join(ok(2))
                           .map(move |v| tx.send(v).unwrap())
                           .forget();
    assert_eq!(rx.recv(), Ok((1, 2)));
    assert!(rx.recv().is_err());
}

#[test]
fn join2() {
    let (c1, p1) = oneshot::channel::<i32>();
    let (c2, p2) = oneshot::channel::<i32>();
    let (tx, rx) = mpsc::channel();
    p1.join(p2).map(move |v| tx.send(v).unwrap()).forget();
    assert!(rx.try_recv().is_err());
    c1.send(1).unwrap();
    assert!(rx.try_recv().is_err());
    c2.send(2).unwrap();
    assert_eq!(rx.recv(), Ok((1, 2)));
    assert!(rx.recv().is_err());
}

#[test]
fn join3() {
    let (c1, p1) = oneshot::channel::<i32>();
    let (c2, p2) = oneshot::channel::<i32>();
    let (tx, rx) = mpsc::channel();
    p1.join(p2).map_err(move |_v| tx.send(1).unwrap()).forget();
    assert!(rx.try_recv().is_err());
    drop(c1);
    assert_eq!(rx.recv(), Ok(1));
    assert!(rx.recv().is_err());
    drop(c2);
}

#[test]
fn join4() {
    let (c1, p1) = oneshot::channel::<i32>();
    let (c2, p2) = oneshot::channel::<i32>();
    let (tx, rx) = mpsc::channel();
    p1.join(p2).map_err(move |v| tx.send(v).unwrap()).forget();
    assert!(rx.try_recv().is_err());
    drop(c1);
    assert!(rx.recv().is_ok());
    drop(c2);
    assert!(rx.recv().is_err());
}

#[test]
fn join5() {
    let (c1, p1) = oneshot::channel::<i32>();
    let (c2, p2) = oneshot::channel::<i32>();
    let (c3, p3) = oneshot::channel::<i32>();
    let (tx, rx) = mpsc::channel();
    p1.join(p2).join(p3).map(move |v| tx.send(v).unwrap()).forget();
    assert!(rx.try_recv().is_err());
    c1.send(1).unwrap();
    assert!(rx.try_recv().is_err());
    c2.send(2).unwrap();
    assert!(rx.try_recv().is_err());
    c3.send(3).unwrap();
    assert_eq!(rx.recv(), Ok(((1, 2), 3)));
    assert!(rx.recv().is_err());
}

#[test]
fn select1() {
    let (c1, p1) = oneshot::channel::<i32>();
    let (c2, p2) = oneshot::channel::<i32>();
    let (tx, rx) = mpsc::channel();
    p1.select(p2).map(move |v| tx.send(v).unwrap()).forget();
    assert!(rx.try_recv().is_err());
    c1.send(1).unwrap();
    let (v, p2) = rx.recv().unwrap();
    assert_eq!(v, 1);
    assert!(rx.recv().is_err());

    let (tx, rx) = mpsc::channel();
    p2.map(move |v| tx.send(v).unwrap()).forget();
    c2.send(2).unwrap();
    assert_eq!(rx.recv(), Ok(2));
    assert!(rx.recv().is_err());
}

#[test]
fn select2() {
    let (c1, p1) = oneshot::channel::<i32>();
    let (c2, p2) = oneshot::channel::<i32>();
    let (tx, rx) = mpsc::channel();
    p1.select(p2).map_err(move |v| tx.send((1, v.1)).unwrap()).forget();
    assert!(rx.try_recv().is_err());
    drop(c1);
    let (v, p2) = rx.recv().unwrap();
    assert_eq!(v, 1);
    assert!(rx.recv().is_err());

    let (tx, rx) = mpsc::channel();
    p2.map(move |v| tx.send(v).unwrap()).forget();
    c2.send(2).unwrap();
    assert_eq!(rx.recv(), Ok(2));
    assert!(rx.recv().is_err());
}

#[test]
fn select3() {
    let (c1, p1) = oneshot::channel::<i32>();
    let (c2, p2) = oneshot::channel::<i32>();
    let (tx, rx) = mpsc::channel();
    p1.select(p2).map_err(move |v| tx.send((1, v.1)).unwrap()).forget();
    assert!(rx.try_recv().is_err());
    drop(c1);
    let (v, p2) = rx.recv().unwrap();
    assert_eq!(v, 1);
    assert!(rx.recv().is_err());

    let (tx, rx) = mpsc::channel();
    p2.map_err(move |_v| tx.send(2).unwrap()).forget();
    drop(c2);
    assert_eq!(rx.recv(), Ok(2));
    assert!(rx.recv().is_err());
}

#[test]
fn select4() {
    let (tx, rx) = mpsc::channel::<oneshot::Sender<i32>>();

    let t = thread::spawn(move || {
        for c in rx {
            c.send(1).unwrap();
        }
    });

    let (tx2, rx2) = mpsc::channel();
    for _ in 0..10000 {
        let (c1, p1) = oneshot::channel::<i32>();
        let (c2, p2) = oneshot::channel::<i32>();

        let tx3 = tx2.clone();
        p1.select(p2).map(move |_| tx3.send(()).unwrap()).forget();
        tx.send(c1).unwrap();
        rx2.recv().unwrap();
        drop(c2);
    }
    drop(tx);

    t.join().unwrap();
}
