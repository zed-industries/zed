//! UEFI Reference Specification Protocol Constants and Definitions
//!
//! This project provides protocol constants and definitions as defined in the UEFI Reference
//! Specification. The aim is to provide all these constants as C-ABI compatible imports to rust.
//! Safe rust abstractions over the UEFI API are out of scope of this project. That is, the
//! purpose is really just to extract all the bits and pieces from the specification and provide
//! them as rust types and constants.
//!
//! While we strongly recommend using safe abstractions to interact with UEFI systems, this
//! project serves both as base to write those abstractions, but also as last resort if you have
//! to deal with quirks and peculiarities of UEFI systems directly. Therefore, several examples
//! are included, which show how to interact with UEFI systems from rust. These serve both as
//! documentation for anyone interested in how the system works, but also as base for anyone
//! implementing safe abstractions on top.
//!
//! # Target Configuration
//!
//! Rust code can be compiled natively for UEFI systems. However, you are quite unlikely to have a
//! rust compiler running in an UEFI environment. Therefore, you will most likely want to cross
//! compile your rust code for UEFI systems. To do this, you need a target-configuration for UEFI
//! systems. As of rust-1.61, upstream rust includes the following UEFI targets:
//!
//!  * `aarch64-unknown-uefi`: A native UEFI target for aarch64 systems (64bit ARM).
//!  * `i686-unknown-uefi`: A native UEFI target for i686 systems (32bit Intel x86).
//!  * `x86_64-unknown-uefi`: A native UEFI target for x86-64 systems (64bit Intel x86).
//!
//! If none of these targets match your architecture, you have to create the target specification
//! yourself. Feel free to contact the `r-efi` project for help.
//!
//! # Transpose Guidelines
//!
//! The UEFI specification provides C language symbols and definitions of all
//! its protocols and features. Those are integral parts of the specification
//! and UEFI programming is often tightly coupled with the C language. For
//! better compatibility to existing UEFI documentation, all the rust symbols
//! are transposed from C following strict rules, aiming for close similarity
//! to specification. This section gives a rationale on some of the less
//! obvious choices and tries to describe as many of those rules as possible.
//!
//!  * `no enums`: Rust enums do not allow random discriminant values. However,
//!    many UEFI enumerations use reserved ranges for vendor defined values.
//!    These cannot be represented with rust enums in an efficient manner.
//!    Hence, any enumerations are turned into rust constants with an
//!    accompanying type alias.
//!
//!    A detailed discussion can be found in:
//!
//!    ```gitlog
//!        commit 401a91901e860a5c0cd0f92b75dda0a72cf65322
//!        Author: David Rheinsberg <david.rheinsberg@gmail.com>
//!        Date:   Wed Apr 21 12:07:07 2021 +0200
//!
//!            r-efi: convert enums to constants
//!    ```
//!
//!  * `no incomplete types`: Several structures use incomplete structure types
//!    by using an unbound array as last member. While rust can easily
//!    represent those within its type-system, such structures become DSTs,
//!    hence even raw pointers to them become fat-pointers, and would thus
//!    violate the UEFI ABI.
//!
//!    Instead, we use const-generics to allow compile-time adjustment of the
//!    variable-sized structures, with a default value of 0. This allows
//!    computing different sizes of the structures without any runtime overhead.
//!
//!  * `nullable callbacks as Option`: Rust has no raw function pointers, but
//!    just normal Rust function pointers. Those, however, have no valid null
//!    value. The Rust ABI guarantees that `Option<fn ...>` is an C-ABI
//!    compatible replacement for nullable function pointers, with `None` being
//!    mapped to `NULL`. Hence, whenever UEFI APIs require nullable function
//!    pointers, we use `Option<fn ...>`.
//!
//!  * `prefer *mut over *const`: Whenever we transpose pointers from the
//!    specification into Rust, we prefer `*mut` in almost all cases. `*const`
//!    should only be used if the underlying value is known not to be accessible
//!    via any other mutable pointer type. Since this is rarely the case in
//!    UEFI, we avoid it.
//!
//!    The reasoning is that Rust allows coercing immutable types into `*const`
//!    pointers, without any explicit casting required. However, immutable Rust
//!    references require that no other mutable reference exists simultaneously.
//!    This is not a guarantee of `const`-pointers in C / UEFI, hence this
//!    coercion is usually ill-advised or even wrong.
//!
//!    Lastly, note that `*mut` and `*const` and be `as`-casted in both
//!    directions without violating any Rust guarantees. Any UB concerns always
//!    stem from the safety guarantees of the surrounding code, not of the
//!    raw-pointer handling.
//!
//! # Specification Details
//!
//! This section lists errata of, and general comments on, the UEFI
//! specification relevant to the development of `r-efi`:
//!
//!  * The `Unload` function-pointer of the LoadedImageProtocol can be `NULL`,
//!    despite the protocol documentation lacking any mention of this. Other
//!    parts of the specification refer to images lacking an unload function,
//!    but there is no explicit documentation how this manifests in the
//!    protocol structure. EDK2 assumes `NULL` indicates a lack of unload
//!    function, and an errata has been submitted to the UEFI forum.
//!
//!  * The specification mandates an 8-byte alignment for the `GUID` structure
//!    However, all widespread implementations (including EDK2) use a 4-byte
//!    alignment. An errata has been reported to EDK2 (still pending).
//!
//! # Examples
//!
//! To write free-standing UEFI applications, you need to disable the entry-point provided by rust
//! and instead provide your own. Most target-configurations look for a function called `efi_main`
//! during linking and set it as entry point. If you use the target-configurations provided with
//! upstream rust, they will pick the function called `efi_main` as entry-point.
//!
//! The following example shows a minimal UEFI application, which simply returns success upon
//! invocation. Note that you must provide your own panic-handler when running without `libstd`.
//! In our case, we use a trivial implementation that simply loops forever.
//!
//! ```ignore
//! #![no_main]
//! #![no_std]
//!
//! use r_efi::efi;
//!
//! #[panic_handler]
//! fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
//!     loop {}
//! }
//!
//! #[export_name = "efi_main"]
//! pub extern fn main(_h: efi::Handle, _st: *mut efi::SystemTable) -> efi::Status {
//!     efi::Status::SUCCESS
//! }
//! ```

// Mark this crate as `no_std`. We have no std::* dependencies (and we better don't have them),
// so no reason to require it. This does not mean that you cannot use std::* with UEFI. You have
// to port it to UEFI first, though.
//
// In case of unit-test compilation, we pull in `std` and drop the `no_std` marker. This allows
// basic unit-tests on the compilation host. For integration tests, we have separate compilation
// units, so they will be unaffected by this.
#![cfg_attr(not(test), no_std)]

// Import the different core modules. We separate them into different modules to make it easier to
// work on them and describe what each part implements. This is different to the reference
// implementation, which uses a flat namespace due to its origins in the C language. For
// compatibility, we provide this flat namespace as well. See the `efi` submodule.
#[macro_use]
pub mod base;
#[macro_use]
pub mod hii;
#[macro_use]
pub mod system;

// Import the protocols. Each protocol is separated into its own module, readily imported by the
// meta `protocols` module. Note that this puts all symbols into their respective protocol
// namespace, thus clearly separating them (unlike the UEFI Specification, which more often than
// not violates its own namespacing).
pub mod protocols;

// Import vendor protocols. They are just like protocols in `protocols`, but
// separated for better namespacing.
pub mod vendor;

/// Flat EFI Namespace
///
/// The EFI namespace re-exports all symbols in a single, flat namespace. This allows mirroring
/// the entire EFI namespace as given in the specification and makes it easier to refer to them
/// with the same names as the reference C implementation.
///
/// Note that the individual protocols are put into submodules. The specification does this in
/// most parts as well (by prefixing all symbols). This is not true in all cases, as the
/// specification suffers from lack of namespaces in the reference C language. However, we decided
/// to namespace the remaining bits as well, for better consistency throughout the API. This
/// should be self-explanatory in nearly all cases.
pub mod efi {
    pub use crate::base::*;
    pub use crate::system::*;

    pub use crate::hii;
    pub use crate::protocols;
    pub use crate::vendor;
}
