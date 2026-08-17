use crate::cli::{IsTerminal, StdoutStream};
use crate::p2;
use bytes::Bytes;
use tokio::io::AsyncWrite;
use wasmtime_wasi_io::streams::OutputStream;

// Implementation for tokio::io::Stdout
impl IsTerminal for tokio::io::Stdout {
    fn is_terminal(&self) -> bool {
        std::io::stdout().is_terminal()
    }
}
impl StdoutStream for tokio::io::Stdout {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(StdioOutputStream::Stdout)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(tokio::io::stdout())
    }
}

// Implementation for std::io::Stdout
impl IsTerminal for std::io::Stdout {
    fn is_terminal(&self) -> bool {
        std::io::IsTerminal::is_terminal(self)
    }
}
impl StdoutStream for std::io::Stdout {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(StdioOutputStream::Stdout)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(tokio::io::stdout())
    }
}

// Implementation for tokio::io::Stderr
impl IsTerminal for tokio::io::Stderr {
    fn is_terminal(&self) -> bool {
        std::io::stderr().is_terminal()
    }
}
impl StdoutStream for tokio::io::Stderr {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(StdioOutputStream::Stderr)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(tokio::io::stderr())
    }
}

// Implementation for std::io::Stderr
impl IsTerminal for std::io::Stderr {
    fn is_terminal(&self) -> bool {
        std::io::IsTerminal::is_terminal(self)
    }
}
impl StdoutStream for std::io::Stderr {
    fn p2_stream(&self) -> Box<dyn OutputStream> {
        Box::new(StdioOutputStream::Stderr)
    }
    fn async_stream(&self) -> Box<dyn AsyncWrite + Send + Sync> {
        Box::new(tokio::io::stderr())
    }
}

enum StdioOutputStream {
    Stdout,
    Stderr,
}

impl OutputStream for StdioOutputStream {
    fn write(&mut self, bytes: Bytes) -> p2::StreamResult<()> {
        use std::io::Write;
        match self {
            StdioOutputStream::Stdout => std::io::stdout().write_all(&bytes),
            StdioOutputStream::Stderr => std::io::stderr().write_all(&bytes),
        }
        .map_err(|e| p2::StreamError::LastOperationFailed(anyhow::anyhow!(e)))
    }

    fn flush(&mut self) -> p2::StreamResult<()> {
        use std::io::Write;
        match self {
            StdioOutputStream::Stdout => std::io::stdout().flush(),
            StdioOutputStream::Stderr => std::io::stderr().flush(),
        }
        .map_err(|e| p2::StreamError::LastOperationFailed(anyhow::anyhow!(e)))
    }

    fn check_write(&mut self) -> p2::StreamResult<usize> {
        Ok(1024 * 1024)
    }
}

#[async_trait::async_trait]
impl p2::Pollable for StdioOutputStream {
    async fn ready(&mut self) {}
}
