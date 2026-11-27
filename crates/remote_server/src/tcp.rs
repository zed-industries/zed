use anyhow::{Context as _, Result, bail};
use futures::{
    AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _, FutureExt as _, select_biased,
};
use smol::net::TcpStream;
#[cfg(unix)]
use std::os::fd::{AsFd as _, AsRawFd as _, BorrowedFd};
use std::{future::Future, io, io::Write as _, net::SocketAddr, process::Stdio, time::Duration};
use util::{ResultExt as _, process::Child};

#[expect(
    clippy::disallowed_methods,
    reason = "This stdio helper runs without a GPUI app or dispatcher"
)]
pub(crate) async fn proxy<Connect>(
    address: SocketAddr,
    timeout: Duration,
    command: &[String],
    connect: impl Fn(SocketAddr) -> Connect,
    mut input: impl AsyncRead + Unpin,
    mut output: impl AsyncWrite + Unpin,
    disconnected: impl Future<Output = io::Result<()>>,
) -> Result<()>
where
    Connect: Future<Output = io::Result<TcpStream>>,
{
    let mut process = AdapterProcess(None);
    let result = async {
        let mut startup_input = Vec::new();
        let startup = async {
            if let Some((program, arguments)) = command.split_first() {
                let mut command = util::command::new_std_command(program);
                command.args(arguments);
                // Adapter logs must not be mixed with the DAP messages on stdout.
                process.0 = Some(Child::spawn(
                    command,
                    Stdio::null(),
                    Stdio::from(io::stderr()),
                    Stdio::inherit(),
                )?);
            }

            let connect = async {
                loop {
                    if let Ok(stream) = connect(address).await {
                        return stream;
                    }
                    smol::Timer::after(Duration::from_millis(100)).await;
                }
            };
            let exited = async {
                match &mut process.0 {
                    Some(child) => child.status().await,
                    None => futures::future::pending().await,
                }
            };
            let exited = exited.fuse();
            let connect = connect.fuse();
            let deadline = smol::Timer::after(timeout).fuse();
            futures::pin_mut!(exited, connect, deadline);
            let mut buffer = [0; 8192];
            loop {
                select_biased! {
                    status = exited => {
                        let status = status?;
                        bail!("Debug adapter exited before connecting to {address}: {status}")
                    }
                    stream = connect => break Ok(Some(stream)),
                    _ = deadline => bail!("Connection to TCP DAP timeout {address}"),
                    length = input.read(&mut buffer).fuse() => {
                        let length = length.context("Reading DAP input during startup")?;
                        if length == 0 {
                            break Ok(None);
                        }
                        startup_input.extend_from_slice(&buffer[..length]);
                    }
                }
            }
        }
        .await;
        let startup = match startup {
            Ok(Some(stream)) => Ok(stream),
            Ok(None) => return Ok(()),
            Err(error) => Err(error),
        };

        // A leading newline separates the status even if adapter logs lack a newline.
        let error = startup.as_ref().err().map(|error| format!("{error:#}"));
        let status = serde_json::to_string(&error)?;
        let message = format!("\n{}{status}\n", proto::DAP_TCP_PROXY_STARTUP_PREFIX);
        io::stderr()
            .lock()
            .write_all(message.as_bytes())
            .context("Reporting TCP debug adapter startup")?;

        let stream = startup?;
        let (mut reader, mut writer) = stream.split();
        select_biased! {
            result = forward(startup_input.as_slice().chain(input), &mut writer).fuse() => {
                result.context("Forwarding DAP input")?;
            }
            result = forward(&mut reader, &mut output).fuse() => {
                result.context("Forwarding DAP output")?;
            }
            result = disconnected.fuse() => {
                result.context("Watching DAP client disconnect")?;
            }
        }
        Ok(())
    }
    .await;

    if let Some(mut child) = process.0.take() {
        child.kill().log_err();
        child.status().await.context("Waiting for debug adapter")?;
    }
    result
}

pub(crate) async fn stdin_closed() -> io::Result<()> {
    #[cfg(unix)]
    {
        input_closed(io::stdin().as_fd()).await
    }
    #[cfg(not(unix))]
    futures::future::pending().await
}

#[cfg(unix)]
#[expect(
    clippy::disallowed_methods,
    reason = "This stdio helper runs without a GPUI app or dispatcher"
)]
async fn input_closed(input: BorrowedFd<'_>) -> io::Result<()> {
    let mut descriptor = libc::pollfd {
        fd: input.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    loop {
        if unsafe { libc::poll(&mut descriptor, 1, 0) } < 0 {
            let error = io::Error::last_os_error();
            if error.kind() != io::ErrorKind::Interrupted {
                return Err(error);
            }
        } else if descriptor.revents & (libc::POLLERR | libc::POLLNVAL) != 0 {
            return Err(io::Error::other("DAP input pipe failed"));
        } else if descriptor.revents & libc::POLLHUP != 0 {
            return Ok(());
        }
        smol::Timer::after(Duration::from_millis(100)).await;
    }
}

async fn forward(
    mut reader: impl AsyncRead + Unpin,
    mut writer: impl AsyncWrite + Unpin,
) -> std::io::Result<()> {
    let mut buffer = [0; 8192];
    loop {
        let length = reader.read(&mut buffer).await?;
        if length == 0 {
            return Ok(());
        }
        writer.write_all(&buffer[..length]).await?;
        writer.flush().await?;
    }
}

struct AdapterProcess(Option<Child>);

impl Drop for AdapterProcess {
    fn drop(&mut self) {
        if let Some(process) = &mut self.0 {
            process.kill().log_err();
        }
    }
}

#[cfg(test)]
#[expect(
    clippy::disallowed_methods,
    reason = "These tests drive real sockets and processes with smol::block_on"
)]
mod tests {
    use super::*;
    use futures::{StreamExt as _, TryStreamExt as _, future};
    use smol::net::TcpListener;
    #[cfg(unix)]
    use std::io::Read as _;

    #[test]
    fn attach_forwards_dap_in_both_directions() -> Result<()> {
        smol::block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let address = listener.local_addr()?;
            let request = b"Content-Length: 2\r\n\r\n{}";
            let response = b"Content-Length: 16\r\n\r\n{\"success\":true}";
            let (allow_connect, wait_to_connect) = smol::channel::bounded(1);
            let (send_body, wait_for_body) = smol::channel::bounded(1);
            let (header, body) = request.split_at(request.len() - 2);
            let input = futures::stream::once(future::ready(Ok::<_, io::Error>(header)))
                .chain(futures::stream::once(async {
                    allow_connect.send(()).await.map_err(io::Error::other)?;
                    wait_for_body.recv().await.map_err(io::Error::other)?;
                    Ok(body)
                }))
                .chain(futures::stream::pending())
                .boxed()
                .into_async_read();
            let mut output = Vec::new();
            let server = async {
                let (mut stream, _) = listener.accept().await?;
                let mut received = vec![0; request.len()];
                let (received_header, received_body) = received.split_at_mut(header.len());
                stream.read_exact(received_header).await?;
                send_body.send(()).await?;
                stream.read_exact(received_body).await?;
                assert_eq!(received, request);
                stream.write_all(response).await?;
                anyhow::Ok(())
            };
            let exchange = future::try_join(
                server,
                proxy(
                    address,
                    Duration::from_secs(2),
                    &[],
                    |address| {
                        let wait_to_connect = &wait_to_connect;
                        async move {
                            wait_to_connect.recv().await.map_err(io::Error::other)?;
                            TcpStream::connect(address).await
                        }
                    },
                    input,
                    &mut output,
                    future::pending(),
                ),
            );
            select_biased! {
                result = exchange.fuse() => { result?; }
                _ = futures::FutureExt::fuse(smol::Timer::after(Duration::from_secs(5))) => bail!("DAP messages were not forwarded"),
            }
            assert_eq!(output, response);
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn stdin_eof_during_startup_terminates_adapter() -> Result<()> {
        smol::block_on(async {
            let directory = tempfile::tempdir()?;
            let marker = directory.path().join("adapter-pid");
            let command = disconnect_test_adapter(&marker);
            let input = futures::stream::once(async {
                while !std::fs::metadata(&marker).is_ok_and(|metadata| metadata.len() > 0) {
                    smol::Timer::after(Duration::from_millis(10)).await;
                }
                Ok::<_, io::Error>(&b"early DAP input"[..])
            })
            .boxed()
            .into_async_read();
            let address = SocketAddr::from(([127, 0, 0, 1], 0));
            select_biased! {
                result = proxy(address, Duration::from_secs(60), &command, |_| future::pending(), input, smol::io::sink(), future::pending()).fuse() => result?,
                _ = futures::FutureExt::fuse(smol::Timer::after(Duration::from_secs(5))) => bail!("Proxy did not exit after startup stdin EOF"),
            }
            assert_adapter_reaped(&marker).await
        })
    }

    #[test]
    fn connection_timeout() -> Result<()> {
        smol::block_on(async {
            let address = SocketAddr::from(([127, 0, 0, 1], 0));
            let error = proxy(
                address,
                Duration::from_millis(20),
                &[],
                |_| future::pending(),
                futures::stream::pending::<io::Result<&[u8]>>().into_async_read(),
                smol::io::sink(),
                future::pending(),
            )
            .await
            .expect_err("There is no TCP server");
            assert_eq!(
                error.to_string(),
                format!("Connection to TCP DAP timeout {address}")
            );
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn client_disconnect_terminates_adapter() -> Result<()> {
        smol::block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let address = listener.local_addr()?;
            let directory = tempfile::tempdir()?;
            let marker = directory.path().join("adapter-pid");
            let command = disconnect_test_adapter(&marker);
            let connected = &std::cell::Cell::new(false);
            let connect = move |address| async move {
                let stream = TcpStream::connect(address).await?;
                connected.set(true);
                Ok(stream)
            };
            let input = futures::stream::once(async {
                loop {
                    if connected.get()
                        && std::fs::metadata(&marker).is_ok_and(|metadata| metadata.len() > 0)
                    {
                        return Ok::<_, std::io::Error>(Vec::<u8>::new());
                    }
                    smol::Timer::after(Duration::from_millis(10)).await;
                }
            })
            .boxed_local()
            .into_async_read();
            select_biased! {
                result = proxy(address, Duration::from_secs(2), &command, connect, input, smol::io::sink(), future::pending()).fuse() => result?,
                _ = futures::FutureExt::fuse(smol::Timer::after(Duration::from_secs(5))) => bail!("Adapter was not terminated"),
            }
            assert_adapter_reaped(&marker).await
        })
    }

    #[cfg(unix)]
    #[test]
    fn pipe_disconnect_terminates_adapter_with_pending_input() -> Result<()> {
        smol::block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let address = listener.local_addr()?;
            let (stream, (_peer, _)) =
                future::try_join(TcpStream::connect(address), listener.accept()).await?;
            let directory = tempfile::tempdir()?;
            let marker = directory.path().join("adapter-pid");
            let command = disconnect_test_adapter(&marker);
            let (mut input, mut writer) = io::pipe()?;
            writer.write_all(b"unread DAP input")?;
            let exchange = future::try_join(
                proxy(
                    address,
                    Duration::from_secs(2),
                    &command,
                    |_| future::ready(Ok(stream.clone())),
                    smol::io::repeat(0),
                    smol::io::sink(),
                    input_closed(input.as_fd()),
                ),
                async {
                    while !std::fs::metadata(&marker).is_ok_and(|metadata| metadata.len() > 0) {
                        smol::Timer::after(Duration::from_millis(10)).await;
                    }
                    drop(writer);
                    anyhow::Ok(())
                },
            );
            select_biased! {
                result = exchange.fuse() => { result?; }
                _ = futures::FutureExt::fuse(smol::Timer::after(Duration::from_secs(5))) => bail!("Proxy did not exit while the adapter stopped reading"),
            }
            assert_adapter_reaped(&marker).await?;
            let mut remaining = Vec::new();
            input.read_to_end(&mut remaining)?;
            assert_eq!(remaining, b"unread DAP input");
            Ok(())
        })
    }

    #[cfg(unix)]
    fn disconnect_test_adapter(marker: &std::path::Path) -> Vec<String> {
        vec![
            "sh".to_owned(),
            "-c".to_owned(),
            "echo $$ > \"$1\"; exec sleep 60".to_owned(),
            "adapter".to_owned(),
            marker.to_string_lossy().into_owned(),
        ]
    }

    #[cfg(unix)]
    async fn assert_adapter_reaped(marker: &std::path::Path) -> Result<()> {
        let process_id = std::fs::read_to_string(marker)?;
        let status = smol::process::Command::new("sh")
            .args(["-c", "kill -0 \"$1\"", "check", process_id.trim()])
            .stderr(Stdio::null())
            .status()
            .await?;
        assert!(
            !status.success(),
            "The adapter must be reaped after stdin closes"
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn reports_adapter_exit_before_connecting() -> Result<()> {
        smol::block_on(async {
            let address = SocketAddr::from(([127, 0, 0, 1], 0));
            let error = proxy(
                address,
                Duration::from_secs(2),
                &["sh".to_owned(), "-c".to_owned(), "exit 7".to_owned()],
                |_| future::pending(),
                futures::stream::pending::<io::Result<&[u8]>>().into_async_read(),
                smol::io::sink(),
                future::pending(),
            )
            .await
            .expect_err("The adapter exits");
            assert_eq!(
                error.to_string(),
                format!("Debug adapter exited before connecting to {address}: exit status: 7")
            );
            Ok(())
        })
    }

    #[test]
    fn flushes_messages_before_eof() -> Result<()> {
        smol::block_on(async {
            let (sender, receiver) = smol::channel::bounded(1);
            struct Writer(smol::channel::Sender<Vec<u8>>, Vec<u8>);
            impl AsyncWrite for Writer {
                fn poll_write(
                    mut self: std::pin::Pin<&mut Self>,
                    _: &mut std::task::Context<'_>,
                    bytes: &[u8],
                ) -> std::task::Poll<std::io::Result<usize>> {
                    self.1.extend_from_slice(bytes);
                    std::task::Poll::Ready(Ok(bytes.len()))
                }
                fn poll_flush(
                    mut self: std::pin::Pin<&mut Self>,
                    _: &mut std::task::Context<'_>,
                ) -> std::task::Poll<std::io::Result<()>> {
                    let bytes = std::mem::take(&mut self.1);
                    self.0.try_send(bytes).expect("The flush is observed");
                    std::task::Poll::Ready(Ok(()))
                }
                fn poll_close(
                    self: std::pin::Pin<&mut Self>,
                    context: &mut std::task::Context<'_>,
                ) -> std::task::Poll<std::io::Result<()>> {
                    self.poll_flush(context)
                }
            }
            let input =
                futures::stream::once(future::ready(Ok::<_, std::io::Error>(&b"message"[..])))
                    .chain(futures::stream::pending())
                    .into_async_read();
            select_biased! {
                message = receiver.recv().fuse() => assert_eq!(message?, b"message"),
                result = forward(input, Writer(sender, Vec::new())).fuse() => {
                    result?;
                    bail!("Input must remain open");
                }
                _ = futures::FutureExt::fuse(smol::Timer::after(Duration::from_secs(2))) => bail!("Message was not flushed"),
            }
            Ok(())
        })
    }
}
