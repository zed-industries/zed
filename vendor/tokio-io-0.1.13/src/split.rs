use std::io::{self, Read, Write};

use bytes::{Buf, BufMut};
use futures::sync::BiLock;
use futures::{Async, Poll};

use {AsyncRead, AsyncWrite};

/// The readable half of an object returned from `AsyncRead::split`.
#[derive(Debug)]
pub struct ReadHalf<T> {
    handle: BiLock<T>,
}

impl<T: AsyncRead + AsyncWrite> ReadHalf<T> {
    /// Reunite with a previously split `WriteHalf`.
    ///
    /// # Panics
    ///
    /// If this `ReadHalf` and the given `WriteHalf` do not originate from
    /// the same `AsyncRead::split` operation this method will panic.
    pub fn unsplit(self, w: WriteHalf<T>) -> T {
        if let Ok(x) = self.handle.reunite(w.handle) {
            x
        } else {
            panic!("Unrelated `WriteHalf` passed to `ReadHalf::unsplit`.")
        }
    }
}

/// The writable half of an object returned from `AsyncRead::split`.
#[derive(Debug)]
pub struct WriteHalf<T> {
    handle: BiLock<T>,
}

impl<T: AsyncRead + AsyncWrite> WriteHalf<T> {
    /// Reunite with a previously split `ReadHalf`.
    ///
    /// # panics
    ///
    /// If this `WriteHalf` and the given `ReadHalf` do not originate from
    /// the same `AsyncRead::split` operation this method will panic.
    pub fn unsplit(self, r: ReadHalf<T>) -> T {
        if let Ok(x) = self.handle.reunite(r.handle) {
            x
        } else {
            panic!("Unrelated `ReadHalf` passed to `WriteHalf::unsplit`.")
        }
    }
}

pub fn split<T: AsyncRead + AsyncWrite>(t: T) -> (ReadHalf<T>, WriteHalf<T>) {
    let (a, b) = BiLock::new(t);
    (ReadHalf { handle: a }, WriteHalf { handle: b })
}

fn would_block() -> io::Error {
    io::Error::new(io::ErrorKind::WouldBlock, "would block")
}

impl<T: AsyncRead> Read for ReadHalf<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.handle.poll_lock() {
            Async::Ready(mut l) => l.read(buf),
            Async::NotReady => Err(would_block()),
        }
    }
}

impl<T: AsyncRead> AsyncRead for ReadHalf<T> {
    fn read_buf<B: BufMut>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        let mut l = try_ready!(wrap_as_io(self.handle.poll_lock()));
        l.read_buf(buf)
    }
}

impl<T: AsyncWrite> Write for WriteHalf<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.handle.poll_lock() {
            Async::Ready(mut l) => l.write(buf),
            Async::NotReady => Err(would_block()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.handle.poll_lock() {
            Async::Ready(mut l) => l.flush(),
            Async::NotReady => Err(would_block()),
        }
    }
}

impl<T: AsyncWrite> AsyncWrite for WriteHalf<T> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        let mut l = try_ready!(wrap_as_io(self.handle.poll_lock()));
        l.shutdown()
    }

    fn write_buf<B: Buf>(&mut self, buf: &mut B) -> Poll<usize, io::Error>
    where
        Self: Sized,
    {
        let mut l = try_ready!(wrap_as_io(self.handle.poll_lock()));
        l.write_buf(buf)
    }
}

fn wrap_as_io<T>(t: Async<T>) -> Result<Async<T>, io::Error> {
    Ok(t)
}

#[cfg(test)]
mod tests {
    extern crate tokio_current_thread;

    use super::{AsyncRead, AsyncWrite, ReadHalf, WriteHalf};
    use bytes::{BytesMut, IntoBuf};
    use futures::sync::BiLock;
    use futures::{future::lazy, future::ok, Async, Poll};

    use std::io::{self, Read, Write};

    struct RW;

    impl Read for RW {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Ok(1)
        }
    }

    impl AsyncRead for RW {}

    impl Write for RW {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Ok(1)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl AsyncWrite for RW {
        fn shutdown(&mut self) -> Poll<(), io::Error> {
            Ok(Async::Ready(()))
        }
    }

    #[test]
    fn split_readhalf_translate_wouldblock_to_not_ready() {
        tokio_current_thread::block_on_all(lazy(move || {
            let rw = RW {};
            let (a, b) = BiLock::new(rw);
            let mut rx = ReadHalf { handle: a };

            let mut buf = BytesMut::with_capacity(64);

            // First read is uncontended, should go through.
            assert!(rx.read_buf(&mut buf).unwrap().is_ready());

            // Take lock from write side.
            let lock = b.poll_lock();

            // Second read should be NotReady.
            assert!(!rx.read_buf(&mut buf).unwrap().is_ready());

            drop(lock);

            // Back to uncontended.
            assert!(rx.read_buf(&mut buf).unwrap().is_ready());

            ok::<(), ()>(())
        }))
        .unwrap();
    }

    #[test]
    fn split_writehalf_translate_wouldblock_to_not_ready() {
        tokio_current_thread::block_on_all(lazy(move || {
            let rw = RW {};
            let (a, b) = BiLock::new(rw);
            let mut tx = WriteHalf { handle: a };

            let bufmut = BytesMut::with_capacity(64);
            let mut buf = bufmut.into_buf();

            // First write is uncontended, should go through.
            assert!(tx.write_buf(&mut buf).unwrap().is_ready());

            // Take lock from read side.
            let lock = b.poll_lock();

            // Second write should be NotReady.
            assert!(!tx.write_buf(&mut buf).unwrap().is_ready());

            drop(lock);

            // Back to uncontended.
            assert!(tx.write_buf(&mut buf).unwrap().is_ready());

            ok::<(), ()>(())
        }))
        .unwrap();
    }

    #[test]
    fn unsplit_ok() {
        let (r, w) = RW.split();
        r.unsplit(w);

        let (r, w) = RW.split();
        w.unsplit(r);
    }

    #[test]
    #[should_panic]
    fn unsplit_err1() {
        let (r, _) = RW.split();
        let (_, w) = RW.split();
        r.unsplit(w);
    }

    #[test]
    #[should_panic]
    fn unsplit_err2() {
        let (_, w) = RW.split();
        let (r, _) = RW.split();
        r.unsplit(w);
    }

    #[test]
    #[should_panic]
    fn unsplit_err3() {
        let (_, w) = RW.split();
        let (r, _) = RW.split();
        w.unsplit(r);
    }

    #[test]
    #[should_panic]
    fn unsplit_err4() {
        let (r, _) = RW.split();
        let (_, w) = RW.split();
        w.unsplit(r);
    }
}
