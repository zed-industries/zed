use crate::I32Exit;
use crate::cli::IsTerminal;
use crate::p3::DEFAULT_BUFFER_CAPACITY;
use crate::p3::bindings::cli::{
    environment, exit, stderr, stdin, stdout, terminal_input, terminal_output, terminal_stderr,
    terminal_stdin, terminal_stdout,
};
use crate::p3::cli::{TerminalInput, TerminalOutput, WasiCli, WasiCliCtxView};
use anyhow::{Context as _, anyhow};
use bytes::BytesMut;
use std::io::Cursor;
use tokio::io::{AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _};
use wasmtime::component::{
    Accessor, AccessorTask, GuardedStreamReader, GuardedStreamWriter, HasData, Resource,
    StreamReader, StreamWriter,
};

struct InputTask<T> {
    rx: T,
    tx: StreamWriter<u8>,
}

impl<T, U, V> AccessorTask<T, U, wasmtime::Result<()>> for InputTask<V>
where
    U: HasData,
    V: AsyncRead + Send + Sync + Unpin + 'static,
{
    async fn run(mut self, store: &Accessor<T, U>) -> wasmtime::Result<()> {
        let mut buf = BytesMut::with_capacity(DEFAULT_BUFFER_CAPACITY);
        let mut tx = GuardedStreamWriter::new(store, self.tx);
        while !tx.is_closed() {
            match self.rx.read_buf(&mut buf).await {
                Ok(0) => return Ok(()),
                Ok(_) => {
                    buf = tx.write_all(Cursor::new(buf)).await.into_inner();
                    buf.clear();
                }
                Err(_err) => {
                    // TODO: Report the error to the guest
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}

struct OutputTask<T> {
    rx: StreamReader<u8>,
    tx: T,
}

impl<T, U, V> AccessorTask<T, U, wasmtime::Result<()>> for OutputTask<V>
where
    U: HasData,
    V: AsyncWrite + Send + Sync + Unpin + 'static,
{
    async fn run(mut self, store: &Accessor<T, U>) -> wasmtime::Result<()> {
        let mut buf = BytesMut::with_capacity(DEFAULT_BUFFER_CAPACITY);
        let mut rx = GuardedStreamReader::new(store, self.rx);
        while !rx.is_closed() {
            buf = rx.read(buf).await;
            match self.tx.write_all(&buf).await {
                Ok(()) => {
                    buf.clear();
                    continue;
                }
                Err(_err) => {
                    // TODO: Report the error to the guest
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}

impl terminal_input::Host for WasiCliCtxView<'_> {}
impl terminal_output::Host for WasiCliCtxView<'_> {}

impl terminal_input::HostTerminalInput for WasiCliCtxView<'_> {
    fn drop(&mut self, rep: Resource<TerminalInput>) -> wasmtime::Result<()> {
        self.table
            .delete(rep)
            .context("failed to delete terminal input resource from table")?;
        Ok(())
    }
}

impl terminal_output::HostTerminalOutput for WasiCliCtxView<'_> {
    fn drop(&mut self, rep: Resource<TerminalOutput>) -> wasmtime::Result<()> {
        self.table
            .delete(rep)
            .context("failed to delete terminal output resource from table")?;
        Ok(())
    }
}

impl terminal_stdin::Host for WasiCliCtxView<'_> {
    fn get_terminal_stdin(&mut self) -> wasmtime::Result<Option<Resource<TerminalInput>>> {
        if self.ctx.stdin.is_terminal() {
            let fd = self
                .table
                .push(TerminalInput)
                .context("failed to push terminal stdin resource to table")?;
            Ok(Some(fd))
        } else {
            Ok(None)
        }
    }
}

impl terminal_stdout::Host for WasiCliCtxView<'_> {
    fn get_terminal_stdout(&mut self) -> wasmtime::Result<Option<Resource<TerminalOutput>>> {
        if self.ctx.stdout.is_terminal() {
            let fd = self
                .table
                .push(TerminalOutput)
                .context("failed to push terminal stdout resource to table")?;
            Ok(Some(fd))
        } else {
            Ok(None)
        }
    }
}

impl terminal_stderr::Host for WasiCliCtxView<'_> {
    fn get_terminal_stderr(&mut self) -> wasmtime::Result<Option<Resource<TerminalOutput>>> {
        if self.ctx.stderr.is_terminal() {
            let fd = self
                .table
                .push(TerminalOutput)
                .context("failed to push terminal stderr resource to table")?;
            Ok(Some(fd))
        } else {
            Ok(None)
        }
    }
}

impl stdin::HostWithStore for WasiCli {
    async fn get_stdin<U>(store: &Accessor<U, Self>) -> wasmtime::Result<StreamReader<u8>> {
        store.with(|mut view| {
            let instance = view.instance();
            let (tx, rx) = instance
                .stream(&mut view)
                .context("failed to create stream")?;
            let stdin = view.get().ctx.stdin.async_stream();
            view.spawn(InputTask {
                rx: Box::into_pin(stdin),
                tx,
            });
            Ok(rx)
        })
    }
}

impl stdin::Host for WasiCliCtxView<'_> {}

impl stdout::HostWithStore for WasiCli {
    async fn set_stdout<U>(
        store: &Accessor<U, Self>,
        data: StreamReader<u8>,
    ) -> wasmtime::Result<()> {
        store.with(|mut view| {
            let tx = view.get().ctx.stdout.async_stream();
            view.spawn(OutputTask {
                rx: data,
                tx: Box::into_pin(tx),
            });
            Ok(())
        })
    }
}

impl stdout::Host for WasiCliCtxView<'_> {}

impl stderr::HostWithStore for WasiCli {
    async fn set_stderr<U>(
        store: &Accessor<U, Self>,
        data: StreamReader<u8>,
    ) -> wasmtime::Result<()> {
        store.with(|mut view| {
            let tx = view.get().ctx.stderr.async_stream();
            view.spawn(OutputTask {
                rx: data,
                tx: Box::into_pin(tx),
            });
            Ok(())
        })
    }
}

impl stderr::Host for WasiCliCtxView<'_> {}

impl environment::Host for WasiCliCtxView<'_> {
    fn get_environment(&mut self) -> wasmtime::Result<Vec<(String, String)>> {
        Ok(self.ctx.environment.clone())
    }

    fn get_arguments(&mut self) -> wasmtime::Result<Vec<String>> {
        Ok(self.ctx.arguments.clone())
    }

    fn initial_cwd(&mut self) -> wasmtime::Result<Option<String>> {
        Ok(self.ctx.initial_cwd.clone())
    }
}

impl exit::Host for WasiCliCtxView<'_> {
    fn exit(&mut self, status: Result<(), ()>) -> wasmtime::Result<()> {
        let status = match status {
            Ok(()) => 0,
            Err(()) => 1,
        };
        Err(anyhow!(I32Exit(status)))
    }

    fn exit_with_code(&mut self, status_code: u8) -> wasmtime::Result<()> {
        Err(anyhow!(I32Exit(status_code.into())))
    }
}
