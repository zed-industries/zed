#![doc = include_str!("../README.md")]
#![cfg(windows)]
#![deny(missing_docs)]
#![allow(bad_style)]
#![doc(html_root_url = "https://docs.rs/miow/latest/")]

use std::cmp;
use std::io;
use std::time::Duration;

use windows_sys::core::BOOL;
use windows_sys::Win32::System::Threading::INFINITE;

#[cfg(test)]
macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {:?}", stringify!($e), e),
        }
    };
}

mod handle;
mod overlapped;

pub mod iocp;
pub mod net;
pub mod pipe;

pub use crate::overlapped::Overlapped;
pub(crate) const TRUE: BOOL = 1;
pub(crate) const FALSE: BOOL = 0;

fn cvt(i: BOOL) -> io::Result<BOOL> {
    if i == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(i)
    }
}

fn dur2ms(dur: Option<Duration>) -> u32 {
    let dur = match dur {
        Some(dur) => dur,
        None => return INFINITE,
    };
    let ms = dur.as_secs().checked_mul(1_000);
    let ms_extra = dur.subsec_millis();
    ms.and_then(|ms| ms.checked_add(ms_extra as u64))
        .map(|ms| cmp::min(u32::MAX as u64, ms) as u32)
        .unwrap_or(INFINITE - 1)
}
