#[cfg(feature = "futures-io")]
mod futures;

/// A compatibility layer for using non-tokio types with this crate.
///
/// Example:
/// ```no_run
/// use smol::net::unix::UnixStream;
/// use tokio_socks::{io::Compat, tcp::Socks5Stream};
/// let socket = Compat::new(UnixStream::connect(proxy_addr)
///     .await
///     .map_err(Error::Io)?); // Compat<UnixStream>
/// let conn =
///     Socks5Stream::connect_with_password_and_socket(socket, target, username, password).await?;
/// // Socks5Stream has implemented futures-io AsyncRead + AsyncWrite.
/// ```
pub struct Compat<S>(S);

#[cfg(feature = "futures-io")]
impl<S> Compat<S> {
    pub fn new(inner: S) -> Self {
        Compat(inner)
    }

    /// Consumes the `Compat``, returning the inner value.
    pub fn into_inner(self) -> S {
        self.0
    }
}

#[cfg(feature = "futures-io")]
impl<S> AsRef<S> for Compat<S> {
    fn as_ref(&self) -> &S {
        &self.0
    }
}

#[cfg(feature = "futures-io")]
impl<S> AsMut<S> for Compat<S> {
    fn as_mut(&mut self) -> &mut S {
        &mut self.0
    }
}
