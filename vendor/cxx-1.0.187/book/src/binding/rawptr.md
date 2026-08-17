{{#title *mut T, *const T — Rust ♡ C++}}
# *mut T,&ensp;*const T

Generally you should use references (`&mut T`, `&T`) or [std::unique_ptr\<T\>]
where possible over raw pointers, but raw pointers are available too as an
unsafe fallback option.

[std::unique_ptr\<T\>]: uniqueptr.md

### Restrictions:

Extern functions and function pointers taking a raw pointer as an argument must
be declared `unsafe fn` i.e. unsafe to call. The same does not apply to
functions which only *return* a raw pointer, though presumably doing anything
useful with the returned pointer is going to involve unsafe code elsewhere
anyway.

## Example

This example illustrates making a Rust call to a canonical C-style `main`
signature involving `char *argv[]`.

```cpp
// include/args.h

#pragma once

void parseArgs(int argc, char *argv[]);
```

```cpp
// src/args.cc

#include "example/include/args.h"
#include <iostream>

void parseArgs(int argc, char *argv[]) {
  std::cout << argc << std::endl;
  for (int i = 0; i < argc; i++) {
    std::cout << '"' << argv[i] << '"' << std::endl;
  }
}
```

```rust,noplayground
// src/main.rs

use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::unix::ffi::OsStrExt;
use std::ptr;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("example/include/args.h");

        unsafe fn parseArgs(argc: i32, argv: *mut *mut c_char);
    }
}

fn main() {
    // Convert from OsString to nul-terminated CString, truncating each argument
    // at the first inner nul byte if present.
    let args: Vec<CString> = env::args_os()
        .map(|os_str| {
            let bytes = os_str.as_bytes();
            CString::new(bytes).unwrap_or_else(|nul_error| {
                let nul_position = nul_error.nul_position();
                let mut bytes = nul_error.into_vec();
                bytes.truncate(nul_position);
                CString::new(bytes).unwrap()
            })
        })
        .collect();

    // Convert from Vec<CString> of owned strings to Vec<*mut c_char> of
    // borrowed string pointers.
    //
    // Once extern type stabilizes (https://github.com/rust-lang/rust/issues/43467)
    // and https://internals.rust-lang.org/t/pre-rfc-make-cstr-a-thin-pointer/6258
    // is implemented, and CStr pointers become thin, we can sidestep this step
    // by accumulating the args as Vec<Box<CStr>> up front, then simply casting
    // from *mut [Box<CStr>] to *mut [*mut CStr] to *mut *mut c_char.
    let argc = args.len();
    let mut argv: Vec<*mut c_char> = Vec::with_capacity(argc + 1);
    for arg in &args {
        argv.push(arg.as_ptr() as *mut c_char);
    }
    argv.push(ptr::null_mut()); // Nul terminator.

    unsafe {
        ffi::parseArgs(argc as i32, argv.as_mut_ptr());
    }

    // The CStrings go out of scope here. C function must not have held on to
    // the pointers beyond this point.
}
```
