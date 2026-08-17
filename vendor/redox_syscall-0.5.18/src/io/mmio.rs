#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
use core::ops::{BitAnd, BitOr, Not};
use core::{mem::MaybeUninit, ptr};

use super::io::Io;

#[repr(transparent)]
pub struct Mmio<T> {
    value: MaybeUninit<T>,
}

impl<T> Mmio<T> {
    pub unsafe fn zeroed() -> Self {
        Self {
            value: MaybeUninit::zeroed(),
        }
    }
    pub unsafe fn uninit() -> Self {
        Self {
            value: MaybeUninit::uninit(),
        }
    }
    pub const fn from(value: T) -> Self {
        Self {
            value: MaybeUninit::new(value),
        }
    }
}

// Generic implementation (WARNING: requires aligned pointers!)
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
impl<T> Io for Mmio<T>
where
    T: Copy + PartialEq + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T>,
{
    type Value = T;

    fn read(&self) -> T {
        unsafe { ptr::read_volatile(ptr::addr_of!(self.value).cast::<T>()) }
    }

    fn write(&mut self, value: T) {
        unsafe { ptr::write_volatile(ptr::addr_of_mut!(self.value).cast::<T>(), value) };
    }
}

// x86 u8 implementation
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Io for Mmio<u8> {
    type Value = u8;

    fn read(&self) -> Self::Value {
        unsafe {
            let value: Self::Value;
            let ptr: *const Self::Value = ptr::addr_of!(self.value).cast::<Self::Value>();
            core::arch::asm!(
                "mov {}, [{}]",
                out(reg_byte) value,
                in(reg) ptr
            );
            value
        }
    }

    fn write(&mut self, value: Self::Value) {
        unsafe {
            let ptr: *mut Self::Value = ptr::addr_of_mut!(self.value).cast::<Self::Value>();
            core::arch::asm!(
                "mov [{}], {}",
                in(reg) ptr,
                in(reg_byte) value,
            );
        }
    }
}

// x86 u16 implementation
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Io for Mmio<u16> {
    type Value = u16;

    fn read(&self) -> Self::Value {
        unsafe {
            let value: Self::Value;
            let ptr: *const Self::Value = ptr::addr_of!(self.value).cast::<Self::Value>();
            core::arch::asm!(
                "mov {:x}, [{}]",
                out(reg) value,
                in(reg) ptr
            );
            value
        }
    }

    fn write(&mut self, value: Self::Value) {
        unsafe {
            let ptr: *mut Self::Value = ptr::addr_of_mut!(self.value).cast::<Self::Value>();
            core::arch::asm!(
                "mov [{}], {:x}",
                in(reg) ptr,
                in(reg) value,
            );
        }
    }
}

// x86 u32 implementation
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Io for Mmio<u32> {
    type Value = u32;

    fn read(&self) -> Self::Value {
        unsafe {
            let value: Self::Value;
            let ptr: *const Self::Value = ptr::addr_of!(self.value).cast::<Self::Value>();
            core::arch::asm!(
                "mov {:e}, [{}]",
                out(reg) value,
                in(reg) ptr
            );
            value
        }
    }

    fn write(&mut self, value: Self::Value) {
        unsafe {
            let ptr: *mut Self::Value = ptr::addr_of_mut!(self.value).cast::<Self::Value>();
            core::arch::asm!(
                "mov [{}], {:e}",
                in(reg) ptr,
                in(reg) value,
            );
        }
    }
}

// x86 u64 implementation (x86_64 only)
#[cfg(target_arch = "x86_64")]
impl Io for Mmio<u64> {
    type Value = u64;

    fn read(&self) -> Self::Value {
        unsafe {
            let value: Self::Value;
            let ptr: *const Self::Value = ptr::addr_of!(self.value).cast::<Self::Value>();
            core::arch::asm!(
                "mov {:r}, [{}]",
                out(reg) value,
                in(reg) ptr
            );
            value
        }
    }

    fn write(&mut self, value: Self::Value) {
        unsafe {
            let ptr: *mut Self::Value = ptr::addr_of_mut!(self.value).cast::<Self::Value>();
            core::arch::asm!(
                "mov [{}], {:r}",
                in(reg) ptr,
                in(reg) value,
            );
        }
    }
}
