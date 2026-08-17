#[macro_use]
mod sys_common;

use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use system_interface::io::IsReadWrite;
#[cfg(feature = "cap_std_impls")]
use {cap_fs_ext::OpenOptions, sys_common::io::tmpdir};

#[cfg(feature = "cap_std_impls")]
#[test]
fn file_is_read_write() {
    let tmpdir = tmpdir();

    let file = check!(tmpdir.open_with(
        "file",
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .read(true)
    ));
    assert_eq!(check!(file.is_read_write()), (true, true));

    let file = check!(tmpdir.open_with("file", OpenOptions::new().append(true).read(true)));
    assert_eq!(check!(file.is_read_write()), (true, true));

    let file = check!(tmpdir.open_with(
        "file",
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .read(false)
    ));
    assert_eq!(check!(file.is_read_write()), (false, true));

    let file = check!(tmpdir.open_with(
        "file",
        OpenOptions::new()
            .create(false)
            .truncate(false)
            .write(false)
            .read(true)
    ));
    assert_eq!(check!(file.is_read_write()), (true, false));
}

#[test]
fn socket_is_read_write() {
    let pair = Arc::new((Mutex::new(0_u16), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let _t = thread::spawn(move || {
        let listener = TcpListener::bind("localhost:0").unwrap();

        let (lock, cvar) = &*pair_clone;
        let mut port = lock.lock().unwrap();

        *port = listener.local_addr().unwrap().port();
        drop(port);
        cvar.notify_one();

        let stream = listener.accept().unwrap().0;

        assert_eq!(stream.is_read_write().unwrap(), (true, true));

        thread::park();
    });

    let (lock, cvar) = &*pair;
    let mut port = lock.lock().unwrap();
    while *port == 0 {
        port = cvar.wait(port).unwrap();
    }

    let stream = TcpStream::connect(("localhost", *port)).unwrap();
    assert_eq!(stream.is_read_write().unwrap(), (true, true));
    stream.shutdown(Shutdown::Read).unwrap();
    assert_eq!(stream.is_read_write().unwrap(), (false, true));
    stream.shutdown(Shutdown::Write).unwrap();
    assert_eq!(stream.is_read_write().unwrap(), (false, false));
}

#[test]
fn socket_is_write_read() {
    let pair = Arc::new((Mutex::new(0_u16), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let _t = thread::spawn(move || {
        let listener = TcpListener::bind("localhost:0").unwrap();

        let (lock, cvar) = &*pair_clone;
        let mut port = lock.lock().unwrap();

        *port = listener.local_addr().unwrap().port();
        drop(port);
        cvar.notify_one();

        let stream = listener.accept().unwrap().0;

        assert_eq!(stream.is_read_write().unwrap(), (true, true));

        thread::park();
    });

    let (lock, cvar) = &*pair;
    let mut port = lock.lock().unwrap();
    while *port == 0 {
        port = cvar.wait(port).unwrap();
    }

    let stream = TcpStream::connect(("localhost", *port)).unwrap();
    assert_eq!(stream.is_read_write().unwrap(), (true, true));
    stream.shutdown(Shutdown::Write).unwrap();
    assert_eq!(stream.is_read_write().unwrap(), (true, false));
    stream.shutdown(Shutdown::Read).unwrap();
    assert_eq!(stream.is_read_write().unwrap(), (false, false));
}
