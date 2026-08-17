// Example: Freestanding
//
// This example is a plain UEFI application without any external requirements
// but `core`. It immediately returns control to the caller upon execution,
// yielding the exit code 0.
//
// The `main` function serves as entry-point. Depending on your
// target-configuration, it must be exported with a pre-configured name so the
// linker will correctly mark it as entry-point. The target configurations
// shipped with upstream rust-lang use `efi_main` as symbol name.
//
// Additionally, a panic handler is provided. This is executed by rust on
// panic. For simplicity, we simply end up in an infinite loop. For real
// applications, this method should probably call into
// `SystemTable->boot_services->exit()` to exit the UEFI application. Note,
// however, that UEFI applications are likely to run in the same address space
// as the entire firmware. Hence, halting the machine might be a viable
// alternative. All that is out-of-scope for this example, though.
//
// Note that as of rust-1.31.0, all features used here are stabilized. No
// unstable features are required, nor do we rely on nightly compilers.

#![no_main]
#![no_std]

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "C" fn main(_h: *mut core::ffi::c_void, _st: *mut core::ffi::c_void) -> usize {
    0
}
