//! Module for importing functions from ntdll.dll.
//! The windows-sys crate does not expose these Windows API functions.

#![allow(nonstandard_style)]

use std::ffi::c_void;
use std::os::raw::c_ulong;
use std::os::windows::io::BorrowedHandle;
use std::sync::atomic::{AtomicUsize, Ordering};
use windows_sys::Win32::Foundation::NTSTATUS;
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows_sys::Win32::System::IO::IO_STATUS_BLOCK;

// https://docs.microsoft.com/en-us/windows-hardware/drivers/kernel/access-mask
type ACCESS_MASK = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) enum FILE_INFORMATION_CLASS {
    FileAccessInformation = 8,
    FileModeInformation = 16,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct FILE_ACCESS_INFORMATION {
    pub AccessFlags: ACCESS_MASK,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct FILE_MODE_INFORMATION {
    pub Mode: c_ulong,
}

macro_rules! ntdll_import {
    { fn $name:ident($($arg:ident: $argty:ty),*) -> $retty:ty; $($tail:tt)* } => {
        pub(crate) unsafe fn $name($($arg: $argty),*) -> $retty {
            static ADDRESS: AtomicUsize = AtomicUsize::new(0);
            let address = match ADDRESS.load(Ordering::Relaxed) {
                0 => {
                    let ntdll = GetModuleHandleA("ntdll\0".as_ptr() as *const u8);
                    let address: usize = std::mem::transmute(GetProcAddress(
                        ntdll,
                        concat!(stringify!($name), "\0").as_ptr() as *const u8,
                    ).unwrap());
                    ADDRESS.store(address, Ordering::Relaxed);
                    address
                }
                address => address
            };
            let func: unsafe fn($($argty),*) -> $retty = std::mem::transmute(address);
            func($($arg),*)
        }
        ntdll_import! { $($tail)* }
    };
    {} => {};
}

ntdll_import! {
    // https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntqueryinformationfile
    fn NtQueryInformationFile(
        FileHandle: BorrowedHandle<'_>,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        FileInformation: *mut c_void,
        Length: c_ulong,
        FileInformationClass: FILE_INFORMATION_CLASS
    ) -> NTSTATUS;
}
