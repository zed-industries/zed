mod bytes;
mod chain;
mod read;
mod read_exact;
mod read_to_end;
mod read_to_string;
mod read_vectored;
mod take;

use read::ReadFuture;
use read_exact::ReadExactFuture;
use read_to_end::{read_to_end_internal, ReadToEndFuture};
use read_to_string::ReadToStringFuture;
use read_vectored::ReadVectoredFuture;

use std::mem;

use crate::io::IoSliceMut;

pub use bytes::Bytes;
pub use chain::Chain;
pub use take::Take;

pub use futures_io::AsyncRead as Read;

#[doc = r#"
    Extension methods for [`Read`].

    [`Read`]: ../trait.Read.html
"#]
pub trait ReadExt: Read {
    #[doc = r#"
        Reads some bytes from the byte stream.

        Returns the number of bytes read from the start of the buffer.

        If the return value is `Ok(n)`, then it must be guaranteed that
        `0 <= n <= buf.len()`. A nonzero `n` value indicates that the buffer has been
        filled in with `n` bytes of data. If `n` is `0`, then it can indicate one of two
        scenarios:

        1. This reader has reached its "end of file" and will likely no longer be able to
           produce bytes. Note that this does not mean that the reader will always no
           longer be able to produce bytes.
        2. The buffer specified was 0 bytes in length.

        # Examples

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::fs::File;
        use async_std::prelude::*;

        let mut file = File::open("a.txt").await?;

        let mut buf = vec![0; 1024];
        let n = file.read(&mut buf).await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn read<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> ReadFuture<'a, Self>
    where
        Self: Unpin
    {
        ReadFuture { reader: self, buf }
    }

    #[doc = r#"
        Like [`read`], except that it reads into a slice of buffers.

        Data is copied to fill each buffer in order, with the final buffer written to
        possibly being only partially filled. This method must behave as a single call to
        [`read`] with the buffers concatenated would.

        The default implementation calls [`read`] with either the first nonempty buffer
        provided, or an empty one if none exists.

        [`read`]: #tymethod.read
    "#]
    fn read_vectored<'a>(
        &'a mut self,
        bufs: &'a mut [IoSliceMut<'a>],
    ) -> ReadVectoredFuture<'a, Self>
    where
        Self: Unpin,
    {
        ReadVectoredFuture { reader: self, bufs }
    }

    #[doc = r#"
        Reads all bytes from the byte stream.

        All bytes read from this stream will be appended to the specified buffer `buf`.
        This function will continuously call [`read`] to append more data to `buf` until
        [`read`] returns either `Ok(0)` or an error.

        If successful, this function will return the total number of bytes read.

        [`read`]: #tymethod.read

        # Examples

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::fs::File;
        use async_std::prelude::*;

        let mut file = File::open("a.txt").await?;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn read_to_end<'a>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
    ) -> ReadToEndFuture<'a, Self>
    where
        Self: Unpin,
    {
        let start_len = buf.len();
        ReadToEndFuture {
            reader: self,
            buf,
            start_len,
        }
    }

    #[doc = r#"
        Reads all bytes from the byte stream and appends them into a string.

        If successful, this function will return the number of bytes read.

        If the data in this stream is not valid UTF-8 then an error will be returned and
        `buf` will be left unmodified.

        # Examples

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::fs::File;
        use async_std::prelude::*;

        let mut file = File::open("a.txt").await?;

        let mut buf = String::new();
        file.read_to_string(&mut buf).await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn read_to_string<'a>(
        &'a mut self,
        buf: &'a mut String,
    ) -> ReadToStringFuture<'a, Self>
    where
        Self: Unpin,
    {
        let start_len = buf.len();
        ReadToStringFuture {
            reader: self,
            bytes: unsafe { mem::replace(buf.as_mut_vec(), Vec::new()) },
            buf,
            start_len,
        }
    }

    #[doc = r#"
        Reads the exact number of bytes required to fill `buf`.

        This function reads as many bytes as necessary to completely fill the specified
        buffer `buf`.

        No guarantees are provided about the contents of `buf` when this function is
        called, implementations cannot rely on any property of the contents of `buf` being
        true. It is recommended that implementations only write data to `buf` instead of
        reading its contents.

        If this function encounters an "end of file" before completely filling the buffer,
        it returns an error of the kind [`ErrorKind::UnexpectedEof`].  The contents of
        `buf` are unspecified in this case.

        If any other read error is encountered then this function immediately returns. The
        contents of `buf` are unspecified in this case.

        If this function returns an error, it is unspecified how many bytes it has read,
        but it will never read more than would be necessary to completely fill the buffer.

        [`ErrorKind::UnexpectedEof`]: enum.ErrorKind.html#variant.UnexpectedEof

        # Examples

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::fs::File;
        use async_std::prelude::*;

        let mut file = File::open("a.txt").await?;

        let mut buf = vec![0; 10];
        file.read_exact(&mut buf).await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn read_exact<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> ReadExactFuture<'a, Self>
    where
        Self: Unpin,
    {
        ReadExactFuture { reader: self, buf }
    }

    #[doc = r#"
        Creates an adaptor which will read at most `limit` bytes from it.

        This function returns a new instance of `Read` which will read at most
        `limit` bytes, after which it will always return EOF ([`Ok(0)`]). Any
        read errors will not count towards the number of bytes read and future
        calls to [`read`] may succeed.

        # Examples

        [`File`]s implement `Read`:

        [`File`]: ../fs/struct.File.html
        [`Ok(0)`]: ../../std/result/enum.Result.html#variant.Ok
        [`read`]: tymethod.read

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::io::prelude::*;
        use async_std::fs::File;

        let f = File::open("foo.txt").await?;
        let mut buffer = [0; 5];

        // read at most five bytes
        let mut handle = f.take(5);

        handle.read(&mut buffer).await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn take(self, limit: u64) -> Take<Self>
    where
        Self: Sized,
    {
        Take { inner: self, limit }
    }

    #[doc = r#"
        Creates a "by reference" adaptor for this instance of `Read`.

        The returned adaptor also implements `Read` and will simply borrow this
        current reader.

        # Examples

        [`File`][file]s implement `Read`:

        [file]: ../fs/struct.File.html

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::fs::File;

        let mut f = File::open("foo.txt").await?;
        let mut buffer = Vec::new();
        let mut other_buffer = Vec::new();

        {
            let reference = f.by_ref();

            // read at most 5 bytes
            reference.take(5).read_to_end(&mut buffer).await?;

        } // drop our &mut reference so we can use f again

        // original file still usable, read the rest
        f.read_to_end(&mut other_buffer).await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn by_ref(&mut self) -> &mut Self where Self: Sized { self }


    #[doc = r#"
        Transforms this `Read` instance to a `Stream` over its bytes.

        The returned type implements `Stream` where the `Item` is
        `Result<u8, io::Error>`.
        The yielded item is `Ok` if a byte was successfully read and `Err`
        otherwise. EOF is mapped to returning `None` from this iterator.

        # Examples

        [`File`][file]s implement `Read`:

        [file]: ../fs/struct.File.html

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::fs::File;

        let f = File::open("foo.txt").await?;
        let mut s = f.bytes();

        while let Some(byte) = s.next().await {
            println!("{}", byte.unwrap());
        }
        #
        # Ok(()) }) }
        ```
    "#]
    fn bytes(self) -> Bytes<Self> where Self: Sized {
        Bytes { inner: self }
    }

    #[doc = r#"
        Creates an adaptor which will chain this stream with another.

        The returned `Read` instance will first read all bytes from this object
        until EOF is encountered. Afterwards the output is equivalent to the
        output of `next`.

        # Examples

        [`File`][file]s implement `Read`:

        [file]: ../fs/struct.File.html

        ```no_run
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::fs::File;

        let f1 = File::open("foo.txt").await?;
        let f2 = File::open("bar.txt").await?;

        let mut handle = f1.chain(f2);
        let mut buffer = String::new();

        // read the value into a String. We could use any Read method here,
        // this is just one example.
        handle.read_to_string(&mut buffer).await?;
        #
        # Ok(()) }) }
        ```
    "#]
    fn chain<R: Read>(self, next: R) -> Chain<Self, R> where Self: Sized {
        Chain { first: self, second: next, done_first: false }
    }
}

impl<T: Read + ?Sized> ReadExt for T {}

/// Initializes a buffer if necessary.
///
/// Currently, a buffer is always initialized because `read_initializer`
/// feature is not stable.
#[inline]
unsafe fn initialize<R: futures_io::AsyncRead>(_reader: &R, buf: &mut [u8]) {
    std::ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len())
}

#[cfg(all(test, not(target_os = "unknown")))]
mod tests {
    use crate::io;
    use crate::prelude::*;

    #[test]
    fn test_read_by_ref() {
        crate::task::block_on(async {
            let mut f = io::Cursor::new(vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8]);
            let mut buffer = Vec::new();
            let mut other_buffer = Vec::new();

            {
                let reference = f.by_ref();

                // read at most 5 bytes
                assert_eq!(reference.take(5).read_to_end(&mut buffer).await.unwrap(), 5);
                assert_eq!(&buffer, &[0, 1, 2, 3, 4])
            } // drop our &mut reference so we can use f again

            // original file still usable, read the rest
            assert_eq!(f.read_to_end(&mut other_buffer).await.unwrap(), 4);
            assert_eq!(&other_buffer, &[5, 6, 7, 8]);
        });
    }
}
