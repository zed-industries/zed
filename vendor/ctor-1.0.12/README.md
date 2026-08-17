The crate is part of the [`linktime`](https://crates.io/crates/linktime) project.

[![GitHub](https://img.shields.io/badge/repo-github-blue)](https://github.com/mmastrac/linktime) [![Crates.io License](https://img.shields.io/crates/l/link-section)](https://crates.io/crates/link-section) [![Build Status](https://github.com/mmastrac/linktime/actions/workflows/rust.yml/badge.svg)](https://github.com/mmastrac/linktime/actions/workflows/rust.yml) 


| crate               |                                                         | docs                                                                                         | version                                                                                                           |
| ------------------- | ------------------------------------------------------- | -------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `linktime`          | Convenience crate for `ctor`, `dtor` and `link-section` | [![docs.rs](https://docs.rs/linktime/badge.svg)](https://docs.rs/linktime)                   | [![crates.io](https://img.shields.io/crates/v/linktime.svg)](https://crates.io/crates/linktime)                   |
| `ctor`              | Module initialization functions before main             | [![docs.rs](https://docs.rs/ctor/badge.svg)](https://docs.rs/ctor)                           | [![crates.io](https://img.shields.io/crates/v/ctor.svg)](https://crates.io/crates/ctor)                           |
| `dtor`              | Module shutdown functions before main                   | [![docs.rs](https://docs.rs/dtor/badge.svg)](https://docs.rs/dtor)                           | [![crates.io](https://img.shields.io/crates/v/dtor.svg)](https://crates.io/crates/dtor)                           |
| `link-section`      | Linker-managed typed (slices) and untyped sections      | [![docs.rs](https://docs.rs/link-section/badge.svg)](https://docs.rs/link-section)           | [![crates.io](https://img.shields.io/crates/v/link-section.svg)](https://crates.io/crates/link-section)           |
| `scattered-collect` | Linker-managed collections: slices, sorted slices, maps | [![docs.rs](https://docs.rs/scattered-collect/badge.svg)](https://docs.rs/scattered-collect) | [![crates.io](https://img.shields.io/crates/v/scattered-collect.svg)](https://crates.io/crates/scattered-collect) |
# ctor
Module initialization functions for Rust (like `__attribute__((constructor))` in
C/C++) for Linux, macOS, Windows, WASM, BSD-likes, and many others.

```rust
use ctor::ctor;
use libc_print::*;

#[ctor(unsafe)]
fn foo() {
    libc_println!("Life before main!");
}
```

## MSRV

For most platforms, this library currently has a MSRV of **Rust >= 1.60**.

The priority feature requires a MSRV of **Rust >= 1.85** on macOS targets.

MSRV for WASM targets is **Rust >= 1.85**.

## Lightweight

`ctor` has no dependencies other than the `linktime-proc-macro` and
`link-section` crates. The proc-macro is only used to delegate to the
declarative macro and should have minimal effect on compilation time.

## Support

This library works and is regularly tested on Linux, macOS, Windows, and
FreeBSD, with both `+crt-static` and `-crt-static` and `bin`/`cdylib` outputs.

Contributions to support other platforms or improve testing are welcome.

| OS           | Supported | CI Tested |
| ------------ | --------- | --------- |
| Linux        | ✅        | 🏅        |
| macOS        | ✅        | 🏅        |
| Windows      | ✅        | 🏅        |
| WASM 🕸️      | ✅        | 🏅        |
| FreeBSD      | ✅        | 💨        |
| NetBSD       | ✅        | 💨        |
| OpenBSD      | ✅        | 💨        |
| DragonFlyBSD | ✅        | 💨        |
| Illumos      | ✅        | -         |
| Android      | ✅        | -         |
| iOS          | ✅        | -         |
| AIX          | ✅        | -         |
| Haiku        | ✅        | -         |
| VxWorks      | ✅        | -         |
| Xtensa       | ✅        | -         |
| NTO          | ✅        | -         |
| UEFI         | ⚠️        | -         |

- 🏅 Full CI (miri, address sanitizer, etc.)
- 💨 Smoke tests (varying levels)
- ⚠️ Needs more feedback
- 🕸️ WASM `wasm-unknown-unknown`, `wasm-wasip1`, `wasm-wasip2` are supported.

- `wasm-unknown-unknown` requires host environment support for `atexit` if used
  with `dtor`.
- `wasm-wasip2` may require you to manually call `__wasm_call_ctors` and
  `__wasm_call_dtors` at the appropriate times.

## Warnings

Rust's philosophy is that nothing happens before or after main and this library
explicitly subverts that. The code that runs in the `ctor` and `dtor` functions
should be careful to limit itself to `libc` functions and code that does not
rely on Rust's stdlib services.

See [`::life_before_main`](crate::life_before_main) for more information.

## Usage

`#[ctor]` decorates a function item to be called as a module constructor. Both
free (a global `fn()`) and impl functions (`Self::method()`) are supported.

The example below marks the function `foo` as a module constructor, called when
a static library is loaded or an executable is started:

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use ctor::ctor;

static INITED: AtomicBool = AtomicBool::new(false);

#[ctor(unsafe)]
fn foo() {
    // ... (do something)
    INITED.store(true, Ordering::SeqCst);
}
```

Implementation methods can also be decorated with `#[ctor]`, as long as they
have no `self` parameter:

```rust
use ctor::ctor;

struct MyStruct {
    // ...
}

impl MyStruct {
    /// Ensure the required C library is loaded at startup time.
    #[ctor(unsafe)]
    fn load_required_c_library() {
        // ... (do something)
    }
}
```

### `static` items

The `#[ctor]` macro also supports decorating `static` items, which are
initialized at startup time. `static` items declared in this way must not be
accessed from other threads before the module constructors have run (if this is
done without caution, the initializer may panic).

The below example creates a `HashMap` populated with strings, which would
normally not be possible with `const` items:

```rust
use std::collections::HashMap;
use ctor::ctor;

#[ctor(unsafe)]
/// This is an immutable static, evaluated at init time
static STATIC_CTOR: HashMap<u32, &'static str> = {
    let mut m = HashMap::new();
    m.insert(0, "foo");
    m.insert(1, "bar");
    m.insert(2, "baz");
    m
};
```

### As a building block

The `#[ctor]` macro can be used as a building block for more complex
initialization logic. Use the [`declarative::ctor`](crate::declarative::ctor) to
easily export macros that re-use `ctor` functionality.

```rust
use ctor::ctor;

trait Driver: 'static + Send + Sync {
    // ...
}

static DRIVERS: ::std::sync::Mutex<Vec<Box<dyn Driver>>> = ::std::sync::Mutex::new(Vec::new());

fn register_driver(name: &'static str, driver: impl Driver) {
    DRIVERS.lock().unwrap().push(Box::new(driver));
}

#[ctor(unsafe, priority = late)]
fn walk_drivers() {
    for driver in DRIVERS.lock().unwrap().iter() {
        // ...
    }
}

macro_rules! register_driver {
    ($name:expr, $driver:expr) => {
        $crate::ctor::declarative::ctor!(
            #[ctor(unsafe, anonymous, priority = 1)]
            fn register() {
                register_driver($name, $driver);
            }
        );
    };
}

struct MyDriver {
    // ...
}

impl Driver for MyDriver {
    // ...
}

register_driver!("my_driver", MyDriver {});
```

## Under the Hood

The `#[ctor]` macro makes use of linker sections to ensure that a function is
run at startup time.

The above example translates into the following Rust code (approximately):

```rust
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func,mod_init_funcs")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
/* ... other platforms elided ... */
static FOO: extern fn() = {
    extern fn foo() { /* ... */ };
    foo
};
```

## Inspiration

The idea for `ctor` was originally inspired by the Neon project.
# Crate Features

| Cargo feature | Description |
| --- | --- |
| `priority` |  Enable support for the priority parameter. |
| `proc_macro` |  Enable support for the proc-macro `#[ctor]` attribute. The declarative form (`ctor!(...)`) is always available. It is recommended that crates re-exporting the `ctor` macro disable this feature and only use the declarative form. |
| `std` |  Enable support for the standard library. |

# Macro Attributes

<table><tr><th>Attribute</th><th>Description</th></tr>
<tr><td><code>anonymous</code></td><td>

 Do not give the constructor a name in the generated code (allows for
 multiple constructors with the same name). Equivalent to wrapping the
 constructor in an anonymous const (i.e.: `const _ = { ... };`).


</td></tr>
<tr><td><code>body(link_section = ".text.startup")</code></td><td>

 Place the constructor body in a custom link section. By default, this
 uses the appropriate platform-specific link section.

 Co-locating startup functions may improve performance by allowing the binary
 to page them in and out of memory together.


</td></tr>
<tr><td><code>crate_path = ::path::to::ctor::crate</code></td><td>

 The path to the `ctor` crate containing the support macros. If you
 re-export `ctor` items as part of your crate, you can use this to
 redirect the macro’s output to the correct crate.

 Using the declarative [`ctor!`][c] form is
 preferred over this parameter.

 [c]: crate::declarative::ctor!


</td></tr>
<tr><td><code>export_name_prefix = "ctor_"</code></td><td>

 Specify a custom export name prefix for the constructor function.

 If specified, an export with the given prefix will be generated in the form:

 `<prefix><priority>_<unique_id>`


</td></tr>
<tr><td><code>link_section = ".ctors"</code></td><td>

 Place the constructor function pointer in a custom link section. By
 default, this uses the appropriate platform-specific link section.


</td></tr>
<tr><td><code>naked</code></td><td>

 Use the least-possibly mangled version of the linker invocation for this
 constructor. This is not recommended for general use as it may prevent
 authors of binary crates from having low-level control over the order of
 initialization.

 There are no guarantees about the order of execution of constructors
 with this attribute, just that it will be called at some point before
 `main`.

 `naked` constructors are always executed directly by the underlying C
 library and/or dynamic loader.

 `naked` cannot be used with the `priority` attribute.


</td></tr>
<tr><td><code>priority = N | early | late</code></td><td>

 The priority of the constructor. Higher-`N`-priority constructors are
 run last. `N` must be between 0 and 999 inclusive for ordering
 guarantees (`N` >= 1000 ordering is platform-defined).

 Priority is specified as numeric value, string literal, or the
 identifiers `early`, `default`, or `late`. The integer value will be
 clamped to a platform-defined range (typically 0-65535), while string
 priorities are passed through unprocessed.

 Most platforms reserve the numeric values range of 0..100 for their own
 internal use and it may not be safe to access platform services (`libc`
 or other) in constructors with those priorities.

 Priority is applied as follows:

  - `N` is run in increasing order, from `0 <= N <= 999`.
  - `early` is run at a priority level where it is safe to access the C
    runtime. This is equivalent to a priority of 101 on most platforms.
  - `default` is the default, and is run after `early`. This is
    equivalent to a priority of 500.
  - `late` is run last, and will be positioned to run after most
    constructors, even outside the range `0 <= N <= 999`. The equivalent
    priority is platform-defined.
  - `main` is run, for binary targets.

 Ordering with explicit priority values outside of `0 <= N <= 999` is
 platform-defined with respect to the list above, however platforms will
 order constructors within a given priority range in ascending order
 (i.e.: 10000 will run before 20000).


</td></tr>
<tr><td><code>unsafe</code></td><td>


 Marks a ctor as unsafe. Required.

 The `ctor` crate rejects `#[ctor]` without marking the item unsafe;
 that error can be suppressed by passing
 `RUSTFLAGS="--cfg linktime_no_fail_on_missing_unsafe"` to Cargo.


</td></tr>
<tr><td><code>used(linker)</code></td><td>


 Mark generated function pointers `used(linker)`. Requires nightly
 for the nightly-only feature `feature(used_with_arg)` (see
 <https://github.com/rust-lang/rust/issues/93798>).

 This can be made the default by using the `cfg` flag
 `linktime_used_linker` (`RUSTFLAGS="--cfg linktime_used_linker"`).

 For a crate using this macro to function correctly with and without
 this flag, it is recommended to add the following line to the top of
 lib.rs in the crate root:

 `#![cfg_attr(linktime_used_linker, feature(used_with_arg))]`


</td></tr>
</table>

# Defaults

## `body_link_section`

 ```rust
#[cfg(target_os = "linux")]
body_link_section = ".text.startup"

#[cfg(target_os = "android")]
body_link_section = ".text.startup"

#[cfg(target_os = "freebsd")]
body_link_section = ".text.startup"

#[cfg(all(target_os = "windows", any(target_env = "gnu", target_env = "msvc")))]
body_link_section = ".text$A"

#[cfg(all(target_os = "windows", not(any(target_env = "gnu", target_env = "msvc"))))]
body_link_section = ".text.startup"

#[cfg(target_vendor = "apple")]
body_link_section = "__TEXT,__text_startup,regular,pure_instructions"

 // default
body_link_section = ()
 ```

## `export_name_prefix`

 ```rust
#[cfg(target_os = "aix")]
export_name_prefix = "__sinit"

 // default
export_name_prefix = ()
 ```

## `link_section`

 ```rust
#[cfg(target_vendor = "apple")]
link_section = "__DATA,__mod_init_func,mod_init_funcs"

#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd",
target_os = "netbsd", target_os = "openbsd", target_os = "dragonfly",
target_os = "illumos", target_os = "haiku", target_os = "vxworks", target_os =
"nto", target_family = "wasm"))]
link_section = ".init_array"

#[cfg(target_os = "none")]
link_section = ".init_array"

#[cfg(target_arch = "xtensa")]
link_section = ".ctors"

#[cfg(all(target_os = "windows", any(target_env = "gnu", target_env = "msvc")))]
link_section = ".CRT$XCU"

#[cfg(all(target_os = "windows", not(any(target_env = "gnu", target_env = "msvc"))))]
link_section = ".ctors"

#[cfg(target_os = "uefi")]
link_section = ".init_array"

#[cfg(target_os = "aix")]
link_section = ()

 // default
link_section = ".init_array"
 ```

## `priority`

 ```rust
#[cfg(feature = "priority")]
priority = default

 // default
priority = ()
 ```

## `r#unsafe`

 ```rust
#[cfg(linktime_no_fail_on_missing_unsafe)]
r#unsafe = (no_fail_on_missing_unsafe)

 // default
r#unsafe = ()
 ```

## `used_linker`

 ```rust
#[cfg(linktime_used_linker)]
used_linker = used_linker

 // default
used_linker = ()
 ```
