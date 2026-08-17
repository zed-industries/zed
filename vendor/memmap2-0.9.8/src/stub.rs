use std::fs::File;
use std::io;

// A stable alternative to https://doc.rust-lang.org/stable/std/primitive.never.html
enum Never {}

pub struct MmapInner {
    never: Never,
}

impl MmapInner {
    fn new() -> io::Result<MmapInner> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "platform not supported",
        ))
    }

    pub fn map(_: usize, _: &File, _: u64, _: bool, _: bool) -> io::Result<MmapInner> {
        MmapInner::new()
    }

    pub fn map_exec(_: usize, _: &File, _: u64, _: bool, _: bool) -> io::Result<MmapInner> {
        MmapInner::new()
    }

    pub fn map_mut(_: usize, _: &File, _: u64, _: bool, _: bool) -> io::Result<MmapInner> {
        MmapInner::new()
    }

    pub fn map_copy(_: usize, _: &File, _: u64, _: bool, _: bool) -> io::Result<MmapInner> {
        MmapInner::new()
    }

    pub fn map_copy_read_only(
        _: usize,
        _: &File,
        _: u64,
        _: bool,
        _: bool,
    ) -> io::Result<MmapInner> {
        MmapInner::new()
    }

    pub fn map_anon(_: usize, _: bool, _: bool, _: Option<u8>, _: bool) -> io::Result<MmapInner> {
        MmapInner::new()
    }

    pub fn flush(&self, _: usize, _: usize) -> io::Result<()> {
        match self.never {}
    }

    pub fn flush_async(&self, _: usize, _: usize) -> io::Result<()> {
        match self.never {}
    }

    pub fn make_read_only(&mut self) -> io::Result<()> {
        match self.never {}
    }

    pub fn make_exec(&mut self) -> io::Result<()> {
        match self.never {}
    }

    pub fn make_mut(&mut self) -> io::Result<()> {
        match self.never {}
    }

    #[inline]
    pub fn ptr(&self) -> *const u8 {
        match self.never {}
    }

    #[inline]
    pub fn mut_ptr(&mut self) -> *mut u8 {
        match self.never {}
    }

    #[inline]
    pub fn len(&self) -> usize {
        match self.never {}
    }
}

pub fn file_len(file: &File) -> io::Result<u64> {
    Ok(file.metadata()?.len())
}
