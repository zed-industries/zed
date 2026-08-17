//! Functions to spawn a [`neovim`](crate::neovim::Neovim) session using
//! [`async-std`](async-std)
use std::{future::Future, io};

#[cfg(unix)]
use async_std::os::unix::net::UnixStream;

use async_std::{
  fs::File as ASFile,
  io::stdin,
  net::{TcpStream, ToSocketAddrs},
  task::{spawn, JoinHandle},
};

#[cfg(unix)]
use async_std::path::Path;

use futures::io::{AsyncReadExt, WriteHalf};

use crate::{
  create::{unbuffered_stdout, Spawner},
  error::LoopError,
  neovim::Neovim,
  Handler,
};

impl<H> Spawner for H
where
  H: Handler,
{
  type Handle = JoinHandle<()>;

  fn spawn<Fut>(&self, future: Fut) -> Self::Handle
  where
    Fut: Future<Output = ()> + Send + 'static,
  {
    spawn(future)
  }
}

/// Connect to a neovim instance via tcp
pub async fn new_tcp<A, H>(
  addr: A,
  handler: H,
) -> io::Result<(
  Neovim<WriteHalf<TcpStream>>,
  JoinHandle<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = WriteHalf<TcpStream>>,
  A: ToSocketAddrs,
{
  let stream = TcpStream::connect(addr).await?;
  let (reader, writer) = stream.split();
  let (neovim, io) =
    Neovim::<WriteHalf<TcpStream>>::new(reader, writer, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

#[cfg(unix)]
/// Connect to a neovim instance via unix socket by path. This is currently
/// only available on Unix for async-std.
pub async fn new_path<H, P: AsRef<Path> + Clone>(
  path: P,
  handler: H,
) -> io::Result<(
  Neovim<WriteHalf<UnixStream>>,
  JoinHandle<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = WriteHalf<UnixStream>> + Send + 'static,
{
  let stream = UnixStream::connect(path).await?;
  let (reader, writer) = stream.split();
  let (neovim, io) =
    Neovim::<WriteHalf<UnixStream>>::new(reader, writer, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

/// Connect to the neovim instance that spawned this process over stdin/stdout
pub async fn new_parent<H>(
  handler: H,
) -> io::Result<(Neovim<ASFile>, JoinHandle<Result<(), Box<LoopError>>>)>
where
  H: Handler<Writer = ASFile>,
{
  let sout: ASFile = unbuffered_stdout()?.into();
  let (neovim, io) = Neovim::<ASFile>::new(stdin(), sout, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}
