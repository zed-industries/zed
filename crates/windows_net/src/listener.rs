use std::{os::windows::io::AsRawSocket, path::Path, sync::Arc};

use smol::Async;
use windows::Win32::Networking::WinSock::{
    bind, getsockname, listen, SOCKADDR_UN, SOCKET, SOMAXCONN,
};

use crate::{
    init, map_ret, sockaddr_un,
    socket::{UnixSocketAddr, WindowsSocket},
    stream::UnixStream,
    util::AsyncExt,
};

// pub struct UnixListener(Arc<Async<uds_windows::UnixListener>>);

// impl UnixListener {
//     fn new(raw: Arc<Async<uds_windows::UnixListener>>) -> Self {
//         Self(raw)
//     }

//     pub fn bind<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
//         let raw = uds_windows::UnixListener::bind(path)?;
//         Ok(Self::new(Arc::new(Async::new(raw)?)))
//     }

//     pub async fn accept(&self) -> std::io::Result<(UnixStream, uds_windows::SocketAddr)> {
//         let (socket, addr) = self.0.accept().await?;
//         Ok((UnixStream::new(Arc::new(socket)), addr))
//     }

//     pub fn local_addr(&self) -> std::io::Result<uds_windows::SocketAddr> {
//         self.0.get_ref().local_addr()
//     }
// }

pub struct UnixListener(WindowsListener);

impl UnixListener {
    pub fn bind<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        Ok(Self(WindowsListener::bind(path)?))
    }

    pub async fn accept(&self) -> std::io::Result<(UnixStream, UnixSocketAddr)> {
        let socket = *self.0.as_raw().as_raw();
        smol::unblock(move || WindowsListener::static_accept(socket)).await
    }
}

struct WindowsListener(WindowsSocket);

impl WindowsListener {
    fn bind<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        init();
        let socket = WindowsSocket::new()?;
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

    fn accept(&self) -> std::io::Result<(UnixStream, UnixSocketAddr)> {
        let socket = *self.0.as_raw();
        Self::static_accept(socket)
    }

    fn static_accept(socket: SOCKET) -> std::io::Result<(UnixStream, UnixSocketAddr)> {
        let mut storage = SOCKADDR_UN::default();
        let mut len = std::mem::size_of_val(&storage) as i32;
        let raw = WindowsSocket::static_accept(socket, &mut storage as *mut _ as *mut _, &mut len)?;
        let addr = UnixSocketAddr::from_parts(storage, len)?;
        Ok((UnixStream::new(raw), addr))
    }

    fn as_raw(&self) -> &WindowsSocket {
        &self.0
    }
}

// impl AsRawSocket for WindowsListener {
//     fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
//         self.0.as_raw().0 as _
//     }
// }

// impl AsyncExt for Async<WindowsListener> {
//     async fn accept(&self) -> std::io::Result<(Async<WindowsStream>, UnixSocketAddr)> {
//         let (stream, addr) = self.read_with(|io| io.accept()).await?;
//         Ok((Async::new(stream)?, addr))
//     }
// }

// impl AsyncExt for Async<uds_windows::UnixListener> {
//     async fn accept(
//         &self,
//     ) -> std::io::Result<(Async<uds_windows::UnixStream>, uds_windows::SocketAddr)> {
//         let (stream, addr) = self.read_with(|io| io.accept()).await?;
//         Ok((Async::new(stream)?, addr))
//     }
// }
