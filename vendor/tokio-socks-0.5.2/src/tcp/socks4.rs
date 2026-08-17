use std::{
    borrow::Borrow,
    io,
    net::{Ipv4Addr, SocketAddr},
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::stream::{self, Fuse, Stream, StreamExt};
#[cfg(feature = "tokio")]
use tokio::net::TcpStream;

#[cfg(feature = "tokio")]
use crate::ToProxyAddrs;
use crate::{
    io::{AsyncSocket, AsyncSocketExt},
    Error,
    IntoTargetAddr,
    Result,
    TargetAddr,
};

#[repr(u8)]
#[derive(Clone, Copy)]
enum CommandV4 {
    Connect = 0x01,
    Bind = 0x02,
}

/// A SOCKS4 client.
///
/// For convenience, it can be dereferenced to it's inner socket.
#[derive(Debug)]
pub struct Socks4Stream<S> {
    socket: S,
    target: TargetAddr<'static>,
}

impl<S> Deref for Socks4Stream<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

impl<S> DerefMut for Socks4Stream<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.socket
    }
}

#[cfg(feature = "tokio")]
impl Socks4Stream<TcpStream> {
    /// Connects to a target server through a SOCKS4 proxy given the proxy
    /// address.
    ///
    /// # Error
    ///
    /// It propagates the error that occurs in the conversion from `T` to
    /// `TargetAddr`.
    pub async fn connect<'t, P, T>(proxy: P, target: T) -> Result<Socks4Stream<TcpStream>>
    where
        P: ToProxyAddrs,
        T: IntoTargetAddr<'t>,
    {
        Self::execute_command(proxy, target, None, CommandV4::Connect).await
    }

    /// Connects to a target server through a SOCKS4 proxy using given username,
    /// password and the address of the proxy.
    ///
    /// # Error
    ///
    /// It propagates the error that occurs in the conversion from `T` to
    /// `TargetAddr`.
    pub async fn connect_with_userid<'a, 't, P, T>(
        proxy: P,
        target: T,
        user_id: &'a str,
    ) -> Result<Socks4Stream<TcpStream>>
    where
        P: ToProxyAddrs,
        T: IntoTargetAddr<'t>,
    {
        Self::execute_command(proxy, target, Some(user_id), CommandV4::Connect).await
    }

    async fn execute_command<'a, 't, P, T>(
        proxy: P,
        target: T,
        user_id: Option<&'a str>,
        command: CommandV4,
    ) -> Result<Socks4Stream<TcpStream>>
    where
        P: ToProxyAddrs,
        T: IntoTargetAddr<'t>,
    {
        Self::validate_userid(user_id)?;

        let sock = Socks4Connector::new(
            user_id,
            command,
            proxy.to_proxy_addrs().fuse(),
            target.into_target_addr()?,
        )
        .execute()
        .await?;

        Ok(sock)
    }
}

impl<S> Socks4Stream<S>
where S: AsyncSocket + Unpin
{
    /// Connects to a target server through a SOCKS4 proxy given a socket to it.
    ///
    /// # Error
    ///
    /// It propagates the error that occurs in the conversion from `T` to
    /// `TargetAddr`.
    pub async fn connect_with_socket<'t, T>(socket: S, target: T) -> Result<Socks4Stream<S>>
    where T: IntoTargetAddr<'t> {
        Self::execute_command_with_socket(socket, target, None, CommandV4::Connect).await
    }

    /// Connects to a target server through a SOCKS4 proxy using given username,
    /// password and a socket to the proxy
    ///
    /// # Error
    ///
    /// It propagates the error that occurs in the conversion from `T` to
    /// `TargetAddr`.
    pub async fn connect_with_userid_and_socket<'a, 't, T>(
        socket: S,
        target: T,
        user_id: &'a str,
    ) -> Result<Socks4Stream<S>>
    where
        T: IntoTargetAddr<'t>,
    {
        Self::execute_command_with_socket(socket, target, Some(user_id), CommandV4::Connect).await
    }

    fn validate_userid(user_id: Option<&str>) -> Result<()> {
        // A hardcode limit for length of userid must be enforced to avoid, buffer
        // overflow.
        if let Some(user_id) = user_id {
            let user_id_len = user_id.len();
            if !(1..=255).contains(&user_id_len) {
                Err(Error::InvalidAuthValues("userid length should between 1 to 255"))?
            }
        }

        Ok(())
    }

    async fn execute_command_with_socket<'a, 't, T>(
        socket: S,
        target: T,
        user_id: Option<&'a str>,
        command: CommandV4,
    ) -> Result<Socks4Stream<S>>
    where
        T: IntoTargetAddr<'t>,
    {
        Self::validate_userid(user_id)?;

        let sock = Socks4Connector::new(user_id, command, stream::empty().fuse(), target.into_target_addr()?)
            .execute_with_socket(socket)
            .await?;

        Ok(sock)
    }

    /// Consumes the `Socks4Stream`, returning the inner socket.
    pub fn into_inner(self) -> S {
        self.socket
    }

    /// Returns the target address that the proxy server connects to.
    pub fn target_addr(&self) -> TargetAddr<'_> {
        match &self.target {
            TargetAddr::Ip(addr) => TargetAddr::Ip(*addr),
            TargetAddr::Domain(domain, port) => {
                let domain: &str = domain.borrow();
                TargetAddr::Domain(domain.into(), *port)
            },
        }
    }
}

/// A `Future` which resolves to a socket to the target server through proxy.
pub struct Socks4Connector<'a, 't, S> {
    user_id: Option<&'a str>,
    command: CommandV4,
    #[allow(dead_code)]
    proxy: Fuse<S>,
    target: TargetAddr<'t>,
    buf: [u8; 513],
    ptr: usize,
    len: usize,
}

impl<'a, 't, S> Socks4Connector<'a, 't, S>
where S: Stream<Item = Result<SocketAddr>> + Unpin
{
    fn new(user_id: Option<&'a str>, command: CommandV4, proxy: Fuse<S>, target: TargetAddr<'t>) -> Self {
        Socks4Connector {
            user_id,
            command,
            proxy,
            target,
            buf: [0; 513],
            ptr: 0,
            len: 0,
        }
    }

    #[cfg(feature = "tokio")]
    /// Connect to the proxy server, authenticate and issue the SOCKS command
    pub async fn execute(&mut self) -> Result<Socks4Stream<TcpStream>> {
        let next_addr = self.proxy.select_next_some().await?;
        let tcp = TcpStream::connect(next_addr)
            .await
            .map_err(|_| Error::ProxyServerUnreachable)?;

        self.execute_with_socket(tcp).await
    }

    pub async fn execute_with_socket<T: AsyncSocket + Unpin>(&mut self, mut socket: T) -> Result<Socks4Stream<T>> {
        // Send request address that should be proxied
        self.prepare_send_request()?;
        socket.write_all(&self.buf[self.ptr..self.len]).await?;

        let target = self.receive_reply(&mut socket).await?;

        Ok(Socks4Stream { socket, target })
    }

    fn prepare_send_request(&mut self) -> Result<()> {
        self.ptr = 0;
        self.buf[..2].copy_from_slice(&[0x04, self.command as u8]);
        match &self.target {
            TargetAddr::Ip(SocketAddr::V4(addr)) => {
                self.buf[2..4].copy_from_slice(&addr.port().to_be_bytes());
                self.buf[4..8].copy_from_slice(&addr.ip().octets());
                self.len = 8;
                if let Some(user_id) = self.user_id {
                    let usr_byts = user_id.as_bytes();
                    let user_id_len = usr_byts.len();
                    self.len += user_id_len;
                    self.buf[8..self.len].copy_from_slice(usr_byts);
                }
                self.buf[self.len] = 0; // null terminator
                self.len += 1;
            },
            TargetAddr::Ip(SocketAddr::V6(_)) => {
                return Err(Error::AddressTypeNotSupported);
            },
            TargetAddr::Domain(domain, port) => {
                self.buf[2..4].copy_from_slice(&port.to_be_bytes());
                self.buf[4..8].copy_from_slice(&[0, 0, 0, 1]);
                self.len = 8;
                if let Some(user_id) = self.user_id {
                    let usr_byts = user_id.as_bytes();
                    let user_id_len = usr_byts.len();
                    self.len += user_id_len;
                    self.buf[8..self.len].copy_from_slice(usr_byts);
                }
                self.buf[self.len] = 0; // null terminator
                self.len += 1;
                let domain = domain.as_bytes();
                let domain_len = domain.len();
                self.buf[self.len..self.len + domain_len].copy_from_slice(domain);
                self.len += domain_len;
                self.buf[self.len] = 0;
                self.len += 1;
            },
        };
        Ok(())
    }

    fn prepare_recv_reply(&mut self) {
        self.ptr = 0;
        self.len = 8;
    }

    async fn receive_reply<T: AsyncSocket + Unpin>(&mut self, tcp: &mut T) -> Result<TargetAddr<'static>> {
        // https://www.openssh.com/txt/socks4.protocol
        // +----+----+----+----+----+----+----+----+
        // | VN | CD | DSTPORT |      DSTIP        |
        // +----+----+----+----+----+----+----+----+
        // # of bytes:	   1    1      2              4
        //
        // VN is the version of the reply code and should be 0. CD is the result
        // code with one of the following values:
        // 90: request granted
        // 91: request rejected or failed
        // 92: request rejected becasue SOCKS server cannot connect to
        // identd on the client
        // 93: request rejected because the client program and identd
        // report different user-ids

        self.prepare_recv_reply();
        self.ptr += tcp.read_exact(&mut self.buf[self.ptr..self.len]).await?;
        if self.buf[0] != 0 {
            return Err(Error::InvalidResponseVersion);
        }

        match self.buf[1] {
            0x5A => {},                                           // request granted
            0x5B => return Err(Error::GeneralSocksServerFailure), // connection rejected/failed
            0x5C => return Err(Error::IdentdAuthFailure),         // cannot connect to identd on the client
            0x5D => return Err(Error::InvalidUserIdAuthFailure),  // different user-ids
            _ => return Err(Error::UnknownError),
        }

        let port = u16::from_be_bytes([self.buf[2], self.buf[3]]);

        let target = Ipv4Addr::from([self.buf[4], self.buf[5], self.buf[6], self.buf[7]]);

        Ok(TargetAddr::Ip(SocketAddr::new(target.into(), port)))
    }
}

/// A SOCKS4 BIND client.
///
/// Once you get an instance of `Socks4Listener`, you should send the
/// `bind_addr` to the remote process via the primary connection. Then, call the
/// `accept` function and wait for the other end connecting to the rendezvous
/// address.
pub struct Socks4Listener<S> {
    inner: Socks4Stream<S>,
}

#[cfg(feature = "tokio")]
impl Socks4Listener<TcpStream> {
    /// Initiates a BIND request to the specified proxy.
    ///
    /// The proxy will filter incoming connections based on the value of
    /// `target`.
    ///
    /// # Error
    ///
    /// It propagates the error that occurs in the conversion from `T` to
    /// `TargetAddr`.
    pub async fn bind<'t, P, T>(proxy: P, target: T) -> Result<Socks4Listener<TcpStream>>
    where
        P: ToProxyAddrs,
        T: IntoTargetAddr<'t>,
    {
        Self::bind_to_target(None, proxy, target).await
    }

    /// Initiates a BIND request to the specified proxy using given username
    /// and password.
    ///
    /// The proxy will filter incoming connections based on the value of
    /// `target`.
    ///
    /// # Error
    ///
    /// It propagates the error that occurs in the conversion from `T` to
    /// `TargetAddr`.
    pub async fn bind_with_userid<'a, 't, P, T>(
        proxy: P,
        target: T,
        user_id: &'a str,
    ) -> Result<Socks4Listener<TcpStream>>
    where
        P: ToProxyAddrs,
        T: IntoTargetAddr<'t>,
    {
        Self::bind_to_target(Some(user_id), proxy, target).await
    }

    async fn bind_to_target<'a, 't, P, T>(
        user_id: Option<&'a str>,
        proxy: P,
        target: T,
    ) -> Result<Socks4Listener<TcpStream>>
    where
        P: ToProxyAddrs,
        T: IntoTargetAddr<'t>,
    {
        let socket = Socks4Connector::new(
            user_id,
            CommandV4::Bind,
            proxy.to_proxy_addrs().fuse(),
            target.into_target_addr()?,
        )
        .execute()
        .await?;

        Ok(Socks4Listener { inner: socket })
    }
}

impl<S> Socks4Listener<S>
where S: AsyncSocket + Unpin
{
    /// Initiates a BIND request to the specified proxy using the given socket
    /// to it.
    ///
    /// The proxy will filter incoming connections based on the value of
    /// `target`.
    ///
    /// # Error
    ///
    /// It propagates the error that occurs in the conversion from `T` to
    /// `TargetAddr`.
    pub async fn bind_with_socket<'t, T>(socket: S, target: T) -> Result<Socks4Listener<S>>
    where T: IntoTargetAddr<'t> {
        Self::bind_to_target_with_socket(None, socket, target).await
    }

    /// Initiates a BIND request to the specified proxy using given username,
    /// password and socket to the proxy.
    ///
    /// The proxy will filter incoming connections based on the value of
    /// `target`.
    ///
    /// # Error
    ///
    /// It propagates the error that occurs in the conversion from `T` to
    /// `TargetAddr`.
    pub async fn bind_with_user_and_socket<'a, 't, T>(
        socket: S,
        target: T,
        user_id: &'a str,
    ) -> Result<Socks4Listener<S>>
    where
        T: IntoTargetAddr<'t>,
    {
        Self::bind_to_target_with_socket(Some(user_id), socket, target).await
    }

    async fn bind_to_target_with_socket<'a, 't, T>(
        auth: Option<&'a str>,
        socket: S,
        target: T,
    ) -> Result<Socks4Listener<S>>
    where
        T: IntoTargetAddr<'t>,
    {
        let socket = Socks4Connector::new(
            auth,
            CommandV4::Bind,
            stream::empty().fuse(),
            target.into_target_addr()?,
        )
        .execute_with_socket(socket)
        .await?;

        Ok(Socks4Listener { inner: socket })
    }

    /// Returns the address of the proxy-side TCP listener.
    ///
    /// This should be forwarded to the remote process, which should open a
    /// connection to it.
    pub fn bind_addr(&self) -> TargetAddr {
        self.inner.target_addr()
    }

    /// Consumes this listener, returning a `Future` which resolves to the
    /// `Socks4Stream` connected to the target server through the proxy.
    ///
    /// The value of `bind_addr` should be forwarded to the remote process
    /// before this method is called.
    pub async fn accept(mut self) -> Result<Socks4Stream<S>> {
        let mut connector = Socks4Connector {
            user_id: None,
            command: CommandV4::Bind,
            proxy: stream::empty().fuse(),
            target: self.inner.target,
            buf: [0; 513],
            ptr: 0,
            len: 0,
        };

        let target = connector.receive_reply(&mut self.inner.socket).await?;

        Ok(Socks4Stream {
            socket: self.inner.socket,
            target,
        })
    }
}

#[cfg(feature = "tokio")]
impl<T> tokio::io::AsyncRead for Socks4Stream<T>
where T: tokio::io::AsyncRead + Unpin
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        tokio::io::AsyncRead::poll_read(Pin::new(&mut self.socket), cx, buf)
    }
}

#[cfg(feature = "tokio")]
impl<T> tokio::io::AsyncWrite for Socks4Stream<T>
where T: tokio::io::AsyncWrite + Unpin
{
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        tokio::io::AsyncWrite::poll_write(Pin::new(&mut self.socket), cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        tokio::io::AsyncWrite::poll_flush(Pin::new(&mut self.socket), cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        tokio::io::AsyncWrite::poll_shutdown(Pin::new(&mut self.socket), cx)
    }
}

#[cfg(feature = "futures-io")]
impl<T> futures_io::AsyncRead for Socks4Stream<T>
where T: futures_io::AsyncRead + Unpin
{
    fn poll_read(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        futures_io::AsyncRead::poll_read(Pin::new(&mut self.socket), cx, buf)
    }
}

#[cfg(feature = "futures-io")]
impl<T> futures_io::AsyncWrite for Socks4Stream<T>
where T: futures_io::AsyncWrite + Unpin
{
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        futures_io::AsyncWrite::poll_write(Pin::new(&mut self.socket), cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        futures_io::AsyncWrite::poll_flush(Pin::new(&mut self.socket), cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        futures_io::AsyncWrite::poll_close(Pin::new(&mut self.socket), cx)
    }
}
