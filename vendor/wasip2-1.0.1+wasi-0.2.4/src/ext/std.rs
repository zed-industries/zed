use std::error::Error;
use std::io;
use std::num::NonZeroU64;

use crate::io::streams::StreamError;

impl Error for crate::io::error::Error {}

impl io::Read for crate::io::streams::InputStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = buf
            .len()
            .try_into()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        match self.blocking_read(n) {
            Ok(chunk) => {
                let n = chunk.len();
                if n > buf.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "more bytes read than requested",
                    ));
                }
                buf[..n].copy_from_slice(&chunk);
                Ok(n)
            }
            Err(StreamError::Closed) => Ok(0),
            Err(StreamError::LastOperationFailed(e)) => {
                Err(io::Error::new(io::ErrorKind::Other, e.to_debug_string()))
            }
        }
    }
}

impl io::Write for crate::io::streams::OutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = loop {
            match self.check_write().map(NonZeroU64::new) {
                Ok(Some(n)) => {
                    break n;
                }
                Ok(None) => {
                    self.subscribe().block();
                }
                Err(StreamError::Closed) => return Ok(0),
                Err(StreamError::LastOperationFailed(e)) => {
                    return Err(io::Error::new(io::ErrorKind::Other, e.to_debug_string()))
                }
            };
        };
        let n = n
            .get()
            .try_into()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let n = buf.len().min(n);
        crate::io::streams::OutputStream::write(self, &buf[..n]).map_err(|e| match e {
            StreamError::Closed => io::ErrorKind::UnexpectedEof.into(),
            StreamError::LastOperationFailed(e) => {
                io::Error::new(io::ErrorKind::Other, e.to_debug_string())
            }
        })?;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.blocking_flush()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
