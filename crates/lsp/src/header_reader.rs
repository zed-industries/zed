use std::{
    io, mem,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{AsyncBufRead, Future};

const HEADER_DELIMITER: &'static [u8; 4] = b"\r\n\r\n";

pub fn read_headers<'a, R>(
    reader: &'a mut R,
    buf: &'a mut Vec<u8>,
) -> impl Future<Output = io::Result<usize>> + 'a
where
    R: AsyncBufRead + ?Sized + Unpin,
{
    HeaderReader {
        reader,
        buf,
        read: 0,
    }
}

struct HeaderReader<'a, R: ?Sized> {
    reader: &'a mut R,
    buf: &'a mut Vec<u8>,
    read: usize,
}

impl<'a, R: ?Sized> HeaderReader<'a, R> {
    fn read_headers<C: AsyncBufRead + ?Sized>(
        mut reader: Pin<&mut C>,
        cx: &mut Context<'_>,
        buf: &mut Vec<u8>,
        read: &mut usize,
    ) -> Poll<io::Result<usize>> {
        loop {
            let (done, used) = {
                let available = futures::ready!(reader.as_mut().poll_fill_buf(cx))?;
                if let Some(i) = find_delimiter(&available) {
                    buf.extend_from_slice(&available[..i + 5]);
                    (true, i + 5)
                } else if buf.len() > 2
                    && available.len() > 0
                    && buf[buf.len() - 3..] == HEADER_DELIMITER[..3]
                    && available[0] == HEADER_DELIMITER[3]
                {
                    buf.extend_from_slice(&available[..1]);
                    (true, 1)
                } else if buf.len() > 1
                    && available.len() > 0
                    && buf[buf.len() - 2..] == HEADER_DELIMITER[..2]
                    && available[..2] == HEADER_DELIMITER[..2]
                {
                    buf.extend_from_slice(&available[..2]);
                    (true, 2)
                } else if buf.len() > 0
                    && available.len() > 0
                    && buf[buf.len() - 1..] == HEADER_DELIMITER[..1]
                    && available[..3] == HEADER_DELIMITER[1..3]
                {
                    buf.extend_from_slice(&available[..3]);
                    (true, 3)
                } else {
                    buf.extend_from_slice(available);
                    (false, available.len())
                }
            };

            reader.as_mut().consume(used);
            *read += used;

            if done || used == 0 {
                return Poll::Ready(Ok(mem::replace(read, 0)));
            }
        }
    }
}

impl<R: AsyncBufRead + ?Sized + Unpin> Future for HeaderReader<'_, R> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { reader, buf, read } = &mut *self;
        Self::read_headers(Pin::new(reader), cx, buf, read)
    }
}

fn find_delimiter(buf: &[u8]) -> Option<usize> {
    if buf.len() < 4 {
        return None;
    }

    if let Some(i) = buf[1..].windows(4).position(|w| w == &HEADER_DELIMITER[..]) {
        return Some(i);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    async fn test_read_delimiters_only() {
        let mut buf = Vec::new();
        let mut reader = futures::io::Cursor::new(b"\r\n\r\n");
        let read = read_headers(&mut reader, &mut buf).await.unwrap();
        assert_eq!(read, 4);
        assert_eq!(buf, b"\r\n\r\n");
    }

    #[gpui::test]
    async fn test_read_content_length() {
        let mut buf = Vec::new();
        let mut reader = futures::io::Cursor::new(b"Content-Length: 123\r\n\r\n");
        let read = read_headers(&mut reader, &mut buf).await.unwrap();
        assert_eq!(read, 23);
        assert_eq!(buf, b"Content-Length: 123\r\n\r\n");
    }

    #[gpui::test]
    async fn test_read_content_type_and_length() {
        let mut buf = Vec::new();
        let mut reader = futures::io::Cursor::new(b"Content-Type: application/vscode-jsonrpc; charset=utf-8\r\nContent-Length: 1235\r\n\r\n{\"somecontent\":true}");
        let read = read_headers(&mut reader, &mut buf).await.unwrap();
        assert_eq!(read, 81);
        assert_eq!(
                buf,
                b"Content-Type: application/vscode-jsonrpc; charset=utf-8\r\nContent-Length: 1235\r\n\r\n"
            );

        let mut buf = Vec::new();
        let mut reader = futures::io::Cursor::new(b"Content-Length: 1235\r\nContent-Type: application/vscode-jsonrpc; charset=utf-8\r\n\r\n{\"somecontent\":true}");
        let read = read_headers(&mut reader, &mut buf).await.unwrap();
        assert_eq!(read, 81);
        assert_eq!(
                buf,
                b"Content-Length: 1235\r\nContent-Type: application/vscode-jsonrpc; charset=utf-8\r\n\r\n"
            );
    }
}
