use core::{
    arch::asm,
    mem,
    ops::{Deref, DerefMut},
    slice,
};

use super::error::{Error, Result};

pub const PAGE_SIZE: usize = 4096;
/// Size of the metadata region used to transfer information from the kernel to the bootstrapper.
pub const KERNEL_METADATA_SIZE: usize = 4 * PAGE_SIZE;

#[cfg(feature = "userspace")]
macro_rules! syscall {
    ($($name:ident($a:ident, $($b:ident, $($c:ident, $($d:ident, $($e:ident, $($f:ident, )?)?)?)?)?);)+) => {
        $(
            pub unsafe fn $name(mut $a: usize, $($b: usize, $($c: usize, $($d: usize, $($e: usize, $($f: usize)?)?)?)?)?) -> Result<usize> {
                asm!(
                    "int 0x80",
                    inout("eax") $a,
                    $(
                        in("ebx") $b,
                        $(
                            in("ecx") $c,
                            $(
                                in("edx") $d,
                                $(
                                    in("esi") $e,
                                    $(
                                        in("edi") $f,
                                    )?
                                )?
                            )?
                        )?
                    )?
                    options(nostack),
                );

                Error::demux($a)
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
    // Must be done custom because LLVM reserves ESI
    //syscall4(a, b, c, d, e,);
    //syscall5(a, b, c, d, e, f,);
}

#[cfg(feature = "userspace")]
pub unsafe fn syscall4(mut a: usize, b: usize, c: usize, d: usize, e: usize) -> Result<usize> {
    asm!(
        "xchg esi, {e}
        int 0x80
        xchg esi, {e}",
        e = in(reg) e,
        inout("eax") a,
        in("ebx") b,
        in("ecx") c,
        in("edx") d,
        options(nostack),
    );

    Error::demux(a)
}

#[cfg(feature = "userspace")]
pub unsafe fn syscall5(
    mut a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    f: usize,
) -> Result<usize> {
    asm!(
        "xchg esi, {e}
        int 0x80
        xchg esi, {e}",
        e = in(reg) e,
        inout("eax") a,
        in("ebx") b,
        in("ecx") c,
        in("edx") d,
        in("edi") f,
        options(nostack),
    );

    Error::demux(a)
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct IntRegisters {
    // TODO: Some of these don't get set by Redox yet. Should they?
    pub ebp: usize,
    pub esi: usize,
    pub edi: usize,
    pub ebx: usize,
    pub eax: usize,
    pub ecx: usize,
    pub edx: usize,
    // pub orig_rax: usize,
    pub eip: usize,
    pub cs: usize,
    pub eflags: usize,
    pub esp: usize,
    pub ss: usize,
    // pub fs_base: usize,
    // pub gs_base: usize,
    // pub ds: usize,
    // pub es: usize,
    pub fs: usize,
    // pub gs: usize
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
    pub fcw: u16,
    pub fsw: u16,
    pub ftw: u8,
    pub _reserved: u8,
    pub fop: u16,
    pub fip: u64,
    pub fdp: u64,
    pub mxcsr: u32,
    pub mxcsr_mask: u32,
    pub st_space: [u128; 8],
    pub xmm_space: [u128; 16],
    // TODO: YMM/ZMM
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
#[repr(C, packed)]
pub struct EnvRegisters {
    pub fsbase: u32,
    pub gsbase: u32,
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
    pub code: usize,
    pub address: usize,
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
