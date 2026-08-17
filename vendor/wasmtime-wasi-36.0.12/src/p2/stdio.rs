use crate::cli::IsTerminal;
use crate::p2::WasiCtxView;
use crate::p2::bindings::cli::{
    stderr, stdin, stdout, terminal_input, terminal_output, terminal_stderr, terminal_stdin,
    terminal_stdout,
};
use wasmtime::component::Resource;
use wasmtime_wasi_io::streams;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsATTY {
    Yes,
    No,
}

impl stdin::Host for WasiCtxView<'_> {
    fn get_stdin(&mut self) -> Result<Resource<streams::DynInputStream>, anyhow::Error> {
        let stream = self.ctx.cli.stdin.p2_stream();
        Ok(self.table.push(stream)?)
    }
}

impl stdout::Host for WasiCtxView<'_> {
    fn get_stdout(&mut self) -> Result<Resource<streams::DynOutputStream>, anyhow::Error> {
        let stream = self.ctx.cli.stdout.p2_stream();
        Ok(self.table.push(stream)?)
    }
}

impl stderr::Host for WasiCtxView<'_> {
    fn get_stderr(&mut self) -> Result<Resource<streams::DynOutputStream>, anyhow::Error> {
        let stream = self.ctx.cli.stderr.p2_stream();
        Ok(self.table.push(stream)?)
    }
}

pub struct TerminalInput;
pub struct TerminalOutput;

impl terminal_input::Host for WasiCtxView<'_> {}
impl terminal_input::HostTerminalInput for WasiCtxView<'_> {
    fn drop(&mut self, r: Resource<TerminalInput>) -> anyhow::Result<()> {
        self.table.delete(r)?;
        Ok(())
    }
}
impl terminal_output::Host for WasiCtxView<'_> {}
impl terminal_output::HostTerminalOutput for WasiCtxView<'_> {
    fn drop(&mut self, r: Resource<TerminalOutput>) -> anyhow::Result<()> {
        self.table.delete(r)?;
        Ok(())
    }
}
impl terminal_stdin::Host for WasiCtxView<'_> {
    fn get_terminal_stdin(&mut self) -> anyhow::Result<Option<Resource<TerminalInput>>> {
        if self.ctx.cli.stdin.is_terminal() {
            let fd = self.table.push(TerminalInput)?;
            Ok(Some(fd))
        } else {
            Ok(None)
        }
    }
}
impl terminal_stdout::Host for WasiCtxView<'_> {
    fn get_terminal_stdout(&mut self) -> anyhow::Result<Option<Resource<TerminalOutput>>> {
        if self.ctx.cli.stdout.is_terminal() {
            let fd = self.table.push(TerminalOutput)?;
            Ok(Some(fd))
        } else {
            Ok(None)
        }
    }
}
impl terminal_stderr::Host for WasiCtxView<'_> {
    fn get_terminal_stderr(&mut self) -> anyhow::Result<Option<Resource<TerminalOutput>>> {
        if self.ctx.cli.stderr.is_terminal() {
            let fd = self.table.push(TerminalOutput)?;
            Ok(Some(fd))
        } else {
            Ok(None)
        }
    }
}
