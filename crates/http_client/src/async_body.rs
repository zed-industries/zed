use std::{borrow::Cow, io::Read, pin::Pin, task::Poll};

use futures::{AsyncRead, AsyncReadExt};

/// Based on the implementation of AsyncBody in
/// https://github.com/sagebind/isahc/blob/5c533f1ef4d6bdf1fd291b5103c22110f41d0bf0/src/body/mod.rs
pub struct AsyncBody(pub(super) Inner);

pub(super) enum Inner {
    /// An empty body.
    Empty,

    /// A body stored in memory.
    SyncReader(std::io::Cursor<Cow<'static, [u8]>>),

    /// An asynchronous reader.
    AsyncReader(Pin<Box<dyn futures::AsyncRead + Send + Sync>>),
}

impl AsyncBody {
    /// Create a new empty body.
    ///
    /// An empty body represents the *absence* of a body, which is semantically
    /// different than the presence of a body of zero length.
    pub fn empty() -> Self {
        Self(Inner::Empty)
    }
    /// Create a streaming body that reads from the given reader.
    pub fn from_reader<R>(read: R) -> Self
    where
        R: AsyncRead + Send + Sync + 'static,
    {
        Self(Inner::AsyncReader(Box::pin(read)))
    }
}

impl std::io::Read for AsyncBody {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.0 {
            Inner::Empty => Ok(0),
            Inner::SyncReader(cursor) => cursor.read(buf),
            Inner::AsyncReader(async_reader) => smol::block_on(async_reader.read(buf)),
        }
    }
}

impl Default for AsyncBody {
    fn default() -> Self {
        Self(Inner::Empty)
    }
}

impl From<()> for AsyncBody {
    fn from(_: ()) -> Self {
        Self(Inner::Empty)
    }
}

impl From<Vec<u8>> for AsyncBody {
    fn from(body: Vec<u8>) -> Self {
        Self(Inner::SyncReader(std::io::Cursor::new(Cow::Owned(body))))
    }
}

impl From<&'_ [u8]> for AsyncBody {
    fn from(body: &[u8]) -> Self {
        body.to_vec().into()
    }
}

impl From<String> for AsyncBody {
    fn from(body: String) -> Self {
        body.into_bytes().into()
    }
}

impl From<&'_ str> for AsyncBody {
    fn from(body: &str) -> Self {
        body.as_bytes().into()
    }
}

impl<T: Into<Self>> From<Option<T>> for AsyncBody {
    fn from(body: Option<T>) -> Self {
        match body {
            Some(body) => body.into(),
            None => Self(Inner::Empty),
        }
    }
}

/// Provides extension methods for consuming HTTP response streams.
pub trait ReadResponseExt<R: Read> {
    /// Read any remaining bytes from the response body stream and discard them
    /// until the end of the stream is reached. It is usually a good idea to
    /// call this method before dropping a response if you know you haven't read
    /// the entire response body.
    ///
    /// # Background
    ///
    /// By default, if a response stream is dropped before it has been
    /// completely read from, then that HTTP connection will be terminated.
    /// Depending on which version of HTTP is being used, this may require
    /// closing the network connection to the server entirely. This can result
    /// in sub-optimal performance for making multiple requests, as it prevents
    /// Isahc from keeping the connection alive to be reused for subsequent
    /// requests.
    ///
    /// If you are downloading a file on behalf of a user and have been
    /// requested to cancel the operation, then this is probably what you want.
    /// But if you are making many small API calls to a known server, then you
    /// may want to call `consume()` before dropping the response, as reading a
    /// few megabytes off a socket is usually more efficient in the long run
    /// than taking a hit on connection reuse, and opening new connections can
    /// be expensive.
    ///
    /// Note that in HTTP/2 and newer, it is not necessary to close the network
    /// connection in order to interrupt the transfer of a particular response.
    /// If you know that you will be using only HTTP/2 or newer, then calling
    /// this method is probably unnecessary.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    ///
    /// let mut response = isahc::get("https://example.org")?;
    ///
    /// println!("Status: {}", response.status());
    /// println!("Headers: {:#?}", response.headers());
    ///
    /// // Read and discard the response body until the end.
    /// response.consume()?;
    /// # Ok::<(), isahc::Error>(())
    /// ```
    fn consume(&mut self) -> io::Result<()> {
        self.copy_to(io::sink())?;

        Ok(())
    }

    /// Copy the response body into a writer.
    ///
    /// Returns the number of bytes that were written.
    ///
    /// # Examples
    ///
    /// Copying the response into an in-memory buffer:
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    ///
    /// let mut buf = vec![];
    /// isahc::get("https://example.org")?.copy_to(&mut buf)?;
    /// println!("Read {} bytes", buf.len());
    /// # Ok::<(), isahc::Error>(())
    /// ```
    fn copy_to<W: Write>(&mut self, writer: W) -> io::Result<u64>;

    /// Write the response body to a file.
    ///
    /// This method makes it convenient to download a file using a GET request
    /// and write it to a file synchronously in a single chain of calls.
    ///
    /// Returns the number of bytes that were written.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    ///
    /// isahc::get("https://httpbin.org/image/jpeg")?
    ///     .copy_to_file("myimage.jpg")?;
    /// # Ok::<(), isahc::Error>(())
    /// ```
    fn copy_to_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<u64> {
        File::create(path).and_then(|f| self.copy_to(f))
    }

    /// Read the entire response body into memory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    ///
    /// let image_bytes = isahc::get("https://httpbin.org/image/jpeg")?.bytes()?;
    /// # Ok::<(), isahc::Error>(())
    /// ```
    fn bytes(&mut self) -> io::Result<Vec<u8>>;

    /// Read the response body as a string.
    ///
    /// The encoding used to decode the response body into a string depends on
    /// the response. If the body begins with a [Byte Order Mark
    /// (BOM)](https://en.wikipedia.org/wiki/Byte_order_mark), then UTF-8,
    /// UTF-16LE or UTF-16BE is used as indicated by the BOM. If no BOM is
    /// present, the encoding specified in the `charset` parameter of the
    /// `Content-Type` header is used if present. Otherwise UTF-8 is assumed.
    ///
    /// If the response body contains any malformed characters or characters not
    /// representable in UTF-8, the offending bytes will be replaced with
    /// `U+FFFD REPLACEMENT CHARACTER`, which looks like this: �.
    ///
    /// This method consumes the entire response body stream and can only be
    /// called once.
    ///
    /// # Availability
    ///
    /// This method is only available when the
    /// [`text-decoding`](index.html#text-decoding) feature is enabled, which it
    /// is by default.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    ///
    /// let text = isahc::get("https://example.org")?.text()?;
    /// println!("{}", text);
    /// # Ok::<(), isahc::Error>(())
    /// ```
    #[cfg(feature = "text-decoding")]
    fn text(&mut self) -> io::Result<String>;

    /// Deserialize the response body as JSON into a given type.
    ///
    /// # Availability
    ///
    /// This method is only available when the [`json`](index.html#json) feature
    /// is enabled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    /// use serde_json::Value;
    ///
    /// let json: Value = isahc::get("https://httpbin.org/json")?.json()?;
    /// println!("author: {}", json["slideshow"]["author"]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg(feature = "json")]
    fn json<T>(&mut self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned;
}

impl<R: Read> ReadResponseExt<R> for Response<R> {
    fn copy_to<W: Write>(&mut self, mut writer: W) -> io::Result<u64> {
        io::copy(self.body_mut(), &mut writer)
    }

    fn bytes(&mut self) -> io::Result<Vec<u8>> {
        let mut buf = allocate_buffer(self);

        self.copy_to(&mut buf)?;

        Ok(buf)
    }

    #[cfg(feature = "text-decoding")]
    fn text(&mut self) -> io::Result<String> {
        crate::text::Decoder::for_response(self).decode_reader(self.body_mut())
    }

    #[cfg(feature = "json")]
    fn json<D>(&mut self) -> Result<D, serde_json::Error>
    where
        D: serde::de::DeserializeOwned,
    {
        serde_json::from_reader(self.body_mut())
    }
}

/// Provides extension methods for consuming asynchronous HTTP response streams.
pub trait AsyncReadResponseExt<R: AsyncRead + Unpin> {
    /// Read any remaining bytes from the response body stream and discard them
    /// until the end of the stream is reached. It is usually a good idea to
    /// call this method before dropping a response if you know you haven't read
    /// the entire response body.
    ///
    /// # Background
    ///
    /// By default, if a response stream is dropped before it has been
    /// completely read from, then that HTTP connection will be terminated.
    /// Depending on which version of HTTP is being used, this may require
    /// closing the network connection to the server entirely. This can result
    /// in sub-optimal performance for making multiple requests, as it prevents
    /// Isahc from keeping the connection alive to be reused for subsequent
    /// requests.
    ///
    /// If you are downloading a file on behalf of a user and have been
    /// requested to cancel the operation, then this is probably what you want.
    /// But if you are making many small API calls to a known server, then you
    /// may want to call `consume()` before dropping the response, as reading a
    /// few megabytes off a socket is usually more efficient in the long run
    /// than taking a hit on connection reuse, and opening new connections can
    /// be expensive.
    ///
    /// Note that in HTTP/2 and newer, it is not necessary to close the network
    /// connection in order to interrupt the transfer of a particular response.
    /// If you know that you will be using only HTTP/2 or newer, then calling
    /// this method is probably unnecessary.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    ///
    /// # async fn run() -> Result<(), isahc::Error> {
    /// let mut response = isahc::get_async("https://example.org").await?;
    ///
    /// println!("Status: {}", response.status());
    /// println!("Headers: {:#?}", response.headers());
    ///
    /// // Read and discard the response body until the end.
    /// response.consume().await?;
    /// # Ok(()) }
    /// ```
    fn consume(&mut self) -> ConsumeFuture<'_, R>;

    /// Copy the response body into a writer asynchronously.
    ///
    /// Returns the number of bytes that were written.
    ///
    /// # Examples
    ///
    /// Copying the response into an in-memory buffer:
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    ///
    /// # async fn run() -> Result<(), isahc::Error> {
    /// let mut buf = vec![];
    /// isahc::get_async("https://example.org").await?
    ///     .copy_to(&mut buf).await?;
    /// println!("Read {} bytes", buf.len());
    /// # Ok(()) }
    /// ```
    fn copy_to<'a, W>(&'a mut self, writer: W) -> CopyFuture<'a, R, W>
    where
        W: AsyncWrite + Unpin + 'a;

    /// Read the entire response body into memory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    ///
    /// # async fn run() -> Result<(), isahc::Error> {
    /// let image_bytes = isahc::get_async("https://httpbin.org/image/jpeg")
    ///     .await?
    ///     .bytes()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    fn bytes(&mut self) -> BytesFuture<'_, &mut R>;

    /// Read the response body as a string asynchronously.
    ///
    /// This method consumes the entire response body stream and can only be
    /// called once.
    ///
    /// # Availability
    ///
    /// This method is only available when the
    /// [`text-decoding`](index.html#text-decoding) feature is enabled, which it
    /// is by default.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    ///
    /// # async fn run() -> Result<(), isahc::Error> {
    /// let text = isahc::get_async("https://example.org").await?
    ///     .text().await?;
    /// println!("{}", text);
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "text-decoding")]
    fn text(&mut self) -> crate::text::TextFuture<'_, &mut R>;

    /// Deserialize the response body as JSON into a given type.
    ///
    /// # Caveats
    ///
    /// Unlike its [synchronous equivalent](ReadResponseExt::json), this method
    /// reads the entire response body into memory before attempting
    /// deserialization. This is due to a Serde limitation since incremental
    /// partial deserializing is not supported.
    ///
    /// # Availability
    ///
    /// This method is only available when the [`json`](index.html#json) feature
    /// is enabled.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use isahc::prelude::*;
    /// use serde_json::Value;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let json: Value = isahc::get_async("https://httpbin.org/json").await?
    ///     .json().await?;
    /// println!("author: {}", json["slideshow"]["author"]);
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "json")]
    fn json<T>(&mut self) -> JsonFuture<'_, R, T>
    where
        T: serde::de::DeserializeOwned;
}

impl<R: AsyncRead + Unpin> AsyncReadResponseExt<R> for Response<R> {
    fn consume(&mut self) -> ConsumeFuture<'_, R> {
        ConsumeFuture::new(async move {
            copy_async(self.body_mut(), futures_lite::io::sink()).await?;

            Ok(())
        })
    }

    fn copy_to<'a, W>(&'a mut self, writer: W) -> CopyFuture<'a, R, W>
    where
        W: AsyncWrite + Unpin + 'a,
    {
        CopyFuture::new(async move { copy_async(self.body_mut(), writer).await })
    }

    fn bytes(&mut self) -> BytesFuture<'_, &mut R> {
        BytesFuture::new(async move {
            let mut buf = allocate_buffer(self);

            copy_async(self.body_mut(), &mut buf).await?;

            Ok(buf)
        })
    }

    #[cfg(feature = "text-decoding")]
    fn text(&mut self) -> crate::text::TextFuture<'_, &mut R> {
        crate::text::Decoder::for_response(self).decode_reader_async(self.body_mut())
    }

    #[cfg(feature = "json")]
    fn json<T>(&mut self) -> JsonFuture<'_, R, T>
    where
        T: serde::de::DeserializeOwned,
    {
        JsonFuture::new(async move {
            let mut buf = allocate_buffer(self);

            // Serde does not support incremental parsing, so we have to resort
            // to reading the entire response into memory first and then
            // deserializing.
            if let Err(e) = copy_async(self.body_mut(), &mut buf).await {
                struct ErrorReader(Option<io::Error>);

                impl Read for ErrorReader {
                    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
                        Err(self.0.take().unwrap())
                    }
                }

                // Serde offers no public way to directly create an error from
                // an I/O error, but we can do so in a roundabout way by parsing
                // a reader that always returns the desired error.
                serde_json::from_reader(ErrorReader(Some(e)))
            } else {
                serde_json::from_slice(&buf)
            }
        })
    }
}
