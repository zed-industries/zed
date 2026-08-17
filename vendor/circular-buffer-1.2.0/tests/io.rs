// Copyright © 2023-2025 Andrea Corbellini and contributors
// SPDX-License-Identifier: BSD-3-Clause

#[cfg(feature = "std")]
mod std {
    use circular_buffer::CircularBuffer;
    use std::io::BufRead;
    use std::io::Read;
    use std::io::Write;

    #[test]
    fn write() {
        let mut buf = CircularBuffer::<4, u8>::new();
        assert_eq!(buf, [] as [u8; 0]);

        assert!(write!(&mut buf, "hello").is_ok());
        assert_eq!(buf, [b'e', b'l', b'l', b'o']);
        assert!(write!(&mut buf, "world").is_ok());
        assert_eq!(buf, [b'o', b'r', b'l', b'd']);
    }

    #[test]
    fn read() {
        fn read_all<R: Read>(mut buf: R) -> Vec<u8> {
            let mut vec = Vec::new();
            buf.read_to_end(&mut vec).expect("read failed");
            vec
        }

        let mut buf = CircularBuffer::<4, u8>::new();
        assert_eq!(buf, [] as [u8; 0]);
        assert_eq!(read_all(&mut buf), []);
        assert_eq!(buf, [] as [u8; 0]);

        buf.push_back(b'a');
        buf.push_back(b'b');
        assert_eq!(buf, [b'a', b'b']);
        assert_eq!(read_all(&mut buf), [b'a', b'b']);
        assert_eq!(buf, [] as [u8; 0]);

        buf.push_back(b'c');
        buf.push_back(b'd');
        buf.push_back(b'e');
        buf.push_back(b'f');
        assert_eq!(buf, [b'c', b'd', b'e', b'f']);
        assert_eq!(read_all(&mut buf), [b'c', b'd', b'e', b'f']);
        assert_eq!(buf, [] as [u8; 0]);
    }

    #[test]
    fn read_buf() {
        let mut buf = CircularBuffer::<4, u8>::new();
        assert_eq!(buf, [] as [u8; 0]);
        assert_eq!(buf.fill_buf().unwrap(), b"");

        buf.push_back(b'a');
        buf.push_back(b'b');
        assert_eq!(buf, [b'a', b'b']);
        assert_eq!(buf.fill_buf().unwrap(), b"ab");

        buf.push_back(b'c');
        buf.push_back(b'd');
        buf.push_back(b'e');
        buf.push_back(b'f');
        assert_eq!(buf, [b'c', b'd', b'e', b'f']);
        assert_eq!(buf.fill_buf().unwrap(), b"cd");

        buf.consume(2);
        assert_eq!(buf.fill_buf().unwrap(), b"ef");

        buf.consume(2);
        assert_eq!(buf.fill_buf().unwrap(), b"");

        buf.consume(2);
        assert_eq!(buf.fill_buf().unwrap(), b"");
    }
}

#[cfg(feature = "embedded-io")]
mod embedded_io {
    use circular_buffer::CircularBuffer;
    use embedded_io::BufRead;
    use embedded_io::Read;
    use embedded_io::Write;

    #[test]
    fn write() {
        let mut buf = CircularBuffer::<4, u8>::new();
        assert_eq!(buf, [] as [u8; 0]);

        assert!(write!(&mut buf, "hello").is_ok());
        assert_eq!(buf, [b'e', b'l', b'l', b'o']);
        assert!(write!(&mut buf, "world").is_ok());
        assert_eq!(buf, [b'o', b'r', b'l', b'd']);
    }

    #[test]
    fn read() {
        fn read_all<R: Read>(mut buf: R) -> Vec<u8> {
            let mut vec = Vec::new();
            loop {
                let mut tmp = [0u8; 4];
                let size = buf.read(&mut tmp[..]).expect("read failed");
                if size == 0 {
                    break;
                }
                vec.extend_from_slice(&tmp[..size]);
            }
            vec
        }

        let mut buf = CircularBuffer::<4, u8>::new();
        assert_eq!(buf, [] as [u8; 0]);
        assert_eq!(read_all(&mut buf), []);
        assert_eq!(buf, [] as [u8; 0]);

        buf.push_back(b'a');
        buf.push_back(b'b');
        assert_eq!(buf, [b'a', b'b']);
        assert_eq!(read_all(&mut buf), [b'a', b'b']);
        assert_eq!(buf, [] as [u8; 0]);

        buf.push_back(b'c');
        buf.push_back(b'd');
        buf.push_back(b'e');
        buf.push_back(b'f');
        assert_eq!(buf, [b'c', b'd', b'e', b'f']);
        assert_eq!(read_all(&mut buf), [b'c', b'd', b'e', b'f']);
        assert_eq!(buf, [] as [u8; 0]);
    }

    #[test]
    fn read_buf() {
        let mut buf = CircularBuffer::<4, u8>::new();
        assert_eq!(buf, [] as [u8; 0]);
        assert_eq!(buf.fill_buf().unwrap(), b"");

        buf.push_back(b'a');
        buf.push_back(b'b');
        assert_eq!(buf, [b'a', b'b']);
        assert_eq!(buf.fill_buf().unwrap(), b"ab");

        buf.push_back(b'c');
        buf.push_back(b'd');
        buf.push_back(b'e');
        buf.push_back(b'f');
        assert_eq!(buf, [b'c', b'd', b'e', b'f']);
        assert_eq!(buf.fill_buf().unwrap(), b"cd");

        buf.consume(2);
        assert_eq!(buf.fill_buf().unwrap(), b"ef");

        buf.consume(2);
        assert_eq!(buf.fill_buf().unwrap(), b"");

        buf.consume(2);
        assert_eq!(buf.fill_buf().unwrap(), b"");
    }
}

#[cfg(feature = "embedded-io-async")]
mod embedded_io_async {
    use circular_buffer::CircularBuffer;
    use embedded_io_async::BufRead;
    use embedded_io_async::Read;
    use embedded_io_async::Write;

    #[test]
    fn write() {
        futures_lite::future::block_on(async {
            let mut buf = CircularBuffer::<4, u8>::new();
            assert_eq!(buf, [] as [u8; 0]);

            assert!(buf.write_all(b"hello").await.is_ok());
            assert_eq!(buf, [b'e', b'l', b'l', b'o']);
            assert!(buf.write_all(b"world").await.is_ok());
            assert_eq!(buf, [b'o', b'r', b'l', b'd']);
        });
    }

    #[test]
    fn read() {
        futures_lite::future::block_on(async {
            async fn read_all<R: Read>(mut buf: R) -> Vec<u8> {
                let mut vec = Vec::new();
                loop {
                    let mut tmp = [0u8; 4];
                    let size = buf.read(&mut tmp[..]).await.expect("read failed");
                    if size == 0 {
                        break;
                    }
                    vec.extend_from_slice(&tmp[..size]);
                }
                vec
            }

            let mut buf = CircularBuffer::<4, u8>::new();
            assert_eq!(buf, [] as [u8; 0]);
            assert_eq!(read_all(&mut buf).await, []);
            assert_eq!(buf, [] as [u8; 0]);

            buf.push_back(b'a');
            buf.push_back(b'b');
            assert_eq!(buf, [b'a', b'b']);
            assert_eq!(read_all(&mut buf).await, [b'a', b'b']);
            assert_eq!(buf, [] as [u8; 0]);

            buf.push_back(b'c');
            buf.push_back(b'd');
            buf.push_back(b'e');
            buf.push_back(b'f');
            assert_eq!(buf, [b'c', b'd', b'e', b'f']);
            assert_eq!(read_all(&mut buf).await, [b'c', b'd', b'e', b'f']);
            assert_eq!(buf, [] as [u8; 0]);
        });
    }

    #[test]
    fn read_buf() {
        futures_lite::future::block_on(async {
            let mut buf = CircularBuffer::<4, u8>::new();
            assert_eq!(buf, [] as [u8; 0]);
            assert_eq!(buf.fill_buf().await.unwrap(), b"");

            buf.push_back(b'a');
            buf.push_back(b'b');
            assert_eq!(buf, [b'a', b'b']);
            assert_eq!(buf.fill_buf().await.unwrap(), b"ab");

            buf.push_back(b'c');
            buf.push_back(b'd');
            buf.push_back(b'e');
            buf.push_back(b'f');
            assert_eq!(buf, [b'c', b'd', b'e', b'f']);
            assert_eq!(buf.fill_buf().await.unwrap(), b"cd");

            buf.consume(2);
            assert_eq!(buf.fill_buf().await.unwrap(), b"ef");

            buf.consume(2);
            assert_eq!(buf.fill_buf().await.unwrap(), b"");

            buf.consume(2);
            assert_eq!(buf.fill_buf().await.unwrap(), b"");
        });
    }
}
