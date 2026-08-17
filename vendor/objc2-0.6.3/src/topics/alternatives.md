# Alternatives to `objc2`

`objc2` is quite strict in its approach to interfacing with Objective-C, and encourages writing everything in Rust. This is, however, not always ideal, as our support for Objective-C will always be worse than that of the Objective-C compiler itself.

But there is another way: If you are willing to write a bit of Objective-C glue-code yourself, you can write a C wrapper, and interface with that instead of directly interoperating with Objective-C bindings.


## Example

Let's say you would like to know the user's current locale. This could be done using a C binding like the following:

```objective-c
// src/helper.m
#import <Foundation/Foundation.h>
#import <string.h>

const char* copy_current_locale_identifier(void) {
    NSString* identifier = [[NSLocale currentLocale] localeIdentifier];
    const char* s = [identifier UTF8String];
    return strdup(s);
}
```

Which is compiled in your `build.rs` (uses the `cc` crate).

```rust, ignore
// build.rs
fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let mut builder = cc::Build::new();
    builder.flag("-xobjective-c");
    builder.flag("-fobjc-arc");
    builder.flag("-fmodules");

    builder.file("src/helper.m");
    println!("cargo::rerun-if-changed=src/helper.m");

    builder.compile("libhelper.a");
}
```

And then exposed to Rust like this:

```rust, ignore
// src/lib.rs
use libc::free;
use std::ffi::{c_char, c_void, CStr, CString};

extern "C-unwind" {
    fn copy_current_locale_identifier() -> *const c_char;
}

pub fn current_locale() -> CString {
    let ptr = unsafe { copy_current_locale_identifier() };
    let ret = unsafe { CStr::from_ptr(ptr).to_owned() };
    unsafe { free(ptr as *mut c_void) };
    ret
}
```
