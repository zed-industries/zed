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
            pub unsafe fn $name(mut $a: usize, $($b: usize, $($c: usize, $($d: usize, $($e: usize, $($f: usize)?)?)?)?)?) -> crate::error::Result<usize> {
                core::arch::asm!(
                    "syscall",
                    inout("rax") $a,
                    $(
                        in("rdi") $b,
                        $(
                            in("rsi") $c,
                            $(
                                in("rdx") $d,
                                $(
                                    in("r10") $e,
                                    $(
                                        in("r8") $f,
                                    )?
                                )?
                            )?
                        )?
                    )?
                    out("rcx") _,
                    out("r11") _,
                    options(nostack),
                );

                crate::error::Error::demux($a)
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
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub rbp: usize,
    pub rbx: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rax: usize,
    pub rcx: usize,
    pub rdx: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rip: usize,
    pub cs: usize,
    pub rflags: usize,
    pub rsp: usize,
    pub ss: usize,
}

impl Deref for IntRegisters {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>()) }
    }
}

impl DerefMut for IntRegisters {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut u8, mem::size_of::<Self>()) }
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
    pub fsbase: u64,
    pub gsbase: u64,
    // TODO: PKRU?
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
