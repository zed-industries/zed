use std::str::from_utf8;

use bytes::{Buf, Bytes};
use memchr::memchr;

use crate::error::Error;

pub trait BufExt: Buf {
    // Read a nul-terminated byte sequence
    fn get_bytes_nul(&mut self) -> Result<Bytes, Error>;

    // Read a byte sequence of the exact length
    fn get_bytes(&mut self, len: usize) -> Bytes;

    // Read a nul-terminated string
    fn get_str_nul(&mut self) -> Result<String, Error>;

    // Read a string of the exact length
    fn get_str(&mut self, len: usize) -> Result<String, Error>;
}

impl BufExt for Bytes {
    fn get_bytes_nul(&mut self) -> Result<Bytes, Error> {
        let nul =
            memchr(b'\0', self).ok_or_else(|| err_protocol!("expected NUL in byte sequence"))?;

        let v = self.slice(0..nul);

        self.advance(nul + 1);

        Ok(v)
    }

    fn get_bytes(&mut self, len: usize) -> Bytes {
        let v = self.slice(..len);
        self.advance(len);

        v
    }

    fn get_str_nul(&mut self) -> Result<String, Error> {
        self.get_bytes_nul().and_then(|bytes| {
            from_utf8(&bytes)
                .map(ToOwned::to_owned)
                .map_err(|err| err_protocol!("{}", err))
        })
    }

    fn get_str(&mut self, len: usize) -> Result<String, Error> {
        let v = from_utf8(&self[..len])
            .map_err(|err| err_protocol!("{}", err))
            .map(ToOwned::to_owned)?;

        self.advance(len);

        Ok(v)
    }
}
