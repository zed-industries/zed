#[cfg(feature = "tokio-runtime")]
use tokio::net::{UnixListener, UnixStream};

#[cfg(any(feature = "async-std-runtime", feature = "async-dispatcher-runtime"))]
use async_std::os::unix::net::{UnixListener, UnixStream};

use super::make_framed;
use super::AcceptStopHandle;
use crate::async_rt;
use crate::codec::FramedIo;
use crate::endpoint::Endpoint;
use crate::task_handle::TaskHandle;
use crate::ZmqResult;

use futures::channel::oneshot;
use futures::{select, FutureExt};

use std::path::Path;

pub(crate) async fn connect(path: &Path) -> ZmqResult<(FramedIo, Endpoint)> {
    let raw_socket = UnixStream::connect(path).await?;
    let peer_addr = raw_socket.peer_addr()?;
    let peer_addr = peer_addr.as_pathname().map(|a| a.to_owned());

    Ok((make_framed(raw_socket), Endpoint::Ipc(peer_addr)))
}

pub(crate) async fn begin_accept<T>(
    path: &Path,
    cback: impl Fn(ZmqResult<(FramedIo, Endpoint)>) -> T + Send + 'static,
) -> ZmqResult<(Endpoint, AcceptStopHandle)>
where
    T: std::future::Future<Output = ()> + Send + 'static,
{
    let wildcard: &Path = "*".as_ref();
    if path == wildcard {
        todo!("Need to implement support for wildcard paths!");
    }

    #[cfg(feature = "tokio-runtime")]
    let listener = UnixListener::bind(path)?;
    #[cfg(any(feature = "async-std-runtime", feature = "async-dispatcher-runtime"))]
    let listener = UnixListener::bind(path).await?;

    let resolved_addr = listener.local_addr()?;
    let resolved_addr = resolved_addr.as_pathname().map(|a| a.to_owned());
    let listener_addr = resolved_addr.clone();
    let (stop_channel, stop_callback) = oneshot::channel::<()>();
    let task_handle = async_rt::task::spawn(async move {
        let mut stop_callback = stop_callback.fuse();
        loop {
            select! {
                incoming = listener.accept().fuse() => {
                    let maybe_accepted: Result<_, _> = incoming.map(|(raw_socket, peer_addr)| {
                        let peer_addr = peer_addr.as_pathname().map(|a| a.to_owned());
                        (make_framed(raw_socket), Endpoint::Ipc(peer_addr))
                    }).map_err(|err| err.into());
                    async_rt::task::spawn(cback(maybe_accepted));
                },
                _ = stop_callback => {
                    log::debug!("Accept task received stop signal. {:?}", listener_addr);
                    break
                }
            }
        }
        drop(listener);
        if let Some(listener_addr) = listener_addr {
            #[cfg(any(feature = "async-std-runtime", feature = "async-dispatcher-runtime"))]
            use async_std::fs::remove_file;
            #[cfg(feature = "tokio-runtime")]
            use tokio::fs::remove_file;

            if let Err(err) = remove_file(&listener_addr).await {
                log::warn!(
                    "Could not delete unix socket at {}: {}",
                    listener_addr.display(),
                    err
                );
            }
        }
        Ok(())
    });
    Ok((
        Endpoint::Ipc(resolved_addr),
        AcceptStopHandle(TaskHandle::new(stop_channel, task_handle)),
    ))
}
