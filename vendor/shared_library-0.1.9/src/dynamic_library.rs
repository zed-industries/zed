// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Dynamic library facilities.
//!
//! A simple wrapper over the platform's dynamic library facilities

#![allow(missing_docs)]

use std::env;
use std::ffi::{CString, OsString};
use std::mem;
use std::path::{Path, PathBuf};
use libc;

pub struct DynamicLibrary {
    handle: *mut u8
}

unsafe impl Send for DynamicLibrary {}
unsafe impl Sync for DynamicLibrary {}

impl Drop for DynamicLibrary {
    fn drop(&mut self) {
        if let Err(str) = dl::check_for_errors_in(|| unsafe {
            dl::close(self.handle)
        }) {
            panic!("{}", str)
        }
    }
}

/// Special handles to be used with the `symbol_special` function. These are 
/// provided by a GNU only extension and are not included as part of the POSIX 
/// standard. 
///
/// See https://linux.die.net/man/3/dlsym for their behaviour.
#[cfg(target_os = "linux")]
pub enum SpecialHandles {
    Next,
    Default,
}

impl DynamicLibrary {
    // FIXME (#12938): Until DST lands, we cannot decompose &str into
    // & and str, so we cannot usefully take ToCStr arguments by
    // reference (without forcing an additional & around &str). So we
    // are instead temporarily adding an instance for &Path, so that
    // we can take ToCStr as owned. When DST lands, the &Path instance
    // should be removed, and arguments bound by ToCStr should be
    // passed by reference. (Here: in the `open` method.)

    /// Lazily loads the dynamic library named `filename` into memory and 
    /// then returns an opaque "handle" for that dynamic library.
    ///
    /// Returns a handle to the calling process when passed `None`.
    pub fn open(filename: Option<&Path>) -> Result<Self, String> {
        // The dynamic library must not be constructed if there is
        // an error opening the library so the destructor does not
        // run.
        dl::open(filename.map(|path| path.as_os_str()))
            .map(|handle| DynamicLibrary { handle })
    }

    /// Prepends a path to this process's search path for dynamic libraries
    pub fn prepend_search_path(path: &Path) {
        let mut search_path = Self::search_path();
        search_path.insert(0, path.to_path_buf());
        env::set_var(Self::envvar(), &Self::create_path(&search_path));
    }

    /// From a slice of paths, create a new vector which is suitable to be an
    /// environment variable for this platforms dylib search path.
    pub fn create_path(path: &[PathBuf]) -> OsString {
        let mut newvar = OsString::new();
        for (i, path) in path.iter().enumerate() {
            if i > 0 { newvar.push(Self::separator()); }
            newvar.push(path);
        }
        newvar
    }

    /// Returns the environment variable for this process's dynamic library
    /// search path
    pub fn envvar() -> &'static str {
        if cfg!(windows) {
            "PATH"
        } else if cfg!(target_os = "macos") {
            "DYLD_LIBRARY_PATH"
        } else {
            "LD_LIBRARY_PATH"
        }
    }

    //TODO: turn this and `envvar` into associated constants
    fn separator() -> &'static str {
        if cfg!(windows) { ";" } else { ":" }
    }

    /// Returns the current search path for dynamic libraries being used by this
    /// process
    pub fn search_path() -> Vec<PathBuf> {
        match env::var_os(Self::envvar()) {
            Some(var) => env::split_paths(&var).collect(),
            None => Vec::new(),
        }
    }

    /// Returns the address of where symbol `symbol` was loaded into memory.
    ///
    /// In POSIX compliant systems, we return 'Err' if the symbol was not found, 
    /// in this library or any of the libraries that were automatically loaded 
    /// when this library was loaded.
    pub unsafe fn symbol<T>(&self, symbol: &str) -> Result<*mut T, String> {
        // This function should have a lifetime constraint of 'a on
        // T but that feature is still unimplemented

        let raw_string = CString::new(symbol).unwrap();
        // The value must not be constructed if there is an error so
        // the destructor does not run.
        dl::check_for_errors_in(|| {
                dl::symbol(self.handle as *mut libc::c_void, raw_string.as_ptr() as *const _)
            })
            .map(|sym| mem::transmute(sym))
    }

    /// Returns the address of the first occurance of symbol `symbol` using the 
    /// default library search order if you use `SpecialHandles::Default`.
    ///
    /// Returns the address of the next occurance of symbol `symbol` after the 
    /// current library in the default library search order if you use 
    /// `SpecialHandles::Next`.
    #[cfg(target_os = "linux")]
    pub unsafe fn symbol_special<T>(handle: SpecialHandles, symbol: &str) -> Result<*mut T, String> {
        // This function should have a lifetime constraint of 'a on
        // T but that feature is still unimplemented

        let handle = match handle {
            SpecialHandles::Next => mem::transmute::<libc::c_long, _>(-1),
            SpecialHandles::Default => ::std::ptr::null_mut(),
        };

        let raw_string = CString::new(symbol).unwrap();
        // The value must not be constructed if there is an error so
        // the destructor does not run.
        dl::check_for_errors_in(|| {
                dl::symbol(handle, raw_string.as_ptr() as *const _)
            })
            .map(|sym| mem::transmute(sym))
    }
}

#[cfg(all(test, not(target_os = "ios")))]
mod test {
    use super::*;
    use std::mem;
    use std::path::Path;

    #[test]
    #[cfg_attr(any(windows, target_os = "android"), ignore)] // FIXME #8818, #10379
    fn test_loading_cosine() {
        // The math library does not need to be loaded since it is already
        // statically linked in
        let libm = match DynamicLibrary::open(None) {
            Err(error) => panic!("Could not load self as module: {}", error),
            Ok(libm) => libm
        };

        let cosine: extern fn(libc::c_double) -> libc::c_double = unsafe {
            match libm.symbol("cos") {
                Err(error) => panic!("Could not load function cos: {}", error),
                Ok(cosine) => mem::transmute::<*mut u8, _>(cosine)
            }
        };

        let argument = 0.0;
        let expected_result = 1.0;
        let result = cosine(argument);
        if result != expected_result {
            panic!("cos({}) != {} but equaled {} instead", argument,
                   expected_result, result)
        }
    }

    #[test]
    #[cfg(any(target_os = "linux",
              target_os = "macos",
              target_os = "freebsd",
              target_os = "fuchsia",
              target_os = "netbsd",
              target_os = "dragonfly",
              target_os = "bitrig",
              target_os = "openbsd",
              target_os = "solaris"))]
    fn test_errors_do_not_crash() {
        // Open /dev/null as a library to get an error, and make sure
        // that only causes an error, and not a crash.
        let path = Path::new("/dev/null");
        match DynamicLibrary::open(Some(&path)) {
            Err(_) => {}
            Ok(_) => panic!("Successfully opened the empty library.")
        }
    }
}

//TODO: use `unix` shortcut?
#[cfg(any(target_os = "linux",
          target_os = "android",
          target_os = "macos",
          target_os = "ios",
          target_os = "fuchsia",
          target_os = "freebsd",
          target_os = "netbsd",
          target_os = "dragonfly",
          target_os = "bitrig",
          target_os = "openbsd",
          target_os = "solaris",
          target_os = "emscripten"))]
mod dl {
    use std::ffi::{CString, CStr, OsStr};
    use std::os::unix::ffi::OsStrExt;
    use std::str;
    use libc;
    use std::ptr;
    use std::sync::Mutex;

    lazy_static! {
        static ref LOCK: Mutex<()> = Mutex::new(());
    }

    pub fn open(filename: Option<&OsStr>) -> Result<*mut u8, String> {
        check_for_errors_in(|| unsafe {
            match filename {
                Some(filename) => open_external(filename),
                None => open_internal(),
            }
        })
    }

    const LAZY: libc::c_int = 1;

    unsafe fn open_external(filename: &OsStr) -> *mut u8 {
        let s = CString::new(filename.as_bytes().to_vec()).unwrap();
        dlopen(s.as_ptr() as *const _, LAZY) as *mut u8
    }

    unsafe fn open_internal() -> *mut u8 {
        dlopen(ptr::null(), LAZY) as *mut u8
    }

    pub fn check_for_errors_in<T, F>(f: F) -> Result<T, String> where
        F: FnOnce() -> T,
    {
        unsafe {
            // dlerror isn't thread safe, so we need to lock around this entire
            // sequence
            let _guard = LOCK.lock();
            let _old_error = dlerror();

            let result = f();

            let last_error = dlerror() as *const _;
            let ret = if ptr::null() == last_error {
                Ok(result)
            } else {
                let s = CStr::from_ptr(last_error).to_bytes();
                Err(str::from_utf8(s).unwrap().to_string())
            };

            ret
        }
    }

    pub unsafe fn symbol(
        handle: *mut libc::c_void,
        symbol: *const libc::c_char,
    ) -> *mut u8 {
        dlsym(handle, symbol) as *mut u8
    }

    pub unsafe fn close(handle: *mut u8) {
        dlclose(handle as *mut libc::c_void); ()
    }

    extern {
        fn dlopen(
            filename: *const libc::c_char,
            flag: libc::c_int,
        ) -> *mut libc::c_void;
        fn dlerror() -> *mut libc::c_char;
        fn dlsym(
            handle: *mut libc::c_void,
            symbol: *const libc::c_char,
        ) -> *mut libc::c_void;
        fn dlclose(
            handle: *mut libc::c_void,
        ) -> libc::c_int;
    }
}

#[cfg(target_os = "windows")]
mod dl {
    use std::ffi::OsStr;
    use std::iter::Iterator;
    use libc;
    use std::ops::FnOnce;
    use std::io::Error as IoError;
    use std::os::windows::prelude::*;
    use std::option::Option::{self, Some, None};
    use std::ptr;
    use std::result::Result;
    use std::result::Result::{Ok, Err};
    use std::string::String;
    use std::vec::Vec;

    pub fn open(filename: Option<&OsStr>) -> Result<*mut u8, String> {
        // disable "dll load failed" error dialog.
        let prev_error_mode = unsafe {
            // SEM_FAILCRITICALERRORS 0x01
            let new_error_mode = 1;
            SetErrorMode(new_error_mode)
        };

        unsafe {
            SetLastError(0);
        }

        let result = match filename {
            Some(filename) => {
                let filename_str: Vec<_> =
                    filename.encode_wide().chain(Some(0).into_iter()).collect();
                let result = unsafe {
                    LoadLibraryW(filename_str.as_ptr() as *const libc::c_void)
                };
                // beware: Vec/String may change errno during drop!
                // so we get error here.
                if result == ptr::null_mut() {
                    Err(format!("{}", IoError::last_os_error()))
                } else {
                    Ok(result as *mut u8)
                }
            }
            None => {
                let mut handle = ptr::null_mut();
                let succeeded = unsafe {
                    GetModuleHandleExW(0, ptr::null(), &mut handle)
                };
                if succeeded == 0 {
                    Err(format!("{}", IoError::last_os_error()))
                } else {
                    Ok(handle as *mut u8)
                }
            }
        };

        unsafe {
            SetErrorMode(prev_error_mode);
        }

        result
    }

    pub fn check_for_errors_in<T, F>(f: F) -> Result<T, String> where
        F: FnOnce() -> T,
    {
        unsafe {
            SetLastError(0);

            let result = f();

            let error = IoError::last_os_error();
            if 0 == error.raw_os_error().unwrap() {
                Ok(result)
            } else {
                Err(format!("{}", error))
            }
        }
    }

    pub unsafe fn symbol(handle: *mut libc::c_void, symbol: *const libc::c_char) -> *mut u8 {
        GetProcAddress(handle, symbol) as *mut u8
    }
    pub unsafe fn close(handle: *mut u8) {
        FreeLibrary(handle as *mut libc::c_void); ()
    }

    #[allow(non_snake_case)]
    extern "system" {
        fn SetLastError(error: libc::size_t);
        fn LoadLibraryW(name: *const libc::c_void) -> *mut libc::c_void;
        fn GetModuleHandleExW(
            dwFlags: u32,
            name: *const u16,
            handle: *mut *mut libc::c_void,
        ) -> i32;
        fn GetProcAddress(
            handle: *mut libc::c_void,
            name: *const libc::c_char,
        ) -> *mut libc::c_void;
        fn FreeLibrary(handle: *mut libc::c_void);
        fn SetErrorMode(uMode: libc::c_uint) -> libc::c_uint;
    }
}
