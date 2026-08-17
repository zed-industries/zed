use crate::{BOOL, FALSE, TRUE};
use std::cmp;
use std::io;
use std::ptr;

use windows_sys::Win32::Foundation::{CloseHandle, ERROR_IO_INCOMPLETE, ERROR_IO_PENDING, HANDLE};
use windows_sys::Win32::Storage::FileSystem::{ReadFile, WriteFile};
use windows_sys::Win32::System::IO::{GetOverlappedResult, OVERLAPPED};

#[derive(Debug)]
pub struct Handle(HANDLE);

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl Handle {
    pub fn new(handle: HANDLE) -> Handle {
        Handle(handle)
    }

    pub fn raw(&self) -> HANDLE {
        self.0
    }

    pub fn into_raw(self) -> HANDLE {
        use std::mem;

        let ret = self.0;
        mem::forget(self);
        ret
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes = 0;
        let len = cmp::min(buf.len(), u32::MAX as usize) as u32;
        crate::cvt(unsafe {
            WriteFile(
                self.0,
                buf.as_ptr() as *const _,
                len,
                &mut bytes,
                std::ptr::null_mut(),
            )
        })?;
        Ok(bytes as usize)
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes = 0;
        let len = cmp::min(buf.len(), u32::MAX as usize) as u32;
        crate::cvt(unsafe {
            ReadFile(
                self.0,
                buf.as_mut_ptr() as *mut _,
                len,
                &mut bytes,
                std::ptr::null_mut(),
            )
        })?;
        Ok(bytes as usize)
    }

    pub unsafe fn read_overlapped(
        &self,
        buf: &mut [u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<Option<usize>> {
        self.read_overlapped_helper(buf, overlapped, FALSE)
    }

    pub unsafe fn read_overlapped_wait(
        &self,
        buf: &mut [u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<usize> {
        match self.read_overlapped_helper(buf, overlapped, TRUE) {
            Ok(Some(bytes)) => Ok(bytes),
            Ok(None) => panic!("logic error"),
            Err(e) => Err(e),
        }
    }

    pub unsafe fn read_overlapped_helper(
        &self,
        buf: &mut [u8],
        overlapped: *mut OVERLAPPED,
        wait: BOOL,
    ) -> io::Result<Option<usize>> {
        let len = cmp::min(buf.len(), u32::MAX as usize) as u32;
        let res = crate::cvt({
            ReadFile(
                self.0,
                buf.as_mut_ptr() as *mut _,
                len,
                ptr::null_mut(),
                overlapped,
            )
        });
        match res {
            Ok(_) => (),
            Err(ref e) if e.raw_os_error() == Some(ERROR_IO_PENDING as i32) => (),
            Err(e) => return Err(e),
        }

        let mut bytes = 0;
        let res = crate::cvt(GetOverlappedResult(self.0, overlapped, &mut bytes, wait));
        match res {
            Ok(_) => Ok(Some(bytes as usize)),
            Err(ref e) if e.raw_os_error() == Some(ERROR_IO_INCOMPLETE as i32) && wait == FALSE => {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    pub unsafe fn write_overlapped(
        &self,
        buf: &[u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<Option<usize>> {
        self.write_overlapped_helper(buf, overlapped, FALSE)
    }

    pub unsafe fn write_overlapped_wait(
        &self,
        buf: &[u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<usize> {
        match self.write_overlapped_helper(buf, overlapped, TRUE) {
            Ok(Some(bytes)) => Ok(bytes),
            Ok(None) => panic!("logic error"),
            Err(e) => Err(e),
        }
    }

    unsafe fn write_overlapped_helper(
        &self,
        buf: &[u8],
        overlapped: *mut OVERLAPPED,
        wait: BOOL,
    ) -> io::Result<Option<usize>> {
        let len = cmp::min(buf.len(), u32::MAX as usize) as u32;
        let res = crate::cvt({
            WriteFile(
                self.0,
                buf.as_ptr() as *const _,
                len,
                ptr::null_mut(),
                overlapped,
            )
        });
        match res {
            Ok(_) => (),
            Err(ref e) if e.raw_os_error() == Some(ERROR_IO_PENDING as i32) => (),
            Err(e) => return Err(e),
        }

        let mut bytes = 0;
        let res = crate::cvt(GetOverlappedResult(self.0, overlapped, &mut bytes, wait));
        match res {
            Ok(_) => Ok(Some(bytes as usize)),
            Err(ref e) if e.raw_os_error() == Some(ERROR_IO_INCOMPLETE as i32) && wait == FALSE => {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}
