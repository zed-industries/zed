use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

use jobserver::Client;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

fn main() {
    if env::var("I_AM_THE_CLIENT").is_ok() {
        client();
    } else {
        server();
    }
}

fn server() {
    let me = t!(env::current_exe());
    let client = t!(Client::new(1));
    let mut cmd = Command::new(me);
    cmd.env("I_AM_THE_CLIENT", "1").stdout(Stdio::piped());
    client.configure(&mut cmd);
    let acq = client.acquire().unwrap();
    let mut child = t!(cmd.spawn());
    let stdout = child.stdout.take().unwrap();
    let (tx, rx) = mpsc::channel();
    let t = thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            tx.send(t!(line)).unwrap();
        }
    });

    for _ in 0..100 {
        assert!(rx.try_recv().is_err());
    }

    drop(acq);
    assert_eq!(rx.recv().unwrap(), "hello!");
    t.join().unwrap();
    assert!(rx.recv().is_err());
    client.acquire().unwrap();
}

fn client() {
    let client = unsafe { Client::from_env().unwrap() };
    let acq = client.acquire().unwrap();
    println!("hello!");
    drop(acq);
}
