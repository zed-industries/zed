use crate::vk;
use core::ffi::c_void;
use core::iter::Iterator;
use core::marker::PhantomData;
use core::mem::size_of;
use core::slice;
#[cfg(feature = "std")]
use std::io;

/// [`Align`] handles dynamic alignment. The is useful for dynamic uniform buffers where
/// the alignment might be different. For example a 4x4 f32 matrix has a size of 64 bytes
/// but the min alignment for a dynamic uniform buffer might be 256 bytes. A slice of `&[Mat4x4<f32>]`
/// has a memory layout of `[[64 bytes], [64 bytes], [64 bytes]]`, but it might need to have a memory
/// layout of `[[256 bytes], [256 bytes], [256 bytes]]`.
/// [`Align::copy_from_slice`] will copy a slice of `&[T]` directly into the host memory without
/// an additional allocation and with the correct alignment.
#[derive(Debug, Clone)]
pub struct Align<T> {
    ptr: *mut c_void,
    elem_size: vk::DeviceSize,
    size: vk::DeviceSize,
    _m: PhantomData<T>,
}

#[derive(Debug)]
pub struct AlignIter<'a, T> {
    align: &'a mut Align<T>,
    current: vk::DeviceSize,
}

impl<T: Copy> Align<T> {
    pub fn copy_from_slice(&mut self, slice: &[T]) {
        if self.elem_size == size_of::<T>() as u64 {
            unsafe {
                let mapped_slice = slice::from_raw_parts_mut(self.ptr.cast(), slice.len());
                mapped_slice.copy_from_slice(slice);
            }
        } else {
            for (i, val) in self.iter_mut().enumerate().take(slice.len()) {
                *val = slice[i];
            }
        }
    }
}

fn calc_padding(adr: vk::DeviceSize, align: vk::DeviceSize) -> vk::DeviceSize {
    (align - adr % align) % align
}

impl<T> Align<T> {
    pub unsafe fn new(ptr: *mut c_void, alignment: vk::DeviceSize, size: vk::DeviceSize) -> Self {
        let padding = calc_padding(size_of::<T>() as vk::DeviceSize, alignment);
        let elem_size = size_of::<T>() as vk::DeviceSize + padding;
        assert!(calc_padding(size, alignment) == 0, "size must be aligned");
        Self {
            ptr,
            elem_size,
            size,
            _m: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> AlignIter<'_, T> {
        AlignIter {
            current: 0,
            align: self,
        }
    }
}

impl<'a, T: Copy + 'a> Iterator for AlignIter<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.align.size {
            return None;
        }
        unsafe {
            // Need to cast to *mut u8 because () has size 0
            let ptr = (self.align.ptr.cast::<u8>())
                .offset(self.current as isize)
                .cast();
            self.current += self.align.elem_size;
            Some(&mut *ptr)
        }
    }
}

/// Decode SPIR-V from bytes.
///
/// This function handles SPIR-V of arbitrary endianness gracefully, and returns correctly aligned
/// storage.
///
/// # Examples
/// ```no_run
/// // Decode SPIR-V from a file
/// let mut file = std::fs::File::open("/path/to/shader.spv").unwrap();
/// let words = ash::util::read_spv(&mut file).unwrap();
/// ```
/// ```
/// // Decode SPIR-V from memory
/// const SPIRV: &[u8] = &[
///     // ...
/// #   0x03, 0x02, 0x23, 0x07,
/// ];
/// let words = ash::util::read_spv(&mut std::io::Cursor::new(&SPIRV[..])).unwrap();
/// ```
#[cfg(feature = "std")]
pub fn read_spv<R: io::Read + io::Seek>(x: &mut R) -> io::Result<Vec<u32>> {
    // TODO use stream_len() once it is stabilized and remove the subsequent rewind() call
    let size = x.seek(io::SeekFrom::End(0))?;
    x.rewind()?;
    if size % 4 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input length not divisible by 4",
        ));
    }
    if size > usize::MAX as u64 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "input too long"));
    }
    let words = (size / 4) as usize;
    // https://github.com/ash-rs/ash/issues/354:
    // Zero-initialize the result to prevent read_exact from possibly
    // reading uninitialized memory.
    let mut result = vec![0u32; words];
    x.read_exact(unsafe {
        slice::from_raw_parts_mut(result.as_mut_ptr().cast::<u8>(), words * 4)
    })?;
    const MAGIC_NUMBER: u32 = 0x0723_0203;
    if !result.is_empty() && result[0] == MAGIC_NUMBER.swap_bytes() {
        for word in &mut result {
            *word = word.swap_bytes();
        }
    }
    if result.is_empty() || result[0] != MAGIC_NUMBER {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input missing SPIR-V magic number",
        ));
    }
    Ok(result)
}
