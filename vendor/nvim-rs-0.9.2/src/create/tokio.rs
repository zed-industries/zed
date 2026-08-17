//! Functions to spawn a [`neovim`](crate::neovim::Neovim) session using
//! [`tokio`](tokio)
use std::{
  future::Future,
  io::{self, Error, ErrorKind},
  path::Path,
  process::Stdio,
};

use tokio::{
  fs::File as TokioFile,
  io::{split, stdin, WriteHalf},
  net::{TcpStream, ToSocketAddrs},
  process::{Child, ChildStdin, Command},
  spawn,
  task::JoinHandle,
};

#[cfg(unix)]
type Connection = tokio::net::UnixStream;
#[cfg(windows)]
type Connection = tokio::net::windows::named_pipe::NamedPipeClient;

use tokio_util::compat::{
  Compat, TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt,
};

use crate::{
  create::{unbuffered_stdout, Spawner},
  error::{HandshakeError, LoopError},
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
  Neovim<Compat<WriteHalf<TcpStream>>>,
  JoinHandle<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = Compat<WriteHalf<TcpStream>>>,
  A: ToSocketAddrs,
{
  let stream = TcpStream::connect(addr).await?;
  let (reader, writer) = split(stream);
  let (neovim, io) = Neovim::<Compat<WriteHalf<TcpStream>>>::new(
    reader.compat(),
    writer.compat_write(),
    handler,
  );
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

/// Connect to a neovim instance via unix socket (Unix) or named pipe (Windows)
pub async fn new_path<H, P: AsRef<Path> + Clone>(
  path: P,
  handler: H,
) -> io::Result<(
  Neovim<Compat<WriteHalf<Connection>>>,
  JoinHandle<Result<(), Box<LoopError>>>,
)>
where
  H: Handler<Writer = Compat<WriteHalf<Connection>>> + Send + 'static,
{
  let stream = {
    #[cfg(unix)]
    {
      use tokio::net::UnixStream;

      UnixStream::connect(path).await?
    }
    #[cfg(windows)]
    {
      use std::time::Duration;
      use tokio::net::windows::named_pipe::ClientOptions;
      use tokio::time;

      // From windows-sys so we don't have to depend on that for just this constant
      // https://docs.rs/windows-sys/latest/windows_sys/Win32/Foundation/constant.ERROR_PIPE_BUSY.html
      pub const ERROR_PIPE_BUSY: i32 = 231i32;

      // Based on the example in the tokio docs, see explanation there
      // https://docs.rs/tokio/latest/tokio/net/windows/named_pipe/struct.NamedPipeClient.html
      let client = loop {
        match ClientOptions::new().open(path.as_ref()) {
          Ok(client) => break client,
          Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) => (),
          Err(e) => return Err(e),
        }

        time::sleep(Duration::from_millis(50)).await;
      };

      client
    }
  };
  let (reader, writer) = split(stream);
  let (neovim, io) = Neovim::<Compat<WriteHalf<Connection>>>::new(
    reader.compat(),
    writer.compat_write(),
    handler,
  );
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

/// Connect to a neovim instance by spawning a new one
pub async fn new_child<H>(
  handler: H,
) -> io::Result<(
  Neovim<Compat<ChildStdin>>,
  JoinHandle<Result<(), Box<LoopError>>>,
  Child,
)>
where
  H: Handler<Writer = Compat<ChildStdin>> + Send + 'static,
{
  if cfg!(target_os = "windows") {
    new_child_path("nvim.exe", handler).await
  } else {
    new_child_path("nvim", handler).await
  }
}

/// Connect to a neovim instance by spawning a new one
pub async fn new_child_path<H, S: AsRef<Path>>(
  program: S,
  handler: H,
) -> io::Result<(
  Neovim<Compat<ChildStdin>>,
  JoinHandle<Result<(), Box<LoopError>>>,
  Child,
)>
where
  H: Handler<Writer = Compat<ChildStdin>> + Send + 'static,
{
  new_child_cmd(Command::new(program.as_ref()).arg("--embed"), handler).await
}

/// Connect to a neovim instance by spawning a new one
///
/// stdin/stdout will be rewritten to `Stdio::piped()`
pub async fn new_child_cmd<H>(
  cmd: &mut Command,
  handler: H,
) -> io::Result<(
  Neovim<Compat<ChildStdin>>,
  JoinHandle<Result<(), Box<LoopError>>>,
  Child,
)>
where
  H: Handler<Writer = Compat<ChildStdin>> + Send + 'static,
{
  let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
  let stdout = child
    .stdout
    .take()
    .ok_or_else(|| Error::new(ErrorKind::Other, "Can't open stdout"))?
    .compat();
  let stdin = child
    .stdin
    .take()
    .ok_or_else(|| Error::new(ErrorKind::Other, "Can't open stdin"))?
    .compat_write();

  let (neovim, io) = Neovim::<Compat<ChildStdin>>::new(stdout, stdin, handler);
  let io_handle = spawn(io);

  Ok((neovim, io_handle, child))
}

/// Connect to the neovim instance that spawned this process over stdin/stdout
pub async fn new_parent<H>(
  handler: H,
) -> Result<
  (
    Neovim<Compat<tokio::fs::File>>,
    JoinHandle<Result<(), Box<LoopError>>>,
  ),
  Error,
>
where
  H: Handler<Writer = Compat<tokio::fs::File>>,
{
  let sout = TokioFile::from_std(unbuffered_stdout()?);

  let (neovim, io) = Neovim::<Compat<tokio::fs::File>>::new(
    stdin().compat(),
    sout.compat(),
    handler,
  );
  let io_handle = spawn(io);

  Ok((neovim, io_handle))
}

/// Connect to a neovim instance by spawning a new one and send a handshake
/// message. Unlike `new_child_cmd`, this function is tolerant to extra
/// data in the reader before the handshake response is received.
///
/// `message` should be a unique string that is normally not found in the
/// stdout. Due to the way Neovim packs strings, the length has to be either
/// less than 20 characters or more than 31 characters long.
/// See https://github.com/neovim/neovim/issues/32784 for more information.
pub async fn new_child_handshake_cmd<H>(
  cmd: &mut Command,
  handler: H,
  message: &str,
) -> Result<
  (
    Neovim<Compat<ChildStdin>>,
    JoinHandle<Result<(), Box<LoopError>>>,
    Child,
  ),
  Box<HandshakeError>,
>
where
  H: Handler<Writer = Compat<ChildStdin>> + Send + 'static,
{
  let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
  let stdout = child
    .stdout
    .take()
    .ok_or_else(|| Error::new(ErrorKind::Other, "Can't open stdout"))?
    .compat();
  let stdin = child
    .stdin
    .take()
    .ok_or_else(|| Error::new(ErrorKind::Other, "Can't open stdin"))?
    .compat_write();

  let (neovim, io) =
    Neovim::<Compat<ChildStdin>>::handshake(stdout, stdin, handler, message)
      .await?;
  let io_handle = spawn(io);

  Ok((neovim, io_handle, child))
}
