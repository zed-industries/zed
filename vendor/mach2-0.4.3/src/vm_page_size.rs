//! This module roughly corresponds to `mach/vm_page_size.h`

use vm_types::{mach_vm_offset_t, mach_vm_size_t, vm_size_t};

extern "C" {
    pub static vm_page_size: vm_size_t;
    pub static vm_page_mask: vm_size_t;
    pub static vm_page_shift: ::libc::c_int;
}

#[allow(clippy::missing_safety_doc)] // FIXME
pub unsafe fn mach_vm_trunc_page(x: mach_vm_offset_t) -> mach_vm_offset_t {
    x & !(vm_page_mask as mach_vm_size_t)
}

#[allow(clippy::missing_safety_doc)] // FIXME
pub unsafe fn mach_vm_round_page(x: mach_vm_offset_t) -> mach_vm_offset_t {
    (x + vm_page_mask as mach_vm_size_t) & !(vm_page_mask as mach_vm_size_t)
}

#[cfg(test)]
mod tests {
    use vm_page_size::*;
    use vm_types::mach_vm_size_t;

    #[test]
    fn page_size() {
        unsafe {
            assert!(vm_page_size > 0);
            assert!(vm_page_size % 2 == 0);
            assert_eq!(mach_vm_round_page(1), vm_page_size as mach_vm_size_t);

            #[cfg(target_arch = "aarch64")]
            assert_eq!(vm_page_size, 16384);

            #[cfg(not(target_arch = "aarch64"))]
            assert_eq!(vm_page_size, 4096);
        }
    }
}
