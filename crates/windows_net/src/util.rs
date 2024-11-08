use smol::Async;

use crate::{socket::UnixSocketAddr, stream::RawUnixStream};

pub(crate) trait AsyncExt {
    async fn accept(&self) -> std::io::Result<(Async<RawUnixStream>, UnixSocketAddr)>;
}
