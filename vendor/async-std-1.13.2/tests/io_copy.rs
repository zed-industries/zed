use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

struct ReaderThatPanicsAfterEof {
    read_count: usize,
    has_sent_eof: bool,
    max_read: usize,
}

impl async_std::io::Read for ReaderThatPanicsAfterEof {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        if self.has_sent_eof {
            panic!("this should be unreachable because we should not poll after eof (Ready(Ok(0)))")
        } else if self.read_count >= self.max_read {
            self.has_sent_eof = true;
            Poll::Ready(Ok(0))
        } else {
            self.read_count += 1;
            Poll::Ready(Ok(buf.len()))
        }
    }
}

struct WriterThatTakesAWhileToFlush {
    max_flush: usize,
    flush_count: usize,
}

impl async_std::io::Write for WriterThatTakesAWhileToFlush {
    fn poll_write(self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.flush_count += 1;
        if self.flush_count >= self.max_flush {
            Poll::Ready(Ok(()))
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[test]
fn io_copy_does_not_poll_after_eof() {
    async_std::task::block_on(async {
        let mut reader = ReaderThatPanicsAfterEof {
            has_sent_eof: false,
            max_read: 10,
            read_count: 0,
        };

        let mut writer = WriterThatTakesAWhileToFlush {
            flush_count: 0,
            max_flush: 10,
        };

        assert!(async_std::io::copy(&mut reader, &mut writer).await.is_ok());
    })
}
