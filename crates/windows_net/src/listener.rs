use std::{os::windows::io::AsRawSocket, path::Path, sync::Arc};

use smol::Async;
use windows::Win32::Networking::WinSock::{bind, listen, SOMAXCONN};

use crate::{init, map_ret, sockaddr_un, socket::RawSocket};

pub struct Listener(Arc<Async<RawListener>>);

impl Listener {
    fn new(raw: Arc<Async<RawListener>>) -> Self {
        Self(raw)
    }

    pub fn bind<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let raw = RawListener::bind(path)?;
        Ok(Self::new(Arc::new(Async::new(raw)?)))
    }
}

struct RawListener(RawSocket);

impl RawListener {
    pub fn bind<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
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
}

impl AsRawSocket for RawListener {
    fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
        self.0.as_raw().0 as _
    }
}
