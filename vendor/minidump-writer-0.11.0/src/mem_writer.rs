use {
    crate::{
        minidump_format::{MDLocationDescriptor, MDRVA},
        serializers::*,
    },
    scroll::ctx::{SizeWith, TryIntoCtx},
};

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum MemoryWriterError {
    #[error("IO error when writing to DumpBuf")]
    IOError(
        #[from]
        #[serde(serialize_with = "serialize_io_error")]
        std::io::Error,
    ),
    #[error("Failed integer conversion")]
    TryFromIntError(
        #[from]
        #[serde(skip)]
        std::num::TryFromIntError,
    ),
    #[error("Failed to write to buffer")]
    Scroll(
        #[from]
        #[serde(serialize_with = "serialize_scroll_error")]
        scroll::Error,
    ),
}

type WriteResult<T> = std::result::Result<T, MemoryWriterError>;

macro_rules! size {
    ($t:ty) => {
        <$t>::size_with(&scroll::Endian::Little)
    };
}

pub struct Buffer {
    inner: Vec<u8>,
}

impl Buffer {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: Vec::with_capacity(cap),
        }
    }

    #[inline]
    pub fn position(&self) -> u64 {
        self.inner.len() as u64
    }

    #[inline]
    #[must_use]
    fn reserve(&mut self, len: usize) -> usize {
        let mark = self.inner.len();
        self.inner.resize(self.inner.len() + len, 0);
        mark
    }

    #[inline]
    fn write<N, E>(&mut self, val: N) -> Result<usize, E>
    where
        N: TryIntoCtx<scroll::Endian, Error = E> + SizeWith<scroll::Endian>,
        E: From<scroll::Error>,
    {
        self.write_at(self.inner.len(), val)
    }

    fn write_at<N, E>(&mut self, offset: usize, val: N) -> Result<usize, E>
    where
        N: TryIntoCtx<scroll::Endian, Error = E> + SizeWith<scroll::Endian>,
        E: From<scroll::Error>,
    {
        let to_write = size!(N);
        let remainder = self.inner.len() - offset;
        if remainder < to_write {
            self.inner
                .resize(self.inner.len() + to_write - remainder, 0);
        }

        let dst = &mut self.inner[offset..offset + to_write];
        val.try_into_ctx(dst, scroll::Endian::Little)
    }

    #[inline]
    pub fn write_all(&mut self, buffer: &[u8]) {
        self.inner.extend_from_slice(buffer);
    }
}

impl From<Buffer> for Vec<u8> {
    fn from(b: Buffer) -> Self {
        b.inner
    }
}

impl std::ops::Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug)]
pub struct MemoryWriter<T> {
    pub position: MDRVA,
    pub size: usize,
    phantom: std::marker::PhantomData<T>,
}

impl<T> MemoryWriter<T>
where
    T: TryIntoCtx<scroll::Endian, Error = scroll::Error> + SizeWith<scroll::Endian>,
{
    /// Create a slot for a type T in the buffer, we can fill right now with real values.
    pub fn alloc_with_val(buffer: &mut Buffer, val: T) -> WriteResult<Self> {
        // Mark the position as we may overwrite later
        let position = buffer.position();
        let size = buffer.write(val)?;

        Ok(Self {
            position: position as u32,
            size,
            phantom: std::marker::PhantomData,
        })
    }

    /// Create a slot for a type T in the buffer, we can fill later with real values.
    pub fn alloc(buffer: &mut Buffer) -> WriteResult<Self> {
        let size = size!(T);
        let position = buffer.reserve(size) as u32;

        Ok(Self {
            position,
            size,
            phantom: std::marker::PhantomData,
        })
    }

    /// Write actual values in the buffer-slot we got during `alloc()`
    #[inline]
    pub fn set_value(&mut self, buffer: &mut Buffer, val: T) -> WriteResult<()> {
        Ok(buffer.write_at(self.position as usize, val).map(|_sz| ())?)
    }

    #[inline]
    pub fn location(&self) -> MDLocationDescriptor {
        MDLocationDescriptor {
            data_size: size!(T) as u32,
            rva: self.position,
        }
    }
}

#[derive(Debug)]
pub struct MemoryArrayWriter<T> {
    pub position: MDRVA,
    array_size: usize,
    phantom: std::marker::PhantomData<T>,
}

#[cfg(any(target_os = "linux", target_os = "android"))]
impl MemoryArrayWriter<u8> {
    #[inline]
    pub fn write_bytes(buffer: &mut Buffer, slice: &[u8]) -> Self {
        let position = buffer.position();
        buffer.write_all(slice);

        Self {
            position: position as u32,
            array_size: slice.len(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> MemoryArrayWriter<T>
where
    T: TryIntoCtx<scroll::Endian, Error = scroll::Error> + SizeWith<scroll::Endian> + Copy,
{
    pub fn alloc_from_array(buffer: &mut Buffer, array: &[T]) -> WriteResult<Self> {
        let array_size = array.len();
        let position = buffer.reserve(array_size * size!(T));

        for (idx, val) in array.iter().enumerate() {
            buffer.write_at(position + idx * size!(T), *val)?;
        }

        Ok(Self {
            position: position as u32,
            array_size,
            phantom: std::marker::PhantomData,
        })
    }
}

impl<T> MemoryArrayWriter<T>
where
    T: TryIntoCtx<scroll::Endian, Error = scroll::Error> + SizeWith<scroll::Endian>,
{
    /// Create a slot for a type T in the buffer, we can fill in the values in one go.
    pub fn alloc_from_iter<I>(
        buffer: &mut Buffer,
        iter: impl IntoIterator<Item = T, IntoIter = I>,
    ) -> WriteResult<Self>
    where
        I: std::iter::ExactSizeIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let array_size = iter.len();
        let size = size!(T);
        let position = buffer.reserve(array_size * size);

        for (idx, val) in iter.enumerate() {
            buffer.write_at(position + idx * size, val)?;
        }

        Ok(Self {
            position: position as u32,
            array_size,
            phantom: std::marker::PhantomData,
        })
    }

    /// Create a slot for a type T in the buffer, we can fill later with real values.
    /// This function fills it with `Default::default()`, which is less performant than
    /// using uninitialized memory, but safe.
    pub fn alloc_array(buffer: &mut Buffer, array_size: usize) -> WriteResult<Self> {
        let position = buffer.reserve(array_size * size!(T));

        Ok(Self {
            position: position as u32,
            array_size,
            phantom: std::marker::PhantomData,
        })
    }

    /// Write actual values in the buffer-slot we got during `alloc()`
    #[inline]
    pub fn set_value_at(&mut self, buffer: &mut Buffer, val: T, index: usize) -> WriteResult<()> {
        Ok(buffer
            .write_at(self.position as usize + size!(T) * index, val)
            .map(|_sz| ())?)
    }

    #[inline]
    pub fn location(&self) -> MDLocationDescriptor {
        MDLocationDescriptor {
            data_size: (self.array_size * size!(T)) as u32,
            rva: self.position,
        }
    }

    #[inline]
    pub fn location_of_index(&self, idx: usize) -> MDLocationDescriptor {
        MDLocationDescriptor {
            data_size: size!(T) as u32,
            rva: self.position + (size!(T) * idx) as u32,
        }
    }
}

pub fn write_string_to_location(
    buffer: &mut Buffer,
    text: &str,
) -> WriteResult<MDLocationDescriptor> {
    let letters: Vec<u16> = text.encode_utf16().collect();

    // First write size of the string (x letters in u16, times the size of u16)
    let text_header = MemoryWriter::<u32>::alloc_with_val(
        buffer,
        (letters.len() * std::mem::size_of::<u16>()).try_into()?,
    )?;

    // Then write utf-16 letters after that
    let mut text_section = MemoryArrayWriter::<u16>::alloc_array(buffer, letters.len())?;
    for (index, letter) in letters.iter().enumerate() {
        text_section.set_value_at(buffer, *letter, index)?;
    }

    let mut location = text_header.location();
    location.data_size += text_section.location().data_size;

    Ok(location)
}
