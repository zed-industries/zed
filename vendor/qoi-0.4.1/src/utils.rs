#[cfg(feature = "std")]
use std::io::Write;

use crate::error::Result;

#[inline(always)]
#[cold]
pub const fn cold() {}

#[inline(always)]
#[allow(unused)]
pub const fn likely(b: bool) -> bool {
    if !b {
        cold();
    }
    b
}

#[inline(always)]
pub const fn unlikely(b: bool) -> bool {
    if b {
        cold();
    }
    b
}

pub trait Writer: Sized {
    fn write_one(self, v: u8) -> Result<Self>;
    fn write_many(self, v: &[u8]) -> Result<Self>;
    fn capacity(&self) -> usize;
}

pub struct BytesMut<'a>(&'a mut [u8]);

impl<'a> BytesMut<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self(buf)
    }

    #[inline]
    pub fn write_one(self, v: u8) -> Self {
        if let Some((first, tail)) = self.0.split_first_mut() {
            *first = v;
            Self(tail)
        } else {
            unreachable!()
        }
    }

    #[inline]
    pub fn write_many(self, v: &[u8]) -> Self {
        if v.len() <= self.0.len() {
            let (head, tail) = self.0.split_at_mut(v.len());
            head.copy_from_slice(v);
            Self(tail)
        } else {
            unreachable!()
        }
    }
}

impl<'a> Writer for BytesMut<'a> {
    #[inline]
    fn write_one(self, v: u8) -> Result<Self> {
        Ok(BytesMut::write_one(self, v))
    }

    #[inline]
    fn write_many(self, v: &[u8]) -> Result<Self> {
        Ok(BytesMut::write_many(self, v))
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.0.len()
    }
}

#[cfg(feature = "std")]
pub struct GenericWriter<W> {
    writer: W,
    n_written: usize,
}

#[cfg(feature = "std")]
impl<W: Write> GenericWriter<W> {
    pub const fn new(writer: W) -> Self {
        Self { writer, n_written: 0 }
    }
}

#[cfg(feature = "std")]
impl<W: Write> Writer for GenericWriter<W> {
    fn write_one(mut self, v: u8) -> Result<Self> {
        self.n_written += 1;
        self.writer.write_all(&[v]).map(|_| self).map_err(Into::into)
    }

    fn write_many(mut self, v: &[u8]) -> Result<Self> {
        self.n_written += v.len();
        self.writer.write_all(v).map(|_| self).map_err(Into::into)
    }

    fn capacity(&self) -> usize {
        usize::MAX - self.n_written
    }
}
