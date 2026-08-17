use std::{io, os::fd::BorrowedFd};

#[cfg(not(feature = "tokio"))]
use async_process::{Child, ChildStdin, ChildStdout};

#[cfg(feature = "tokio")]
use tokio::{
    io::{AsyncReadExt, ReadBuf},
    process::{Child, ChildStdin, ChildStdout},
};

use super::{ReadHalf, RecvmsgResult, Socket, Split, WriteHalf};

/// A Command stream socket.
///
/// This socket communicates with a spawned child process via its standard input
/// and output streams.
#[derive(Debug)]
pub(crate) struct Command {
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl Command {
    fn into_split(self) -> (ChildStdout, ChildStdin) {
        (self.stdout, self.stdin)
    }
}

impl Socket for Command {
    type ReadHalf = ChildStdout;
    type WriteHalf = ChildStdin;

    fn split(self) -> Split<Self::ReadHalf, Self::WriteHalf> {
        let (read, write) = self.into_split();

        Split { read, write }
    }
}

impl TryFrom<&mut Child> for Command {
    type Error = crate::Error;

    fn try_from(child: &mut Child) -> Result<Self, Self::Error> {
        let stdin = child
            .stdin
            .take()
            .ok_or(crate::Error::Failure("child stdin not found".into()))?;

        let stdout = child
            .stdout
            .take()
            .ok_or(crate::Error::Failure("child stdout not found".into()))?;

        Ok(Command { stdin, stdout })
    }
}

#[cfg(not(feature = "tokio"))]
#[async_trait::async_trait]
impl ReadHalf for ChildStdout {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult {
        match futures_lite::AsyncReadExt::read(&mut self, buf).await {
            Err(e) => Err(e),
            Ok(len) => {
                #[cfg(unix)]
                let ret = (len, vec![]);
                #[cfg(not(unix))]
                let ret = len;
                Ok(ret)
            }
        }
    }
}

#[cfg(feature = "tokio")]
#[async_trait::async_trait]
impl ReadHalf for ChildStdout {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult {
        let mut read_buf = ReadBuf::new(buf);
        self.read_buf(&mut read_buf).await.map(|_| {
            let ret = read_buf.filled().len();
            #[cfg(unix)]
            let ret = (ret, vec![]);

            ret
        })
    }
}

#[cfg(not(feature = "tokio"))]
#[async_trait::async_trait]
impl WriteHalf for ChildStdin {
    async fn sendmsg(
        &mut self,
        buf: &[u8],
        #[cfg(unix)] fds: &[BorrowedFd<'_>],
    ) -> io::Result<usize> {
        #[cfg(unix)]
        if !fds.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a command stream",
            ));
        }

        futures_lite::AsyncWriteExt::write(&mut self, buf).await
    }

    async fn close(&mut self) -> io::Result<()> {
        futures_lite::AsyncWriteExt::close(&mut self).await
    }
}

#[cfg(feature = "tokio")]
#[async_trait::async_trait]
impl WriteHalf for ChildStdin {
    async fn sendmsg(
        &mut self,
        buf: &[u8],
        #[cfg(unix)] fds: &[BorrowedFd<'_>],
    ) -> io::Result<usize> {
        #[cfg(unix)]
        if !fds.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a command stream",
            ));
        }

        tokio::io::AsyncWriteExt::write(&mut self, buf).await
    }

    async fn close(&mut self) -> io::Result<()> {
        tokio::io::AsyncWriteExt::shutdown(&mut self).await
    }
}
