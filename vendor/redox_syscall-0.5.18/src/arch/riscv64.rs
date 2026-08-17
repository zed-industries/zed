use super::error::{Error, Result};
use core::arch::asm;
use core::{
    mem,
    ops::{Deref, DerefMut},
    slice,
};

pub const PAGE_SIZE: usize = 4096;
/// Size of the metadata region used to transfer information from the kernel to the bootstrapper.
pub const KERNEL_METADATA_SIZE: usize = 4 * PAGE_SIZE;

#[cfg(feature = "userspace")]
macro_rules! syscall {
    ($($name:ident($a:ident, $($b:ident, $($c:ident, $($d:ident, $($e:ident, $($f:ident, )?)?)?)?)?);)+) => {
        $(
            pub unsafe fn $name($a: usize, $($b: usize, $($c: usize, $($d: usize, $($e: usize, $($f: usize)?)?)?)?)?) -> Result<usize> {
                let ret: usize;

                asm!(
                    "ecall",
                    in("a7") $a,
                    $(
                        in("a0") $b,
                        $(
                            in("a1") $c,
                            $(
                                in("a2") $d,
                                $(
                                    in("a3") $e,
                                    $(
                                        in("a4") $f,
                                    )?
                                )?
                            )?
                        )?
                    )?
                    lateout("a0") ret,
                    options(nostack),
                );

                Error::demux(ret)
            }
        )+
    };
}

#[cfg(feature = "userspace")]
syscall! {
    syscall0(a,);
    syscall1(a, b,);
    syscall2(a, b, c,);
    syscall3(a, b, c, d,);
    syscall4(a, b, c, d, e,);
    syscall5(a, b, c, d, e, f,);
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct IntRegisters {
    pub pc: usize,
    pub x31: usize,
    pub x30: usize,
    pub x29: usize,
    pub x28: usize,
    pub x27: usize,
    pub x26: usize,
    pub x25: usize,
    pub x24: usize,
    pub x23: usize,
    pub x22: usize,
    pub x21: usize,
    pub x20: usize,
    pub x19: usize,
    pub x18: usize,
    pub x17: usize,
    pub x16: usize,
    pub x15: usize,
    pub x14: usize,
    pub x13: usize,
    pub x12: usize,
    pub x11: usize,
    pub x10: usize,
    pub x9: usize,
    pub x8: usize,
    pub x7: usize,
    pub x6: usize,
    pub x5: usize,
    // x4(tp) is in env
    // x3(gp) is a platform scratch register
    pub x2: usize,
    pub x1: usize,
}

impl Deref for IntRegisters {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const IntRegisters as *const u8,
                mem::size_of::<IntRegisters>(),
            )
        }
    }
}

impl DerefMut for IntRegisters {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut IntRegisters as *mut u8,
                mem::size_of::<IntRegisters>(),
            )
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct FloatRegisters {
    pub fregs: [u64; 32],
    pub fcsr: u32,
}

impl Deref for FloatRegisters {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const FloatRegisters as *const u8,
                mem::size_of::<FloatRegisters>(),
            )
        }
    }
}

impl DerefMut for FloatRegisters {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut FloatRegisters as *mut u8,
                mem::size_of::<FloatRegisters>(),
            )
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(packed)]
pub struct EnvRegisters {
    pub tp: usize,
}
impl Deref for EnvRegisters {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const EnvRegisters as *const u8,
                mem::size_of::<EnvRegisters>(),
            )
        }
    }
}

impl DerefMut for EnvRegisters {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut EnvRegisters as *mut u8,
                mem::size_of::<EnvRegisters>(),
            )
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct Exception {
    pub kind: usize,
    // TODO
}
impl Deref for Exception {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const Exception as *const u8,
                mem::size_of::<Exception>(),
            )
        }
    }
}

impl DerefMut for Exception {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut Exception as *mut u8,
                mem::size_of::<Exception>(),
            )
        }
    }
}
