// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Interfaces to the operating system provided random number
//! generators.

use std::{io, fmt};

#[cfg(not(target_env = "sgx"))]
use std::mem;

use Rng;

/// A random number generator that retrieves randomness straight from
/// the operating system. Platform sources:
///
/// - Unix-like systems (Linux, Android, Mac OSX): read directly from
///   `/dev/urandom`, or from `getrandom(2)` system call if available.
/// - OpenBSD: calls `getentropy(2)`
/// - FreeBSD: uses the `kern.arandom` `sysctl(2)` mib
/// - Windows: calls `RtlGenRandom`, exported from `advapi32.dll` as
///   `SystemFunction036`.
/// - iOS: calls SecRandomCopyBytes as /dev/(u)random is sandboxed.
/// - PNaCl: calls into the `nacl-irt-random-0.1` IRT interface.
///
/// This usually does not block. On some systems (e.g. FreeBSD, OpenBSD,
/// Max OS X, and modern Linux) this may block very early in the init
/// process, if the CSPRNG has not been seeded yet.[1]
///
/// [1] See <https://www.python.org/dev/peps/pep-0524/> for a more
///     in-depth discussion.
pub struct OsRng(imp::OsRng);

impl OsRng {
    /// Create a new `OsRng`.
    pub fn new() -> io::Result<OsRng> {
        imp::OsRng::new().map(OsRng)
    }
}

impl Rng for OsRng {
    fn next_u32(&mut self) -> u32 { self.0.next_u32() }
    fn next_u64(&mut self) -> u64 { self.0.next_u64() }
    fn fill_bytes(&mut self, v: &mut [u8]) { self.0.fill_bytes(v) }
}

impl fmt::Debug for OsRng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OsRng {{}}")
    }
}

#[cfg(not(target_env = "sgx"))]
fn next_u32(fill_buf: &mut FnMut(&mut [u8])) -> u32 {
    let mut buf: [u8; 4] = [0; 4];
    fill_buf(&mut buf);
    unsafe { mem::transmute::<[u8; 4], u32>(buf) }
}

#[cfg(not(target_env = "sgx"))]
fn next_u64(fill_buf: &mut FnMut(&mut [u8])) -> u64 {
    let mut buf: [u8; 8] = [0; 8];
    fill_buf(&mut buf);
    unsafe { mem::transmute::<[u8; 8], u64>(buf) }
}

#[cfg(all(unix, not(target_os = "ios"),
          not(target_os = "nacl"),
          not(target_os = "freebsd"),
          not(target_os = "fuchsia"),
          not(target_os = "openbsd"),
          not(target_os = "redox")))]
mod imp {
    extern crate libc;

    use super::{next_u32, next_u64};
    use self::OsRngInner::*;

    use std::io;
    use std::fs::File;
    use Rng;
    use read::ReadRng;

    #[cfg(all(target_os = "linux",
              any(target_arch = "x86_64",
                  target_arch = "x86",
                  target_arch = "arm",
                  target_arch = "aarch64",
                  target_arch = "powerpc")))]
    fn getrandom(buf: &mut [u8]) -> libc::c_long {
        extern "C" {
            fn syscall(number: libc::c_long, ...) -> libc::c_long;
        }

        #[cfg(target_arch = "x86_64")]
        const NR_GETRANDOM: libc::c_long = 318;
        #[cfg(target_arch = "x86")]
        const NR_GETRANDOM: libc::c_long = 355;
        #[cfg(target_arch = "arm")]
        const NR_GETRANDOM: libc::c_long = 384;
        #[cfg(target_arch = "aarch64")]
        const NR_GETRANDOM: libc::c_long = 278;
        #[cfg(target_arch = "powerpc")]
        const NR_GETRANDOM: libc::c_long = 359;

        unsafe {
            syscall(NR_GETRANDOM, buf.as_mut_ptr(), buf.len(), 0)
        }
    }

    #[cfg(not(all(target_os = "linux",
                  any(target_arch = "x86_64",
                      target_arch = "x86",
                      target_arch = "arm",
                      target_arch = "aarch64",
                      target_arch = "powerpc"))))]
    fn getrandom(_buf: &mut [u8]) -> libc::c_long { -1 }

    fn getrandom_fill_bytes(v: &mut [u8]) {
        let mut read = 0;
        let len = v.len();
        while read < len {
            let result = getrandom(&mut v[read..]);
            if result == -1 {
                let err = io::Error::last_os_error();
                if err.kind() == io::ErrorKind::Interrupted {
                    continue
                } else {
                    panic!("unexpected getrandom error: {}", err);
                }
            } else {
                read += result as usize;
            }
        }
    }

    #[cfg(all(target_os = "linux",
              any(target_arch = "x86_64",
                  target_arch = "x86",
                  target_arch = "arm",
                  target_arch = "aarch64",
                  target_arch = "powerpc")))]
    fn is_getrandom_available() -> bool {
        use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
        use std::sync::{Once, ONCE_INIT};

        static CHECKER: Once = ONCE_INIT;
        static AVAILABLE: AtomicBool = ATOMIC_BOOL_INIT;

        CHECKER.call_once(|| {
            let mut buf: [u8; 0] = [];
            let result = getrandom(&mut buf);
            let available = if result == -1 {
                let err = io::Error::last_os_error().raw_os_error();
                err != Some(libc::ENOSYS)
            } else {
                true
            };
            AVAILABLE.store(available, Ordering::Relaxed);
        });

        AVAILABLE.load(Ordering::Relaxed)
    }

    #[cfg(not(all(target_os = "linux",
                  any(target_arch = "x86_64",
                      target_arch = "x86",
                      target_arch = "arm",
                      target_arch = "aarch64",
                      target_arch = "powerpc"))))]
    fn is_getrandom_available() -> bool { false }

    pub struct OsRng {
        inner: OsRngInner,
    }

    enum OsRngInner {
        OsGetrandomRng,
        OsReadRng(ReadRng<File>),
    }

    impl OsRng {
        pub fn new() -> io::Result<OsRng> {
            if is_getrandom_available() {
                return Ok(OsRng { inner: OsGetrandomRng });
            }

            let reader = try!(File::open("/dev/urandom"));
            let reader_rng = ReadRng::new(reader);

            Ok(OsRng { inner: OsReadRng(reader_rng) })
        }
    }

    impl Rng for OsRng {
        fn next_u32(&mut self) -> u32 {
            match self.inner {
                OsGetrandomRng => next_u32(&mut getrandom_fill_bytes),
                OsReadRng(ref mut rng) => rng.next_u32(),
            }
        }
        fn next_u64(&mut self) -> u64 {
            match self.inner {
                OsGetrandomRng => next_u64(&mut getrandom_fill_bytes),
                OsReadRng(ref mut rng) => rng.next_u64(),
            }
        }
        fn fill_bytes(&mut self, v: &mut [u8]) {
            match self.inner {
                OsGetrandomRng => getrandom_fill_bytes(v),
                OsReadRng(ref mut rng) => rng.fill_bytes(v)
            }
        }
    }
}

#[cfg(target_os = "ios")]
mod imp {
    extern crate libc;

    use super::{next_u32, next_u64};

    use std::io;
    use Rng;
    use self::libc::{c_int, size_t};

    #[derive(Debug)]
    pub struct OsRng;

    enum SecRandom {}

    #[allow(non_upper_case_globals)]
    const kSecRandomDefault: *const SecRandom = 0 as *const SecRandom;

    #[link(name = "Security", kind = "framework")]
    extern {
        fn SecRandomCopyBytes(rnd: *const SecRandom,
                              count: size_t, bytes: *mut u8) -> c_int;
    }

    impl OsRng {
        pub fn new() -> io::Result<OsRng> {
            Ok(OsRng)
        }
    }

    impl Rng for OsRng {
        fn next_u32(&mut self) -> u32 {
            next_u32(&mut |v| self.fill_bytes(v))
        }
        fn next_u64(&mut self) -> u64 {
            next_u64(&mut |v| self.fill_bytes(v))
        }
        fn fill_bytes(&mut self, v: &mut [u8]) {
            let ret = unsafe {
                SecRandomCopyBytes(kSecRandomDefault, v.len() as size_t, v.as_mut_ptr())
            };
            if ret == -1 {
                panic!("couldn't generate random bytes: {}", io::Error::last_os_error());
            }
        }
    }
}

#[cfg(target_os = "freebsd")]
mod imp {
    extern crate libc;

    use std::{io, ptr};
    use Rng;

    use super::{next_u32, next_u64};

    #[derive(Debug)]
    pub struct OsRng;

    impl OsRng {
        pub fn new() -> io::Result<OsRng> {
            Ok(OsRng)
        }
    }

    impl Rng for OsRng {
        fn next_u32(&mut self) -> u32 {
            next_u32(&mut |v| self.fill_bytes(v))
        }
        fn next_u64(&mut self) -> u64 {
            next_u64(&mut |v| self.fill_bytes(v))
        }
        fn fill_bytes(&mut self, v: &mut [u8]) {
            let mib = [libc::CTL_KERN, libc::KERN_ARND];
            // kern.arandom permits a maximum buffer size of 256 bytes
            for s in v.chunks_mut(256) {
                let mut s_len = s.len();
                let ret = unsafe {
                    libc::sysctl(mib.as_ptr(), mib.len() as libc::c_uint,
                                 s.as_mut_ptr() as *mut _, &mut s_len,
                                 ptr::null(), 0)
                };
                if ret == -1 || s_len != s.len() {
                    panic!("kern.arandom sysctl failed! (returned {}, s.len() {}, oldlenp {})",
                           ret, s.len(), s_len);
                }
            }
        }
    }
}

#[cfg(target_os = "openbsd")]
mod imp {
    extern crate libc;

    use std::io;
    use Rng;

    use super::{next_u32, next_u64};

    #[derive(Debug)]
    pub struct OsRng;

    impl OsRng {
        pub fn new() -> io::Result<OsRng> {
            Ok(OsRng)
        }
    }

    impl Rng for OsRng {
        fn next_u32(&mut self) -> u32 {
            next_u32(&mut |v| self.fill_bytes(v))
        }
        fn next_u64(&mut self) -> u64 {
            next_u64(&mut |v| self.fill_bytes(v))
        }
        fn fill_bytes(&mut self, v: &mut [u8]) {
            // getentropy(2) permits a maximum buffer size of 256 bytes
            for s in v.chunks_mut(256) {
                let ret = unsafe {
                    libc::getentropy(s.as_mut_ptr() as *mut libc::c_void, s.len())
                };
                if ret == -1 {
                    let err = io::Error::last_os_error();
                    panic!("getentropy failed: {}", err);
                }
            }
        }
    }
}

#[cfg(target_os = "redox")]
mod imp {
    use std::io;
    use std::fs::File;
    use Rng;
    use read::ReadRng;

    #[derive(Debug)]
    pub struct OsRng {
        inner: ReadRng<File>,
    }

    impl OsRng {
        pub fn new() -> io::Result<OsRng> {
            let reader = try!(File::open("rand:"));
            let reader_rng = ReadRng::new(reader);

            Ok(OsRng { inner: reader_rng })
        }
    }

    impl Rng for OsRng {
        fn next_u32(&mut self) -> u32 {
            self.inner.next_u32()
        }
        fn next_u64(&mut self) -> u64 {
            self.inner.next_u64()
        }
        fn fill_bytes(&mut self, v: &mut [u8]) {
            self.inner.fill_bytes(v)
        }
    }
}

#[cfg(target_os = "fuchsia")]
mod imp {
    extern crate fuchsia_cprng;

    use std::io;
    use Rng;

    use super::{next_u32, next_u64};

    #[derive(Debug)]
    pub struct OsRng;

    impl OsRng {
        pub fn new() -> io::Result<OsRng> {
            Ok(OsRng)
        }
    }

    impl Rng for OsRng {
        fn next_u32(&mut self) -> u32 {
            next_u32(&mut |v| self.fill_bytes(v))
        }
        fn next_u64(&mut self) -> u64 {
            next_u64(&mut |v| self.fill_bytes(v))
        }
        fn fill_bytes(&mut self, v: &mut [u8]) {
            fuchsia_cprng::cprng_draw(v);
        }
    }
}

#[cfg(windows)]
mod imp {
    extern crate winapi;

    use std::io;
    use Rng;

    use super::{next_u32, next_u64};

    use self::winapi::shared::minwindef::ULONG;
    use self::winapi::um::ntsecapi::RtlGenRandom;
    use self::winapi::um::winnt::PVOID;

    #[derive(Debug)]
    pub struct OsRng;

    impl OsRng {
        pub fn new() -> io::Result<OsRng> {
            Ok(OsRng)
        }
    }

    impl Rng for OsRng {
        fn next_u32(&mut self) -> u32 {
            next_u32(&mut |v| self.fill_bytes(v))
        }
        fn next_u64(&mut self) -> u64 {
            next_u64(&mut |v| self.fill_bytes(v))
        }
        fn fill_bytes(&mut self, v: &mut [u8]) {
            // RtlGenRandom takes an ULONG (u32) for the length so we need to
            // split up the buffer.
            for slice in v.chunks_mut(<ULONG>::max_value() as usize) {
                let ret = unsafe {
                    RtlGenRandom(slice.as_mut_ptr() as PVOID, slice.len() as ULONG)
                };
                if ret == 0 {
                    panic!("couldn't generate random bytes: {}",
                           io::Error::last_os_error());
                }
            }
        }
    }
}

#[cfg(target_os = "nacl")]
mod imp {
    extern crate libc;

    use std::io;
    use std::mem;
    use Rng;

    use super::{next_u32, next_u64};

    #[derive(Debug)]
    pub struct OsRng(extern fn(dest: *mut libc::c_void,
                               bytes: libc::size_t,
                               read: *mut libc::size_t) -> libc::c_int);

    extern {
        fn nacl_interface_query(name: *const libc::c_char,
                                table: *mut libc::c_void,
                                table_size: libc::size_t) -> libc::size_t;
    }

    const INTERFACE: &'static [u8] = b"nacl-irt-random-0.1\0";

    #[repr(C)]
    struct NaClIRTRandom {
        get_random_bytes: Option<extern fn(dest: *mut libc::c_void,
                                           bytes: libc::size_t,
                                           read: *mut libc::size_t) -> libc::c_int>,
    }

    impl OsRng {
        pub fn new() -> io::Result<OsRng> {
            let mut iface = NaClIRTRandom {
                get_random_bytes: None,
            };
            let result = unsafe {
                nacl_interface_query(INTERFACE.as_ptr() as *const _,
                                     mem::transmute(&mut iface),
                                     mem::size_of::<NaClIRTRandom>() as libc::size_t)
            };
            if result != 0 {
                assert!(iface.get_random_bytes.is_some());
                let result = OsRng(iface.get_random_bytes.take().unwrap());
                Ok(result)
            } else {
                let error = io::ErrorKind::NotFound;
                let error = io::Error::new(error, "IRT random interface missing");
                Err(error)
            }
        }
    }

    impl Rng for OsRng {
        fn next_u32(&mut self) -> u32 {
            next_u32(&mut |v| self.fill_bytes(v))
        }
        fn next_u64(&mut self) -> u64 {
            next_u64(&mut |v| self.fill_bytes(v))
        }
        fn fill_bytes(&mut self, v: &mut [u8]) {
            let mut read = 0;
            loop {
                let mut r: libc::size_t = 0;
                let len = v.len();
                let error = (self.0)(v[read..].as_mut_ptr() as *mut _,
                                     (len - read) as libc::size_t,
                                     &mut r as *mut _);
                assert!(error == 0, "`get_random_bytes` failed!");
                read += r as usize;

                if read >= v.len() { break; }
            }
        }
    }
}

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
mod imp {
    use std::io;
    use Rng;

    #[derive(Debug)]
    pub struct OsRng;

    impl OsRng {
        pub fn new() -> io::Result<OsRng> {
            Err(io::Error::new(io::ErrorKind::Other, "Not supported"))
        }
    }

    impl Rng for OsRng {
        fn next_u32(&mut self) -> u32 {
            panic!("Not supported")
        }
    }
}

#[cfg(target_env = "sgx")]
mod imp {
    use rdrand::RdRand;
    use std::io;
    use rand_core::RngCore;

    pub struct OsRng{
        gen: RdRand
    }

    impl OsRng {
        pub fn new() -> io::Result<OsRng> {
            match RdRand::new() {
                Ok(rng) => Ok(OsRng { gen: rng }),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Not supported"))
            }
        }

        pub(crate) fn next_u32(&mut self) -> u32 {
            match self.gen.try_next_u32() {
                Some(n) => n,
                None => panic!("Non-recoverable hardware failure has occured")
            }
        }

        pub(crate) fn next_u64(&mut self) -> u64 {
            match self.gen.try_next_u64() {
                Some(n) => n,
                None => panic!("Non-recoverable hardware failure has occured")
            }
        }

        pub(crate) fn fill_bytes(&mut self, v: &mut [u8]) {
            match self.gen.try_fill_bytes(v) {
                Ok(_) => {},
                Err(_) => panic!("Non-recoverable hardware failure has occured")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;
    use Rng;
    use OsRng;
    use std::thread;

    #[test]
    fn test_os_rng() {
        let mut r = OsRng::new().unwrap();

        r.next_u32();
        r.next_u64();

        let mut v = [0u8; 1000];
        r.fill_bytes(&mut v);
    }

    #[test]
    fn test_os_rng_tasks() {

        let mut txs = vec!();
        for _ in 0..20 {
            let (tx, rx) = channel();
            txs.push(tx);

            thread::spawn(move|| {
                // wait until all the tasks are ready to go.
                rx.recv().unwrap();

                // deschedule to attempt to interleave things as much
                // as possible (XXX: is this a good test?)
                let mut r = OsRng::new().unwrap();
                thread::yield_now();
                let mut v = [0u8; 1000];

                for _ in 0..100 {
                    r.next_u32();
                    thread::yield_now();
                    r.next_u64();
                    thread::yield_now();
                    r.fill_bytes(&mut v);
                    thread::yield_now();
                }
            });
        }

        // start all the tasks
        for tx in txs.iter() {
            tx.send(()).unwrap();
        }
    }
}
