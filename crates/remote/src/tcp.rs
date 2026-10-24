use anyhow::{Context as _, Result, bail};
use futures::{
    AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _, FutureExt as _, select_biased,
};
use smol::net::TcpStream;
use std::{net::SocketAddr, process::Stdio, time::Duration};
use util::{ResultExt as _, process::Child};

#[expect(
    clippy::disallowed_methods,
    reason = "This stdio helper runs without a GPUI app or dispatcher"
)]
pub async fn proxy(
    address: SocketAddr,
    timeout: Duration,
    command: &[String],
    mut input: impl AsyncRead + Unpin,
    mut output: impl AsyncWrite + Unpin,
) -> Result<()> {
    let mut process = AdapterProcess(None);
    if let Some((program, arguments)) = command.split_first() {
        let mut command = util::command::new_std_command(program);
        command.args(arguments);
        // Adapter logs must not be mixed with the DAP messages on stdout.
        process.0 = Some(Child::spawn(
            command,
            Stdio::null(),
            Stdio::from(std::io::stderr()),
            Stdio::inherit(),
        )?);
    }

    let result = async {
        let connect = async {
            loop {
                if let Some(process) = &mut process.0
                    && let Some(status) = process.try_status()?
                {
                    bail!("Debug adapter exited before connecting to {address}: {status}");
                }
                if let Ok(stream) = TcpStream::connect(address).await {
                    return Ok(stream);
                }
                smol::Timer::after(Duration::from_millis(100)).await;
            }
        };
        let stream = select_biased! {
            result = connect.fuse() => result?,
            _ = smol::Timer::after(timeout).fuse() => {
                bail!("Connection to TCP DAP timeout {address}");
            }
        };
        let (mut reader, mut writer) = stream.split();
        select_biased! {
            result = forward(&mut input, &mut writer).fuse() => {
                result.context("Forwarding DAP input")?;
            }
            result = forward(&mut reader, &mut output).fuse() => {
                result.context("Forwarding DAP output")?;
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

    #[test]
    fn adapter_process() -> Result<()> {
        use std::io::{Read as _, Write as _};
        let Some(port) = std::env::args()
            .skip_while(|argument| argument != "--skip")
            .nth(1)
        else {
            return Ok(());
        };
        let listener =
            std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port.parse::<u16>()?))?;
        println!("adapter stdout log");
        eprintln!("adapter stderr log");
        let (mut stream, _) = listener.accept()?;
        let mut request = [0; 4];
        stream.read_exact(&mut request)?;
        assert_eq!(&request, b"ping");
        stream.write_all(b"pong")?;
        Ok(())
    }

    #[test]
    fn launch_waits_for_adapter_and_keeps_logs_out_of_dap() -> Result<()> {
        smol::block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let address = listener.local_addr()?;
            drop(listener);
            let input = futures::stream::once(future::ready(Ok::<_, std::io::Error>(&b"ping"[..])))
                .chain(futures::stream::pending())
                .into_async_read();
            let mut output = Vec::new();
            proxy(
                address,
                Duration::from_secs(5),
                &[
                    std::env::current_exe()?.to_string_lossy().into_owned(),
                    "--exact".to_owned(),
                    "tcp::tests::adapter_process".to_owned(),
                    "--nocapture".to_owned(),
                    "--skip".to_owned(),
                    address.port().to_string(),
                ],
                input,
                &mut output,
            )
            .await?;
            assert_eq!(output, b"pong");
            Ok(())
        })
    }

    #[test]
    fn attach_forwards_dap_in_both_directions() -> Result<()> {
        smol::block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let address = listener.local_addr()?;
            let request = b"Content-Length: 2\r\n\r\n{}";
            let response = b"Content-Length: 16\r\n\r\n{\"success\":true}";
            let input = futures::stream::once(future::ready(Ok::<_, std::io::Error>(request)))
                .chain(futures::stream::pending())
                .into_async_read();
            let mut output = Vec::new();
            let server = async {
                let (mut stream, _) = listener.accept().await?;
                let mut received = vec![0; request.len()];
                stream.read_exact(&mut received).await?;
                assert_eq!(received, request);
                stream.write_all(response).await?;
                anyhow::Ok(())
            };
            let (server, proxy) = future::join(
                server,
                proxy(address, Duration::from_secs(2), &[], input, &mut output),
            )
            .await;
            server?;
            proxy?;
            assert_eq!(output, response);
            Ok(())
        })
    }

    #[test]
    fn connection_timeout() -> Result<()> {
        smol::block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let address = listener.local_addr()?;
            drop(listener);
            let error = proxy(
                address,
                Duration::from_millis(20),
                &[],
                smol::io::empty(),
                smol::io::sink(),
            )
            .await
            .expect_err("There is no TCP server");
            assert!(error.to_string().contains("Connection to TCP DAP timeout"));
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
            let command = vec![
                "sh".to_owned(),
                "-c".to_owned(),
                "echo $$ > \"$1\"; exec sleep 60".to_owned(),
                "adapter".to_owned(),
                marker.to_string_lossy().into_owned(),
            ];
            let input = futures::stream::once(async {
                loop {
                    if std::fs::metadata(&marker).is_ok_and(|metadata| metadata.len() > 0) {
                        return Ok::<_, std::io::Error>(Vec::<u8>::new());
                    }
                    smol::Timer::after(Duration::from_millis(10)).await;
                }
            })
            .boxed()
            .into_async_read();
            select_biased! {
                result = proxy(address, Duration::from_secs(2), &command, input, smol::io::sink()).fuse() => result?,
                _ = futures::FutureExt::fuse(smol::Timer::after(Duration::from_secs(5))) => bail!("Adapter was not terminated"),
            }
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
        })
    }

    #[cfg(unix)]
    #[test]
    fn reports_adapter_exit_before_connecting() -> Result<()> {
        smol::block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let address = listener.local_addr()?;
            drop(listener);
            let error = proxy(
                address,
                Duration::from_secs(2),
                &["sh".to_owned(), "-c".to_owned(), "exit 7".to_owned()],
                smol::io::empty(),
                smol::io::sink(),
            )
            .await
            .expect_err("The adapter exits");
            assert!(
                error
                    .to_string()
                    .contains("Debug adapter exited before connecting")
            );
            assert!(error.to_string().contains('7'));
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
