use std::path::PathBuf;
use std::pin::Pin;

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use futures::io::{BufReader, BufWriter};
use futures::{
    AsyncBufReadExt as _, AsyncRead, AsyncWrite, AsyncWriteExt as _, Stream, StreamExt as _,
};
use gpui::AsyncApp;
use settings::Settings as _;
use smol::channel;
use smol::process::Child;
use terminal::terminal_settings::TerminalSettings;
use util::TryFutureExt as _;
use util::shell_builder::ShellBuilder;

use crate::client::ModelContextServerBinary;
use crate::transport::Transport;

pub struct StdioTransport {
    stdout_sender: channel::Sender<String>,
    stdin_receiver: channel::Receiver<String>,
    stderr_receiver: channel::Receiver<String>,
    server: Child,
}

impl StdioTransport {
    pub fn new(
        binary: ModelContextServerBinary,
        working_directory: &Option<PathBuf>,
        cx: &AsyncApp,
    ) -> Result<Self> {
        let shell = cx.update(|cx| TerminalSettings::get(None, cx).shell.clone())?;
        let builder = ShellBuilder::new(&shell, cfg!(windows)).non_interactive();
        let mut command =
            builder.build_command(Some(binary.executable.display().to_string()), &binary.args);

        command
            .envs(binary.env.unwrap_or_default())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);

        if let Some(working_directory) = working_directory {
            command.current_dir(working_directory);
        }

        let mut server = command
            .spawn()
            .with_context(|| format!("failed to spawn command {command:?})",))?;

        let stdin = server.stdin.take().unwrap();
        let stdout = server.stdout.take().unwrap();
        let stderr = server.stderr.take().unwrap();

        let (stdin_sender, stdin_receiver) = channel::unbounded::<String>();
        let (stdout_sender, stdout_receiver) = channel::unbounded::<String>();
        let (stderr_sender, stderr_receiver) = channel::unbounded::<String>();

        cx.spawn(async move |_| Self::handle_output(stdin, stdout_receiver).log_err().await)
            .detach();

        cx.spawn(async move |_| Self::handle_input(stdout, stdin_sender).await)
            .detach();

        cx.spawn(async move |_| Self::handle_err(stderr, stderr_sender).await)
            .detach();

        Ok(Self {
            stdout_sender,
            stdin_receiver,
            stderr_receiver,
            server,
        })
    }

    async fn handle_input<Stdout>(stdin: Stdout, inbound_rx: channel::Sender<String>)
    where
        Stdout: AsyncRead + Unpin + Send + 'static,
    {
        let mut stdin = BufReader::new(stdin);
        let mut line = String::new();
        while let Ok(n) = stdin.read_line(&mut line).await {
            if n == 0 {
                break;
            }
            if inbound_rx.send(line.clone()).await.is_err() {
                break;
            }
            line.clear();
        }
    }

    async fn handle_output<Stdin>(
        stdin: Stdin,
        outbound_rx: channel::Receiver<String>,
    ) -> Result<()>
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
    {
        let mut stdin = BufWriter::new(stdin);
        let mut pinned_rx = Box::pin(outbound_rx);
        while let Some(message) = pinned_rx.next().await {
            log::trace!("outgoing message: {}", message);

            stdin.write_all(message.as_bytes()).await?;
            stdin.write_all(b"\n").await?;
            stdin.flush().await?;
        }
        Ok(())
    }

    async fn handle_err<Stderr>(stderr: Stderr, stderr_tx: channel::Sender<String>)
    where
        Stderr: AsyncRead + Unpin + Send + 'static,
    {
        let mut stderr = BufReader::new(stderr);
        let mut line = String::new();
        while let Ok(n) = stderr.read_line(&mut line).await {
            if n == 0 {
                break;
            }
            if stderr_tx.send(line.clone()).await.is_err() {
                break;
            }
            line.clear();
        }
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn send(&self, message: String) -> Result<()> {
        Ok(self.stdout_sender.send(message).await?)
    }

    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.stdin_receiver.clone())
    }

    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.stderr_receiver.clone())
    }
}

impl Drop for StdioTransport {
    fn drop(&mut self) {
        let _ = self.server.kill();
    }
}
