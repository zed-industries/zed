#![no_std]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![deny(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]

//! A drop-in compatible rust implementation of bzip2

#[cfg(feature = "std")]
extern crate std;

use core::ffi::c_int;
#[cfg(not(feature = "std"))]
use core::sync::atomic::{AtomicI32, Ordering};

mod allocator;
mod blocksort;
mod bzlib;
mod compress;
mod crctable;
mod decompress;
#[cfg(feature = "stdio")]
mod high_level;
mod huffman;
mod randtable;

pub(crate) use bzlib::{Action, ReturnCode};

pub const BZ_OK: c_int = ReturnCode::BZ_OK as c_int;
pub const BZ_RUN_OK: c_int = ReturnCode::BZ_RUN_OK as c_int;
pub const BZ_FLUSH_OK: c_int = ReturnCode::BZ_FLUSH_OK as c_int;
pub const BZ_FINISH_OK: c_int = ReturnCode::BZ_FINISH_OK as c_int;
pub const BZ_STREAM_END: c_int = ReturnCode::BZ_STREAM_END as c_int;
pub const BZ_SEQUENCE_ERROR: c_int = ReturnCode::BZ_SEQUENCE_ERROR as c_int;
pub const BZ_PARAM_ERROR: c_int = ReturnCode::BZ_PARAM_ERROR as c_int;
pub const BZ_MEM_ERROR: c_int = ReturnCode::BZ_MEM_ERROR as c_int;
pub const BZ_DATA_ERROR: c_int = ReturnCode::BZ_DATA_ERROR as c_int;
pub const BZ_DATA_ERROR_MAGIC: c_int = ReturnCode::BZ_DATA_ERROR_MAGIC as c_int;
pub const BZ_IO_ERROR: c_int = ReturnCode::BZ_IO_ERROR as c_int;
pub const BZ_UNEXPECTED_EOF: c_int = ReturnCode::BZ_UNEXPECTED_EOF as c_int;
pub const BZ_OUTBUFF_FULL: c_int = ReturnCode::BZ_OUTBUFF_FULL as c_int;
pub const BZ_CONFIG_ERROR: c_int = ReturnCode::BZ_CONFIG_ERROR as c_int;

pub const BZ_RUN: c_int = Action::Run as c_int;
pub const BZ_FLUSH: c_int = Action::Flush as c_int;
pub const BZ_FINISH: c_int = Action::Finish as c_int;

pub const BZ_MAX_UNUSED: c_int = bzlib::BZ_MAX_UNUSED_U32 as c_int;

// types
pub use bzlib::bz_stream;
#[cfg(feature = "stdio")]
pub use bzlib::BZFILE;

// the low-level interface
pub use bzlib::{BZ2_bzCompress, BZ2_bzCompressEnd, BZ2_bzCompressInit};
pub use bzlib::{BZ2_bzDecompress, BZ2_bzDecompressEnd, BZ2_bzDecompressInit};

// utility functions
pub use bzlib::{BZ2_bzBuffToBuffCompress, BZ2_bzBuffToBuffDecompress};

// the high-level interface
#[cfg(feature = "stdio")]
pub use bzlib::{BZ2_bzRead, BZ2_bzReadClose, BZ2_bzReadGetUnused, BZ2_bzReadOpen};
#[cfg(feature = "stdio")]
pub use bzlib::{BZ2_bzWrite, BZ2_bzWriteClose, BZ2_bzWriteClose64, BZ2_bzWriteOpen};

// zlib compatibility functions
#[cfg(feature = "stdio")]
pub use bzlib::{
    BZ2_bzclose, BZ2_bzdopen, BZ2_bzerror, BZ2_bzflush, BZ2_bzlibVersion, BZ2_bzopen, BZ2_bzread,
    BZ2_bzwrite,
};

// --- version number logic

macro_rules! libbz2_rs_sys_version {
    () => {
        concat!("1.1.0-libbz2-rs-sys-", env!("CARGO_PKG_VERSION"))
    };
}

pub(crate) use libbz2_rs_sys_version;

// --- debug logs

#[cfg(all(not(feature = "std"), feature = "stdio"))]
pub(crate) struct StderrWritter;

#[cfg(all(not(feature = "std"), feature = "stdio"))]
impl core::fmt::Write for StderrWritter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        use core::ffi::c_void;
        use libc::write;

        unsafe { write(2, s.as_ptr() as *const c_void, s.len() as _) };

        Ok(())
    }
}

macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "std")]
        std::eprint!($($arg)*);
        #[cfg(all(not(feature = "std"), feature = "stdio"))]
        {
            use core::fmt::Write;
            let _ = write!($crate::StderrWritter, $($arg)*);
        }
    };
}

macro_rules! debug_logln {
    ($($arg:tt)*) => {
        #[cfg(feature = "std")]
        std::eprintln!($($arg)*);
        #[cfg(all(not(feature = "std"), feature = "stdio"))]
        {
            use core::fmt::Write;
            let _ = writeln!($crate::StderrWritter, $($arg)*);
        }
    };
}

pub(crate) use debug_log;
pub(crate) use debug_logln;

// --- assert failure logic

macro_rules! assert_h {
    ($condition:expr, $errcode:expr) => {
        if !$condition {
            $crate::handle_assert_failure($errcode)
        }
    };
}

#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub static ASSERT_CODE: AtomicI32 = AtomicI32::new(-1);

#[cold]
fn handle_assert_failure(errcode: c_int) -> ! {
    #[cfg(feature = "std")]
    std::eprint!("{}", AssertFail(errcode));
    #[cfg(feature = "std")]
    std::process::exit(3);

    // Stash the assertion code for the panic handler in the cdylib to pass to bz_internal_error.
    // Using relaxed ordering as this will be accessed on the same thread.
    #[cfg(not(feature = "std"))]
    #[allow(clippy::unnecessary_cast)]
    ASSERT_CODE.store(errcode as i32, Ordering::Relaxed);
    #[cfg(not(feature = "std"))]
    panic!("{}", AssertFail(errcode));
}

use assert_h;

struct AssertFail(i32);

impl core::fmt::Display for AssertFail {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            concat!(
                "\n",
                "\n",
                "libbzip2-rs: internal error number {}.\n",
                "This is a bug in libbzip2-rs, {}.\n",
                "Please report it at: https://github.com/trifectatechfoundation/libbzip2-rs/issues\n",
                "If this happened when you were using some program which uses\n",
                "libbzip2-rs as a component, you should also report this bug to\n",
                "the author(s) of that program.\n",
                "Please make an effort to report this bug;\n",
                "timely and accurate bug reports eventually lead to higher\n",
                "quality software.  Thanks.\n",
                "\n"
            ),
            self.0,
            libbz2_rs_sys_version!(),
        )?;

        if self.0 == 1007 {
            write!(
                f,
                concat!(
                    "\n",
                    "*** A special note about internal error number 1007 ***\n",
                    "\n",
                    "Experience suggests that a common cause of i.e. 1007\n",
                    "is unreliable memory or other hardware.  The 1007 assertion\n",
                    "just happens to cross-check the results of huge numbers of\n",
                    "memory reads/writes, and so acts (unintendedly) as a stress\n",
                    "test of your memory system.\n",
                    "\n",
                    "I suggest the following: try compressing the file again,\n",
                    "possibly monitoring progress in detail with the -vv flag.\n",
                    "\n",
                    "* If the error cannot be reproduced, and/or happens at different\n",
                    "  points in compression, you may have a flaky memory system.\n",
                    "  Try a memory-test program.  I have used Memtest86\n",
                    "  (www.memtest86.com).  At the time of writing it is free (GPLd).\n",
                    "  Memtest86 tests memory much more thorougly than your BIOSs\n",
                    "  power-on test, and may find failures that the BIOS doesn't.\n",
                    "\n",
                    "* If the error can be repeatably reproduced, this is a bug in\n",
                    "  bzip2, and I would very much like to hear about it.  Please\n",
                    "  let me know, and, ideally, save a copy of the file causing the\n",
                    "  problem -- without which I will be unable to investigate it.\n",
                    "\n"
                )
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;

    use super::*;

    use alloc::string::String;

    #[test]
    fn print_assert_fail_coverage() {
        use core::fmt::Write;
        write!(&mut String::new(), "{}", AssertFail(1007)).unwrap();
    }
}
