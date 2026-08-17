//! Compile behavior test for derive_builder on no_std. Unfortunately, this has broken
//! too many times due to changes in requirements for no_std, and therefore this test
//! is no longer part of the required nightly pass.

// requires nightly toolchain!
//
// compile-flags:-C panic=abort
#![no_std]
#![feature(alloc, allocator_api, lang_items, start, core_intrinsics, oom)]
#![allow(dead_code)]
use core::intrinsics;
use core::panic::PanicInfo;


// Pull in the system libc library for what crt0.o likely requires.
// extern crate libc;

#[macro_use]
extern crate derive_builder;
extern crate alloc;

#[derive(Builder)]
#[builder(no_std)]
struct IgnoreEmptyStruct {}

#[derive(Builder, PartialEq, Debug)]
#[builder(no_std)]
struct Foo {
    #[builder(default)]
    defaulted: u32,
    #[builder(setter(skip), try_setter)]
    skipped: u32,
}

fn main() {
    let foo = FooBuilder::default()
        .build()
        .unwrap();

    assert_eq!(foo, Foo {
        defaulted: 0,
        skipped: 0,
    })
}

///////////////////////////////////////////////////////////////
// some no_std-boilerplate
// from https://doc.rust-lang.org/book/no-stdlib.html
///////////////////////////////////////////////////////////////

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern  fn eh_personality() {}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[lang = "oom"]
#[no_mangle]
pub extern fn rust_oom(_: core::alloc::Layout) -> ! {
    unsafe { intrinsics::abort() }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    unsafe { intrinsics::abort() }
}

// Entry point for this program
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}
