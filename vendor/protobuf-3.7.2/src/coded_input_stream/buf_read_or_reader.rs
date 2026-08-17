//! `BufRead` pointer or `BufReader` owned.

use std::cmp;
use std::fmt;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::mem::MaybeUninit;

use crate::misc::maybe_uninit_write_slice;

/// Helper type to simplify `BufReadIter` implementation.
pub(crate) enum BufReadOrReader<'a> {
    BufReader(BufReader<&'a mut dyn Read>),
    BufRead(&'a mut dyn BufRead),
}

impl<'a> fmt::Debug for BufReadOrReader<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufReadOrReader::BufReader(..) => write!(f, "BufReader(...)"),
            BufReadOrReader::BufRead(..) => write!(f, "BufRead(...)"),
        }
    }
}

impl<'a> Read for BufReadOrReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        match self {
            BufReadOrReader::BufReader(r) => r.read(buf),
            BufReadOrReader::BufRead(r) => r.read(buf),
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, io::Error> {
        match self {
            BufReadOrReader::BufReader(r) => r.read_to_end(buf),
            BufReadOrReader::BufRead(r) => r.read_to_end(buf),
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), io::Error> {
        match self {
            BufReadOrReader::BufReader(r) => r.read_exact(buf),
            BufReadOrReader::BufRead(r) => r.read_exact(buf),
        }
    }
}

impl<'a> BufReadOrReader<'a> {
    /// Similar to `read_exact` but reads into `MaybeUninit`.
    pub(crate) fn read_exact_uninit(
        &mut self,
        buf: &mut [MaybeUninit<u8>],
    ) -> Result<(), io::Error> {
        let mut pos = 0;
        while pos != buf.len() {
            let fill_buf = match self {
                BufReadOrReader::BufReader(r) => r.fill_buf()?,
                BufReadOrReader::BufRead(r) => r.fill_buf()?,
            };
            if fill_buf.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Unexpected end of file",
                ));
            }
            let consume = cmp::min(fill_buf.len(), buf.len() - pos);
            maybe_uninit_write_slice(&mut buf[pos..pos + consume], &fill_buf[..consume]);
            match self {
                BufReadOrReader::BufReader(r) => r.consume(consume),
                BufReadOrReader::BufRead(r) => r.consume(consume),
            }
            pos += consume;
        }
        Ok(())
    }

    pub(crate) fn skip_bytes(&mut self, count: usize) -> Result<(), io::Error> {
        let mut rem = count;
        while rem != 0 {
            let buf = self.fill_buf()?;
            if buf.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Unexpected end of file",
                ));
            }
            let consume = cmp::min(buf.len(), rem);
            self.consume(consume);
            rem -= consume;
        }
        Ok(())
    }
}

impl<'a> BufRead for BufReadOrReader<'a> {
    fn fill_buf(&mut self) -> Result<&[u8], io::Error> {
        match self {
            BufReadOrReader::BufReader(r) => r.fill_buf(),
            BufReadOrReader::BufRead(r) => r.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            BufReadOrReader::BufReader(r) => r.consume(amt),
            BufReadOrReader::BufRead(r) => r.consume(amt),
        }
    }
}
