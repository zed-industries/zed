//! # Raw bindings to Objective-C runtimes
//!
//! These bindings contain almost no documentation, so it is highly
//! recommended to read Apple's [documentation about the Objective-C
//! runtime][runtime-guide], Apple's [runtime reference][apple], or to use
//! `objc2::runtime` which provides a higher-level API.
//!
//! [runtime-guide]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Introduction/Introduction.html
//! [apple]: https://developer.apple.com/documentation/objectivec/objective-c_runtime?language=objc
//!
//!
//! ## Runtime Support
//!
//! Objective-C has a runtime, different implementations of said runtime
//! exist, and they act in slightly different ways. By default, Apple
//! platforms link to Apple's runtime, but if you're using another runtime you
//! must tell it to this library using feature flags (you might have to
//! disable the default `apple` feature first).
//!
//! One could ask, why even bother supporting other runtimes? For me, the
//! primary reasoning iss _robustness_. By testing with these alternative
//! runtimes in CI, we become by extension much more confident that our
//! implementation doesn't rely on brittle unspecified behaviour, and works
//! across different macOS and iOS versions.
//!
//!
//! ### Apple's [`objc4`](https://github.com/apple-oss-distributions/objc4)
//!
//! - Feature flag: `apple`.
//!
//! This is used by default, and has the highest support priority (all of
//! `objc2` will work with this runtime).
//!
//! The supported runtime version (higher versions lets the compiler enable
//! newer optimizations, at the cost of not supporting older operating
//! systems) can be chosen using the standard `X_DEPLOYMENT_TARGET`
//! environment variables:
//!
//! - macOS: `MACOSX_DEPLOYMENT_TARGET`, default `10.12`, `11.0` on Aarch64.
//! - iOS / iPadOS: `IPHONEOS_DEPLOYMENT_TARGET`, default `10.0`.
//! - tvOS: `TVOS_DEPLOYMENT_TARGET`, default `10.0`.
//! - watchOS: `WATCHOS_DEPLOYMENT_TARGET`, default `5.0`.
//!
//! The default (and minimum) versions are the [same as those Rust itself
//! has][rust-apple-spec].
//!
//! [rust-apple-spec]: https://github.com/rust-lang/rust/blob/1.74.0/compiler/rustc_target/src/spec/apple_base.rs
//!
//!
//! ### GNUStep's [`libobjc2`](https://github.com/gnustep/libobjc2)
//!
//! - Feature flag: `gnustep-1-7`, `gnustep-1-8`, `gnustep-1-9`, `gnustep-2-0`
//!   and `gnustep-2-1` depending on the version you're using.
//!
//!
//! ### Microsoft's [`WinObjC`](https://github.com/microsoft/WinObjC)
//!
//! - Feature flag: `unstable-winobjc`.
//!
//! **Unstable: Hasn't been tested on Windows yet!**
//!
//! [A fork](https://github.com/microsoft/libobjc2) based on GNUStep's
//! `libobjc2` version 1.8, with very few user-facing changes.
//!
//!
//! ### [`ObjFW`](https://github.com/ObjFW/ObjFW)
//!
//! - Feature flag: `unstable-objfw`.
//!
//! **Unstable: Doesn't work yet!**
//!
//! TODO.
//!
//!
//! ### Other runtimes
//!
//! This library will probably only ever support ["Modern"][modern]
//! Objective-C runtimes, since support for reference-counting primitives like
//! `objc_retain` and `objc_autoreleasePoolPop` is a vital requirement for
//! most applications.
//!
//! This rules out the GCC [`libobjc`][gcc-libobjc] runtime (see
//! [this][gcc-objc-support]), the [`mulle-objc`] runtime and [cocotron]. (But
//! support for [`darling`] may be added). More information on different
//! runtimes can be found in GNUStep's [Objective-C Compiler and Runtime
//! FAQ][gnustep-faq].
//!
//! [modern]: https://en.wikipedia.org/wiki/Objective-C#Modern_Objective-C
//! [gcc-libobjc]: https://github.com/gcc-mirror/gcc/tree/master/libobjc
//! [gcc-objc-support]: https://gcc.gnu.org/onlinedocs/gcc/Standards.html#Objective-C-and-Objective-C_002b_002b-Languages
//! [`mulle-objc`]: https://github.com/mulle-objc/mulle-objc-runtime
//! [cocotron]: https://cocotron.org/
//! [`darling`]: https://github.com/darlinghq/darling-objc4
//! [gnustep-faq]: http://wiki.gnustep.org/index.php/Objective-C_Compiler_and_Runtime_FAQ
//!
//!
//! ## Advanced linking configuration
//!
//! This crate defines the `links` key in `Cargo.toml` so it's possible to
//! change the linking to `libobjc`, see [the relevant cargo
//! docs][overriding].
//!
//! In the future, this crate may vendor the required source code to
//! automatically build and link to the runtimes. Choosing static vs. dynamic
//! linking here may also become an option.
//!
//! [overriding]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts
//!
//!
//! ## Objective-C Compiler configuration
//!
//! Objective-C compilers like `clang` and `gcc` requires configuring the
//! calling ABI to the runtime you're using:
//! - `clang` uses the [`-fobjc-runtime`] flag, of which there are a few
//!   different [options][clang-objc-kinds].
//! - `gcc` uses the [`-fgnu-runtime` or `-fnext-runtime`][gcc-flags] options.
//!   Note that Modern Objective-C features are ill supported.
//!
//! This is relevant if you're building and linking to custom Objective-C
//! sources in a build script. To assist in compiling Objective-C sources,
//! this crate's  build script expose the `DEP_OBJC_0_3_CC_ARGS` environment
//! variable to downstream build scripts.
//!
//! Example usage in your `build.rs` (using the `cc` crate) would be as
//! follows:
//!
//! ```ignore
//! fn main() {
//!     let mut builder = cc::Build::new();
//!     builder.compiler("clang");
//!     builder.file("my_objective_c_script.m");
//!
//!     for flag in std::env::var("DEP_OBJC_0_3_CC_ARGS").unwrap().split(' ') {
//!         builder.flag(flag);
//!     }
//!
//!     builder.compile("libmy_objective_c_script.a");
//! }
//! ```
//!
//! [`-fobjc-runtime`]: https://clang.llvm.org/docs/ClangCommandLineReference.html#cmdoption-clang-fobjc-runtime
//! [clang-objc-kinds]: https://clang.llvm.org/doxygen/classclang_1_1ObjCRuntime.html#af19fe070a7073df4ecc666b44137c4e5
//! [gcc-flags]: https://gcc.gnu.org/onlinedocs/gcc/Objective-C-and-Objective-C_002b_002b-Dialect-Options.html
//!
//!
//! ## Design choices
//!
//! It is recognized that the most primary consumer of this library will be
//! macOS and secondly iOS applications. Therefore it was chosen not to use
//! `bindgen`[^1] in our build script to not add compilation cost to those
//! targets.
//!
//! Deprecated functions are also not included for future compability, since
//! they could be removed in any macOS release, and then our code would break.
//! If you have a need for these, please open an issue and we can discuss it!
//!
//! Some items (in particular the `objc_msgSend_X` family) have `cfg`s that
//! prevent their usage on different platforms; these are **semver-stable** in
//! the sense that they will only get less restrictive, never more.
//!
//! [^1]: That said, most of this is created with the help of `bindgen`'s
//! commandline interface, so huge thanks to them!

#![no_std]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/objc-sys/0.3.5")]
#![cfg_attr(feature = "unstable-c-unwind", feature(c_unwind))]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(doc)))]

// TODO: Remove this and add "no-std" category to Cargo.toml
// Requires a better solution for C-types in `no_std` crates.
// See https://github.com/japaric/cty/issues/14.
extern crate std;

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

use core::cell::UnsafeCell;
use core::marker::{PhantomData, PhantomPinned};

macro_rules! generate_linking_tests {
    {
        extern $abi:literal {$(
            $(#[$m:meta])*
            $v:vis fn $name:ident(
                $($(#[$a_m:meta])* $a:ident: $t:ty),* $(,)?
            ) $(-> $r:ty)?;
        )+}
        mod $test_name:ident;
    } => {
        extern $abi {$(
            $(#[$m])*
            $v fn $name($($(#[$a_m])* $a: $t),*) $(-> $r)?;
        )+}

        #[allow(deprecated)]
        #[cfg(test)]
        mod $test_name {
            #[allow(unused)]
            use super::*;

            $(
                $(#[$m])*
                #[test]
                fn $name() {
                    // Get function pointer to make the linker require the
                    // symbol to be available.
                    let f: unsafe extern $abi fn($($(#[$a_m])* $t),*) $(-> $r)? = crate::$name;
                    // Workaround for https://github.com/rust-lang/rust/pull/92964
                    #[cfg(feature = "unstable-c-unwind")]
                    #[allow(clippy::useless_transmute)]
                    let f: unsafe extern "C" fn() = unsafe { core::mem::transmute(f) };
                    // Execute side-effect to ensure it is not optimized away.
                    std::println!("{:p}", f);
                }
            )+
        }
    };
}

macro_rules! extern_c {
    {
        $(
            $(#[$m:meta])*
            $v:vis fn $name:ident(
                $($(#[$a_m:meta])* $a:ident: $t:ty),* $(,)?
            ) $(-> $r:ty)?;
        )+
    } => {
        generate_linking_tests! {
            extern "C" {$(
                $(#[$m])*
                $v fn $name($($(#[$a_m])* $a: $t),*) $(-> $r)?;
            )+}
            mod test_linkable;
        }
    };
}

// A lot of places may call `+initialize`, but the runtime guards those calls
// with `@try/@catch` blocks already, so we don't need to mark every function
// "C-unwind", only certain ones!
macro_rules! extern_c_unwind {
    {
        $(
            $(#[$m:meta])*
            $v:vis fn $name:ident(
                $($(#[$a_m:meta])* $a:ident: $t:ty),* $(,)?
            ) $(-> $r:ty)?;
        )+
    } => {
        #[cfg(not(feature = "unstable-c-unwind"))]
        generate_linking_tests! {
            extern "C" {$(
                $(#[$m])*
                $v fn $name($($(#[$a_m])* $a: $t),*) $(-> $r)?;
            )+}
            mod test_linkable_unwind;
        }

        #[cfg(feature = "unstable-c-unwind")]
        generate_linking_tests! {
            extern "C-unwind" {$(
                $(#[$m])*
                $v fn $name($($(#[$a_m])* $a: $t),*) $(-> $r)?;
            )+}
            mod test_linkable_unwind;
        }
    };
}

mod class;
mod constants;
mod exception;
mod image_info;
mod libc;
mod message;
mod method;
mod object;
mod property;
mod protocol;
mod rc;
mod selector;
mod types;
mod various;

pub use self::class::*;
pub use self::constants::*;
pub use self::exception::*;
pub use self::image_info::*;
pub use self::libc::*;
pub use self::message::*;
pub use self::method::*;
pub use self::object::*;
pub use self::property::*;
pub use self::protocol::*;
pub use self::rc::*;
pub use self::selector::*;
pub use self::types::*;
pub use self::various::*;

/// We don't know much about the actual structs, so better mark them `!Send`,
/// `!Sync`, `!UnwindSafe`, `!RefUnwindSafe`, `!Unpin` and as mutable behind
/// shared references.
///
/// Downstream libraries can always manually opt in to these types afterwards.
/// (It's also less of a breaking change on our part if we re-add these).
///
/// TODO: Replace this with `extern type` to also mark it as `!Sized`.
type OpaqueData = UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn smoke() {
        // Verify that this library links and works fine by itself
        let name = CStr::from_bytes_with_nul(b"abc:def:\0").unwrap();
        let sel = unsafe { sel_registerName(name.as_ptr()) };
        let rtn = unsafe { CStr::from_ptr(sel_getName(sel)) };
        assert_eq!(name, rtn);
    }
}
