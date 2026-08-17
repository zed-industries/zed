#![cfg(feature = "tokio")]

extern crate bzip2;
extern crate futures;
extern crate rand;
extern crate tokio_core;
extern crate tokio_io;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener};
use std::thread;

use bzip2::read;
use bzip2::write;
use bzip2::Compression;
use futures::Future;
use rand::{thread_rng, Rng};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::io::{copy, shutdown};
use tokio_io::AsyncRead;

#[test]
fn tcp_stream_echo_pattern() {
    const N: u8 = 16;
    const M: usize = 16 * 1024;

    let mut core = Core::new().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let t = thread::spawn(move || {
        let a = listener.accept().unwrap().0;
        let b = a.try_clone().unwrap();

        let t = thread::spawn(move || {
            let mut b = read::BzDecoder::new(b);
            let mut buf = [0; M];
            for i in 0..N {
                b.read_exact(&mut buf).unwrap();
                for byte in buf.iter() {
                    assert_eq!(*byte, i);
                }
            }

            assert_eq!(b.read(&mut buf).unwrap(), 0);
        });

        let mut a = write::BzEncoder::new(a, Compression::default());
        for i in 0..N {
            let buf = [i; M];
            a.write_all(&buf).unwrap();
        }
        a.finish().unwrap().shutdown(Shutdown::Write).unwrap();

        t.join().unwrap();
    });

    let handle = core.handle();
    let stream = TcpStream::connect(&addr, &handle);
    let copy = stream
        .and_then(|s| {
            let (a, b) = s.split();
            let a = read::BzDecoder::new(a);
            let b = write::BzEncoder::new(b, Compression::default());
            copy(a, b)
        })
        .then(|result| {
            let (amt, _a, b) = result.unwrap();
            assert_eq!(amt, (N as u64) * (M as u64));
            shutdown(b).map(|_| ())
        });

    core.run(copy).unwrap();
    t.join().unwrap();
}

#[test]
fn echo_random() {
    let mut v = vec![0; 1024 * 1024];
    thread_rng().fill(v.as_mut_slice());
    let mut core = Core::new().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let v2 = v.clone();
    let t = thread::spawn(move || {
        let a = listener.accept().unwrap().0;
        let b = a.try_clone().unwrap();

        let mut v3 = v2.clone();
        let t = thread::spawn(move || {
            let mut b = read::BzDecoder::new(b);
            let mut buf = [0; 1024];
            while v3.len() > 0 {
                let n = b.read(&mut buf).unwrap();
                for (actual, expected) in buf[..n].iter().zip(&v3) {
                    assert_eq!(*actual, *expected);
                }
                v3.drain(..n);
            }

            assert_eq!(b.read(&mut buf).unwrap(), 0);
        });

        let mut a = write::BzEncoder::new(a, Compression::default());
        a.write_all(&v2).unwrap();
        a.finish().unwrap().shutdown(Shutdown::Write).unwrap();

        t.join().unwrap();
    });

    let handle = core.handle();
    let stream = TcpStream::connect(&addr, &handle);
    let copy = stream
        .and_then(|s| {
            let (a, b) = s.split();
            let a = read::BzDecoder::new(a);
            let b = write::BzEncoder::new(b, Compression::default());
            copy(a, b)
        })
        .then(|result| {
            let (amt, _a, b) = result.unwrap();
            assert_eq!(amt, v.len() as u64);
            shutdown(b).map(|_| ())
        });

    core.run(copy).unwrap();
    t.join().unwrap();
}
