#![recursion_limit = "256"]
#![no_std]
#![doc = include_str!("../docs/BUILD.md")]
//! # ctor
#![doc = include_str!("../docs/PREAMBLE.md")]
#![doc = include_str!("../docs/GENERATED.md")]
// Used as part of ctor collection
#![cfg_attr(
    all(target_vendor = "apple", linktime_used_linker),
    feature(used_with_arg)
)]
#![cfg_attr(all(target_vendor = "apple", linktime_asan), feature(sanitize))]

#[cfg(feature = "std")]
extern crate std;

mod macros;
mod parse;
mod priority;
#[cfg(target_os = "aix")]
mod priority_aix;

pub mod statics;

#[doc = include_str!("../docs/LIFE_BEFORE_MAIN.md")]
pub mod life_before_main {}

#[doc(hidden)]
#[allow(unused)]
pub mod __support {
    // Required for proc_macro.
    pub use crate::__ctor_parse as ctor_parse;

    // Re-export link_section::TypedSection and declarative::{section, in_section}
    #[cfg(all(feature = "priority", target_vendor = "apple"))]
    pub use link_section::declarative::in_section;
}

#[cfg(all(feature = "priority", target_vendor = "apple"))]
crate::__ctor_parse_internal!(
    __ctor_features,
    /// Define a link section when using the priority parameter on Apple
    /// targets. This is awkwardly placed in the root module because it needs to
    /// use a generated macro and we cannot use an absolute path to it. (see
    /// <https://github.com/rust-lang/rust/issues/52234>)
    #[ctor(unsafe, naked)]
    #[allow(unsafe_code)]
    #[doc(hidden)]
    fn priority_ctor() {
        unsafe {
            crate::collect::run_constructors();
        }
    }
);

/// Collected constructors for platforms requiring manual invocation.
#[cfg(all(feature = "priority", target_vendor = "apple"))]
#[doc(hidden)]
pub mod collect {
    use core::sync::atomic::{AtomicU8, Ordering};

    #[doc(hidden)]
    pub const PROCESSED: isize = isize::MIN;
    #[doc(hidden)]
    pub const LATE: isize = isize::MAX;

    const GUARD_NOT_RUN: u8 = 0;
    const GUARD_RUNNING: u8 = 1;
    const GUARD_FINISHED: u8 = 2;

    /// A constructor record.
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Constructor {
        pub priority: isize,
        pub ctor: unsafe extern "C" fn(),
    }

    /// Run all constructors in the CTOR section. It is assumed that there is
    /// only ever one of these calls active at any time, regardless of how many
    /// versions of the ctor crate are in use.
    ///
    /// # Safety
    ///
    /// We use a guard section to ensure that only one version of the ctor crate
    /// is running constructors at any time.
    ///
    /// If another copy of this function is running, we will return early, but
    /// the constructors will not have been guaranteed to have run.
    #[allow(unsafe_code)]
    pub(crate) unsafe fn run_constructors() {
        // Multiple ctor crates may contribute multiple guards, but there will
        // only ever be one "first" guard.
        let Some(guard) = _CTR0GR_ISIZE_FN.first() else {
            return;
        };

        // In the unlikely case we are racing multiple threads, one will win.
        loop {
            match guard.compare_exchange_weak(
                GUARD_NOT_RUN,
                GUARD_RUNNING,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(GUARD_NOT_RUN) => {
                    // Spurious failure, try again
                    continue;
                }
                Err(_) => return,
            }
        }

        // SAFETY: Limit the scope of the mutable slice. This slice is only ever
        // accessed under the guard.
        unsafe {
            let slice = _CTOR0_ISIZE_FN.as_mut_slice();
            slice.sort_unstable_by_key(|constructor| constructor.priority);
        }

        unsafe {
            let start = _CTOR0_ISIZE_FN.start_ptr_mut();
            let end = _CTOR0_ISIZE_FN.end_ptr_mut();
            let mut ptr = start;
            while ptr < end {
                let mut constructor = ptr.read();
                if constructor.priority != crate::collect::PROCESSED {
                    constructor.priority = crate::collect::PROCESSED;
                    ptr.write(constructor);
                    (constructor.ctor)();
                }
                ptr = ptr.add(1);
            }
        }

        guard.store(GUARD_FINISHED, Ordering::Release);
    }

    // Note: The section names must be <= 16 characters long to fit in the mach-o limits.
    // These sections are shared between multiple versions of the ctor crate.

    link_section::declarative::section!(
        #[section(unsafe, type = mutable, name = _CTOR0_ISIZE_FN)]
        static _CTOR0_ISIZE_FN: link_section::TypedMutableSection<Constructor>;
    );

    link_section::declarative::section!(
        #[section(unsafe, type = typed, name = _CTR0GR_ISIZE_FN)]
        static _CTR0GR_ISIZE_FN: link_section::TypedSection<AtomicU8>;
    );

    link_section::declarative::in_section!(
        #[in_section(unsafe, type = typed, name = _CTR0GR_ISIZE_FN)]
        pub static GUARD_ATOMIC: AtomicU8 = AtomicU8::new(GUARD_NOT_RUN);
    );

    // TODO: We should allow explicit export_name on the ctor itself.
    #[cfg(all(feature = "priority", target_vendor = "apple"))]
    #[used]
    #[doc(hidden)]
    #[allow(unsafe_code)]
    #[cfg_attr(clippy, allow(unknown_lints, unsafe_attr_outside_unsafe))]
    #[export_name = concat!(
        "ctor_ap_",
        env!("CARGO_PKG_VERSION_MAJOR"),
        "_",
        env!("CARGO_PKG_VERSION_MINOR"),
        "_",
        env!("CARGO_PKG_VERSION_PATCH")
    )]
    pub static APPLE_PRIORITY_ANCHOR: fn() = crate::priority_ctor;

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __keep_alive {
        () => {
            /// Force `ld64` to pull the archive member owning `APPLE_PRIORITY_ANCHOR`
            /// (see https://github.com/mmastrac/linktime/issues/496).
            const _: () = {
                mod __ctor_force {
                    ::core::arch::global_asm!(
                        ".pushsection __DATA,__ctor_force,regular,no_dead_strip\n",
                        // Pointer-align a `.quad` with the anchor
                        ".p2align 3\n",
                        ".quad {0}\n",
                        ".popsection\n",
                        sym $crate::collect::APPLE_PRIORITY_ANCHOR,
                    );
                }
            };
        };
    }

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __register_ctor {
        (priority = (), fn = $fn:ident) => {
            $crate::__register_ctor!(priority = ($crate::collect::EARLY), fn = $fn);
        };
        (priority = early, fn = $fn:ident) => {
            $crate::__register_ctor!(priority = ($crate::collect::EARLY), fn = $fn);
        };
        (priority = late, fn = $fn:ident) => {
            $crate::__register_ctor!(priority = ($crate::collect::LATE), fn = $fn);
        };
        (priority = $priority:tt, fn = $fn:ident) => {
            $crate::__keep_alive!();
            $crate::__support::in_section!(
                #[in_section(unsafe, type = mutable, name = _CTOR0_ISIZE_FN)]
                const _: $crate::collect::Constructor = $crate::collect::Constructor {
                    priority: $priority,
                    ctor: $fn,
                };
            );
        };
        (priority = $priority:tt, fn = (array $array:ident)) => {
            $crate::__keep_alive!();
            $crate::__support::in_section!(
                #[in_section(unsafe, type = mutable, name = _CTOR0_ISIZE_FN)]
                const _: [$crate::collect::Constructor; if $array.len() == 0 { 1 } else { $array.len() }] = {
                    use core::mem::MaybeUninit;

                    // If length zero, register a stub that doesn't get executed
                    if $array.len() == 0 {
                        unsafe extern "C" fn empty_ctor() {}
                        [$crate::collect::Constructor {
                            priority: $crate::collect::PROCESSED,
                            ctor: empty_ctor,
                        }; if $array.len() == 0 { 1 } else { $array.len() }]
                    } else {
                        let mut array: MaybeUninit<[$crate::collect::Constructor; if $array.len() == 0 { 1 } else { $array.len() }]> = MaybeUninit::uninit();
                        let mut array_ptr: *mut $crate::collect::Constructor = array.as_mut_ptr() as _;
                        const fn ctor_fn(i: usize) -> $crate::collect::Constructor {
                            $crate::collect::Constructor {
                                priority: $priority,
                                ctor: $array[i],
                            }
                        }

                        let mut i = 0;
                        while i < $array.len() {
                            unsafe { array_ptr.add(i).write(ctor_fn(i)) };
                            i += 1;
                        }

                        unsafe { array.assume_init() }
                    }
                };
            );
        };
    }
}

/// Declarative form of the `#[ctor]` macro.
pub mod declarative {
    /// Declarative form of the [`#[ctor]`](crate::ctor) macro.
    ///
    /// The declarative forms wrap and parse a proc_macro-like syntax like so, and
    /// are identical in expansion to the undecorated procedural macros. The
    /// declarative forms support the same attribute parameters as the procedural
    /// macros.
    ///
    /// ```rust
    /// # #[cfg(not(miri))] mod test { use ctor::*; use libc_print::*;
    /// ctor::declarative::ctor! {
    ///   #[ctor(unsafe)]
    ///   fn foo() {
    ///     libc_println!("Hello, world!");
    ///   }
    /// }
    /// # }
    ///
    /// // ... the above is identical to:
    ///
    /// # #[cfg(not(miri))] mod test_2 { use ctor::*; use libc_print::*;
    /// #[ctor(unsafe)]
    /// fn foo() {
    ///   libc_println!("Hello, world!");
    /// }
    /// # }
    /// ```
    #[doc(inline)]
    pub use crate::__support::ctor_parse as ctor;
}

/// Marks a function or static variable as a library/executable constructor.
/// This uses OS-specific linker sections to call a specific function at load
/// time.
///
/// # Important notes
///
/// Rust does not make any guarantees about stdlib support for life-before or
/// life-after main. This means that the `ctor` crate may not work as expected
/// in some cases, such as when used in an `async` runtime or making use of
/// stdlib services.
///
/// Multiple startup functions/statics are supported, but the invocation order
/// is not guaranteed.
///
/// The `ctor` crate assumes it is available as a direct dependency, If you
/// re-export `ctor` items as part of your crate, you can use the `crate_path`
/// parameter to redirect the macro's output to the correct crate, or use the
/// [`declarative::ctor`] form.
///
/// # Examples
///
/// Print a startup message (using `libc_print` for safety):
///
/// ```rust
/// # #[cfg(not(miri))] mod test {
/// # use ctor::ctor;
/// use libc_print::std_name::println;
///
/// #[ctor(unsafe)]
/// fn foo() {
///   // Using libc_print which is safe in `#[ctor]`
///   println!("Hello, world!");
/// }
///
/// # fn main() {
/// println!("main()");
/// # }}
/// ```
///
/// Make changes to `static` variables:
///
/// ```rust
/// # mod test {
/// use ctor::*;
/// use std::sync::atomic::{AtomicBool, Ordering};
/// static INITED: AtomicBool = AtomicBool::new(false);
///
/// #[ctor(unsafe)]
/// fn set_inited() {
///   INITED.store(true, Ordering::SeqCst);
/// }
/// # }
/// ```
///
/// Initialize a `HashMap` at startup time:
///
/// ```rust
/// # mod test {
/// # use std::collections::HashMap;
/// # use ctor::*;
/// #[ctor(unsafe)]
/// pub static STATIC_CTOR: HashMap<u32, String> = {
///   let mut m = HashMap::new();
///   for i in 0..100 {
///     m.insert(i, format!("x*100={}", i*100));
///   }
///   m
/// };
/// # }
/// # pub fn main() {
/// #   assert_eq!(test::STATIC_CTOR.len(), 100);
/// #   assert_eq!(test::STATIC_CTOR[&20], "x*100=2000");
/// # }
/// ```
///
/// # Details
///
/// The `#[ctor]` macro makes use of linker sections to ensure that a function
/// is run at startup time.
///
/// ```rust
/// # mod test {
/// # use ctor::*;
/// #[ctor(unsafe)]
/// fn my_init_fn() {
///   /* ... */
/// }
/// # }
/// ```
///
/// The above example translates into the following Rust code (approximately):
///
/// ```rust
/// # fn my_init_fn() {}
/// #[used]
/// #[cfg_attr(target_os = "linux", link_section = ".init_array")]
/// #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func,mod_init_funcs")]
/// #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
/// /* ... other platforms elided ... */
/// static INIT_FN: extern fn() = {
///     extern fn init_fn() { my_init_fn(); };
///     init_fn
/// };
/// ```
///
/// For `static` items, the macro generates a `std::sync::OnceLock` that is
/// initialized at startup time. `#[ctor]` on `static` items requires the
/// default `std` feature.
///
/// ```rust
/// # mod test {
/// # use ctor::*;
/// # use std::collections::HashMap;
/// #[ctor]
/// static FOO: HashMap<u32, String> = unsafe {
///   let mut m = HashMap::new();
///   for i in 0..100 {
///     m.insert(i, format!("x*100={}", i*100));
///   }
///   m
/// };
/// # }
/// ```
///
/// The above example translates into the following Rust code (approximately),
/// which eagerly initializes the `HashMap` inside a `OnceLock` at startup time:
///
/// ```rust
/// # mod test {
/// # use ctor::ctor;
/// # use std::collections::HashMap;
/// static FOO: FooStatic = FooStatic { value: ::std::sync::OnceLock::new() };
/// struct FooStatic {
///   value: ::std::sync::OnceLock<HashMap<u32, String>>,
/// }
///
/// impl ::core::ops::Deref for FooStatic {
///   type Target = HashMap<u32, String>;
///   fn deref(&self) -> &Self::Target {
///     self.value.get_or_init(|| unsafe {
///       let mut m = HashMap::new();
///       for i in 0..100 {
///         m.insert(i, format!("x*100={}", i*100));
///       }
///       m
///     })
///   }
/// }
///
/// #[ctor]
/// unsafe fn init_foo_ctor() {
///   _ = &*FOO;
/// }
/// # }
/// ```
#[doc(inline)]
#[cfg(feature = "proc_macro")]
pub use ::linktime_proc_macro::ctor;

__declare_features!(
    ctor: __ctor_features;

    /// Do not give the constructor a name in the generated code (allows for
    /// multiple constructors with the same name). Equivalent to wrapping the
    /// constructor in an anonymous const (i.e.: `const _ = { ... };`).
    anonymous {
        attr: [(anonymous) => (anonymous)];
    };
    /// Place the constructor body in a custom link section. By default, this
    /// uses the appropriate platform-specific link section.
    ///
    /// Co-locating startup functions may improve performance by allowing the binary
    /// to page them in and out of memory together.
    body_link_section {
        attr: [(body(link_section = $body_section:literal)) => ($body_section)];
        example: "body(link_section = \".text.startup\")";
        default {
            (target_os = "linux") => ".text.startup",
            (target_os = "android") => ".text.startup",
            (target_os = "freebsd") => ".text.startup",
            // Windows MSVC: sort startup functions near the start of the binary
            (all(target_os = "windows", any(target_env = "gnu", target_env = "msvc"))) => ".text$A",
            // Windows non-MSVC: .text.startup
            (all(target_os = "windows", not(any(target_env = "gnu", target_env = "msvc")))) => ".text.startup",
            (target_vendor = "apple") => "__TEXT,__text_startup,regular,pure_instructions",
            _ => ()
        }
    };
    /// The path to the `ctor` crate containing the support macros. If you
    /// re-export `ctor` items as part of your crate, you can use this to
    /// redirect the macro’s output to the correct crate.
    ///
    /// Using the declarative [`ctor!`][c] form is
    /// preferred over this parameter.
    ///
    /// [c]: crate::declarative::ctor!
    crate_path {
        attr: [(crate_path = $path:pat) => (($path))];
        example: "crate_path = ::path::to::ctor::crate";
    };
    /// Specify a custom export name prefix for the constructor function.
    ///
    /// If specified, an export with the given prefix will be generated in the form:
    ///
    /// `<prefix><priority>_<unique_id>`
    export_name_prefix {
        attr: [(export_name_prefix = $export_name_prefix_str:literal) => ($export_name_prefix_str)];
        example: "export_name_prefix = \"ctor_\"";
        default {
            (target_os = "aix") => "__sinit",
            _ => (),
        }
    };
    /// Place the constructor function pointer in a custom link section. By
    /// default, this uses the appropriate platform-specific link section.
    // NOTE: Keep in sync w/dtor::ctor_link_section!
    link_section {
        attr: [(link_section = $section:literal) => ($section)];
        example: "link_section = \".ctors\"";
        // Historical note: GCC 4.7 stopped providing .ctors/.dtors compatible
        // crt files. Modern compilers, with the exception of Apple, MSVC, and
        // AIX, will use `.init_array`.
        default {
            (target_vendor = "apple") => "__DATA,__mod_init_func,mod_init_funcs",
            // Most LLVM/GCC targets can use .init_array
            (any(
                target_os = "linux",
                target_os = "android",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
                target_os = "dragonfly",
                target_os = "illumos",
                target_os = "haiku",
                target_os = "vxworks",
                target_os = "nto",
                target_family = "wasm"
            )) => ".init_array",
            // No OS
            (target_os = "none") => ".init_array",
            // xtensa targets: .ctors
            (target_arch = "xtensa") => ".ctors",
            // Windows targets: .CRT$XCU
            (all(target_os = "windows", any(target_env = "gnu", target_env = "msvc"))) => ".CRT$XCU",
            // ... except mingw32 (https://llvm.googlesource.com/clang/+/1a209b667f83588866326a0384fa943ea2287b6c)
            (all(target_os = "windows", not(any(target_env = "gnu", target_env = "msvc")))) => ".ctors",
            // Research suggests that MSVC will use .CRT$XCU for UEFI targets, but won't actually
            // run them. The gnu-efi project _does_ at least document .init_array support:
            // https://github.com/vathpela/gnu-efi/blob/master/gnuefi/elf_x86_64_efi.lds
            (target_os = "uefi") => ".init_array",
            (target_os = "aix") => (), // AIX uses export_name_prefix
            // Fall back to .init_array which is effectively the gold standard
            // for LLVM/GCC targets moving forward
            #[warn("Falling back to .init_array for unsupported target. If this \
            works for you, please file an issue to add support for your target \
            at https://github.com/mmastrac/linktime/issues")]
            _ => ".init_array",
        }
    };
    /// Use the least-possibly mangled version of the linker invocation for this
    /// constructor. This is not recommended for general use as it may prevent
    /// authors of binary crates from having low-level control over the order of
    /// initialization.
    ///
    /// There are no guarantees about the order of execution of constructors
    /// with this attribute, just that it will be called at some point before
    /// `main`.
    ///
    /// `naked` constructors are always executed directly by the underlying C
    /// library and/or dynamic loader.
    ///
    /// `naked` cannot be used with the `priority` attribute.
    naked {
        attr: [(naked) => (naked)];
    };
    /// The priority of the constructor. Higher-`N`-priority constructors are
    /// run last. `N` must be between 0 and 999 inclusive for ordering
    /// guarantees (`N` >= 1000 ordering is platform-defined).
    ///
    /// Priority is specified as numeric value, string literal, or the
    /// identifiers `early`, `default`, or `late`. The integer value will be
    /// clamped to a platform-defined range (typically 0-65535), while string
    /// priorities are passed through unprocessed.
    ///
    /// Most platforms reserve the numeric values range of 0..100 for their own
    /// internal use and it may not be safe to access platform services (`libc`
    /// or other) in constructors with those priorities.
    ///
    /// Priority is applied as follows:
    ///
    ///  - `N` is run in increasing order, from `0 <= N <= 999`.
    ///  - `early` is run at a priority level where it is safe to access the C
    ///    runtime. This is equivalent to a priority of 101 on most platforms.
    ///  - `default` is the default, and is run after `early`. This is
    ///    equivalent to a priority of 500.
    ///  - `late` is run last, and will be positioned to run after most
    ///    constructors, even outside the range `0 <= N <= 999`. The equivalent
    ///    priority is platform-defined.
    ///  - `main` is run, for binary targets.
    ///
    /// Ordering with explicit priority values outside of `0 <= N <= 999` is
    /// platform-defined with respect to the list above, however platforms will
    /// order constructors within a given priority range in ascending order
    /// (i.e.: 10000 will run before 20000).
    priority {
        attr: [(priority = $priority_value:tt) => ($priority_value)];
        example: "priority = N | early | late";
        validate: [($priority:literal), (early), (late), (default)];
        default {
            (feature = "priority") => default,
            _ => ()
        }
    };
    /// Enable support for the priority parameter.
    priority_enabled {
        feature: "priority";
    };
    /// Enable support for the proc-macro `#[ctor]` attribute. The declarative
    /// form (`ctor!(...)`) is always available. It is recommended that crates
    /// re-exporting the `ctor` macro disable this feature and only use the
    /// declarative form.
    proc_macro {
        feature: "proc_macro";
    };
    /// Enable support for the standard library.
    std {
        feature: "std";
    };
    r#unsafe {
        /// attr
        ///
        /// Marks a ctor as unsafe. Required.
        ///
        /// The `ctor` crate rejects `#[ctor]` without marking the item unsafe;
        /// that error can be suppressed by passing
        /// `RUSTFLAGS="--cfg linktime_no_fail_on_missing_unsafe"` to Cargo.
        attr: [(unsafe) => (no_fail_on_missing_unsafe)];
        default {
            (linktime_no_fail_on_missing_unsafe) => (no_fail_on_missing_unsafe),
            _ => (),
        }
    };
    used_linker {
        /// attr
        ///
        /// Mark generated function pointers `used(linker)`. Requires nightly
        /// for the nightly-only feature `feature(used_with_arg)` (see
        /// <https://github.com/rust-lang/rust/issues/93798>).
        ///
        /// This can be made the default by using the `cfg` flag
        /// `linktime_used_linker` (`RUSTFLAGS="--cfg linktime_used_linker"`).
        ///
        /// For a crate using this macro to function correctly with and without
        /// this flag, it is recommended to add the following line to the top of
        /// lib.rs in the crate root:
        ///
        /// `#![cfg_attr(linktime_used_linker, feature(used_with_arg))]`
        attr: [(used(linker)) => (used_linker)];
        default {
            (linktime_used_linker) => used_linker,
            _ => (),
        }
    };
);

#[cfg(doc)]
__generate_docs!(__ctor_features);
