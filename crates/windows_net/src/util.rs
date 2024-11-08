use smol::Async;

// use crate::{socket::UnixSocketAddr, stream::WindowsStream};

pub(crate) trait AsyncExt {
    async fn accept(
        &self,
    ) -> std::io::Result<(Async<uds_windows::UnixStream>, uds_windows::SocketAddr)>;
}
