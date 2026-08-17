use crate::p2::{
    StreamError, StreamResult,
    bindings::sync::io::poll::Pollable,
    bindings::sync::io::streams::{self, InputStream, OutputStream},
};
use crate::runtime::in_tokio;
use wasmtime::component::{Resource, ResourceTable};
use wasmtime_wasi_io::bindings::wasi::io::streams::{
    self as async_streams, Host as AsyncHost, HostInputStream as AsyncHostInputStream,
    HostOutputStream as AsyncHostOutputStream,
};

impl From<async_streams::StreamError> for streams::StreamError {
    fn from(other: async_streams::StreamError) -> Self {
        match other {
            async_streams::StreamError::LastOperationFailed(e) => Self::LastOperationFailed(e),
            async_streams::StreamError::Closed => Self::Closed,
        }
    }
}

impl streams::Host for ResourceTable {
    fn convert_stream_error(&mut self, err: StreamError) -> anyhow::Result<streams::StreamError> {
        Ok(AsyncHost::convert_stream_error(self, err)?.into())
    }
}

impl streams::HostOutputStream for ResourceTable {
    fn drop(&mut self, stream: Resource<OutputStream>) -> anyhow::Result<()> {
        in_tokio(async { AsyncHostOutputStream::drop(self, stream).await })
    }

    fn check_write(&mut self, stream: Resource<OutputStream>) -> StreamResult<u64> {
        Ok(AsyncHostOutputStream::check_write(self, stream)?)
    }

    fn write(&mut self, stream: Resource<OutputStream>, bytes: Vec<u8>) -> StreamResult<()> {
        Ok(AsyncHostOutputStream::write(self, stream, bytes)?)
    }

    fn blocking_write_and_flush(
        &mut self,
        stream: Resource<OutputStream>,
        bytes: Vec<u8>,
    ) -> StreamResult<()> {
        in_tokio(async {
            AsyncHostOutputStream::blocking_write_and_flush(self, stream, bytes).await
        })
    }

    fn blocking_write_zeroes_and_flush(
        &mut self,
        stream: Resource<OutputStream>,
        len: u64,
    ) -> StreamResult<()> {
        in_tokio(async {
            AsyncHostOutputStream::blocking_write_zeroes_and_flush(self, stream, len).await
        })
    }

    fn subscribe(&mut self, stream: Resource<OutputStream>) -> anyhow::Result<Resource<Pollable>> {
        Ok(AsyncHostOutputStream::subscribe(self, stream)?)
    }

    fn write_zeroes(&mut self, stream: Resource<OutputStream>, len: u64) -> StreamResult<()> {
        Ok(AsyncHostOutputStream::write_zeroes(self, stream, len)?)
    }

    fn flush(&mut self, stream: Resource<OutputStream>) -> StreamResult<()> {
        Ok(AsyncHostOutputStream::flush(
            self,
            Resource::new_borrow(stream.rep()),
        )?)
    }

    fn blocking_flush(&mut self, stream: Resource<OutputStream>) -> StreamResult<()> {
        in_tokio(async {
            AsyncHostOutputStream::blocking_flush(self, Resource::new_borrow(stream.rep())).await
        })
    }

    fn splice(
        &mut self,
        dst: Resource<OutputStream>,
        src: Resource<InputStream>,
        len: u64,
    ) -> StreamResult<u64> {
        AsyncHostOutputStream::splice(self, dst, src, len)
    }

    fn blocking_splice(
        &mut self,
        dst: Resource<OutputStream>,
        src: Resource<InputStream>,
        len: u64,
    ) -> StreamResult<u64> {
        in_tokio(async { AsyncHostOutputStream::blocking_splice(self, dst, src, len).await })
    }
}

impl streams::HostInputStream for ResourceTable {
    fn drop(&mut self, stream: Resource<InputStream>) -> anyhow::Result<()> {
        in_tokio(async { AsyncHostInputStream::drop(self, stream).await })
    }

    fn read(&mut self, stream: Resource<InputStream>, len: u64) -> StreamResult<Vec<u8>> {
        AsyncHostInputStream::read(self, stream, len)
    }

    fn blocking_read(&mut self, stream: Resource<InputStream>, len: u64) -> StreamResult<Vec<u8>> {
        in_tokio(async { AsyncHostInputStream::blocking_read(self, stream, len).await })
    }

    fn skip(&mut self, stream: Resource<InputStream>, len: u64) -> StreamResult<u64> {
        AsyncHostInputStream::skip(self, stream, len)
    }

    fn blocking_skip(&mut self, stream: Resource<InputStream>, len: u64) -> StreamResult<u64> {
        in_tokio(async { AsyncHostInputStream::blocking_skip(self, stream, len).await })
    }

    fn subscribe(&mut self, stream: Resource<InputStream>) -> anyhow::Result<Resource<Pollable>> {
        AsyncHostInputStream::subscribe(self, stream)
    }
}
