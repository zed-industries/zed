use std::{path::PathBuf, sync::Arc};

use anyhow::{anyhow, Result};
use collections::HashMap;
use futures::{channel::{mpsc::{Sender, UnboundedReceiver, UnboundedSender}, oneshot}, AsyncReadExt, FutureExt, StreamExt};
use gpui::{AppContext, AsyncAppContext, BorrowAppContext, Context, Global, Model, Task, WeakModel};
use smol::process::Child;
use rpc::{proto::Envelope, ErrorExt};

use crate::{
    protocol::{
        message_len_from_buffer, read_message_with_len, write_message, MessageId, MESSAGE_LEN_SIZE,
    }, ssh_session::{run_cmd, SshRemoteConnection, SshRemoteProcess, SshSocket}, SshClientDelegate, SshConnectionOptions
};

pub(crate) struct ConnectionPool {
    connections: HashMap<SshConnectionOptions, WeakModel<ConnectionState>>,
}

struct ConnectionState {
    refcount: usize,
    options: SshConnectionOptions,
    connecting: Task<()>,
    connected: Option<Connected>,
    waiters: Vec<oneshot::Sender<Result<()>>>,
};

struct Connected {
    connection: SshRemoteConnection,
    remote_binary_path: PathBuf,
}

impl ConnectionState {
    pub(crate) async fn connect(
        &mut self,
        unique_identifier: String,
        reconnect: bool,
        incoming_tx: UnboundedSender<rpc::proto::Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AppContext,
    ) -> Result<(Box<dyn SshRemoteProcess>, Task<Result<i32>>)> {
        let Some(Connected { connection, remote_binary_path }) = connection.connected.as_ref() else {
            let (tx, rx) = oneshot::channel();
            self.waiters.push(tx);
            return cx.spawn(|this, cx| async move {
                rx.await?;
                this.update(|this, cx| this.connect(
                    unique_identifier,
                    reconnect,
                    incoming_tx,
                    outgoing_rx,
                    connection_activity_tx,
                    delegate,
                    cx,
                ))?
            })
        };

        delegate.set_status(Some("Starting proxy"), cx);
        let mut start_proxy_command = format!(
            "RUST_LOG={} RUST_BACKTRACE={} {:?} proxy --identifier {}",
            std::env::var("RUST_LOG").unwrap_or_default(),
            std::env::var("RUST_BACKTRACE").unwrap_or_default(),
            remote_binary_path,
            unique_identifier,
        );
        if reconnect {
            start_proxy_command.push_str(" --reconnect");
        }

        let ssh_proxy_process = connection.socket
            .ssh_command(start_proxy_command)
            // IMPORTANT: we kill this process when we drop the task that uses it.
            .kill_on_drop(true)
            .spawn()
            .context("failed to spawn remote server")?;

        let io_task = Self::multiplex(
            ssh_proxy_process,
            incoming_tx,
            outgoing_rx,
            connection_activity_tx,
            &cx,
        );

        Ok((Box::new(handle) as _, io_task))
    }
}

impl Global for ConnectionPool {}

impl ConnectionPool {
    pub(crate) fn connection(&mut self, opts: SshConnectionOptions, delegate: &Arc<dyn SshClientDelegate>, cx: &mut AppContext) -> Model<ConnectionState> {
        if let Some(connection) = self.connections.get(&opts).and_then(|connection| connection.upgrade()) {
            return connection
        }

        let connection = cx.new_model(|cx| {
            ConnectionState {
                refcount: 0,
                options: opts.clone(),
                connecting: Self::create_master_process(opts.clone(), delegate.clone(), &mut cx.to_async()),
                connected: None,
                waiters: vec![],
            }
        });
        cx.observe_release(&connection, |c, cx| {
            cx.update_global(|pool: &mut Self, _| {
                pool.connections.remove(&c.options);
            });
        });
        self.connections.insert(opts, connection.downgrade());
        connection
    }
}

    pub(crate) async fn connect(
        &mut self,
        unique_identifier: String,
        reconnect: bool,
        connection_options: SshConnectionOptions,
        incoming_tx: UnboundedSender<rpc::proto::Envelope>,
        outgoing_rx: UnboundedReceiver<Envelope>,
        connection_activity_tx: Sender<()>,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AppContext,
    ) -> Task<Result<(Box<dyn SshRemoteProcess>, Task<Result<i32>>)>> {
        let connection = self.connections.entry(connection_options.clone()).or_insert_with(|| {
            cx.new_model(|cx| {
                ConnectionState {
                    refcount: 0,
                    options: connection_options.clone(),
                    connecting: Self::create_master_process(connection_options, delegate.clone(), cx),
                    connected: None,
                    waiters: vec![],
                }
            })
        });
    }

    fn create_master_process(
        connection_options: SshConnectionOptions,
        delegate: Arc<dyn SshClientDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Task<()> {
        let task: Task<Result<Connected>> = cx.spawn({
            let connection_options = connection_options.clone();

            |mut cx| async move {
            let ssh_connection = SshRemoteConnection::new(connection_options, delegate.clone(), &mut cx).await?;

            let platform = ssh_connection.query_platform().await?;
            let remote_binary_path = delegate.remote_server_binary_path(platform, &mut cx)?;
            ssh_connection
                .ensure_server_binary(&delegate, &remote_binary_path, platform, &mut cx)
                .await?;

            let socket = ssh_connection.socket.clone();
            // do this as part of ensure server binary?
            run_cmd(socket.ssh_command(&remote_binary_path).arg("version")).await?;

            Ok(Connected{
                connection: ssh_connection,
                remote_binary_path,
            })
        }});
        cx.spawn(|cx| async move {
            let result = task.await;

            cx.update_global(|connection_pool: &mut Self, _| {
                let Some(connection_state) = connection_pool.connections.get_mut(&connection_options) else {
                    log::error!("connection dropped while connecting");
                    return;
                };
                match result {
                    Ok(connection) => {
                        connection_state.connected = Some(connection);
                        for tx in connection_state.waiters.drain(..) {
                            tx.send(Ok(())).ok();
                        }
                    },
                    Err(e) => {
                        for tx in connection_state.waiters.drain(..) {
                            tx.send(Err(e.cloned())).ok();
                        }
                        connection_pool.connections.remove(&connection_options);
                    }
                }
            }).ok();
        })
    }

    fn multiplex(
        mut ssh_proxy_process: Child,
        incoming_tx: UnboundedSender<Envelope>,
        mut outgoing_rx: UnboundedReceiver<Envelope>,
        mut connection_activity_tx: Sender<()>,
        cx: &AsyncAppContext,
    ) -> Task<Result<i32>> {
        let mut child_stderr = ssh_proxy_process.stderr.take().unwrap();
        let mut child_stdout = ssh_proxy_process.stdout.take().unwrap();
        let mut child_stdin = ssh_proxy_process.stdin.take().unwrap();

        let mut stdin_buffer = Vec::new();
        let mut stdout_buffer = Vec::new();
        let mut stderr_buffer = Vec::new();
        let mut stderr_offset = 0;

        let stdin_task = cx.background_executor().spawn(async move {
            while let Some(outgoing) = outgoing_rx.next().await {
                write_message(&mut child_stdin, &mut stdin_buffer, outgoing).await?;
            }
            anyhow::Ok(())
        });

        let stdout_task = cx.background_executor().spawn({
            let mut connection_activity_tx = connection_activity_tx.clone();
            async move {
                loop {
                    stdout_buffer.resize(MESSAGE_LEN_SIZE, 0);
                    let len = child_stdout.read(&mut stdout_buffer).await?;

                    if len == 0 {
                        return anyhow::Ok(());
                    }

                    if len < MESSAGE_LEN_SIZE {
                        child_stdout.read_exact(&mut stdout_buffer[len..]).await?;
                    }

                    let message_len = message_len_from_buffer(&stdout_buffer);
                    let envelope =
                        read_message_with_len(&mut child_stdout, &mut stdout_buffer, message_len)
                            .await?;
                    connection_activity_tx.try_send(()).ok();
                    incoming_tx.unbounded_send(envelope).ok();
                }
            }
        });

        let stderr_task: Task<anyhow::Result<()>> = cx.background_executor().spawn(async move {
            loop {
                stderr_buffer.resize(stderr_offset + 1024, 0);

                let len = child_stderr
                    .read(&mut stderr_buffer[stderr_offset..])
                    .await?;
                if len == 0 {
                    return anyhow::Ok(());
                }

                stderr_offset += len;
                let mut start_ix = 0;
                while let Some(ix) = stderr_buffer[start_ix..stderr_offset]
                    .iter()
                    .position(|b| b == &b'\n')
                {
                    let line_ix = start_ix + ix;
                    let content = &stderr_buffer[start_ix..line_ix];
                    start_ix = line_ix + 1;
                    if let Ok(record) = serde_json::from_slice::<LogRecord>(content) {
                        record.log(log::logger())
                    } else {
                        eprintln!("(remote) {}", String::from_utf8_lossy(content));
                    }
                }
                stderr_buffer.drain(0..start_ix);
                stderr_offset -= start_ix;

                connection_activity_tx.try_send(()).ok();
            }
        });

        cx.spawn(|_| async move {
            let result = futures::select! {
                result = stdin_task.fuse() => {
                    result.context("stdin")
                }
                result = stdout_task.fuse() => {
                    result.context("stdout")
                }
                result = stderr_task.fuse() => {
                    result.context("stderr")
                }
            };

            let status = ssh_proxy_process.status().await?.code().unwrap_or(1);
            drop(handle);
            match result {
                Ok(_) => Ok(status),
                Err(error) => Err(error),
            }
        })
    }
}
