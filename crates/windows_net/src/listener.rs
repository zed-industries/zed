use std::{net::SocketAddr, os::windows::io::AsRawSocket, path::Path, sync::Arc};

use smol::Async;
use windows::Win32::Networking::WinSock::{bind, listen, SOCKADDR_UN, SOMAXCONN};

use crate::{
    init, map_ret, sockaddr_un,
    socket::{RawSocket, UnixSocketAddr},
    stream::{RawUnixStream, UnixStream},
    util::AsyncExt,
};

pub struct UnixListener(Arc<Async<RawListener>>);

impl UnixListener {
    fn new(raw: Arc<Async<RawListener>>) -> Self {
        Self(raw)
    }

    pub fn bind<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let raw = RawListener::bind(path)?;
        Ok(Self::new(Arc::new(Async::new(raw)?)))
    }

    pub async fn accept(&self) -> std::io::Result<(UnixStream, UnixSocketAddr)> {
        let (socket, addr) = self.0.accept().await?;
        Ok((UnixStream::new(Arc::new(socket)), addr))
    }
}

struct RawListener(RawSocket);

impl RawListener {
    fn bind<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        init();
        let socket = RawSocket::new()?;
        let (addr, len) = sockaddr_un(path.as_ref())?;
        unsafe {
            map_ret(bind(
                *socket.as_raw(),
                &addr as *const _ as *const _,
                len as i32,
            ))?
        };
        unsafe { map_ret(listen(*socket.as_raw(), SOMAXCONN as _))? };
        Ok(Self(socket))
    }

    fn accept(&self) -> std::io::Result<(RawUnixStream, UnixSocketAddr)> {
        let mut storage = SOCKADDR_UN::default();
        let mut len = std::mem::size_of_val(&storage) as i32;
        let raw = self.0.accept(&mut storage as *mut _ as *mut _, &mut len)?;
        let addr = UnixSocketAddr::from_parts(storage, len)?;
        Ok((RawUnixStream::new(raw), addr))
    }
}

impl AsRawSocket for RawListener {
    fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
        self.0.as_raw().0 as _
    }
}

impl AsyncExt for Async<RawListener> {
    async fn accept(&self) -> std::io::Result<(Async<RawUnixStream>, UnixSocketAddr)> {
        let (stream, addr) = self.read_with(|io| io.accept()).await?;
        Ok((Async::new(stream)?, addr))
    }
}
