#![allow(unsafe_code)]

use crate::fs::OpenOptions;
use std::io;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{ERROR_INVALID_PARAMETER, GENERIC_READ, GENERIC_WRITE};
use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
use windows_sys::Win32::Storage::FileSystem::{
    CREATE_ALWAYS, CREATE_NEW, FILE_FLAG_OPEN_REPARSE_POINT, FILE_GENERIC_WRITE, FILE_SHARE_DELETE,
    FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_WRITE_DATA, OPEN_ALWAYS, OPEN_EXISTING,
    SECURITY_SQOS_PRESENT, TRUNCATE_EXISTING,
};

#[derive(Debug, Clone)]
pub(crate) struct ImplOpenOptionsExt {
    pub(super) access_mode: Option<u32>,
    pub(super) share_mode: u32,
    pub(super) custom_flags: u32,
    pub(super) attributes: u32,
    pub(super) security_attributes: *mut SECURITY_ATTRIBUTES,
    pub(super) security_qos_flags: u32,
}

unsafe impl Send for ImplOpenOptionsExt {}
unsafe impl Sync for ImplOpenOptionsExt {}

impl ImplOpenOptionsExt {
    pub(crate) const fn new() -> Self {
        Self {
            access_mode: None,
            share_mode: FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            custom_flags: 0,
            attributes: 0,
            security_attributes: null_mut(),
            security_qos_flags: 0,
        }
    }

    pub(crate) fn access_mode(&mut self, mode: u32) -> &mut Self {
        self.access_mode = Some(mode);
        self
    }

    pub(crate) fn share_mode(&mut self, share: u32) -> &mut Self {
        self.share_mode = share;
        self
    }

    pub(crate) fn custom_flags(&mut self, flags: u32) -> &mut Self {
        self.custom_flags = flags;
        self
    }

    pub(crate) fn attributes(&mut self, attributes: u32) -> &mut Self {
        self.attributes = attributes;
        self
    }

    pub(crate) fn security_qos_flags(&mut self, flags: u32) -> &mut Self {
        self.security_qos_flags = flags | SECURITY_SQOS_PRESENT;
        self
    }
}

pub(crate) fn get_access_mode(options: &OpenOptions) -> io::Result<u32> {
    match (
        options.read,
        options.write,
        options.append,
        options.ext.access_mode,
    ) {
        (.., Some(mode)) => Ok(mode),
        (true, false, false, None) => Ok(GENERIC_READ),
        (false, true, false, None) => Ok(GENERIC_WRITE),
        (true, true, false, None) => Ok(GENERIC_READ | GENERIC_WRITE),
        (false, _, true, None) => Ok(FILE_GENERIC_WRITE & !FILE_WRITE_DATA),
        (true, _, true, None) => Ok(GENERIC_READ | (FILE_GENERIC_WRITE & !FILE_WRITE_DATA)),
        (false, false, false, None) => {
            Err(io::Error::from_raw_os_error(ERROR_INVALID_PARAMETER as i32))
        }
    }
}

pub(crate) fn get_flags_and_attributes(options: &OpenOptions) -> u32 {
    options.ext.custom_flags
        | options.ext.attributes
        | options.ext.security_qos_flags
        | if options.create_new {
            FILE_FLAG_OPEN_REPARSE_POINT
        } else {
            0
        }
}

pub(crate) fn get_creation_mode(options: &OpenOptions) -> io::Result<u32> {
    const ERROR_INVALID_PARAMETER: i32 = 87;

    match (options.write, options.append) {
        (true, false) => {}
        (false, false) => {
            if options.truncate || options.create || options.create_new {
                return Err(io::Error::from_raw_os_error(ERROR_INVALID_PARAMETER));
            }
        }
        (_, true) => {
            if options.truncate && !options.create_new {
                return Err(io::Error::from_raw_os_error(ERROR_INVALID_PARAMETER));
            }
        }
    }

    Ok(
        match (options.create, options.truncate, options.create_new) {
            (false, false, false) => OPEN_EXISTING,
            (true, false, false) => OPEN_ALWAYS,
            (false, true, false) => TRUNCATE_EXISTING,
            (true, true, false) => CREATE_ALWAYS,
            (_, _, true) => CREATE_NEW,
        },
    )
}
