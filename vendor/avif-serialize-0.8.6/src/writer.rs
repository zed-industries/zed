use std::convert::TryFrom;
use std::io;

pub struct OOM;

pub trait WriterBackend {
    type Error;
    fn reserve(&mut self, size: usize) -> Result<(), Self::Error>;
    fn extend_from_slice_in_capacity(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

/// `io::Write` generates bloated code (with backtrace for every byte written),
/// so small boxes are written infallibly.
impl WriterBackend for Vec<u8> {
    type Error = OOM;

    #[inline]
    fn reserve(&mut self, size: usize) -> Result<(), Self::Error> {
        self.try_reserve(size).map_err(|_| OOM)
    }

    #[inline(always)]
    fn extend_from_slice_in_capacity(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        let has_capacity = self.capacity() - self.len() >= data.len();
        debug_assert!(has_capacity);
        if has_capacity {
            self.extend_from_slice(data);
            Ok(())
        } else {
            Err(OOM)
        }
    }
}

pub struct IO<W>(pub W);

impl<W: io::Write> WriterBackend for IO<W> {
    type Error = io::Error;

    #[inline]
    fn reserve(&mut self, _size: usize) -> io::Result<()> {
        Ok(())
    }

    #[inline(always)]
    fn extend_from_slice_in_capacity(&mut self, data: &[u8]) -> io::Result<()> {
        self.0.write_all(data)
    }
}

pub struct Writer<'p, 'w, B> {
    out: &'w mut B,
    #[cfg(debug_assertions)]
    parent: Option<&'p mut usize>,
    #[cfg(not(debug_assertions))]
    parent: std::marker::PhantomData<&'p mut usize>,
    #[cfg(debug_assertions)]
    left: usize,
}

impl<'w, B> Writer<'static, 'w, B> {
    #[inline]
    pub fn new(out: &'w mut B) -> Self {
        Self {
            parent: Default::default(),
            #[cfg(debug_assertions)]
            left: 0,
            out,
        }
    }
}

impl<B: WriterBackend> Writer<'_, '_, B> {
    #[inline(always)]
    pub fn full_box(&mut self, len: usize, typ: [u8; 4], version: u8) -> Result<Writer<'_, '_, B>, B::Error> {
        let mut b = self.basic_box(len, typ)?;
        b.push(&[version, 0, 0, 0])?;
        Ok(b)
    }

    #[inline]
    pub fn basic_box(&mut self, len: usize, typ: [u8; 4]) -> Result<Writer<'_, '_, B>, B::Error> {
        let mut b = Writer {
            out: self.out,
            parent: Default::default(),
            #[cfg(debug_assertions)]
            left: len,
        };
        #[cfg(debug_assertions)]
        if self.left > 0 {
            self.left -= len;
            b.parent = Some(&mut self.left);
        } else {
            debug_assert!(self.parent.is_none());
        }
        b.out.reserve(len)?;

        if let Ok(len) = u32::try_from(len) {
            b.u32(len)?;
        } else {
            debug_assert!(false, "constants for box size don't include this");
            b.u32(1)?;
            b.u64(len as u64)?;
        }
        b.push(&typ)?;
        Ok(b)
    }

    #[inline(always)]
    pub fn push(&mut self, data: &[u8]) -> Result<(), B::Error> {
        #[cfg(debug_assertions)] {
            self.left -= data.len();
        }
        self.out.extend_from_slice_in_capacity(data)
    }

    #[inline(always)]
    pub fn u8(&mut self, val: u8) -> Result<(), B::Error> {
        self.push(std::slice::from_ref(&val))
    }

    #[inline(always)]
    pub fn u16(&mut self, val: u16) -> Result<(), B::Error> {
        self.push(&val.to_be_bytes())
    }

    #[inline(always)]
    pub fn u32(&mut self, val: u32) -> Result<(), B::Error> {
        self.push(&val.to_be_bytes())
    }

    #[inline(always)]
    pub fn u64(&mut self, val: u64) -> Result<(), B::Error> {
        self.push(&val.to_be_bytes())
    }
}

#[cfg(debug_assertions)]
impl<B> Drop for Writer<'_, '_, B> {
    fn drop(&mut self) {
        assert_eq!(self.left, 0);
    }
}
