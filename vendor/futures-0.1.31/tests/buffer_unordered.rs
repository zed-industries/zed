extern crate futures;

use std::sync::mpsc as std_mpsc;
use std::thread;

use futures::prelude::*;
use futures::sync::oneshot;
use futures::sync::mpsc;

#[test]
fn works() {
    const N: usize = 4;

    let (mut tx, rx) = mpsc::channel(1);

    let (tx2, rx2) = std_mpsc::channel();
    let (tx3, rx3) = std_mpsc::channel();
    let t1 = thread::spawn(move || {
        for _ in 0..N+1 {
            let (mytx, myrx) = oneshot::channel();
            tx = tx.send(myrx).wait().unwrap();
            tx3.send(mytx).unwrap();
        }
        rx2.recv().unwrap();
        for _ in 0..N {
            let (mytx, myrx) = oneshot::channel();
            tx = tx.send(myrx).wait().unwrap();
            tx3.send(mytx).unwrap();
        }
    });

    let (tx4, rx4) = std_mpsc::channel();
    let t2 = thread::spawn(move || {
        for item in rx.map_err(|_| panic!()).buffer_unordered(N).wait() {
            tx4.send(item.unwrap()).unwrap();
        }
    });

    let o1 = rx3.recv().unwrap();
    let o2 = rx3.recv().unwrap();
    let o3 = rx3.recv().unwrap();
    let o4 = rx3.recv().unwrap();
    assert!(rx4.try_recv().is_err());

    o1.send(1).unwrap();
    assert_eq!(rx4.recv(), Ok(1));
    o3.send(3).unwrap();
    assert_eq!(rx4.recv(), Ok(3));
    tx2.send(()).unwrap();
    o2.send(2).unwrap();
    assert_eq!(rx4.recv(), Ok(2));
    o4.send(4).unwrap();
    assert_eq!(rx4.recv(), Ok(4));

    let o5 = rx3.recv().unwrap();
    let o6 = rx3.recv().unwrap();
    let o7 = rx3.recv().unwrap();
    let o8 = rx3.recv().unwrap();
    let o9 = rx3.recv().unwrap();

    o5.send(5).unwrap();
    assert_eq!(rx4.recv(), Ok(5));
    o8.send(8).unwrap();
    assert_eq!(rx4.recv(), Ok(8));
    o9.send(9).unwrap();
    assert_eq!(rx4.recv(), Ok(9));
    o7.send(7).unwrap();
    assert_eq!(rx4.recv(), Ok(7));
    o6.send(6).unwrap();
    assert_eq!(rx4.recv(), Ok(6));

    t1.join().unwrap();
    t2.join().unwrap();
}
