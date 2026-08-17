// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
<!-- Note: Document from sync-markdown-to-rustdoc:start through sync-markdown-to-rustdoc:end
     is synchronized from README.md. Any changes to that range are not preserved. -->
<!-- tidy:sync-markdown-to-rustdoc:start -->

Portable atomic types including support for 128-bit atomics, atomic float, etc.

- Provide all atomic integer types (`Atomic{I,U}{8,16,32,64}`) for all targets that can use atomic CAS. (i.e., all targets that can use `std`, and most no-std targets)
- Provide `AtomicI128` and `AtomicU128`.
- Provide `AtomicF32` and `AtomicF64`. ([optional, requires the `float` feature](#optional-features-float))
- Provide `AtomicF16` and `AtomicF128` for [unstable `f16` and `f128`](https://github.com/rust-lang/rust/issues/116909). ([optional, requires the `float` feature and unstable cfgs](#optional-features-float))
- Provide atomic load/store for targets where atomic is not available at all in the standard library. (RISC-V without A-extension, MSP430, AVR)
- Provide atomic CAS for targets where atomic CAS is not available in the standard library. (thumbv6m, pre-v6 Arm, RISC-V without A-extension, MSP430, AVR, Xtensa, etc.) (always enabled for MSP430 and AVR, [optional](#optional-features-critical-section) otherwise)
- Provide stable equivalents of the standard library's atomic types' unstable APIs, such as [`AtomicPtr::fetch_*`](https://github.com/rust-lang/rust/issues/99108).
- Make features that require newer compilers, such as [`fetch_{max,min}`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_max), [`fetch_update`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.fetch_update), [`as_ptr`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.as_ptr), [`from_ptr`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html#method.from_ptr), [`AtomicBool::fetch_not`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html#method.fetch_not) and [stronger CAS failure ordering](https://github.com/rust-lang/rust/pull/98383) available on Rust 1.34+.
- Provide workaround for bugs in the standard library's atomic-related APIs, such as [rust-lang/rust#100650], `fence`/`compiler_fence` on MSP430 that cause LLVM error, etc.

<!-- TODO:
- mention Atomic{I,U}*::fetch_neg, Atomic{I*,U*,Ptr}::bit_*, etc.
- mention optimizations not available in the standard library's equivalents
-->

portable-atomic version of `std::sync::Arc` is provided by the [portable-atomic-util](https://github.com/taiki-e/portable-atomic/tree/HEAD/portable-atomic-util) crate.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
portable-atomic = "1"
```

The default features are mainly for users who use atomics larger than the pointer width.
If you don't need them, disabling the default features may reduce code size and compile time slightly.

```toml
[dependencies]
portable-atomic = { version = "1", default-features = false }
```

If your crate supports no-std environment and requires atomic CAS, enabling the `require-cas` feature will allow the `portable-atomic` to display a [helpful error message](https://github.com/taiki-e/portable-atomic/pull/100) to users on targets requiring additional action on the user side to provide atomic CAS.

```toml
[dependencies]
portable-atomic = { version = "1.3", default-features = false, features = ["require-cas"] }
```

## 128-bit atomics support

Native 128-bit atomic operations are available on x86_64 (Rust 1.59+), AArch64 (Rust 1.59+), riscv64 (Rust 1.59+), Arm64EC (Rust 1.84+), s390x (Rust 1.84+), and powerpc64 (nightly only), otherwise the fallback implementation is used.

On x86_64, even if `cmpxchg16b` is not available at compile-time (Note: `cmpxchg16b` target feature is enabled by default only on Apple, Windows (except Windows 7), and Fuchsia targets), run-time detection checks whether `cmpxchg16b` is available. If `cmpxchg16b` is not available at either compile-time or run-time detection, the fallback implementation is used. See also [`portable_atomic_no_outline_atomics`](#optional-cfg-no-outline-atomics) cfg.

They are usually implemented using inline assembly, and when using Miri or ThreadSanitizer that do not support inline assembly, core intrinsics are used instead of inline assembly if possible.

See the [`atomic128` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/atomic128/README.md) for details.

## Optional features

- **`fallback`** *(enabled by default)*<br>
  Enable fallback implementations.

  Disabling this allows only atomic types for which the platform natively supports atomic operations.

- <a name="optional-features-float"></a>**`float`**<br>
  Provide `AtomicF{32,64}`.

  - When unstable `--cfg portable_atomic_unstable_f16` is also enabled, `AtomicF16` for [unstable `f16`](https://github.com/rust-lang/rust/issues/116909) is also provided.
  - When unstable `--cfg portable_atomic_unstable_f128` is also enabled, `AtomicF128` for [unstable `f128`](https://github.com/rust-lang/rust/issues/116909) is also provided.

  Note:
  - Atomic float's `fetch_{add,sub,min,max}` are usually implemented using CAS loops, which can be slower than equivalent operations of atomic integers. As an exception, AArch64 with FEAT_LSFE and GPU targets have atomic float instructions and we use them on AArch64 when `lsfe` target feature is available at compile-time. We [plan to use atomic float instructions for GPU targets as well in the future.](https://github.com/taiki-e/portable-atomic/issues/34))
  - Unstable cfgs are outside of the normal semver guarantees and minor or patch versions of portable-atomic may make breaking changes to them at any time.

- **`std`**<br>
  Use `std`.

- <a name="optional-features-require-cas"></a>**`require-cas`**<br>
  Emit compile error if atomic CAS is not available. See [Usage](#usage) section and [#100](https://github.com/taiki-e/portable-atomic/pull/100) for more.

- <a name="optional-features-serde"></a>**`serde`**<br>
  Implement `serde::{Serialize,Deserialize}` for atomic types.

  Note:
  - The MSRV when this feature is enabled depends on the MSRV of [serde].

- <a name="optional-features-critical-section"></a>**`critical-section`**<br>
  When this feature is enabled, this crate uses [critical-section] to provide atomic CAS for targets where
  it is not natively available. When enabling it, you should provide a suitable critical section implementation
  for the current target, see the [critical-section] documentation for details on how to do so.

  `critical-section` support is useful to get atomic CAS when the [`unsafe-assume-single-core` feature](#optional-features-unsafe-assume-single-core) can't be used,
  such as multi-core targets, unprivileged code running under some RTOS, or environments where disabling interrupts
  needs extra care due to e.g. real-time requirements.

  Note that with the `critical-section` feature, critical sections are taken for all atomic operations, while with
  [`unsafe-assume-single-core` feature](#optional-features-unsafe-assume-single-core) some operations don't require disabling interrupts (loads and stores, but
  additionally on MSP430 `add`, `sub`, `and`, `or`, `xor`, `not`). Therefore, for better performance, if
  all the `critical-section` implementation for your target does is disable interrupts, prefer using
  `unsafe-assume-single-core` feature instead.

  Note:
  - The MSRV when this feature is enabled depends on the MSRV of [critical-section].
  - It is usually *not* recommended to always enable this feature in dependencies of the library.

    Enabling this feature will prevent the end user from having the chance to take advantage of other (potentially) efficient implementations ([Implementations provided by `unsafe-assume-single-core` feature, default implementations on MSP430 and AVR](#optional-features-unsafe-assume-single-core), implementation proposed in [#60], etc. Other systems may also be supported in the future).

    The recommended approach for libraries is to leave it up to the end user whether or not to enable this feature. (However, it may make sense to enable this feature by default for libraries specific to a platform where other implementations are known not to work.)

    As an example, the end-user's `Cargo.toml` that uses a crate that provides a critical-section implementation and a crate that depends on portable-atomic as an option would be expected to look like this:

    ```toml
    [dependencies]
    portable-atomic = { version = "1", default-features = false, features = ["critical-section"] }
    crate-provides-critical-section-impl = "..."
    crate-uses-portable-atomic-as-feature = { version = "...", features = ["portable-atomic"] }
    ```

- <a name="optional-features-unsafe-assume-single-core"></a>**`unsafe-assume-single-core`**<br>
  Assume that the target is single-core.
  When this feature is enabled, this crate provides atomic CAS for targets where atomic CAS is not available in the standard library by disabling interrupts.

  This feature is `unsafe`, and note the following safety requirements:
  - Enabling this feature for multi-core systems is always **unsound**.
  - This uses privileged instructions to disable interrupts, so it usually doesn't work on unprivileged mode.
    Enabling this feature in an environment where privileged instructions are not available, or if the instructions used are not sufficient to disable interrupts in the system, it is also usually considered **unsound**, although the details are system-dependent.

    The following are known cases:
    - On pre-v6 Arm, this disables only IRQs by default. For many systems (e.g., GBA) this is enough. If the system need to disable both IRQs and FIQs, you need to enable the `disable-fiq` feature together.
    - On RISC-V without A-extension, this generates code for machine-mode (M-mode) by default. If you enable the `s-mode` together, this generates code for supervisor-mode (S-mode). In particular, `qemu-system-riscv*` uses [OpenSBI](https://github.com/riscv-software-src/opensbi) as the default firmware.

    See also the [`interrupt` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/interrupt/README.md).

  Consider using the [`critical-section` feature](#optional-features-critical-section) for systems that cannot use this feature.

  It is **very strongly discouraged** to enable this feature in libraries that depend on `portable-atomic`. The recommended approach for libraries is to leave it up to the end user whether or not to enable this feature. (However, it may make sense to enable this feature by default for libraries specific to a platform where it is guaranteed to always be sound, for example in a hardware abstraction layer targeting a single-core chip.)

  Armv6-M (thumbv6m), pre-v6 Arm (e.g., thumbv4t, thumbv5te), RISC-V without A-extension, and Xtensa are currently supported.

  Since all MSP430 and AVR are single-core, we always provide atomic CAS for them without this feature.

  Enabling this feature for targets that have atomic CAS will result in a compile error.

  Feel free to submit an issue if your target is not supported yet.

## Optional cfg

One of the ways to enable cfg is to set [rustflags in the cargo config](https://doc.rust-lang.org/cargo/reference/config.html#targettriplerustflags):

```toml
# .cargo/config.toml
[target.<target>]
rustflags = ["--cfg", "portable_atomic_no_outline_atomics"]
```

Or set environment variable:

```sh
RUSTFLAGS="--cfg portable_atomic_no_outline_atomics" cargo ...
```

- <a name="optional-cfg-unsafe-assume-single-core"></a>**`--cfg portable_atomic_unsafe_assume_single_core`**<br>
  Since 1.4.0, this cfg is an alias of [`unsafe-assume-single-core` feature](#optional-features-unsafe-assume-single-core).

  Originally, we were providing these as cfgs instead of features, but based on a strong request from the embedded ecosystem, we have agreed to provide them as features as well. See [#94](https://github.com/taiki-e/portable-atomic/pull/94) for more.

- <a name="optional-cfg-no-outline-atomics"></a>**`--cfg portable_atomic_no_outline_atomics`**<br>
  Disable dynamic dispatching by run-time CPU feature detection.

  If dynamic dispatching by run-time CPU feature detection is enabled, it allows maintaining support for older CPUs while using features that are not supported on older CPUs, such as CMPXCHG16B (x86_64) and FEAT_LSE/FEAT_LSE2 (AArch64).

  Note:
  - Dynamic detection is currently only supported in x86_64, AArch64, Arm, RISC-V, Arm64EC, and powerpc64, otherwise it works the same as when this cfg is set.
  - If the required target features are enabled at compile-time, the atomic operations are inlined.
  - This is compatible with no-std (as with all features except `std`).
  - On some targets, run-time detection is disabled by default mainly for compatibility with incomplete build environments or support for it is experimental, and can be enabled by `--cfg portable_atomic_outline_atomics`. (When both cfg are enabled, `*_no_*` cfg is preferred.)
  - Some AArch64 targets enable LLVM's `outline-atomics` target feature by default, so if you set this cfg, you may want to disable that as well. (portable-atomic's outline-atomics does not depend on the compiler-rt symbols, so even if you need to disable LLVM's outline-atomics, you may not need to disable portable-atomic's outline-atomics.)

  See also the [`atomic128` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/atomic128/README.md).

## Related Projects

- [atomic-maybe-uninit]: Atomic operations on potentially uninitialized integers.
- [atomic-memcpy]: Byte-wise atomic memcpy.

[#60]: https://github.com/taiki-e/portable-atomic/issues/60
[atomic-maybe-uninit]: https://github.com/taiki-e/atomic-maybe-uninit
[atomic-memcpy]: https://github.com/taiki-e/atomic-memcpy
[critical-section]: https://github.com/rust-embedded/critical-section
[rust-lang/rust#100650]: https://github.com/rust-lang/rust/issues/100650
[serde]: https://github.com/serde-rs/serde

<!-- tidy:sync-markdown-to-rustdoc:end -->
*/

#![no_std]
#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![cfg_attr(not(portable_atomic_no_unsafe_op_in_unsafe_fn), warn(unsafe_op_in_unsafe_fn))] // unsafe_op_in_unsafe_fn requires Rust 1.52
#![cfg_attr(portable_atomic_no_unsafe_op_in_unsafe_fn, allow(unused_unsafe))]
#![warn(
    // Lints that may help when writing public library.
    missing_debug_implementations,
    // missing_docs,
    clippy::alloc_instead_of_core,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::impl_trait_in_params,
    clippy::missing_inline_in_public_items,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    // Code outside of cfg(feature = "float") shouldn't use float.
    clippy::float_arithmetic,
)]
#![cfg_attr(not(portable_atomic_no_asm), warn(missing_docs))] // module-level #![allow(missing_docs)] doesn't work for macros on old rustc
#![cfg_attr(portable_atomic_no_strict_provenance, allow(unstable_name_collisions))]
#![allow(clippy::inline_always, clippy::used_underscore_items)]
// asm_experimental_arch
// AVR, MSP430, and Xtensa are tier 3 platforms and require nightly anyway.
// On tier 2 platforms (powerpc64), we use cfg set by build script to
// determine whether this feature is available or not.
#![cfg_attr(
    all(
        not(portable_atomic_no_asm),
        any(
            target_arch = "avr",
            target_arch = "msp430",
            all(target_arch = "xtensa", portable_atomic_unsafe_assume_single_core),
            all(target_arch = "powerpc64", portable_atomic_unstable_asm_experimental_arch),
        ),
    ),
    feature(asm_experimental_arch)
)]
// f16/f128
// cfg is unstable and explicitly enabled by the user
#![cfg_attr(portable_atomic_unstable_f16, feature(f16))]
#![cfg_attr(portable_atomic_unstable_f128, feature(f128))]
// Old nightly only
// These features are already stabilized or have already been removed from compilers,
// and can safely be enabled for old nightly as long as version detection works.
// - cfg(target_has_atomic)
// - asm! on AArch64, Arm, RISC-V, x86, x86_64, Arm64EC, s390x
// - llvm_asm! on AVR (tier 3) and MSP430 (tier 3)
// - #[instruction_set] on non-Linux/Android pre-v6 Arm (tier 3)
// This also helps us test that our assembly code works with the minimum external
// LLVM version of the first rustc version that inline assembly stabilized.
#![cfg_attr(portable_atomic_unstable_cfg_target_has_atomic, feature(cfg_target_has_atomic))]
#![cfg_attr(
    all(
        portable_atomic_unstable_asm,
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "x86",
            target_arch = "x86_64",
        ),
    ),
    feature(asm)
)]
#![cfg_attr(
    all(
        portable_atomic_unstable_asm_experimental_arch,
        any(target_arch = "arm64ec", target_arch = "s390x"),
    ),
    feature(asm_experimental_arch)
)]
#![cfg_attr(
    all(any(target_arch = "avr", target_arch = "msp430"), portable_atomic_no_asm),
    feature(llvm_asm)
)]
#![cfg_attr(
    all(
        target_arch = "arm",
        portable_atomic_unstable_isa_attribute,
        any(test, portable_atomic_unsafe_assume_single_core),
        not(any(target_feature = "v6", portable_atomic_target_feature = "v6")),
        not(target_has_atomic = "ptr"),
    ),
    feature(isa_attribute)
)]
// Miri and/or ThreadSanitizer only
// They do not support inline assembly, so we need to use unstable features instead.
// Since they require nightly compilers anyway, we can use the unstable features.
// This is not an ideal situation, but it is still better than always using lock-based
// fallback and causing memory ordering problems to be missed by these checkers.
#![cfg_attr(
    all(
        any(
            target_arch = "aarch64",
            target_arch = "arm64ec",
            target_arch = "powerpc64",
            target_arch = "s390x",
        ),
        any(miri, portable_atomic_sanitize_thread),
    ),
    allow(internal_features)
)]
#![cfg_attr(
    all(
        any(
            target_arch = "aarch64",
            target_arch = "arm64ec",
            target_arch = "powerpc64",
            target_arch = "s390x",
        ),
        any(miri, portable_atomic_sanitize_thread),
    ),
    feature(core_intrinsics)
)]
// docs.rs only (cfg is enabled by docs.rs, not build script)
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    all(
        portable_atomic_no_atomic_load_store,
        not(any(
            target_arch = "avr",
            target_arch = "bpf",
            target_arch = "msp430",
            target_arch = "riscv32",
            target_arch = "riscv64",
            feature = "critical-section",
        )),
    ),
    allow(unused_imports, unused_macros, clippy::unused_trait_names)
)]

// There are currently no 128-bit or higher builtin targets.
// (Although some of our generic code is written with the future
// addition of 128-bit targets in mind.)
// Note that Rust (and C99) pointers must be at least 16-bit (i.e., 8-bit targets are impossible): https://github.com/rust-lang/rust/pull/49305
#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
)))]
compile_error!(
    "portable-atomic currently only supports targets with {16,32,64}-bit pointer width; \
     if you need support for others, \
     please submit an issue at <https://github.com/taiki-e/portable-atomic>"
);

#[cfg(portable_atomic_unsafe_assume_single_core)]
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(not(portable_atomic_no_atomic_cas)))]
#[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(target_has_atomic = "ptr"))]
compile_error!(
    "`portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature) \
     is not compatible with target that supports atomic CAS;\n\
     see also <https://github.com/taiki-e/portable-atomic/issues/148> for troubleshooting"
);
#[cfg(portable_atomic_unsafe_assume_single_core)]
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(portable_atomic_no_atomic_cas))]
#[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(not(target_has_atomic = "ptr")))]
#[cfg(not(any(
    target_arch = "arm",
    target_arch = "avr",
    target_arch = "msp430",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "xtensa",
)))]
compile_error!(
    "`portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature) \
     is not supported yet on this target;\n\
     if you need unsafe-assume-single-core support for this target,\n\
     please submit an issue at <https://github.com/taiki-e/portable-atomic>"
);

#[cfg(portable_atomic_no_outline_atomics)]
#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "arm64ec",
    target_arch = "powerpc64",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "x86_64",
)))]
compile_error!("`portable_atomic_no_outline_atomics` cfg does not compatible with this target");
#[cfg(portable_atomic_outline_atomics)]
#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "powerpc64",
    target_arch = "riscv32",
    target_arch = "riscv64",
)))]
compile_error!("`portable_atomic_outline_atomics` cfg does not compatible with this target");

#[cfg(portable_atomic_disable_fiq)]
#[cfg(not(all(
    target_arch = "arm",
    not(any(target_feature = "mclass", portable_atomic_target_feature = "mclass")),
)))]
compile_error!(
    "`portable_atomic_disable_fiq` cfg (`disable-fiq` feature) is only available on pre-v6 Arm"
);
#[cfg(portable_atomic_s_mode)]
#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
compile_error!("`portable_atomic_s_mode` cfg (`s-mode` feature) is only available on RISC-V");
#[cfg(portable_atomic_force_amo)]
#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
compile_error!("`portable_atomic_force_amo` cfg (`force-amo` feature) is only available on RISC-V");

#[cfg(portable_atomic_disable_fiq)]
#[cfg(not(portable_atomic_unsafe_assume_single_core))]
compile_error!(
    "`portable_atomic_disable_fiq` cfg (`disable-fiq` feature) may only be used together with `portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature)"
);
#[cfg(portable_atomic_s_mode)]
#[cfg(not(portable_atomic_unsafe_assume_single_core))]
compile_error!(
    "`portable_atomic_s_mode` cfg (`s-mode` feature) may only be used together with `portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature)"
);
#[cfg(portable_atomic_force_amo)]
#[cfg(not(portable_atomic_unsafe_assume_single_core))]
compile_error!(
    "`portable_atomic_force_amo` cfg (`force-amo` feature) may only be used together with `portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature)"
);

#[cfg(all(portable_atomic_unsafe_assume_single_core, feature = "critical-section"))]
compile_error!(
    "you may not enable `critical-section` feature and `portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature) at the same time"
);

#[cfg(feature = "require-cas")]
#[cfg_attr(
    portable_atomic_no_cfg_target_has_atomic,
    cfg(not(any(
        not(portable_atomic_no_atomic_cas),
        portable_atomic_unsafe_assume_single_core,
        feature = "critical-section",
        target_arch = "avr",
        target_arch = "msp430",
    )))
)]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(not(any(
        target_has_atomic = "ptr",
        portable_atomic_unsafe_assume_single_core,
        feature = "critical-section",
        target_arch = "avr",
        target_arch = "msp430",
    )))
)]
compile_error!(
    "dependents require atomic CAS but not available on this target by default;\n\
    consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features.\n\
    see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
);

#[cfg(any(test, feature = "std"))]
extern crate std;

#[macro_use]
mod cfgs;
#[cfg(target_pointer_width = "16")]
pub use self::{cfg_has_atomic_16 as cfg_has_atomic_ptr, cfg_no_atomic_16 as cfg_no_atomic_ptr};
#[cfg(target_pointer_width = "32")]
pub use self::{cfg_has_atomic_32 as cfg_has_atomic_ptr, cfg_no_atomic_32 as cfg_no_atomic_ptr};
#[cfg(target_pointer_width = "64")]
pub use self::{cfg_has_atomic_64 as cfg_has_atomic_ptr, cfg_no_atomic_64 as cfg_no_atomic_ptr};
#[cfg(target_pointer_width = "128")]
pub use self::{cfg_has_atomic_128 as cfg_has_atomic_ptr, cfg_no_atomic_128 as cfg_no_atomic_ptr};

#[macro_use]
mod utils;

#[cfg(test)]
#[macro_use]
mod tests;

#[doc(no_inline)]
pub use core::sync::atomic::Ordering;

// LLVM doesn't support fence/compiler_fence for MSP430.
#[cfg(target_arch = "msp430")]
pub use self::imp::msp430::{compiler_fence, fence};
#[doc(no_inline)]
#[cfg(not(target_arch = "msp430"))]
pub use core::sync::atomic::{compiler_fence, fence};

mod imp;

pub mod hint {
    //! Re-export of the [`core::hint`] module.
    //!
    //! The only difference from the [`core::hint`] module is that [`spin_loop`]
    //! is available in all rust versions that this crate supports.
    //!
    //! ```
    //! use portable_atomic::hint;
    //!
    //! hint::spin_loop();
    //! ```

    #[doc(no_inline)]
    pub use core::hint::*;

    /// Emits a machine instruction to signal the processor that it is running in
    /// a busy-wait spin-loop ("spin lock").
    ///
    /// Upon receiving the spin-loop signal the processor can optimize its behavior by,
    /// for example, saving power or switching hyper-threads.
    ///
    /// This function is different from [`thread::yield_now`] which directly
    /// yields to the system's scheduler, whereas `spin_loop` does not interact
    /// with the operating system.
    ///
    /// A common use case for `spin_loop` is implementing bounded optimistic
    /// spinning in a CAS loop in synchronization primitives. To avoid problems
    /// like priority inversion, it is strongly recommended that the spin loop is
    /// terminated after a finite amount of iterations and an appropriate blocking
    /// syscall is made.
    ///
    /// **Note:** On platforms that do not support receiving spin-loop hints this
    /// function does not do anything at all.
    ///
    /// [`thread::yield_now`]: https://doc.rust-lang.org/std/thread/fn.yield_now.html
    #[inline]
    pub fn spin_loop() {
        #[allow(deprecated)]
        core::sync::atomic::spin_loop_hint();
    }
}

#[cfg(doc)]
use core::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release, SeqCst};
use core::{fmt, ptr};

#[cfg(portable_atomic_no_strict_provenance)]
#[cfg(miri)]
use crate::utils::ptr::PtrExt as _;

cfg_has_atomic_8! {
/// A boolean type which can be safely shared between threads.
///
/// This type has the same in-memory representation as a [`bool`].
///
/// If the compiler and the platform support atomic loads and stores of `u8`,
/// this type is a wrapper for the standard library's
/// [`AtomicBool`](core::sync::atomic::AtomicBool). If the platform supports it
/// but the compiler does not, atomic operations are implemented using inline
/// assembly.
#[repr(C, align(1))]
pub struct AtomicBool {
    v: core::cell::UnsafeCell<u8>,
}

impl Default for AtomicBool {
    /// Creates an `AtomicBool` initialized to `false`.
    #[inline]
    fn default() -> Self {
        Self::new(false)
    }
}

impl From<bool> for AtomicBool {
    /// Converts a `bool` into an `AtomicBool`.
    #[inline]
    fn from(b: bool) -> Self {
        Self::new(b)
    }
}

// Send is implicitly implemented.
// SAFETY: any data races are prevented by disabling interrupts or
// atomic intrinsics (see module-level comments).
unsafe impl Sync for AtomicBool {}

// UnwindSafe is implicitly implemented.
#[cfg(not(portable_atomic_no_core_unwind_safe))]
impl core::panic::RefUnwindSafe for AtomicBool {}
#[cfg(all(portable_atomic_no_core_unwind_safe, feature = "std"))]
impl std::panic::RefUnwindSafe for AtomicBool {}

impl_debug_and_serde!(AtomicBool);

impl AtomicBool {
    /// Creates a new `AtomicBool`.
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::AtomicBool;
    ///
    /// let atomic_true = AtomicBool::new(true);
    /// let atomic_false = AtomicBool::new(false);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(v: bool) -> Self {
        static_assert_layout!(AtomicBool, bool);
        Self { v: core::cell::UnsafeCell::new(v as u8) }
    }

    // TODO: update docs based on https://github.com/rust-lang/rust/pull/116762
    const_fn! {
        const_if: #[cfg(not(portable_atomic_no_const_mut_refs))];
        /// Creates a new `AtomicBool` from a pointer.
        ///
        /// This is `const fn` on Rust 1.83+.
        ///
        /// # Safety
        ///
        /// * `ptr` must be aligned to `align_of::<AtomicBool>()` (note that on some platforms this can
        ///   be bigger than `align_of::<bool>()`).
        /// * `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`.
        /// * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value
        ///   behind `ptr` must have a happens-before relationship with atomic accesses via the returned
        ///   value (or vice-versa).
        ///   * In other words, time periods where the value is accessed atomically may not overlap
        ///     with periods where the value is accessed non-atomically.
        ///   * This requirement is trivially satisfied if `ptr` is never used non-atomically for the
        ///     duration of lifetime `'a`. Most use cases should be able to follow this guideline.
        ///   * This requirement is also trivially satisfied if all accesses (atomic or not) are done
        ///     from the same thread.
        /// * If this atomic type is *not* lock-free:
        ///   * Any accesses to the value behind `ptr` must have a happens-before relationship
        ///     with accesses via the returned value (or vice-versa).
        ///   * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must
        ///     be compatible with operations performed by this atomic type.
        /// * This method must not be used to create overlapping or mixed-size atomic accesses, as
        ///   these are not supported by the memory model.
        ///
        /// [valid]: core::ptr#safety
        #[inline]
        #[must_use]
        pub const unsafe fn from_ptr<'a>(ptr: *mut bool) -> &'a Self {
            #[allow(clippy::cast_ptr_alignment)]
            // SAFETY: guaranteed by the caller
            unsafe { &*(ptr as *mut Self) }
        }
    }

    /// Returns `true` if operations on values of this type are lock-free.
    ///
    /// If the compiler or the platform doesn't support the necessary
    /// atomic instructions, global locks for every potentially
    /// concurrent atomic operation will be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::AtomicBool;
    ///
    /// let is_lock_free = AtomicBool::is_lock_free();
    /// ```
    #[inline]
    #[must_use]
    pub fn is_lock_free() -> bool {
        imp::AtomicU8::is_lock_free()
    }

    /// Returns `true` if operations on values of this type are lock-free.
    ///
    /// If the compiler or the platform doesn't support the necessary
    /// atomic instructions, global locks for every potentially
    /// concurrent atomic operation will be used.
    ///
    /// **Note:** If the atomic operation relies on dynamic CPU feature detection,
    /// this type may be lock-free even if the function returns false.
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::AtomicBool;
    ///
    /// const IS_ALWAYS_LOCK_FREE: bool = AtomicBool::is_always_lock_free();
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_always_lock_free() -> bool {
        imp::AtomicU8::IS_ALWAYS_LOCK_FREE
    }
    #[cfg(test)]
    const IS_ALWAYS_LOCK_FREE: bool = Self::is_always_lock_free();

    const_fn! {
        const_if: #[cfg(not(portable_atomic_no_const_mut_refs))];
        /// Returns a mutable reference to the underlying [`bool`].
        ///
        /// This is safe because the mutable reference guarantees that no other threads are
        /// concurrently accessing the atomic data.
        ///
        /// This is `const fn` on Rust 1.83+.
        ///
        /// # Examples
        ///
        /// ```
        /// use portable_atomic::{AtomicBool, Ordering};
        ///
        /// let mut some_bool = AtomicBool::new(true);
        /// assert_eq!(*some_bool.get_mut(), true);
        /// *some_bool.get_mut() = false;
        /// assert_eq!(some_bool.load(Ordering::SeqCst), false);
        /// ```
        #[inline]
        pub const fn get_mut(&mut self) -> &mut bool {
            // SAFETY: the mutable reference guarantees unique ownership.
            unsafe { &mut *self.as_ptr() }
        }
    }

    // TODO: Add from_mut/get_mut_slice/from_mut_slice once it is stable on std atomic types.
    // https://github.com/rust-lang/rust/issues/76314

    const_fn! {
        const_if: #[cfg(not(portable_atomic_no_const_transmute))];
        /// Consumes the atomic and returns the contained value.
        ///
        /// This is safe because passing `self` by value guarantees that no other threads are
        /// concurrently accessing the atomic data.
        ///
        /// This is `const fn` on Rust 1.56+.
        ///
        /// # Examples
        ///
        /// ```
        /// use portable_atomic::AtomicBool;
        ///
        /// let some_bool = AtomicBool::new(true);
        /// assert_eq!(some_bool.into_inner(), true);
        /// ```
        #[inline]
        pub const fn into_inner(self) -> bool {
            // SAFETY: AtomicBool and u8 have the same size and in-memory representations,
            // so they can be safely transmuted.
            // (const UnsafeCell::into_inner is unstable)
            unsafe { core::mem::transmute::<AtomicBool, u8>(self) != 0 }
        }
    }

    /// Loads a value from the bool.
    ///
    /// `load` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. Possible values are [`SeqCst`], [`Acquire`] and [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `order` is [`Release`] or [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let some_bool = AtomicBool::new(true);
    ///
    /// assert_eq!(some_bool.load(Ordering::Relaxed), true);
    /// ```
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub fn load(&self, order: Ordering) -> bool {
        self.as_atomic_u8().load(order) != 0
    }

    /// Stores a value into the bool.
    ///
    /// `store` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. Possible values are [`SeqCst`], [`Release`] and [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `order` is [`Acquire`] or [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let some_bool = AtomicBool::new(true);
    ///
    /// some_bool.store(false, Ordering::Relaxed);
    /// assert_eq!(some_bool.load(Ordering::Relaxed), false);
    /// ```
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub fn store(&self, val: bool, order: Ordering) {
        self.as_atomic_u8().store(val as u8, order);
    }

    cfg_has_atomic_cas_or_amo32! {
    /// Stores a value into the bool, returning the previous value.
    ///
    /// `swap` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let some_bool = AtomicBool::new(true);
    ///
    /// assert_eq!(some_bool.swap(false, Ordering::Relaxed), true);
    /// assert_eq!(some_bool.load(Ordering::Relaxed), false);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn swap(&self, val: bool, order: Ordering) -> bool {
        #[cfg(any(
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "loongarch32",
            target_arch = "loongarch64",
        ))]
        {
            // See https://github.com/rust-lang/rust/pull/114034 for details.
            // https://github.com/rust-lang/rust/blob/1.84.0/library/core/src/sync/atomic.rs#L249
            // https://godbolt.org/z/ofbGGdx44
            if val { self.fetch_or(true, order) } else { self.fetch_and(false, order) }
        }
        #[cfg(not(any(
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "loongarch32",
            target_arch = "loongarch64",
        )))]
        {
            self.as_atomic_u8().swap(val as u8, order) != 0
        }
    }

    /// Stores a value into the [`bool`] if the current value is the same as the `current` value.
    ///
    /// The return value is a result indicating whether the new value was written and containing
    /// the previous value. On success this value is guaranteed to be equal to `current`.
    ///
    /// `compare_exchange` takes two [`Ordering`] arguments to describe the memory
    /// ordering of this operation. `success` describes the required ordering for the
    /// read-modify-write operation that takes place if the comparison with `current` succeeds.
    /// `failure` describes the required ordering for the load operation that takes place when
    /// the comparison fails. Using [`Acquire`] as success ordering makes the store part
    /// of this operation [`Relaxed`], and using [`Release`] makes the successful load
    /// [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `failure` is [`Release`], [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let some_bool = AtomicBool::new(true);
    ///
    /// assert_eq!(
    ///     some_bool.compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed),
    ///     Ok(true)
    /// );
    /// assert_eq!(some_bool.load(Ordering::Relaxed), false);
    ///
    /// assert_eq!(
    ///     some_bool.compare_exchange(true, true, Ordering::SeqCst, Ordering::Acquire),
    ///     Err(false)
    /// );
    /// assert_eq!(some_bool.load(Ordering::Relaxed), false);
    /// ```
    #[cfg_attr(docsrs, doc(alias = "compare_and_swap"))]
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        #[cfg(any(
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "loongarch32",
            target_arch = "loongarch64",
        ))]
        {
            // See https://github.com/rust-lang/rust/pull/114034 for details.
            // https://github.com/rust-lang/rust/blob/1.84.0/library/core/src/sync/atomic.rs#L249
            // https://godbolt.org/z/ofbGGdx44
            crate::utils::assert_compare_exchange_ordering(success, failure);
            let order = crate::utils::upgrade_success_ordering(success, failure);
            let old = if current == new {
                // This is a no-op, but we still need to perform the operation
                // for memory ordering reasons.
                self.fetch_or(false, order)
            } else {
                // This sets the value to the new one and returns the old one.
                self.swap(new, order)
            };
            if old == current { Ok(old) } else { Err(old) }
        }
        #[cfg(not(any(
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "loongarch32",
            target_arch = "loongarch64",
        )))]
        {
            match self.as_atomic_u8().compare_exchange(current as u8, new as u8, success, failure) {
                Ok(x) => Ok(x != 0),
                Err(x) => Err(x != 0),
            }
        }
    }

    /// Stores a value into the [`bool`] if the current value is the same as the `current` value.
    ///
    /// Unlike [`AtomicBool::compare_exchange`], this function is allowed to spuriously fail even when the
    /// comparison succeeds, which can result in more efficient code on some platforms. The
    /// return value is a result indicating whether the new value was written and containing the
    /// previous value.
    ///
    /// `compare_exchange_weak` takes two [`Ordering`] arguments to describe the memory
    /// ordering of this operation. `success` describes the required ordering for the
    /// read-modify-write operation that takes place if the comparison with `current` succeeds.
    /// `failure` describes the required ordering for the load operation that takes place when
    /// the comparison fails. Using [`Acquire`] as success ordering makes the store part
    /// of this operation [`Relaxed`], and using [`Release`] makes the successful load
    /// [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `failure` is [`Release`], [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let val = AtomicBool::new(false);
    ///
    /// let new = true;
    /// let mut old = val.load(Ordering::Relaxed);
    /// loop {
    ///     match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
    ///         Ok(_) => break,
    ///         Err(x) => old = x,
    ///     }
    /// }
    /// ```
    #[cfg_attr(docsrs, doc(alias = "compare_and_swap"))]
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub fn compare_exchange_weak(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        #[cfg(any(
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "loongarch32",
            target_arch = "loongarch64",
        ))]
        {
            // See https://github.com/rust-lang/rust/pull/114034 for details.
            // https://github.com/rust-lang/rust/blob/1.84.0/library/core/src/sync/atomic.rs#L249
            // https://godbolt.org/z/ofbGGdx44
            self.compare_exchange(current, new, success, failure)
        }
        #[cfg(not(any(
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "loongarch32",
            target_arch = "loongarch64",
        )))]
        {
            match self
                .as_atomic_u8()
                .compare_exchange_weak(current as u8, new as u8, success, failure)
            {
                Ok(x) => Ok(x != 0),
                Err(x) => Err(x != 0),
            }
        }
    }

    /// Logical "and" with a boolean value.
    ///
    /// Performs a logical "and" operation on the current value and the argument `val`, and sets
    /// the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// `fetch_and` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let foo = AtomicBool::new(true);
    /// assert_eq!(foo.fetch_and(false, Ordering::SeqCst), true);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    ///
    /// let foo = AtomicBool::new(true);
    /// assert_eq!(foo.fetch_and(true, Ordering::SeqCst), true);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    ///
    /// let foo = AtomicBool::new(false);
    /// assert_eq!(foo.fetch_and(false, Ordering::SeqCst), false);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_and(&self, val: bool, order: Ordering) -> bool {
        self.as_atomic_u8().fetch_and(val as u8, order) != 0
    }

    /// Logical "and" with a boolean value.
    ///
    /// Performs a logical "and" operation on the current value and the argument `val`, and sets
    /// the new value to the result.
    ///
    /// Unlike `fetch_and`, this does not return the previous value.
    ///
    /// `and` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// This function may generate more efficient code than `fetch_and` on some platforms.
    ///
    /// - x86/x86_64: `lock and` instead of `cmpxchg` loop
    /// - MSP430: `and` instead of disabling interrupts
    ///
    /// Note: On x86/x86_64, the use of either function should not usually
    /// affect the generated code, because LLVM can properly optimize the case
    /// where the result is unused.
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let foo = AtomicBool::new(true);
    /// foo.and(false, Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    ///
    /// let foo = AtomicBool::new(true);
    /// foo.and(true, Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    ///
    /// let foo = AtomicBool::new(false);
    /// foo.and(false, Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn and(&self, val: bool, order: Ordering) {
        self.as_atomic_u8().and(val as u8, order);
    }

    /// Logical "nand" with a boolean value.
    ///
    /// Performs a logical "nand" operation on the current value and the argument `val`, and sets
    /// the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// `fetch_nand` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let foo = AtomicBool::new(true);
    /// assert_eq!(foo.fetch_nand(false, Ordering::SeqCst), true);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    ///
    /// let foo = AtomicBool::new(true);
    /// assert_eq!(foo.fetch_nand(true, Ordering::SeqCst), true);
    /// assert_eq!(foo.load(Ordering::SeqCst) as usize, 0);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    ///
    /// let foo = AtomicBool::new(false);
    /// assert_eq!(foo.fetch_nand(false, Ordering::SeqCst), false);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_nand(&self, val: bool, order: Ordering) -> bool {
        // https://github.com/rust-lang/rust/blob/1.84.0/library/core/src/sync/atomic.rs#L973-L985
        if val {
            // !(x & true) == !x
            // We must invert the bool.
            self.fetch_xor(true, order)
        } else {
            // !(x & false) == true
            // We must set the bool to true.
            self.swap(true, order)
        }
    }

    /// Logical "or" with a boolean value.
    ///
    /// Performs a logical "or" operation on the current value and the argument `val`, and sets the
    /// new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// `fetch_or` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let foo = AtomicBool::new(true);
    /// assert_eq!(foo.fetch_or(false, Ordering::SeqCst), true);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    ///
    /// let foo = AtomicBool::new(true);
    /// assert_eq!(foo.fetch_or(true, Ordering::SeqCst), true);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    ///
    /// let foo = AtomicBool::new(false);
    /// assert_eq!(foo.fetch_or(false, Ordering::SeqCst), false);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_or(&self, val: bool, order: Ordering) -> bool {
        self.as_atomic_u8().fetch_or(val as u8, order) != 0
    }

    /// Logical "or" with a boolean value.
    ///
    /// Performs a logical "or" operation on the current value and the argument `val`, and sets the
    /// new value to the result.
    ///
    /// Unlike `fetch_or`, this does not return the previous value.
    ///
    /// `or` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// This function may generate more efficient code than `fetch_or` on some platforms.
    ///
    /// - x86/x86_64: `lock or` instead of `cmpxchg` loop
    /// - MSP430: `bis` instead of disabling interrupts
    ///
    /// Note: On x86/x86_64, the use of either function should not usually
    /// affect the generated code, because LLVM can properly optimize the case
    /// where the result is unused.
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let foo = AtomicBool::new(true);
    /// foo.or(false, Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    ///
    /// let foo = AtomicBool::new(true);
    /// foo.or(true, Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    ///
    /// let foo = AtomicBool::new(false);
    /// foo.or(false, Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn or(&self, val: bool, order: Ordering) {
        self.as_atomic_u8().or(val as u8, order);
    }

    /// Logical "xor" with a boolean value.
    ///
    /// Performs a logical "xor" operation on the current value and the argument `val`, and sets
    /// the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// `fetch_xor` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let foo = AtomicBool::new(true);
    /// assert_eq!(foo.fetch_xor(false, Ordering::SeqCst), true);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    ///
    /// let foo = AtomicBool::new(true);
    /// assert_eq!(foo.fetch_xor(true, Ordering::SeqCst), true);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    ///
    /// let foo = AtomicBool::new(false);
    /// assert_eq!(foo.fetch_xor(false, Ordering::SeqCst), false);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_xor(&self, val: bool, order: Ordering) -> bool {
        self.as_atomic_u8().fetch_xor(val as u8, order) != 0
    }

    /// Logical "xor" with a boolean value.
    ///
    /// Performs a logical "xor" operation on the current value and the argument `val`, and sets
    /// the new value to the result.
    ///
    /// Unlike `fetch_xor`, this does not return the previous value.
    ///
    /// `xor` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// This function may generate more efficient code than `fetch_xor` on some platforms.
    ///
    /// - x86/x86_64: `lock xor` instead of `cmpxchg` loop
    /// - MSP430: `xor` instead of disabling interrupts
    ///
    /// Note: On x86/x86_64, the use of either function should not usually
    /// affect the generated code, because LLVM can properly optimize the case
    /// where the result is unused.
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let foo = AtomicBool::new(true);
    /// foo.xor(false, Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    ///
    /// let foo = AtomicBool::new(true);
    /// foo.xor(true, Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    ///
    /// let foo = AtomicBool::new(false);
    /// foo.xor(false, Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn xor(&self, val: bool, order: Ordering) {
        self.as_atomic_u8().xor(val as u8, order);
    }

    /// Logical "not" with a boolean value.
    ///
    /// Performs a logical "not" operation on the current value, and sets
    /// the new value to the result.
    ///
    /// Returns the previous value.
    ///
    /// `fetch_not` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let foo = AtomicBool::new(true);
    /// assert_eq!(foo.fetch_not(Ordering::SeqCst), true);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    ///
    /// let foo = AtomicBool::new(false);
    /// assert_eq!(foo.fetch_not(Ordering::SeqCst), false);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_not(&self, order: Ordering) -> bool {
        self.fetch_xor(true, order)
    }

    /// Logical "not" with a boolean value.
    ///
    /// Performs a logical "not" operation on the current value, and sets
    /// the new value to the result.
    ///
    /// Unlike `fetch_not`, this does not return the previous value.
    ///
    /// `not` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// This function may generate more efficient code than `fetch_not` on some platforms.
    ///
    /// - x86/x86_64: `lock xor` instead of `cmpxchg` loop
    /// - MSP430: `xor` instead of disabling interrupts
    ///
    /// Note: On x86/x86_64, the use of either function should not usually
    /// affect the generated code, because LLVM can properly optimize the case
    /// where the result is unused.
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let foo = AtomicBool::new(true);
    /// foo.not(Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), false);
    ///
    /// let foo = AtomicBool::new(false);
    /// foo.not(Ordering::SeqCst);
    /// assert_eq!(foo.load(Ordering::SeqCst), true);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn not(&self, order: Ordering) {
        self.xor(true, order);
    }

    /// Fetches the value, and applies a function to it that returns an optional
    /// new value. Returns a `Result` of `Ok(previous_value)` if the function
    /// returned `Some(_)`, else `Err(previous_value)`.
    ///
    /// Note: This may call the function multiple times if the value has been
    /// changed from other threads in the meantime, as long as the function
    /// returns `Some(_)`, but the function will have been applied only once to
    /// the stored value.
    ///
    /// `fetch_update` takes two [`Ordering`] arguments to describe the memory
    /// ordering of this operation. The first describes the required ordering for
    /// when the operation finally succeeds while the second describes the
    /// required ordering for loads. These correspond to the success and failure
    /// orderings of [`compare_exchange`](Self::compare_exchange) respectively.
    ///
    /// Using [`Acquire`] as success ordering makes the store part of this
    /// operation [`Relaxed`], and using [`Release`] makes the final successful
    /// load [`Relaxed`]. The (failed) load ordering can only be [`SeqCst`],
    /// [`Acquire`] or [`Relaxed`].
    ///
    /// # Considerations
    ///
    /// This method is not magic; it is not provided by the hardware.
    /// It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),
    /// and suffers from the same drawbacks.
    /// In particular, this method will not circumvent the [ABA Problem].
    ///
    /// [ABA Problem]: https://en.wikipedia.org/wiki/ABA_problem
    ///
    /// # Panics
    ///
    /// Panics if `fetch_order` is [`Release`], [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicBool, Ordering};
    ///
    /// let x = AtomicBool::new(false);
    /// assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(false));
    /// assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(!x)), Ok(false));
    /// assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(!x)), Ok(true));
    /// assert_eq!(x.load(Ordering::SeqCst), false);
    /// ```
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        mut f: F,
    ) -> Result<bool, bool>
    where
        F: FnMut(bool) -> Option<bool>,
    {
        let mut prev = self.load(fetch_order);
        while let Some(next) = f(prev) {
            match self.compare_exchange_weak(prev, next, set_order, fetch_order) {
                x @ Ok(_) => return x,
                Err(next_prev) => prev = next_prev,
            }
        }
        Err(prev)
    }
    } // cfg_has_atomic_cas_or_amo32!

    const_fn! {
        // This function is actually `const fn`-compatible on Rust 1.32+,
        // but makes `const fn` only on Rust 1.58+ to match other atomic types.
        const_if: #[cfg(not(portable_atomic_no_const_raw_ptr_deref))];
        /// Returns a mutable pointer to the underlying [`bool`].
        ///
        /// Returning an `*mut` pointer from a shared reference to this atomic is
        /// safe because the atomic types work with interior mutability. Any use of
        /// the returned raw pointer requires an `unsafe` block and has to uphold
        /// the safety requirements. If there is concurrent access, note the following
        /// additional safety requirements:
        ///
        /// - If this atomic type is [lock-free](Self::is_lock_free), any concurrent
        ///   operations on it must be atomic.
        /// - Otherwise, any concurrent operations on it must be compatible with
        ///   operations performed by this atomic type.
        ///
        /// This is `const fn` on Rust 1.58+.
        #[inline]
        pub const fn as_ptr(&self) -> *mut bool {
            self.v.get() as *mut bool
        }
    }

    #[inline(always)]
    fn as_atomic_u8(&self) -> &imp::AtomicU8 {
        // SAFETY: AtomicBool and imp::AtomicU8 have the same layout,
        // and both access data in the same way.
        unsafe { &*(self as *const Self as *const imp::AtomicU8) }
    }
}
// See https://github.com/taiki-e/portable-atomic/issues/180
#[cfg(not(feature = "require-cas"))]
cfg_no_atomic_cas! {
#[doc(hidden)]
#[allow(unused_variables, clippy::unused_self, clippy::extra_unused_lifetimes)]
impl<'a> AtomicBool {
    cfg_no_atomic_cas_or_amo32! {
    #[inline]
    pub fn swap(&self, val: bool, order: Ordering) -> bool
    where
        &'a Self: HasSwap,
    {
        unimplemented!()
    }
    #[inline]
    pub fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool>
    where
        &'a Self: HasCompareExchange,
    {
        unimplemented!()
    }
    #[inline]
    pub fn compare_exchange_weak(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool>
    where
        &'a Self: HasCompareExchangeWeak,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_and(&self, val: bool, order: Ordering) -> bool
    where
        &'a Self: HasFetchAnd,
    {
        unimplemented!()
    }
    #[inline]
    pub fn and(&self, val: bool, order: Ordering)
    where
        &'a Self: HasAnd,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_nand(&self, val: bool, order: Ordering) -> bool
    where
        &'a Self: HasFetchNand,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_or(&self, val: bool, order: Ordering) -> bool
    where
        &'a Self: HasFetchOr,
    {
        unimplemented!()
    }
    #[inline]
    pub fn or(&self, val: bool, order: Ordering)
    where
        &'a Self: HasOr,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_xor(&self, val: bool, order: Ordering) -> bool
    where
        &'a Self: HasFetchXor,
    {
        unimplemented!()
    }
    #[inline]
    pub fn xor(&self, val: bool, order: Ordering)
    where
        &'a Self: HasXor,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_not(&self, order: Ordering) -> bool
    where
        &'a Self: HasFetchNot,
    {
        unimplemented!()
    }
    #[inline]
    pub fn not(&self, order: Ordering)
    where
        &'a Self: HasNot,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F,
    ) -> Result<bool, bool>
    where
        F: FnMut(bool) -> Option<bool>,
        &'a Self: HasFetchUpdate,
    {
        unimplemented!()
    }
    } // cfg_no_atomic_cas_or_amo32!
}
} // cfg_no_atomic_cas!
} // cfg_has_atomic_8!

cfg_has_atomic_ptr! {
/// A raw pointer type which can be safely shared between threads.
///
/// This type has the same in-memory representation as a `*mut T`.
///
/// If the compiler and the platform support atomic loads and stores of pointers,
/// this type is a wrapper for the standard library's
/// [`AtomicPtr`](core::sync::atomic::AtomicPtr). If the platform supports it
/// but the compiler does not, atomic operations are implemented using inline
/// assembly.
// We can use #[repr(transparent)] here, but #[repr(C, align(N))]
// will show clearer docs.
#[cfg_attr(target_pointer_width = "16", repr(C, align(2)))]
#[cfg_attr(target_pointer_width = "32", repr(C, align(4)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(8)))]
#[cfg_attr(target_pointer_width = "128", repr(C, align(16)))]
pub struct AtomicPtr<T> {
    inner: imp::AtomicPtr<T>,
}

impl<T> Default for AtomicPtr<T> {
    /// Creates a null `AtomicPtr<T>`.
    #[inline]
    fn default() -> Self {
        Self::new(ptr::null_mut())
    }
}

impl<T> From<*mut T> for AtomicPtr<T> {
    #[inline]
    fn from(p: *mut T) -> Self {
        Self::new(p)
    }
}

impl<T> fmt::Debug for AtomicPtr<T> {
    #[inline] // fmt is not hot path, but #[inline] on fmt seems to still be useful: https://github.com/rust-lang/rust/pull/117727
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // std atomic types use Relaxed in Debug::fmt: https://github.com/rust-lang/rust/blob/1.84.0/library/core/src/sync/atomic.rs#L2188
        fmt::Debug::fmt(&self.load(Ordering::Relaxed), f)
    }
}

impl<T> fmt::Pointer for AtomicPtr<T> {
    #[inline] // fmt is not hot path, but #[inline] on fmt seems to still be useful: https://github.com/rust-lang/rust/pull/117727
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // std atomic types use Relaxed in Debug::fmt: https://github.com/rust-lang/rust/blob/1.84.0/library/core/src/sync/atomic.rs#L2188
        fmt::Pointer::fmt(&self.load(Ordering::Relaxed), f)
    }
}

// UnwindSafe is implicitly implemented.
#[cfg(not(portable_atomic_no_core_unwind_safe))]
impl<T> core::panic::RefUnwindSafe for AtomicPtr<T> {}
#[cfg(all(portable_atomic_no_core_unwind_safe, feature = "std"))]
impl<T> std::panic::RefUnwindSafe for AtomicPtr<T> {}

impl<T> AtomicPtr<T> {
    /// Creates a new `AtomicPtr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::AtomicPtr;
    ///
    /// let ptr = &mut 5;
    /// let atomic_ptr = AtomicPtr::new(ptr);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(p: *mut T) -> Self {
        static_assert_layout!(AtomicPtr<()>, *mut ());
        Self { inner: imp::AtomicPtr::new(p) }
    }

    // TODO: update docs based on https://github.com/rust-lang/rust/pull/116762
    const_fn! {
        const_if: #[cfg(not(portable_atomic_no_const_mut_refs))];
        /// Creates a new `AtomicPtr` from a pointer.
        ///
        /// This is `const fn` on Rust 1.83+.
        ///
        /// # Safety
        ///
        /// * `ptr` must be aligned to `align_of::<AtomicPtr<T>>()` (note that on some platforms this
        ///   can be bigger than `align_of::<*mut T>()`).
        /// * `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`.
        /// * If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value
        ///   behind `ptr` must have a happens-before relationship with atomic accesses via the returned
        ///   value (or vice-versa).
        ///   * In other words, time periods where the value is accessed atomically may not overlap
        ///     with periods where the value is accessed non-atomically.
        ///   * This requirement is trivially satisfied if `ptr` is never used non-atomically for the
        ///     duration of lifetime `'a`. Most use cases should be able to follow this guideline.
        ///   * This requirement is also trivially satisfied if all accesses (atomic or not) are done
        ///     from the same thread.
        /// * If this atomic type is *not* lock-free:
        ///   * Any accesses to the value behind `ptr` must have a happens-before relationship
        ///     with accesses via the returned value (or vice-versa).
        ///   * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must
        ///     be compatible with operations performed by this atomic type.
        /// * This method must not be used to create overlapping or mixed-size atomic accesses, as
        ///   these are not supported by the memory model.
        ///
        /// [valid]: core::ptr#safety
        #[inline]
        #[must_use]
        pub const unsafe fn from_ptr<'a>(ptr: *mut *mut T) -> &'a Self {
            #[allow(clippy::cast_ptr_alignment)]
            // SAFETY: guaranteed by the caller
            unsafe { &*(ptr as *mut Self) }
        }
    }

    /// Returns `true` if operations on values of this type are lock-free.
    ///
    /// If the compiler or the platform doesn't support the necessary
    /// atomic instructions, global locks for every potentially
    /// concurrent atomic operation will be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::AtomicPtr;
    ///
    /// let is_lock_free = AtomicPtr::<()>::is_lock_free();
    /// ```
    #[inline]
    #[must_use]
    pub fn is_lock_free() -> bool {
        <imp::AtomicPtr<T>>::is_lock_free()
    }

    /// Returns `true` if operations on values of this type are lock-free.
    ///
    /// If the compiler or the platform doesn't support the necessary
    /// atomic instructions, global locks for every potentially
    /// concurrent atomic operation will be used.
    ///
    /// **Note:** If the atomic operation relies on dynamic CPU feature detection,
    /// this type may be lock-free even if the function returns false.
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::AtomicPtr;
    ///
    /// const IS_ALWAYS_LOCK_FREE: bool = AtomicPtr::<()>::is_always_lock_free();
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_always_lock_free() -> bool {
        <imp::AtomicPtr<T>>::IS_ALWAYS_LOCK_FREE
    }
    #[cfg(test)]
    const IS_ALWAYS_LOCK_FREE: bool = Self::is_always_lock_free();

    const_fn! {
        const_if: #[cfg(not(portable_atomic_no_const_mut_refs))];
        /// Returns a mutable reference to the underlying pointer.
        ///
        /// This is safe because the mutable reference guarantees that no other threads are
        /// concurrently accessing the atomic data.
        ///
        /// This is `const fn` on Rust 1.83+.
        ///
        /// # Examples
        ///
        /// ```
        /// use portable_atomic::{AtomicPtr, Ordering};
        ///
        /// let mut data = 10;
        /// let mut atomic_ptr = AtomicPtr::new(&mut data);
        /// let mut other_data = 5;
        /// *atomic_ptr.get_mut() = &mut other_data;
        /// assert_eq!(unsafe { *atomic_ptr.load(Ordering::SeqCst) }, 5);
        /// ```
        #[inline]
        pub const fn get_mut(&mut self) -> &mut *mut T {
            // SAFETY: the mutable reference guarantees unique ownership.
            // (core::sync::atomic::Atomic*::get_mut is not const yet)
            unsafe { &mut *self.as_ptr() }
        }
    }

    // TODO: Add from_mut/get_mut_slice/from_mut_slice once it is stable on std atomic types.
    // https://github.com/rust-lang/rust/issues/76314

    const_fn! {
        const_if: #[cfg(not(portable_atomic_no_const_transmute))];
        /// Consumes the atomic and returns the contained value.
        ///
        /// This is safe because passing `self` by value guarantees that no other threads are
        /// concurrently accessing the atomic data.
        ///
        /// This is `const fn` on Rust 1.56+.
        ///
        /// # Examples
        ///
        /// ```
        /// use portable_atomic::AtomicPtr;
        ///
        /// let mut data = 5;
        /// let atomic_ptr = AtomicPtr::new(&mut data);
        /// assert_eq!(unsafe { *atomic_ptr.into_inner() }, 5);
        /// ```
        #[inline]
        pub const fn into_inner(self) -> *mut T {
            // SAFETY: AtomicPtr<T> and *mut T have the same size and in-memory representations,
            // so they can be safely transmuted.
            // (const UnsafeCell::into_inner is unstable)
            unsafe { core::mem::transmute(self) }
        }
    }

    /// Loads a value from the pointer.
    ///
    /// `load` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. Possible values are [`SeqCst`], [`Acquire`] and [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `order` is [`Release`] or [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let ptr = &mut 5;
    /// let some_ptr = AtomicPtr::new(ptr);
    ///
    /// let value = some_ptr.load(Ordering::Relaxed);
    /// ```
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub fn load(&self, order: Ordering) -> *mut T {
        self.inner.load(order)
    }

    /// Stores a value into the pointer.
    ///
    /// `store` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. Possible values are [`SeqCst`], [`Release`] and [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `order` is [`Acquire`] or [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let ptr = &mut 5;
    /// let some_ptr = AtomicPtr::new(ptr);
    ///
    /// let other_ptr = &mut 10;
    ///
    /// some_ptr.store(other_ptr, Ordering::Relaxed);
    /// ```
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub fn store(&self, ptr: *mut T, order: Ordering) {
        self.inner.store(ptr, order);
    }

    cfg_has_atomic_cas_or_amo32! {
    /// Stores a value into the pointer, returning the previous value.
    ///
    /// `swap` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let ptr = &mut 5;
    /// let some_ptr = AtomicPtr::new(ptr);
    ///
    /// let other_ptr = &mut 10;
    ///
    /// let value = some_ptr.swap(other_ptr, Ordering::Relaxed);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn swap(&self, ptr: *mut T, order: Ordering) -> *mut T {
        self.inner.swap(ptr, order)
    }

    cfg_has_atomic_cas! {
    /// Stores a value into the pointer if the current value is the same as the `current` value.
    ///
    /// The return value is a result indicating whether the new value was written and containing
    /// the previous value. On success this value is guaranteed to be equal to `current`.
    ///
    /// `compare_exchange` takes two [`Ordering`] arguments to describe the memory
    /// ordering of this operation. `success` describes the required ordering for the
    /// read-modify-write operation that takes place if the comparison with `current` succeeds.
    /// `failure` describes the required ordering for the load operation that takes place when
    /// the comparison fails. Using [`Acquire`] as success ordering makes the store part
    /// of this operation [`Relaxed`], and using [`Release`] makes the successful load
    /// [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `failure` is [`Release`], [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let ptr = &mut 5;
    /// let some_ptr = AtomicPtr::new(ptr);
    ///
    /// let other_ptr = &mut 10;
    ///
    /// let value = some_ptr.compare_exchange(ptr, other_ptr, Ordering::SeqCst, Ordering::Relaxed);
    /// ```
    #[cfg_attr(docsrs, doc(alias = "compare_and_swap"))]
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub fn compare_exchange(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.inner.compare_exchange(current, new, success, failure)
    }

    /// Stores a value into the pointer if the current value is the same as the `current` value.
    ///
    /// Unlike [`AtomicPtr::compare_exchange`], this function is allowed to spuriously fail even when the
    /// comparison succeeds, which can result in more efficient code on some platforms. The
    /// return value is a result indicating whether the new value was written and containing the
    /// previous value.
    ///
    /// `compare_exchange_weak` takes two [`Ordering`] arguments to describe the memory
    /// ordering of this operation. `success` describes the required ordering for the
    /// read-modify-write operation that takes place if the comparison with `current` succeeds.
    /// `failure` describes the required ordering for the load operation that takes place when
    /// the comparison fails. Using [`Acquire`] as success ordering makes the store part
    /// of this operation [`Relaxed`], and using [`Release`] makes the successful load
    /// [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `failure` is [`Release`], [`AcqRel`].
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let some_ptr = AtomicPtr::new(&mut 5);
    ///
    /// let new = &mut 10;
    /// let mut old = some_ptr.load(Ordering::Relaxed);
    /// loop {
    ///     match some_ptr.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
    ///         Ok(_) => break,
    ///         Err(x) => old = x,
    ///     }
    /// }
    /// ```
    #[cfg_attr(docsrs, doc(alias = "compare_and_swap"))]
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub fn compare_exchange_weak(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.inner.compare_exchange_weak(current, new, success, failure)
    }

    /// Fetches the value, and applies a function to it that returns an optional
    /// new value. Returns a `Result` of `Ok(previous_value)` if the function
    /// returned `Some(_)`, else `Err(previous_value)`.
    ///
    /// Note: This may call the function multiple times if the value has been
    /// changed from other threads in the meantime, as long as the function
    /// returns `Some(_)`, but the function will have been applied only once to
    /// the stored value.
    ///
    /// `fetch_update` takes two [`Ordering`] arguments to describe the memory
    /// ordering of this operation. The first describes the required ordering for
    /// when the operation finally succeeds while the second describes the
    /// required ordering for loads. These correspond to the success and failure
    /// orderings of [`compare_exchange`](Self::compare_exchange) respectively.
    ///
    /// Using [`Acquire`] as success ordering makes the store part of this
    /// operation [`Relaxed`], and using [`Release`] makes the final successful
    /// load [`Relaxed`]. The (failed) load ordering can only be [`SeqCst`],
    /// [`Acquire`] or [`Relaxed`].
    ///
    /// # Panics
    ///
    /// Panics if `fetch_order` is [`Release`], [`AcqRel`].
    ///
    /// # Considerations
    ///
    /// This method is not magic; it is not provided by the hardware.
    /// It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),
    /// and suffers from the same drawbacks.
    /// In particular, this method will not circumvent the [ABA Problem].
    ///
    /// [ABA Problem]: https://en.wikipedia.org/wiki/ABA_problem
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let ptr: *mut _ = &mut 5;
    /// let some_ptr = AtomicPtr::new(ptr);
    ///
    /// let new: *mut _ = &mut 10;
    /// assert_eq!(some_ptr.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(ptr));
    /// let result = some_ptr.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
    ///     if x == ptr {
    ///         Some(new)
    ///     } else {
    ///         None
    ///     }
    /// });
    /// assert_eq!(result, Ok(ptr));
    /// assert_eq!(some_ptr.load(Ordering::SeqCst), new);
    /// ```
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        mut f: F,
    ) -> Result<*mut T, *mut T>
    where
        F: FnMut(*mut T) -> Option<*mut T>,
    {
        let mut prev = self.load(fetch_order);
        while let Some(next) = f(prev) {
            match self.compare_exchange_weak(prev, next, set_order, fetch_order) {
                x @ Ok(_) => return x,
                Err(next_prev) => prev = next_prev,
            }
        }
        Err(prev)
    }

    #[cfg(miri)]
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    fn fetch_update_<F>(&self, order: Ordering, mut f: F) -> *mut T
    where
        F: FnMut(*mut T) -> *mut T,
    {
        // This is a private function and all instances of `f` only operate on the value
        // loaded, so there is no need to synchronize the first load/failed CAS.
        let mut prev = self.load(Ordering::Relaxed);
        loop {
            let next = f(prev);
            match self.compare_exchange_weak(prev, next, order, Ordering::Relaxed) {
                Ok(x) => return x,
                Err(next_prev) => prev = next_prev,
            }
        }
    }
    } // cfg_has_atomic_cas!

    /// Offsets the pointer's address by adding `val` (in units of `T`),
    /// returning the previous pointer.
    ///
    /// This is equivalent to using [`wrapping_add`] to atomically perform the
    /// equivalent of `ptr = ptr.wrapping_add(val);`.
    ///
    /// This method operates in units of `T`, which means that it cannot be used
    /// to offset the pointer by an amount which is not a multiple of
    /// `size_of::<T>()`. This can sometimes be inconvenient, as you may want to
    /// work with a deliberately misaligned pointer. In such cases, you may use
    /// the [`fetch_byte_add`](Self::fetch_byte_add) method instead.
    ///
    /// `fetch_ptr_add` takes an [`Ordering`] argument which describes the
    /// memory ordering of this operation. All ordering modes are possible. Note
    /// that using [`Acquire`] makes the store part of this operation
    /// [`Relaxed`], and using [`Release`] makes the load part [`Relaxed`].
    ///
    /// [`wrapping_add`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_add
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unstable_name_collisions)]
    /// # #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let atom = AtomicPtr::<i64>::new(core::ptr::null_mut());
    /// assert_eq!(atom.fetch_ptr_add(1, Ordering::Relaxed).addr(), 0);
    /// // Note: units of `size_of::<i64>()`.
    /// assert_eq!(atom.load(Ordering::Relaxed).addr(), 8);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_ptr_add(&self, val: usize, order: Ordering) -> *mut T {
        self.fetch_byte_add(val.wrapping_mul(core::mem::size_of::<T>()), order)
    }

    /// Offsets the pointer's address by subtracting `val` (in units of `T`),
    /// returning the previous pointer.
    ///
    /// This is equivalent to using [`wrapping_sub`] to atomically perform the
    /// equivalent of `ptr = ptr.wrapping_sub(val);`.
    ///
    /// This method operates in units of `T`, which means that it cannot be used
    /// to offset the pointer by an amount which is not a multiple of
    /// `size_of::<T>()`. This can sometimes be inconvenient, as you may want to
    /// work with a deliberately misaligned pointer. In such cases, you may use
    /// the [`fetch_byte_sub`](Self::fetch_byte_sub) method instead.
    ///
    /// `fetch_ptr_sub` takes an [`Ordering`] argument which describes the memory
    /// ordering of this operation. All ordering modes are possible. Note that
    /// using [`Acquire`] makes the store part of this operation [`Relaxed`],
    /// and using [`Release`] makes the load part [`Relaxed`].
    ///
    /// [`wrapping_sub`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_sub
    ///
    /// # Examples
    ///
    /// ```
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let array = [1i32, 2i32];
    /// let atom = AtomicPtr::new(array.as_ptr().wrapping_add(1) as *mut _);
    ///
    /// assert!(core::ptr::eq(atom.fetch_ptr_sub(1, Ordering::Relaxed), &array[1]));
    /// assert!(core::ptr::eq(atom.load(Ordering::Relaxed), &array[0]));
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_ptr_sub(&self, val: usize, order: Ordering) -> *mut T {
        self.fetch_byte_sub(val.wrapping_mul(core::mem::size_of::<T>()), order)
    }

    /// Offsets the pointer's address by adding `val` *bytes*, returning the
    /// previous pointer.
    ///
    /// This is equivalent to using [`wrapping_add`] and [`cast`] to atomically
    /// perform `ptr = ptr.cast::<u8>().wrapping_add(val).cast::<T>()`.
    ///
    /// `fetch_byte_add` takes an [`Ordering`] argument which describes the
    /// memory ordering of this operation. All ordering modes are possible. Note
    /// that using [`Acquire`] makes the store part of this operation
    /// [`Relaxed`], and using [`Release`] makes the load part [`Relaxed`].
    ///
    /// [`wrapping_add`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_add
    /// [`cast`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.cast
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unstable_name_collisions)]
    /// # #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let atom = AtomicPtr::<i64>::new(core::ptr::null_mut());
    /// assert_eq!(atom.fetch_byte_add(1, Ordering::Relaxed).addr(), 0);
    /// // Note: in units of bytes, not `size_of::<i64>()`.
    /// assert_eq!(atom.load(Ordering::Relaxed).addr(), 1);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_byte_add(&self, val: usize, order: Ordering) -> *mut T {
        // Ideally, we would always use AtomicPtr::fetch_* since it is strict-provenance
        // compatible, but it is unstable. So, for now emulate it only on cfg(miri).
        // Code using AtomicUsize::fetch_* via casts is still permissive-provenance
        // compatible and is sound.
        // TODO: Once `#![feature(strict_provenance_atomic_ptr)]` is stabilized,
        // use AtomicPtr::fetch_* in all cases from the version in which it is stabilized.
        #[cfg(miri)]
        {
            self.fetch_update_(order, |x| x.with_addr(x.addr().wrapping_add(val)))
        }
        #[cfg(not(miri))]
        {
            crate::utils::ptr::with_exposed_provenance_mut(
                self.as_atomic_usize().fetch_add(val, order)
            )
        }
    }

    /// Offsets the pointer's address by subtracting `val` *bytes*, returning the
    /// previous pointer.
    ///
    /// This is equivalent to using [`wrapping_sub`] and [`cast`] to atomically
    /// perform `ptr = ptr.cast::<u8>().wrapping_sub(val).cast::<T>()`.
    ///
    /// `fetch_byte_sub` takes an [`Ordering`] argument which describes the
    /// memory ordering of this operation. All ordering modes are possible. Note
    /// that using [`Acquire`] makes the store part of this operation
    /// [`Relaxed`], and using [`Release`] makes the load part [`Relaxed`].
    ///
    /// [`wrapping_sub`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_sub
    /// [`cast`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.cast
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unstable_name_collisions)]
    /// # #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let atom = AtomicPtr::<i64>::new(sptr::invalid_mut(1));
    /// assert_eq!(atom.fetch_byte_sub(1, Ordering::Relaxed).addr(), 1);
    /// assert_eq!(atom.load(Ordering::Relaxed).addr(), 0);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_byte_sub(&self, val: usize, order: Ordering) -> *mut T {
        // Ideally, we would always use AtomicPtr::fetch_* since it is strict-provenance
        // compatible, but it is unstable. So, for now emulate it only on cfg(miri).
        // Code using AtomicUsize::fetch_* via casts is still permissive-provenance
        // compatible and is sound.
        // TODO: Once `#![feature(strict_provenance_atomic_ptr)]` is stabilized,
        // use AtomicPtr::fetch_* in all cases from the version in which it is stabilized.
        #[cfg(miri)]
        {
            self.fetch_update_(order, |x| x.with_addr(x.addr().wrapping_sub(val)))
        }
        #[cfg(not(miri))]
        {
            crate::utils::ptr::with_exposed_provenance_mut(
                self.as_atomic_usize().fetch_sub(val, order)
            )
        }
    }

    /// Performs a bitwise "or" operation on the address of the current pointer,
    /// and the argument `val`, and stores a pointer with provenance of the
    /// current pointer and the resulting address.
    ///
    /// This is equivalent to using [`map_addr`] to atomically perform
    /// `ptr = ptr.map_addr(|a| a | val)`. This can be used in tagged
    /// pointer schemes to atomically set tag bits.
    ///
    /// **Caveat**: This operation returns the previous value. To compute the
    /// stored value without losing provenance, you may use [`map_addr`]. For
    /// example: `a.fetch_or(val).map_addr(|a| a | val)`.
    ///
    /// `fetch_or` takes an [`Ordering`] argument which describes the memory
    /// ordering of this operation. All ordering modes are possible. Note that
    /// using [`Acquire`] makes the store part of this operation [`Relaxed`],
    /// and using [`Release`] makes the load part [`Relaxed`].
    ///
    /// This API and its claimed semantics are part of the Strict Provenance
    /// experiment, see the [module documentation for `ptr`][core::ptr] for
    /// details.
    ///
    /// [`map_addr`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.map_addr
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unstable_name_collisions)]
    /// # #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let pointer = &mut 3i64 as *mut i64;
    ///
    /// let atom = AtomicPtr::<i64>::new(pointer);
    /// // Tag the bottom bit of the pointer.
    /// assert_eq!(atom.fetch_or(1, Ordering::Relaxed).addr() & 1, 0);
    /// // Extract and untag.
    /// let tagged = atom.load(Ordering::Relaxed);
    /// assert_eq!(tagged.addr() & 1, 1);
    /// assert_eq!(tagged.map_addr(|p| p & !1), pointer);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_or(&self, val: usize, order: Ordering) -> *mut T {
        // Ideally, we would always use AtomicPtr::fetch_* since it is strict-provenance
        // compatible, but it is unstable. So, for now emulate it only on cfg(miri).
        // Code using AtomicUsize::fetch_* via casts is still permissive-provenance
        // compatible and is sound.
        // TODO: Once `#![feature(strict_provenance_atomic_ptr)]` is stabilized,
        // use AtomicPtr::fetch_* in all cases from the version in which it is stabilized.
        #[cfg(miri)]
        {
            self.fetch_update_(order, |x| x.with_addr(x.addr() | val))
        }
        #[cfg(not(miri))]
        {
            crate::utils::ptr::with_exposed_provenance_mut(
                self.as_atomic_usize().fetch_or(val, order)
            )
        }
    }

    /// Performs a bitwise "and" operation on the address of the current
    /// pointer, and the argument `val`, and stores a pointer with provenance of
    /// the current pointer and the resulting address.
    ///
    /// This is equivalent to using [`map_addr`] to atomically perform
    /// `ptr = ptr.map_addr(|a| a & val)`. This can be used in tagged
    /// pointer schemes to atomically unset tag bits.
    ///
    /// **Caveat**: This operation returns the previous value. To compute the
    /// stored value without losing provenance, you may use [`map_addr`]. For
    /// example: `a.fetch_and(val).map_addr(|a| a & val)`.
    ///
    /// `fetch_and` takes an [`Ordering`] argument which describes the memory
    /// ordering of this operation. All ordering modes are possible. Note that
    /// using [`Acquire`] makes the store part of this operation [`Relaxed`],
    /// and using [`Release`] makes the load part [`Relaxed`].
    ///
    /// This API and its claimed semantics are part of the Strict Provenance
    /// experiment, see the [module documentation for `ptr`][core::ptr] for
    /// details.
    ///
    /// [`map_addr`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.map_addr
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unstable_name_collisions)]
    /// # #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let pointer = &mut 3i64 as *mut i64;
    /// // A tagged pointer
    /// let atom = AtomicPtr::<i64>::new(pointer.map_addr(|a| a | 1));
    /// assert_eq!(atom.fetch_or(1, Ordering::Relaxed).addr() & 1, 1);
    /// // Untag, and extract the previously tagged pointer.
    /// let untagged = atom.fetch_and(!1, Ordering::Relaxed).map_addr(|a| a & !1);
    /// assert_eq!(untagged, pointer);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_and(&self, val: usize, order: Ordering) -> *mut T {
        // Ideally, we would always use AtomicPtr::fetch_* since it is strict-provenance
        // compatible, but it is unstable. So, for now emulate it only on cfg(miri).
        // Code using AtomicUsize::fetch_* via casts is still permissive-provenance
        // compatible and is sound.
        // TODO: Once `#![feature(strict_provenance_atomic_ptr)]` is stabilized,
        // use AtomicPtr::fetch_* in all cases from the version in which it is stabilized.
        #[cfg(miri)]
        {
            self.fetch_update_(order, |x| x.with_addr(x.addr() & val))
        }
        #[cfg(not(miri))]
        {
            crate::utils::ptr::with_exposed_provenance_mut(
                self.as_atomic_usize().fetch_and(val, order)
            )
        }
    }

    /// Performs a bitwise "xor" operation on the address of the current
    /// pointer, and the argument `val`, and stores a pointer with provenance of
    /// the current pointer and the resulting address.
    ///
    /// This is equivalent to using [`map_addr`] to atomically perform
    /// `ptr = ptr.map_addr(|a| a ^ val)`. This can be used in tagged
    /// pointer schemes to atomically toggle tag bits.
    ///
    /// **Caveat**: This operation returns the previous value. To compute the
    /// stored value without losing provenance, you may use [`map_addr`]. For
    /// example: `a.fetch_xor(val).map_addr(|a| a ^ val)`.
    ///
    /// `fetch_xor` takes an [`Ordering`] argument which describes the memory
    /// ordering of this operation. All ordering modes are possible. Note that
    /// using [`Acquire`] makes the store part of this operation [`Relaxed`],
    /// and using [`Release`] makes the load part [`Relaxed`].
    ///
    /// This API and its claimed semantics are part of the Strict Provenance
    /// experiment, see the [module documentation for `ptr`][core::ptr] for
    /// details.
    ///
    /// [`map_addr`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.map_addr
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unstable_name_collisions)]
    /// # #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let pointer = &mut 3i64 as *mut i64;
    /// let atom = AtomicPtr::<i64>::new(pointer);
    ///
    /// // Toggle a tag bit on the pointer.
    /// atom.fetch_xor(1, Ordering::Relaxed);
    /// assert_eq!(atom.load(Ordering::Relaxed).addr() & 1, 1);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn fetch_xor(&self, val: usize, order: Ordering) -> *mut T {
        // Ideally, we would always use AtomicPtr::fetch_* since it is strict-provenance
        // compatible, but it is unstable. So, for now emulate it only on cfg(miri).
        // Code using AtomicUsize::fetch_* via casts is still permissive-provenance
        // compatible and is sound.
        // TODO: Once `#![feature(strict_provenance_atomic_ptr)]` is stabilized,
        // use AtomicPtr::fetch_* in all cases from the version in which it is stabilized.
        #[cfg(miri)]
        {
            self.fetch_update_(order, |x| x.with_addr(x.addr() ^ val))
        }
        #[cfg(not(miri))]
        {
            crate::utils::ptr::with_exposed_provenance_mut(
                self.as_atomic_usize().fetch_xor(val, order)
            )
        }
    }

    /// Sets the bit at the specified bit-position to 1.
    ///
    /// Returns `true` if the specified bit was previously set to 1.
    ///
    /// `bit_set` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unstable_name_collisions)]
    /// # #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let pointer = &mut 3i64 as *mut i64;
    ///
    /// let atom = AtomicPtr::<i64>::new(pointer);
    /// // Tag the bottom bit of the pointer.
    /// assert!(!atom.bit_set(0, Ordering::Relaxed));
    /// // Extract and untag.
    /// let tagged = atom.load(Ordering::Relaxed);
    /// assert_eq!(tagged.addr() & 1, 1);
    /// assert_eq!(tagged.map_addr(|p| p & !1), pointer);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn bit_set(&self, bit: u32, order: Ordering) -> bool {
        // Ideally, we would always use AtomicPtr::fetch_* since it is strict-provenance
        // compatible, but it is unstable. So, for now emulate it only on cfg(miri).
        // Code using AtomicUsize::fetch_* via casts is still permissive-provenance
        // compatible and is sound.
        // TODO: Once `#![feature(strict_provenance_atomic_ptr)]` is stabilized,
        // use AtomicPtr::fetch_* in all cases from the version in which it is stabilized.
        #[cfg(miri)]
        {
            let mask = 1_usize.wrapping_shl(bit);
            self.fetch_or(mask, order).addr() & mask != 0
        }
        #[cfg(not(miri))]
        {
            self.as_atomic_usize().bit_set(bit, order)
        }
    }

    /// Clears the bit at the specified bit-position to 1.
    ///
    /// Returns `true` if the specified bit was previously set to 1.
    ///
    /// `bit_clear` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unstable_name_collisions)]
    /// # #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let pointer = &mut 3i64 as *mut i64;
    /// // A tagged pointer
    /// let atom = AtomicPtr::<i64>::new(pointer.map_addr(|a| a | 1));
    /// assert!(atom.bit_set(0, Ordering::Relaxed));
    /// // Untag
    /// assert!(atom.bit_clear(0, Ordering::Relaxed));
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn bit_clear(&self, bit: u32, order: Ordering) -> bool {
        // Ideally, we would always use AtomicPtr::fetch_* since it is strict-provenance
        // compatible, but it is unstable. So, for now emulate it only on cfg(miri).
        // Code using AtomicUsize::fetch_* via casts is still permissive-provenance
        // compatible and is sound.
        // TODO: Once `#![feature(strict_provenance_atomic_ptr)]` is stabilized,
        // use AtomicPtr::fetch_* in all cases from the version in which it is stabilized.
        #[cfg(miri)]
        {
            let mask = 1_usize.wrapping_shl(bit);
            self.fetch_and(!mask, order).addr() & mask != 0
        }
        #[cfg(not(miri))]
        {
            self.as_atomic_usize().bit_clear(bit, order)
        }
    }

    /// Toggles the bit at the specified bit-position.
    ///
    /// Returns `true` if the specified bit was previously set to 1.
    ///
    /// `bit_toggle` takes an [`Ordering`] argument which describes the memory ordering
    /// of this operation. All ordering modes are possible. Note that using
    /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
    /// using [`Release`] makes the load part [`Relaxed`].
    ///
    /// This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unstable_name_collisions)]
    /// # #[allow(unused_imports)] use sptr::Strict as _; // strict provenance polyfill for old rustc
    /// use portable_atomic::{AtomicPtr, Ordering};
    ///
    /// let pointer = &mut 3i64 as *mut i64;
    /// let atom = AtomicPtr::<i64>::new(pointer);
    ///
    /// // Toggle a tag bit on the pointer.
    /// atom.bit_toggle(0, Ordering::Relaxed);
    /// assert_eq!(atom.load(Ordering::Relaxed).addr() & 1, 1);
    /// ```
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub fn bit_toggle(&self, bit: u32, order: Ordering) -> bool {
        // Ideally, we would always use AtomicPtr::fetch_* since it is strict-provenance
        // compatible, but it is unstable. So, for now emulate it only on cfg(miri).
        // Code using AtomicUsize::fetch_* via casts is still permissive-provenance
        // compatible and is sound.
        // TODO: Once `#![feature(strict_provenance_atomic_ptr)]` is stabilized,
        // use AtomicPtr::fetch_* in all cases from the version in which it is stabilized.
        #[cfg(miri)]
        {
            let mask = 1_usize.wrapping_shl(bit);
            self.fetch_xor(mask, order).addr() & mask != 0
        }
        #[cfg(not(miri))]
        {
            self.as_atomic_usize().bit_toggle(bit, order)
        }
    }

    #[cfg(not(miri))]
    #[inline(always)]
    fn as_atomic_usize(&self) -> &AtomicUsize {
        static_assert!(
            core::mem::size_of::<AtomicPtr<()>>() == core::mem::size_of::<AtomicUsize>()
        );
        static_assert!(
            core::mem::align_of::<AtomicPtr<()>>() == core::mem::align_of::<AtomicUsize>()
        );
        // SAFETY: AtomicPtr and AtomicUsize have the same layout,
        // and both access data in the same way.
        unsafe { &*(self as *const Self as *const AtomicUsize) }
    }
    } // cfg_has_atomic_cas_or_amo32!

    const_fn! {
        const_if: #[cfg(not(portable_atomic_no_const_raw_ptr_deref))];
        /// Returns a mutable pointer to the underlying pointer.
        ///
        /// Returning an `*mut` pointer from a shared reference to this atomic is
        /// safe because the atomic types work with interior mutability. Any use of
        /// the returned raw pointer requires an `unsafe` block and has to uphold
        /// the safety requirements. If there is concurrent access, note the following
        /// additional safety requirements:
        ///
        /// - If this atomic type is [lock-free](Self::is_lock_free), any concurrent
        ///   operations on it must be atomic.
        /// - Otherwise, any concurrent operations on it must be compatible with
        ///   operations performed by this atomic type.
        ///
        /// This is `const fn` on Rust 1.58+.
        #[inline]
        pub const fn as_ptr(&self) -> *mut *mut T {
            self.inner.as_ptr()
        }
    }
}
// See https://github.com/taiki-e/portable-atomic/issues/180
#[cfg(not(feature = "require-cas"))]
cfg_no_atomic_cas! {
#[doc(hidden)]
#[allow(unused_variables, clippy::unused_self, clippy::extra_unused_lifetimes)]
impl<'a, T: 'a> AtomicPtr<T> {
    cfg_no_atomic_cas_or_amo32! {
    #[inline]
    pub fn swap(&self, ptr: *mut T, order: Ordering) -> *mut T
    where
        &'a Self: HasSwap,
    {
        unimplemented!()
    }
    } // cfg_no_atomic_cas_or_amo32!
    #[inline]
    pub fn compare_exchange(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T>
    where
        &'a Self: HasCompareExchange,
    {
        unimplemented!()
    }
    #[inline]
    pub fn compare_exchange_weak(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T>
    where
        &'a Self: HasCompareExchangeWeak,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F,
    ) -> Result<*mut T, *mut T>
    where
        F: FnMut(*mut T) -> Option<*mut T>,
        &'a Self: HasFetchUpdate,
    {
        unimplemented!()
    }
    cfg_no_atomic_cas_or_amo32! {
    #[inline]
    pub fn fetch_ptr_add(&self, val: usize, order: Ordering) -> *mut T
    where
        &'a Self: HasFetchPtrAdd,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_ptr_sub(&self, val: usize, order: Ordering) -> *mut T
    where
        &'a Self: HasFetchPtrSub,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_byte_add(&self, val: usize, order: Ordering) -> *mut T
    where
        &'a Self: HasFetchByteAdd,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_byte_sub(&self, val: usize, order: Ordering) -> *mut T
    where
        &'a Self: HasFetchByteSub,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_or(&self, val: usize, order: Ordering) -> *mut T
    where
        &'a Self: HasFetchOr,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_and(&self, val: usize, order: Ordering) -> *mut T
    where
        &'a Self: HasFetchAnd,
    {
        unimplemented!()
    }
    #[inline]
    pub fn fetch_xor(&self, val: usize, order: Ordering) -> *mut T
    where
        &'a Self: HasFetchXor,
    {
        unimplemented!()
    }
    #[inline]
    pub fn bit_set(&self, bit: u32, order: Ordering) -> bool
    where
        &'a Self: HasBitSet,
    {
        unimplemented!()
    }
    #[inline]
    pub fn bit_clear(&self, bit: u32, order: Ordering) -> bool
    where
        &'a Self: HasBitClear,
    {
        unimplemented!()
    }
    #[inline]
    pub fn bit_toggle(&self, bit: u32, order: Ordering) -> bool
    where
        &'a Self: HasBitToggle,
    {
        unimplemented!()
    }
    } // cfg_no_atomic_cas_or_amo32!
}
} // cfg_no_atomic_cas!
} // cfg_has_atomic_ptr!

macro_rules! atomic_int {
    // Atomic{I,U}* impls
    ($atomic_type:ident, $int_type:ident, $align:literal,
        $cfg_has_atomic_cas_or_amo32_or_8:ident, $cfg_no_atomic_cas_or_amo32_or_8:ident
        $(, #[$cfg_float:meta] $atomic_float_type:ident, $float_type:ident)?
    ) => {
        doc_comment! {
            concat!("An integer type which can be safely shared between threads.

This type has the same in-memory representation as the underlying integer type,
[`", stringify!($int_type), "`].

If the compiler and the platform support atomic loads and stores of [`", stringify!($int_type),
"`], this type is a wrapper for the standard library's `", stringify!($atomic_type),
"`. If the platform supports it but the compiler does not, atomic operations are implemented using
inline assembly. Otherwise synchronizes using global locks.
You can call [`", stringify!($atomic_type), "::is_lock_free()`] to check whether
atomic instructions or locks will be used.
"
            ),
            // We can use #[repr(transparent)] here, but #[repr(C, align(N))]
            // will show clearer docs.
            #[repr(C, align($align))]
            pub struct $atomic_type {
                inner: imp::$atomic_type,
            }
        }

        impl Default for $atomic_type {
            #[inline]
            fn default() -> Self {
                Self::new($int_type::default())
            }
        }

        impl From<$int_type> for $atomic_type {
            #[inline]
            fn from(v: $int_type) -> Self {
                Self::new(v)
            }
        }

        // UnwindSafe is implicitly implemented.
        #[cfg(not(portable_atomic_no_core_unwind_safe))]
        impl core::panic::RefUnwindSafe for $atomic_type {}
        #[cfg(all(portable_atomic_no_core_unwind_safe, feature = "std"))]
        impl std::panic::RefUnwindSafe for $atomic_type {}

        impl_debug_and_serde!($atomic_type);

        impl $atomic_type {
            doc_comment! {
                concat!(
                    "Creates a new atomic integer.

# Examples

```
use portable_atomic::", stringify!($atomic_type), ";

let atomic_forty_two = ", stringify!($atomic_type), "::new(42);
```"
                ),
                #[inline]
                #[must_use]
                pub const fn new(v: $int_type) -> Self {
                    static_assert_layout!($atomic_type, $int_type);
                    Self { inner: imp::$atomic_type::new(v) }
                }
            }

            // TODO: update docs based on https://github.com/rust-lang/rust/pull/116762
            #[cfg(not(portable_atomic_no_const_mut_refs))]
            doc_comment! {
                concat!("Creates a new reference to an atomic integer from a pointer.

This is `const fn` on Rust 1.83+.

# Safety

* `ptr` must be aligned to `align_of::<", stringify!($atomic_type), ">()` (note that on some platforms this
  can be bigger than `align_of::<", stringify!($int_type), ">()`).
* `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`.
* If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value
  behind `ptr` must have a happens-before relationship with atomic accesses via
  the returned value (or vice-versa).
  * In other words, time periods where the value is accessed atomically may not
    overlap with periods where the value is accessed non-atomically.
  * This requirement is trivially satisfied if `ptr` is never used non-atomically
    for the duration of lifetime `'a`. Most use cases should be able to follow
    this guideline.
  * This requirement is also trivially satisfied if all accesses (atomic or not) are
    done from the same thread.
* If this atomic type is *not* lock-free:
  * Any accesses to the value behind `ptr` must have a happens-before relationship
    with accesses via the returned value (or vice-versa).
  * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must
    be compatible with operations performed by this atomic type.
* This method must not be used to create overlapping or mixed-size atomic
  accesses, as these are not supported by the memory model.

[valid]: core::ptr#safety"),
                #[inline]
                #[must_use]
                pub const unsafe fn from_ptr<'a>(ptr: *mut $int_type) -> &'a Self {
                    #[allow(clippy::cast_ptr_alignment)]
                    // SAFETY: guaranteed by the caller
                    unsafe { &*(ptr as *mut Self) }
                }
            }
            #[cfg(portable_atomic_no_const_mut_refs)]
            doc_comment! {
                concat!("Creates a new reference to an atomic integer from a pointer.

This is `const fn` on Rust 1.83+.

# Safety

* `ptr` must be aligned to `align_of::<", stringify!($atomic_type), ">()` (note that on some platforms this
  can be bigger than `align_of::<", stringify!($int_type), ">()`).
* `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`.
* If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value
  behind `ptr` must have a happens-before relationship with atomic accesses via
  the returned value (or vice-versa).
  * In other words, time periods where the value is accessed atomically may not
    overlap with periods where the value is accessed non-atomically.
  * This requirement is trivially satisfied if `ptr` is never used non-atomically
    for the duration of lifetime `'a`. Most use cases should be able to follow
    this guideline.
  * This requirement is also trivially satisfied if all accesses (atomic or not) are
    done from the same thread.
* If this atomic type is *not* lock-free:
  * Any accesses to the value behind `ptr` must have a happens-before relationship
    with accesses via the returned value (or vice-versa).
  * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must
    be compatible with operations performed by this atomic type.
* This method must not be used to create overlapping or mixed-size atomic
  accesses, as these are not supported by the memory model.

[valid]: core::ptr#safety"),
                #[inline]
                #[must_use]
                pub unsafe fn from_ptr<'a>(ptr: *mut $int_type) -> &'a Self {
                    #[allow(clippy::cast_ptr_alignment)]
                    // SAFETY: guaranteed by the caller
                    unsafe { &*(ptr as *mut Self) }
                }
            }

            doc_comment! {
                concat!("Returns `true` if operations on values of this type are lock-free.

If the compiler or the platform doesn't support the necessary
atomic instructions, global locks for every potentially
concurrent atomic operation will be used.

# Examples

```
use portable_atomic::", stringify!($atomic_type), ";

let is_lock_free = ", stringify!($atomic_type), "::is_lock_free();
```"),
                #[inline]
                #[must_use]
                pub fn is_lock_free() -> bool {
                    <imp::$atomic_type>::is_lock_free()
                }
            }

            doc_comment! {
                concat!("Returns `true` if operations on values of this type are lock-free.

If the compiler or the platform doesn't support the necessary
atomic instructions, global locks for every potentially
concurrent atomic operation will be used.

**Note:** If the atomic operation relies on dynamic CPU feature detection,
this type may be lock-free even if the function returns false.

# Examples

```
use portable_atomic::", stringify!($atomic_type), ";

const IS_ALWAYS_LOCK_FREE: bool = ", stringify!($atomic_type), "::is_always_lock_free();
```"),
                #[inline]
                #[must_use]
                pub const fn is_always_lock_free() -> bool {
                    <imp::$atomic_type>::IS_ALWAYS_LOCK_FREE
                }
            }
            #[cfg(test)]
            const IS_ALWAYS_LOCK_FREE: bool = Self::is_always_lock_free();

            #[cfg(not(portable_atomic_no_const_mut_refs))]
            doc_comment! {
                concat!("Returns a mutable reference to the underlying integer.\n
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.

This is `const fn` on Rust 1.83+.

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let mut some_var = ", stringify!($atomic_type), "::new(10);
assert_eq!(*some_var.get_mut(), 10);
*some_var.get_mut() = 5;
assert_eq!(some_var.load(Ordering::SeqCst), 5);
```"),
                #[inline]
                pub const fn get_mut(&mut self) -> &mut $int_type {
                    // SAFETY: the mutable reference guarantees unique ownership.
                    // (core::sync::atomic::Atomic*::get_mut is not const yet)
                    unsafe { &mut *self.as_ptr() }
                }
            }
            #[cfg(portable_atomic_no_const_mut_refs)]
            doc_comment! {
                concat!("Returns a mutable reference to the underlying integer.\n
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.

This is `const fn` on Rust 1.83+.

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let mut some_var = ", stringify!($atomic_type), "::new(10);
assert_eq!(*some_var.get_mut(), 10);
*some_var.get_mut() = 5;
assert_eq!(some_var.load(Ordering::SeqCst), 5);
```"),
                #[inline]
                pub fn get_mut(&mut self) -> &mut $int_type {
                    // SAFETY: the mutable reference guarantees unique ownership.
                    unsafe { &mut *self.as_ptr() }
                }
            }

            // TODO: Add from_mut/get_mut_slice/from_mut_slice once it is stable on std atomic types.
            // https://github.com/rust-lang/rust/issues/76314

            #[cfg(not(portable_atomic_no_const_transmute))]
            doc_comment! {
                concat!("Consumes the atomic and returns the contained value.

This is safe because passing `self` by value guarantees that no other threads are
concurrently accessing the atomic data.

This is `const fn` on Rust 1.56+.

# Examples

```
use portable_atomic::", stringify!($atomic_type), ";

let some_var = ", stringify!($atomic_type), "::new(5);
assert_eq!(some_var.into_inner(), 5);
```"),
                #[inline]
                pub const fn into_inner(self) -> $int_type {
                    // SAFETY: $atomic_type and $int_type have the same size and in-memory representations,
                    // so they can be safely transmuted.
                    // (const UnsafeCell::into_inner is unstable)
                    unsafe { core::mem::transmute(self) }
                }
            }
            #[cfg(portable_atomic_no_const_transmute)]
            doc_comment! {
                concat!("Consumes the atomic and returns the contained value.

This is safe because passing `self` by value guarantees that no other threads are
concurrently accessing the atomic data.

This is `const fn` on Rust 1.56+.

# Examples

```
use portable_atomic::", stringify!($atomic_type), ";

let some_var = ", stringify!($atomic_type), "::new(5);
assert_eq!(some_var.into_inner(), 5);
```"),
                #[inline]
                pub fn into_inner(self) -> $int_type {
                    // SAFETY: $atomic_type and $int_type have the same size and in-memory representations,
                    // so they can be safely transmuted.
                    // (const UnsafeCell::into_inner is unstable)
                    unsafe { core::mem::transmute(self) }
                }
            }

            doc_comment! {
                concat!("Loads a value from the atomic integer.

`load` takes an [`Ordering`] argument which describes the memory ordering of this operation.
Possible values are [`SeqCst`], [`Acquire`] and [`Relaxed`].

# Panics

Panics if `order` is [`Release`] or [`AcqRel`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let some_var = ", stringify!($atomic_type), "::new(5);

assert_eq!(some_var.load(Ordering::Relaxed), 5);
```"),
                #[inline]
                #[cfg_attr(
                    any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                    track_caller
                )]
                pub fn load(&self, order: Ordering) -> $int_type {
                    self.inner.load(order)
                }
            }

            doc_comment! {
                concat!("Stores a value into the atomic integer.

`store` takes an [`Ordering`] argument which describes the memory ordering of this operation.
Possible values are [`SeqCst`], [`Release`] and [`Relaxed`].

# Panics

Panics if `order` is [`Acquire`] or [`AcqRel`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let some_var = ", stringify!($atomic_type), "::new(5);

some_var.store(10, Ordering::Relaxed);
assert_eq!(some_var.load(Ordering::Relaxed), 10);
```"),
                #[inline]
                #[cfg_attr(
                    any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                    track_caller
                )]
                pub fn store(&self, val: $int_type, order: Ordering) {
                    self.inner.store(val, order)
                }
            }

            cfg_has_atomic_cas_or_amo32! {
            $cfg_has_atomic_cas_or_amo32_or_8! {
            doc_comment! {
                concat!("Stores a value into the atomic integer, returning the previous value.

`swap` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let some_var = ", stringify!($atomic_type), "::new(5);

assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn swap(&self, val: $int_type, order: Ordering) -> $int_type {
                    self.inner.swap(val, order)
                }
            }
            } // $cfg_has_atomic_cas_or_amo32_or_8!

            cfg_has_atomic_cas! {
            doc_comment! {
                concat!("Stores a value into the atomic integer if the current value is the same as
the `current` value.

The return value is a result indicating whether the new value was written and
containing the previous value. On success this value is guaranteed to be equal to
`current`.

`compare_exchange` takes two [`Ordering`] arguments to describe the memory
ordering of this operation. `success` describes the required ordering for the
read-modify-write operation that takes place if the comparison with `current` succeeds.
`failure` describes the required ordering for the load operation that takes place when
the comparison fails. Using [`Acquire`] as success ordering makes the store part
of this operation [`Relaxed`], and using [`Release`] makes the successful load
[`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].

# Panics

Panics if `failure` is [`Release`], [`AcqRel`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let some_var = ", stringify!($atomic_type), "::new(5);

assert_eq!(
    some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),
    Ok(5),
);
assert_eq!(some_var.load(Ordering::Relaxed), 10);

assert_eq!(
    some_var.compare_exchange(6, 12, Ordering::SeqCst, Ordering::Acquire),
    Err(10),
);
assert_eq!(some_var.load(Ordering::Relaxed), 10);
```"),
                #[cfg_attr(docsrs, doc(alias = "compare_and_swap"))]
                #[inline]
                #[cfg_attr(
                    any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                    track_caller
                )]
                pub fn compare_exchange(
                    &self,
                    current: $int_type,
                    new: $int_type,
                    success: Ordering,
                    failure: Ordering,
                ) -> Result<$int_type, $int_type> {
                    self.inner.compare_exchange(current, new, success, failure)
                }
            }

            doc_comment! {
                concat!("Stores a value into the atomic integer if the current value is the same as
the `current` value.
Unlike [`compare_exchange`](Self::compare_exchange)
this function is allowed to spuriously fail even
when the comparison succeeds, which can result in more efficient code on some
platforms. The return value is a result indicating whether the new value was
written and containing the previous value.

`compare_exchange_weak` takes two [`Ordering`] arguments to describe the memory
ordering of this operation. `success` describes the required ordering for the
read-modify-write operation that takes place if the comparison with `current` succeeds.
`failure` describes the required ordering for the load operation that takes place when
the comparison fails. Using [`Acquire`] as success ordering makes the store part
of this operation [`Relaxed`], and using [`Release`] makes the successful load
[`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].

# Panics

Panics if `failure` is [`Release`], [`AcqRel`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let val = ", stringify!($atomic_type), "::new(4);

let mut old = val.load(Ordering::Relaxed);
loop {
    let new = old * 2;
    match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
        Ok(_) => break,
        Err(x) => old = x,
    }
}
```"),
                #[cfg_attr(docsrs, doc(alias = "compare_and_swap"))]
                #[inline]
                #[cfg_attr(
                    any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                    track_caller
                )]
                pub fn compare_exchange_weak(
                    &self,
                    current: $int_type,
                    new: $int_type,
                    success: Ordering,
                    failure: Ordering,
                ) -> Result<$int_type, $int_type> {
                    self.inner.compare_exchange_weak(current, new, success, failure)
                }
            }
            } // cfg_has_atomic_cas!

            $cfg_has_atomic_cas_or_amo32_or_8! {
            doc_comment! {
                concat!("Adds to the current value, returning the previous value.

This operation wraps around on overflow.

`fetch_add` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0);
assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
assert_eq!(foo.load(Ordering::SeqCst), 10);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn fetch_add(&self, val: $int_type, order: Ordering) -> $int_type {
                    self.inner.fetch_add(val, order)
                }
            }

            doc_comment! {
                concat!("Adds to the current value.

This operation wraps around on overflow.

Unlike `fetch_add`, this does not return the previous value.

`add` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_add` on some platforms.

- MSP430: `add` instead of disabling interrupts ({8,16}-bit atomics)

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0);
foo.add(10, Ordering::SeqCst);
assert_eq!(foo.load(Ordering::SeqCst), 10);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn add(&self, val: $int_type, order: Ordering) {
                    self.inner.add(val, order);
                }
            }

            doc_comment! {
                concat!("Subtracts from the current value, returning the previous value.

This operation wraps around on overflow.

`fetch_sub` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(20);
assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);
assert_eq!(foo.load(Ordering::SeqCst), 10);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn fetch_sub(&self, val: $int_type, order: Ordering) -> $int_type {
                    self.inner.fetch_sub(val, order)
                }
            }

            doc_comment! {
                concat!("Subtracts from the current value.

This operation wraps around on overflow.

Unlike `fetch_sub`, this does not return the previous value.

`sub` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_sub` on some platforms.

- MSP430: `sub` instead of disabling interrupts ({8,16}-bit atomics)

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(20);
foo.sub(10, Ordering::SeqCst);
assert_eq!(foo.load(Ordering::SeqCst), 10);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn sub(&self, val: $int_type, order: Ordering) {
                    self.inner.sub(val, order);
                }
            }
            } // $cfg_has_atomic_cas_or_amo32_or_8!

            doc_comment! {
                concat!("Bitwise \"and\" with the current value.

Performs a bitwise \"and\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_and` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b101101);
assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b100001);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn fetch_and(&self, val: $int_type, order: Ordering) -> $int_type {
                    self.inner.fetch_and(val, order)
                }
            }

            doc_comment! {
                concat!("Bitwise \"and\" with the current value.

Performs a bitwise \"and\" operation on the current value and the argument `val`, and
sets the new value to the result.

Unlike `fetch_and`, this does not return the previous value.

`and` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_and` on some platforms.

- x86/x86_64: `lock and` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)
- MSP430: `and` instead of disabling interrupts ({8,16}-bit atomics)

Note: On x86/x86_64, the use of either function should not usually
affect the generated code, because LLVM can properly optimize the case
where the result is unused.

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b101101);
assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b100001);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn and(&self, val: $int_type, order: Ordering) {
                    self.inner.and(val, order);
                }
            }

            cfg_has_atomic_cas! {
            doc_comment! {
                concat!("Bitwise \"nand\" with the current value.

Performs a bitwise \"nand\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_nand` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0x13);
assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);
assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn fetch_nand(&self, val: $int_type, order: Ordering) -> $int_type {
                    self.inner.fetch_nand(val, order)
                }
            }
            } // cfg_has_atomic_cas!

            doc_comment! {
                concat!("Bitwise \"or\" with the current value.

Performs a bitwise \"or\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_or` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b101101);
assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b111111);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn fetch_or(&self, val: $int_type, order: Ordering) -> $int_type {
                    self.inner.fetch_or(val, order)
                }
            }

            doc_comment! {
                concat!("Bitwise \"or\" with the current value.

Performs a bitwise \"or\" operation on the current value and the argument `val`, and
sets the new value to the result.

Unlike `fetch_or`, this does not return the previous value.

`or` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_or` on some platforms.

- x86/x86_64: `lock or` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)
- MSP430: `or` instead of disabling interrupts ({8,16}-bit atomics)

Note: On x86/x86_64, the use of either function should not usually
affect the generated code, because LLVM can properly optimize the case
where the result is unused.

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b101101);
assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b111111);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn or(&self, val: $int_type, order: Ordering) {
                    self.inner.or(val, order);
                }
            }

            doc_comment! {
                concat!("Bitwise \"xor\" with the current value.

Performs a bitwise \"xor\" operation on the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_xor` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b101101);
assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);
assert_eq!(foo.load(Ordering::SeqCst), 0b011110);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn fetch_xor(&self, val: $int_type, order: Ordering) -> $int_type {
                    self.inner.fetch_xor(val, order)
                }
            }

            doc_comment! {
                concat!("Bitwise \"xor\" with the current value.

Performs a bitwise \"xor\" operation on the current value and the argument `val`, and
sets the new value to the result.

Unlike `fetch_xor`, this does not return the previous value.

`xor` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_xor` on some platforms.

- x86/x86_64: `lock xor` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)
- MSP430: `xor` instead of disabling interrupts ({8,16}-bit atomics)

Note: On x86/x86_64, the use of either function should not usually
affect the generated code, because LLVM can properly optimize the case
where the result is unused.

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b101101);
foo.xor(0b110011, Ordering::SeqCst);
assert_eq!(foo.load(Ordering::SeqCst), 0b011110);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn xor(&self, val: $int_type, order: Ordering) {
                    self.inner.xor(val, order);
                }
            }

            cfg_has_atomic_cas! {
            doc_comment! {
                concat!("Fetches the value, and applies a function to it that returns an optional
new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else
`Err(previous_value)`.

Note: This may call the function multiple times if the value has been changed from other threads in
the meantime, as long as the function returns `Some(_)`, but the function will have been applied
only once to the stored value.

`fetch_update` takes two [`Ordering`] arguments to describe the memory ordering of this operation.
The first describes the required ordering for when the operation finally succeeds while the second
describes the required ordering for loads. These correspond to the success and failure orderings of
[`compare_exchange`](Self::compare_exchange) respectively.

Using [`Acquire`] as success ordering makes the store part
of this operation [`Relaxed`], and using [`Release`] makes the final successful load
[`Relaxed`]. The (failed) load ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].

# Panics

Panics if `fetch_order` is [`Release`], [`AcqRel`].

# Considerations

This method is not magic; it is not provided by the hardware.
It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),
and suffers from the same drawbacks.
In particular, this method will not circumvent the [ABA Problem].

[ABA Problem]: https://en.wikipedia.org/wiki/ABA_problem

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let x = ", stringify!($atomic_type), "::new(7);
assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));
assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));
assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));
assert_eq!(x.load(Ordering::SeqCst), 9);
```"),
                #[inline]
                #[cfg_attr(
                    any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                    track_caller
                )]
                pub fn fetch_update<F>(
                    &self,
                    set_order: Ordering,
                    fetch_order: Ordering,
                    mut f: F,
                ) -> Result<$int_type, $int_type>
                where
                    F: FnMut($int_type) -> Option<$int_type>,
                {
                    let mut prev = self.load(fetch_order);
                    while let Some(next) = f(prev) {
                        match self.compare_exchange_weak(prev, next, set_order, fetch_order) {
                            x @ Ok(_) => return x,
                            Err(next_prev) => prev = next_prev,
                        }
                    }
                    Err(prev)
                }
            }
            } // cfg_has_atomic_cas!

            $cfg_has_atomic_cas_or_amo32_or_8! {
            doc_comment! {
                concat!("Maximum with the current value.

Finds the maximum of the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_max` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(23);
assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);
assert_eq!(foo.load(Ordering::SeqCst), 42);
```

If you want to obtain the maximum value in one step, you can use the following:

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(23);
let bar = 42;
let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);
assert!(max_foo == 42);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn fetch_max(&self, val: $int_type, order: Ordering) -> $int_type {
                    self.inner.fetch_max(val, order)
                }
            }

            doc_comment! {
                concat!("Minimum with the current value.

Finds the minimum of the current value and the argument `val`, and
sets the new value to the result.

Returns the previous value.

`fetch_min` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(23);
assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);
assert_eq!(foo.load(Ordering::Relaxed), 23);
assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);
assert_eq!(foo.load(Ordering::Relaxed), 22);
```

If you want to obtain the minimum value in one step, you can use the following:

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(23);
let bar = 12;
let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);
assert_eq!(min_foo, 12);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn fetch_min(&self, val: $int_type, order: Ordering) -> $int_type {
                    self.inner.fetch_min(val, order)
                }
            }
            } // $cfg_has_atomic_cas_or_amo32_or_8!

            doc_comment! {
                concat!("Sets the bit at the specified bit-position to 1.

Returns `true` if the specified bit was previously set to 1.

`bit_set` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This corresponds to x86's `lock bts`, and the implementation calls them on x86/x86_64.

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b0000);
assert!(!foo.bit_set(0, Ordering::Relaxed));
assert_eq!(foo.load(Ordering::Relaxed), 0b0001);
assert!(foo.bit_set(0, Ordering::Relaxed));
assert_eq!(foo.load(Ordering::Relaxed), 0b0001);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn bit_set(&self, bit: u32, order: Ordering) -> bool {
                    self.inner.bit_set(bit, order)
                }
            }

            doc_comment! {
                concat!("Clears the bit at the specified bit-position to 1.

Returns `true` if the specified bit was previously set to 1.

`bit_clear` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This corresponds to x86's `lock btr`, and the implementation calls them on x86/x86_64.

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b0001);
assert!(foo.bit_clear(0, Ordering::Relaxed));
assert_eq!(foo.load(Ordering::Relaxed), 0b0000);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn bit_clear(&self, bit: u32, order: Ordering) -> bool {
                    self.inner.bit_clear(bit, order)
                }
            }

            doc_comment! {
                concat!("Toggles the bit at the specified bit-position.

Returns `true` if the specified bit was previously set to 1.

`bit_toggle` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This corresponds to x86's `lock btc`, and the implementation calls them on x86/x86_64.

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0b0000);
assert!(!foo.bit_toggle(0, Ordering::Relaxed));
assert_eq!(foo.load(Ordering::Relaxed), 0b0001);
assert!(foo.bit_toggle(0, Ordering::Relaxed));
assert_eq!(foo.load(Ordering::Relaxed), 0b0000);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn bit_toggle(&self, bit: u32, order: Ordering) -> bool {
                    self.inner.bit_toggle(bit, order)
                }
            }

            doc_comment! {
                concat!("Logical negates the current value, and sets the new value to the result.

Returns the previous value.

`fetch_not` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0);
assert_eq!(foo.fetch_not(Ordering::Relaxed), 0);
assert_eq!(foo.load(Ordering::Relaxed), !0);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn fetch_not(&self, order: Ordering) -> $int_type {
                    self.inner.fetch_not(order)
                }
            }

            doc_comment! {
                concat!("Logical negates the current value, and sets the new value to the result.

Unlike `fetch_not`, this does not return the previous value.

`not` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_not` on some platforms.

- x86/x86_64: `lock not` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)
- MSP430: `inv` instead of disabling interrupts ({8,16}-bit atomics)

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(0);
foo.not(Ordering::Relaxed);
assert_eq!(foo.load(Ordering::Relaxed), !0);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn not(&self, order: Ordering) {
                    self.inner.not(order);
                }
            }

            cfg_has_atomic_cas! {
            doc_comment! {
                concat!("Negates the current value, and sets the new value to the result.

Returns the previous value.

`fetch_neg` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(5);
assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5);
assert_eq!(foo.load(Ordering::Relaxed), 5_", stringify!($int_type), ".wrapping_neg());
assert_eq!(foo.fetch_neg(Ordering::Relaxed), 5_", stringify!($int_type), ".wrapping_neg());
assert_eq!(foo.load(Ordering::Relaxed), 5);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn fetch_neg(&self, order: Ordering) -> $int_type {
                    self.inner.fetch_neg(order)
                }
            }

            doc_comment! {
                concat!("Negates the current value, and sets the new value to the result.

Unlike `fetch_neg`, this does not return the previous value.

`neg` takes an [`Ordering`] argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
[`Acquire`] makes the store part of this operation [`Relaxed`], and
using [`Release`] makes the load part [`Relaxed`].

This function may generate more efficient code than `fetch_neg` on some platforms.

- x86/x86_64: `lock neg` instead of `cmpxchg` loop ({8,16,32}-bit atomics on x86, but additionally 64-bit atomics on x86_64)

# Examples

```
use portable_atomic::{", stringify!($atomic_type), ", Ordering};

let foo = ", stringify!($atomic_type), "::new(5);
foo.neg(Ordering::Relaxed);
assert_eq!(foo.load(Ordering::Relaxed), 5_", stringify!($int_type), ".wrapping_neg());
foo.neg(Ordering::Relaxed);
assert_eq!(foo.load(Ordering::Relaxed), 5);
```"),
                #[inline]
                #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                pub fn neg(&self, order: Ordering) {
                    self.inner.neg(order);
                }
            }
            } // cfg_has_atomic_cas!
            } // cfg_has_atomic_cas_or_amo32!

            const_fn! {
                const_if: #[cfg(not(portable_atomic_no_const_raw_ptr_deref))];
                /// Returns a mutable pointer to the underlying integer.
                ///
                /// Returning an `*mut` pointer from a shared reference to this atomic is
                /// safe because the atomic types work with interior mutability. Any use of
                /// the returned raw pointer requires an `unsafe` block and has to uphold
                /// the safety requirements. If there is concurrent access, note the following
                /// additional safety requirements:
                ///
                /// - If this atomic type is [lock-free](Self::is_lock_free), any concurrent
                ///   operations on it must be atomic.
                /// - Otherwise, any concurrent operations on it must be compatible with
                ///   operations performed by this atomic type.
                ///
                /// This is `const fn` on Rust 1.58+.
                #[inline]
                pub const fn as_ptr(&self) -> *mut $int_type {
                    self.inner.as_ptr()
                }
            }
        }
        // See https://github.com/taiki-e/portable-atomic/issues/180
        #[cfg(not(feature = "require-cas"))]
        cfg_no_atomic_cas! {
        #[doc(hidden)]
        #[allow(unused_variables, clippy::unused_self, clippy::extra_unused_lifetimes)]
        impl<'a> $atomic_type {
            $cfg_no_atomic_cas_or_amo32_or_8! {
            #[inline]
            pub fn swap(&self, val: $int_type, order: Ordering) -> $int_type
            where
                &'a Self: HasSwap,
            {
                unimplemented!()
            }
            } // $cfg_no_atomic_cas_or_amo32_or_8!
            #[inline]
            pub fn compare_exchange(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type>
            where
                &'a Self: HasCompareExchange,
            {
                unimplemented!()
            }
            #[inline]
            pub fn compare_exchange_weak(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type>
            where
                &'a Self: HasCompareExchangeWeak,
            {
                unimplemented!()
            }
            $cfg_no_atomic_cas_or_amo32_or_8! {
            #[inline]
            pub fn fetch_add(&self, val: $int_type, order: Ordering) -> $int_type
            where
                &'a Self: HasFetchAdd,
            {
                unimplemented!()
            }
            #[inline]
            pub fn add(&self, val: $int_type, order: Ordering)
            where
                &'a Self: HasAdd,
            {
                unimplemented!()
            }
            #[inline]
            pub fn fetch_sub(&self, val: $int_type, order: Ordering) -> $int_type
            where
                &'a Self: HasFetchSub,
            {
                unimplemented!()
            }
            #[inline]
            pub fn sub(&self, val: $int_type, order: Ordering)
            where
                &'a Self: HasSub,
            {
                unimplemented!()
            }
            } // $cfg_no_atomic_cas_or_amo32_or_8!
            cfg_no_atomic_cas_or_amo32! {
            #[inline]
            pub fn fetch_and(&self, val: $int_type, order: Ordering) -> $int_type
            where
                &'a Self: HasFetchAnd,
            {
                unimplemented!()
            }
            #[inline]
            pub fn and(&self, val: $int_type, order: Ordering)
            where
                &'a Self: HasAnd,
            {
                unimplemented!()
            }
            } // cfg_no_atomic_cas_or_amo32!
            #[inline]
            pub fn fetch_nand(&self, val: $int_type, order: Ordering) -> $int_type
            where
                &'a Self: HasFetchNand,
            {
                unimplemented!()
            }
            cfg_no_atomic_cas_or_amo32! {
            #[inline]
            pub fn fetch_or(&self, val: $int_type, order: Ordering) -> $int_type
            where
                &'a Self: HasFetchOr,
            {
                unimplemented!()
            }
            #[inline]
            pub fn or(&self, val: $int_type, order: Ordering)
            where
                &'a Self: HasOr,
            {
                unimplemented!()
            }
            #[inline]
            pub fn fetch_xor(&self, val: $int_type, order: Ordering) -> $int_type
            where
                &'a Self: HasFetchXor,
            {
                unimplemented!()
            }
            #[inline]
            pub fn xor(&self, val: $int_type, order: Ordering)
            where
                &'a Self: HasXor,
            {
                unimplemented!()
            }
            } // cfg_no_atomic_cas_or_amo32!
            #[inline]
            pub fn fetch_update<F>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                f: F,
            ) -> Result<$int_type, $int_type>
            where
                F: FnMut($int_type) -> Option<$int_type>,
                &'a Self: HasFetchUpdate,
            {
                unimplemented!()
            }
            $cfg_no_atomic_cas_or_amo32_or_8! {
            #[inline]
            pub fn fetch_max(&self, val: $int_type, order: Ordering) -> $int_type
            where
                &'a Self: HasFetchMax,
            {
                unimplemented!()
            }
            #[inline]
            pub fn fetch_min(&self, val: $int_type, order: Ordering) -> $int_type
            where
                &'a Self: HasFetchMin,
            {
                unimplemented!()
            }
            } // $cfg_no_atomic_cas_or_amo32_or_8!
            cfg_no_atomic_cas_or_amo32! {
            #[inline]
            pub fn bit_set(&self, bit: u32, order: Ordering) -> bool
            where
                &'a Self: HasBitSet,
            {
                unimplemented!()
            }
            #[inline]
            pub fn bit_clear(&self, bit: u32, order: Ordering) -> bool
            where
                &'a Self: HasBitClear,
            {
                unimplemented!()
            }
            #[inline]
            pub fn bit_toggle(&self, bit: u32, order: Ordering) -> bool
            where
                &'a Self: HasBitToggle,
            {
                unimplemented!()
            }
            #[inline]
            pub fn fetch_not(&self, order: Ordering) -> $int_type
            where
                &'a Self: HasFetchNot,
            {
                unimplemented!()
            }
            #[inline]
            pub fn not(&self, order: Ordering)
            where
                &'a Self: HasNot,
            {
                unimplemented!()
            }
            } // cfg_no_atomic_cas_or_amo32!
            #[inline]
            pub fn fetch_neg(&self, order: Ordering) -> $int_type
            where
                &'a Self: HasFetchNeg,
            {
                unimplemented!()
            }
            #[inline]
            pub fn neg(&self, order: Ordering)
            where
                &'a Self: HasNeg,
            {
                unimplemented!()
            }
        }
        } // cfg_no_atomic_cas!
        $(
            #[$cfg_float]
            atomic_int!(float,
                #[$cfg_float] $atomic_float_type, $float_type, $atomic_type, $int_type, $align
            );
        )?
    };

    // AtomicF* impls
    (float,
        #[$cfg_float:meta]
        $atomic_type:ident,
        $float_type:ident,
        $atomic_int_type:ident,
        $int_type:ident,
        $align:literal
    ) => {
        doc_comment! {
            concat!("A floating point type which can be safely shared between threads.

This type has the same in-memory representation as the underlying floating point type,
[`", stringify!($float_type), "`].
"
            ),
            #[cfg_attr(docsrs, doc($cfg_float))]
            // We can use #[repr(transparent)] here, but #[repr(C, align(N))]
            // will show clearer docs.
            #[repr(C, align($align))]
            pub struct $atomic_type {
                inner: imp::float::$atomic_type,
            }
        }

        impl Default for $atomic_type {
            #[inline]
            fn default() -> Self {
                Self::new($float_type::default())
            }
        }

        impl From<$float_type> for $atomic_type {
            #[inline]
            fn from(v: $float_type) -> Self {
                Self::new(v)
            }
        }

        // UnwindSafe is implicitly implemented.
        #[cfg(not(portable_atomic_no_core_unwind_safe))]
        impl core::panic::RefUnwindSafe for $atomic_type {}
        #[cfg(all(portable_atomic_no_core_unwind_safe, feature = "std"))]
        impl std::panic::RefUnwindSafe for $atomic_type {}

        impl_debug_and_serde!($atomic_type);

        impl $atomic_type {
            /// Creates a new atomic float.
            #[inline]
            #[must_use]
            pub const fn new(v: $float_type) -> Self {
                static_assert_layout!($atomic_type, $float_type);
                Self { inner: imp::float::$atomic_type::new(v) }
            }

            // TODO: update docs based on https://github.com/rust-lang/rust/pull/116762
            #[cfg(not(portable_atomic_no_const_mut_refs))]
            doc_comment! {
                concat!("Creates a new reference to an atomic float from a pointer.

This is `const fn` on Rust 1.83+.

# Safety

* `ptr` must be aligned to `align_of::<", stringify!($atomic_type), ">()` (note that on some platforms this
  can be bigger than `align_of::<", stringify!($float_type), ">()`).
* `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`.
* If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value
  behind `ptr` must have a happens-before relationship with atomic accesses via
  the returned value (or vice-versa).
  * In other words, time periods where the value is accessed atomically may not
    overlap with periods where the value is accessed non-atomically.
  * This requirement is trivially satisfied if `ptr` is never used non-atomically
    for the duration of lifetime `'a`. Most use cases should be able to follow
    this guideline.
  * This requirement is also trivially satisfied if all accesses (atomic or not) are
    done from the same thread.
* If this atomic type is *not* lock-free:
  * Any accesses to the value behind `ptr` must have a happens-before relationship
    with accesses via the returned value (or vice-versa).
  * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must
    be compatible with operations performed by this atomic type.
* This method must not be used to create overlapping or mixed-size atomic
  accesses, as these are not supported by the memory model.

[valid]: core::ptr#safety"),
                #[inline]
                #[must_use]
                pub const unsafe fn from_ptr<'a>(ptr: *mut $float_type) -> &'a Self {
                    #[allow(clippy::cast_ptr_alignment)]
                    // SAFETY: guaranteed by the caller
                    unsafe { &*(ptr as *mut Self) }
                }
            }
            #[cfg(portable_atomic_no_const_mut_refs)]
            doc_comment! {
                concat!("Creates a new reference to an atomic float from a pointer.

This is `const fn` on Rust 1.83+.

# Safety

* `ptr` must be aligned to `align_of::<", stringify!($atomic_type), ">()` (note that on some platforms this
  can be bigger than `align_of::<", stringify!($float_type), ">()`).
* `ptr` must be [valid] for both reads and writes for the whole lifetime `'a`.
* If this atomic type is [lock-free](Self::is_lock_free), non-atomic accesses to the value
  behind `ptr` must have a happens-before relationship with atomic accesses via
  the returned value (or vice-versa).
  * In other words, time periods where the value is accessed atomically may not
    overlap with periods where the value is accessed non-atomically.
  * This requirement is trivially satisfied if `ptr` is never used non-atomically
    for the duration of lifetime `'a`. Most use cases should be able to follow
    this guideline.
  * This requirement is also trivially satisfied if all accesses (atomic or not) are
    done from the same thread.
* If this atomic type is *not* lock-free:
  * Any accesses to the value behind `ptr` must have a happens-before relationship
    with accesses via the returned value (or vice-versa).
  * Any concurrent accesses to the value behind `ptr` for the duration of lifetime `'a` must
    be compatible with operations performed by this atomic type.
* This method must not be used to create overlapping or mixed-size atomic
  accesses, as these are not supported by the memory model.

[valid]: core::ptr#safety"),
                #[inline]
                #[must_use]
                pub unsafe fn from_ptr<'a>(ptr: *mut $float_type) -> &'a Self {
                    #[allow(clippy::cast_ptr_alignment)]
                    // SAFETY: guaranteed by the caller
                    unsafe { &*(ptr as *mut Self) }
                }
            }

            /// Returns `true` if operations on values of this type are lock-free.
            ///
            /// If the compiler or the platform doesn't support the necessary
            /// atomic instructions, global locks for every potentially
            /// concurrent atomic operation will be used.
            #[inline]
            #[must_use]
            pub fn is_lock_free() -> bool {
                <imp::float::$atomic_type>::is_lock_free()
            }

            /// Returns `true` if operations on values of this type are lock-free.
            ///
            /// If the compiler or the platform doesn't support the necessary
            /// atomic instructions, global locks for every potentially
            /// concurrent atomic operation will be used.
            ///
            /// **Note:** If the atomic operation relies on dynamic CPU feature detection,
            /// this type may be lock-free even if the function returns false.
            #[inline]
            #[must_use]
            pub const fn is_always_lock_free() -> bool {
                <imp::float::$atomic_type>::IS_ALWAYS_LOCK_FREE
            }
            #[cfg(test)]
            const IS_ALWAYS_LOCK_FREE: bool = Self::is_always_lock_free();

            const_fn! {
                const_if: #[cfg(not(portable_atomic_no_const_mut_refs))];
                /// Returns a mutable reference to the underlying float.
                ///
                /// This is safe because the mutable reference guarantees that no other threads are
                /// concurrently accessing the atomic data.
                ///
                /// This is `const fn` on Rust 1.83+.
                #[inline]
                pub const fn get_mut(&mut self) -> &mut $float_type {
                    // SAFETY: the mutable reference guarantees unique ownership.
                    unsafe { &mut *self.as_ptr() }
                }
            }

            // TODO: Add from_mut/get_mut_slice/from_mut_slice once it is stable on std atomic types.
            // https://github.com/rust-lang/rust/issues/76314

            const_fn! {
                const_if: #[cfg(not(portable_atomic_no_const_transmute))];
                /// Consumes the atomic and returns the contained value.
                ///
                /// This is safe because passing `self` by value guarantees that no other threads are
                /// concurrently accessing the atomic data.
                ///
                /// This is `const fn` on Rust 1.56+.
                #[inline]
                pub const fn into_inner(self) -> $float_type {
                    // SAFETY: $atomic_type and $float_type have the same size and in-memory representations,
                    // so they can be safely transmuted.
                    // (const UnsafeCell::into_inner is unstable)
                    unsafe { core::mem::transmute(self) }
                }
            }

            /// Loads a value from the atomic float.
            ///
            /// `load` takes an [`Ordering`] argument which describes the memory ordering of this operation.
            /// Possible values are [`SeqCst`], [`Acquire`] and [`Relaxed`].
            ///
            /// # Panics
            ///
            /// Panics if `order` is [`Release`] or [`AcqRel`].
            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub fn load(&self, order: Ordering) -> $float_type {
                self.inner.load(order)
            }

            /// Stores a value into the atomic float.
            ///
            /// `store` takes an [`Ordering`] argument which describes the memory ordering of this operation.
            ///  Possible values are [`SeqCst`], [`Release`] and [`Relaxed`].
            ///
            /// # Panics
            ///
            /// Panics if `order` is [`Acquire`] or [`AcqRel`].
            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub fn store(&self, val: $float_type, order: Ordering) {
                self.inner.store(val, order)
            }

            cfg_has_atomic_cas_or_amo32! {
            /// Stores a value into the atomic float, returning the previous value.
            ///
            /// `swap` takes an [`Ordering`] argument which describes the memory ordering
            /// of this operation. All ordering modes are possible. Note that using
            /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
            /// using [`Release`] makes the load part [`Relaxed`].
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub fn swap(&self, val: $float_type, order: Ordering) -> $float_type {
                self.inner.swap(val, order)
            }

            cfg_has_atomic_cas! {
            /// Stores a value into the atomic float if the current value is the same as
            /// the `current` value.
            ///
            /// The return value is a result indicating whether the new value was written and
            /// containing the previous value. On success this value is guaranteed to be equal to
            /// `current`.
            ///
            /// `compare_exchange` takes two [`Ordering`] arguments to describe the memory
            /// ordering of this operation. `success` describes the required ordering for the
            /// read-modify-write operation that takes place if the comparison with `current` succeeds.
            /// `failure` describes the required ordering for the load operation that takes place when
            /// the comparison fails. Using [`Acquire`] as success ordering makes the store part
            /// of this operation [`Relaxed`], and using [`Release`] makes the successful load
            /// [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
            ///
            /// # Panics
            ///
            /// Panics if `failure` is [`Release`], [`AcqRel`].
            #[cfg_attr(docsrs, doc(alias = "compare_and_swap"))]
            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub fn compare_exchange(
                &self,
                current: $float_type,
                new: $float_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$float_type, $float_type> {
                self.inner.compare_exchange(current, new, success, failure)
            }

            /// Stores a value into the atomic float if the current value is the same as
            /// the `current` value.
            /// Unlike [`compare_exchange`](Self::compare_exchange)
            /// this function is allowed to spuriously fail even
            /// when the comparison succeeds, which can result in more efficient code on some
            /// platforms. The return value is a result indicating whether the new value was
            /// written and containing the previous value.
            ///
            /// `compare_exchange_weak` takes two [`Ordering`] arguments to describe the memory
            /// ordering of this operation. `success` describes the required ordering for the
            /// read-modify-write operation that takes place if the comparison with `current` succeeds.
            /// `failure` describes the required ordering for the load operation that takes place when
            /// the comparison fails. Using [`Acquire`] as success ordering makes the store part
            /// of this operation [`Relaxed`], and using [`Release`] makes the successful load
            /// [`Relaxed`]. The failure ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
            ///
            /// # Panics
            ///
            /// Panics if `failure` is [`Release`], [`AcqRel`].
            #[cfg_attr(docsrs, doc(alias = "compare_and_swap"))]
            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub fn compare_exchange_weak(
                &self,
                current: $float_type,
                new: $float_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$float_type, $float_type> {
                self.inner.compare_exchange_weak(current, new, success, failure)
            }

            /// Adds to the current value, returning the previous value.
            ///
            /// This operation wraps around on overflow.
            ///
            /// `fetch_add` takes an [`Ordering`] argument which describes the memory ordering
            /// of this operation. All ordering modes are possible. Note that using
            /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
            /// using [`Release`] makes the load part [`Relaxed`].
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub fn fetch_add(&self, val: $float_type, order: Ordering) -> $float_type {
                self.inner.fetch_add(val, order)
            }

            /// Subtracts from the current value, returning the previous value.
            ///
            /// This operation wraps around on overflow.
            ///
            /// `fetch_sub` takes an [`Ordering`] argument which describes the memory ordering
            /// of this operation. All ordering modes are possible. Note that using
            /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
            /// using [`Release`] makes the load part [`Relaxed`].
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub fn fetch_sub(&self, val: $float_type, order: Ordering) -> $float_type {
                self.inner.fetch_sub(val, order)
            }

            /// Fetches the value, and applies a function to it that returns an optional
            /// new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else
            /// `Err(previous_value)`.
            ///
            /// Note: This may call the function multiple times if the value has been changed from other threads in
            /// the meantime, as long as the function returns `Some(_)`, but the function will have been applied
            /// only once to the stored value.
            ///
            /// `fetch_update` takes two [`Ordering`] arguments to describe the memory ordering of this operation.
            /// The first describes the required ordering for when the operation finally succeeds while the second
            /// describes the required ordering for loads. These correspond to the success and failure orderings of
            /// [`compare_exchange`](Self::compare_exchange) respectively.
            ///
            /// Using [`Acquire`] as success ordering makes the store part
            /// of this operation [`Relaxed`], and using [`Release`] makes the final successful load
            /// [`Relaxed`]. The (failed) load ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`].
            ///
            /// # Panics
            ///
            /// Panics if `fetch_order` is [`Release`], [`AcqRel`].
            ///
            /// # Considerations
            ///
            /// This method is not magic; it is not provided by the hardware.
            /// It is implemented in terms of [`compare_exchange_weak`](Self::compare_exchange_weak),
            /// and suffers from the same drawbacks.
            /// In particular, this method will not circumvent the [ABA Problem].
            ///
            /// [ABA Problem]: https://en.wikipedia.org/wiki/ABA_problem
            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub fn fetch_update<F>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                mut f: F,
            ) -> Result<$float_type, $float_type>
            where
                F: FnMut($float_type) -> Option<$float_type>,
            {
                let mut prev = self.load(fetch_order);
                while let Some(next) = f(prev) {
                    match self.compare_exchange_weak(prev, next, set_order, fetch_order) {
                        x @ Ok(_) => return x,
                        Err(next_prev) => prev = next_prev,
                    }
                }
                Err(prev)
            }

            /// Maximum with the current value.
            ///
            /// Finds the maximum of the current value and the argument `val`, and
            /// sets the new value to the result.
            ///
            /// Returns the previous value.
            ///
            /// `fetch_max` takes an [`Ordering`] argument which describes the memory ordering
            /// of this operation. All ordering modes are possible. Note that using
            /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
            /// using [`Release`] makes the load part [`Relaxed`].
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub fn fetch_max(&self, val: $float_type, order: Ordering) -> $float_type {
                self.inner.fetch_max(val, order)
            }

            /// Minimum with the current value.
            ///
            /// Finds the minimum of the current value and the argument `val`, and
            /// sets the new value to the result.
            ///
            /// Returns the previous value.
            ///
            /// `fetch_min` takes an [`Ordering`] argument which describes the memory ordering
            /// of this operation. All ordering modes are possible. Note that using
            /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
            /// using [`Release`] makes the load part [`Relaxed`].
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub fn fetch_min(&self, val: $float_type, order: Ordering) -> $float_type {
                self.inner.fetch_min(val, order)
            }
            } // cfg_has_atomic_cas!

            /// Negates the current value, and sets the new value to the result.
            ///
            /// Returns the previous value.
            ///
            /// `fetch_neg` takes an [`Ordering`] argument which describes the memory ordering
            /// of this operation. All ordering modes are possible. Note that using
            /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
            /// using [`Release`] makes the load part [`Relaxed`].
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub fn fetch_neg(&self, order: Ordering) -> $float_type {
                self.inner.fetch_neg(order)
            }

            /// Computes the absolute value of the current value, and sets the
            /// new value to the result.
            ///
            /// Returns the previous value.
            ///
            /// `fetch_abs` takes an [`Ordering`] argument which describes the memory ordering
            /// of this operation. All ordering modes are possible. Note that using
            /// [`Acquire`] makes the store part of this operation [`Relaxed`], and
            /// using [`Release`] makes the load part [`Relaxed`].
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub fn fetch_abs(&self, order: Ordering) -> $float_type {
                self.inner.fetch_abs(order)
            }
            } // cfg_has_atomic_cas_or_amo32!

            #[cfg(not(portable_atomic_no_const_raw_ptr_deref))]
            doc_comment! {
                concat!("Raw transmutation to `&", stringify!($atomic_int_type), "`.

See [`", stringify!($float_type) ,"::from_bits`] for some discussion of the
portability of this operation (there are almost no issues).

This is `const fn` on Rust 1.58+."),
                #[inline]
                pub const fn as_bits(&self) -> &$atomic_int_type {
                    self.inner.as_bits()
                }
            }
            #[cfg(portable_atomic_no_const_raw_ptr_deref)]
            doc_comment! {
                concat!("Raw transmutation to `&", stringify!($atomic_int_type), "`.

See [`", stringify!($float_type) ,"::from_bits`] for some discussion of the
portability of this operation (there are almost no issues).

This is `const fn` on Rust 1.58+."),
                #[inline]
                pub fn as_bits(&self) -> &$atomic_int_type {
                    self.inner.as_bits()
                }
            }

            const_fn! {
                const_if: #[cfg(not(portable_atomic_no_const_raw_ptr_deref))];
                /// Returns a mutable pointer to the underlying float.
                ///
                /// Returning an `*mut` pointer from a shared reference to this atomic is
                /// safe because the atomic types work with interior mutability. Any use of
                /// the returned raw pointer requires an `unsafe` block and has to uphold
                /// the safety requirements. If there is concurrent access, note the following
                /// additional safety requirements:
                ///
                /// - If this atomic type is [lock-free](Self::is_lock_free), any concurrent
                ///   operations on it must be atomic.
                /// - Otherwise, any concurrent operations on it must be compatible with
                ///   operations performed by this atomic type.
                ///
                /// This is `const fn` on Rust 1.58+.
                #[inline]
                pub const fn as_ptr(&self) -> *mut $float_type {
                    self.inner.as_ptr()
                }
            }
        }
        // See https://github.com/taiki-e/portable-atomic/issues/180
        #[cfg(not(feature = "require-cas"))]
        cfg_no_atomic_cas! {
        #[doc(hidden)]
        #[allow(unused_variables, clippy::unused_self, clippy::extra_unused_lifetimes)]
        impl<'a> $atomic_type {
            cfg_no_atomic_cas_or_amo32! {
            #[inline]
            pub fn swap(&self, val: $float_type, order: Ordering) -> $float_type
            where
                &'a Self: HasSwap,
            {
                unimplemented!()
            }
            } // cfg_no_atomic_cas_or_amo32!
            #[inline]
            pub fn compare_exchange(
                &self,
                current: $float_type,
                new: $float_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$float_type, $float_type>
            where
                &'a Self: HasCompareExchange,
            {
                unimplemented!()
            }
            #[inline]
            pub fn compare_exchange_weak(
                &self,
                current: $float_type,
                new: $float_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$float_type, $float_type>
            where
                &'a Self: HasCompareExchangeWeak,
            {
                unimplemented!()
            }
            #[inline]
            pub fn fetch_add(&self, val: $float_type, order: Ordering) -> $float_type
            where
                &'a Self: HasFetchAdd,
            {
                unimplemented!()
            }
            #[inline]
            pub fn fetch_sub(&self, val: $float_type, order: Ordering) -> $float_type
            where
                &'a Self: HasFetchSub,
            {
                unimplemented!()
            }
            #[inline]
            pub fn fetch_update<F>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                f: F,
            ) -> Result<$float_type, $float_type>
            where
                F: FnMut($float_type) -> Option<$float_type>,
                &'a Self: HasFetchUpdate,
            {
                unimplemented!()
            }
            #[inline]
            pub fn fetch_max(&self, val: $float_type, order: Ordering) -> $float_type
            where
                &'a Self: HasFetchMax,
            {
                unimplemented!()
            }
            #[inline]
            pub fn fetch_min(&self, val: $float_type, order: Ordering) -> $float_type
            where
                &'a Self: HasFetchMin,
            {
                unimplemented!()
            }
            cfg_no_atomic_cas_or_amo32! {
            #[inline]
            pub fn fetch_neg(&self, order: Ordering) -> $float_type
            where
                &'a Self: HasFetchNeg,
            {
                unimplemented!()
            }
            #[inline]
            pub fn fetch_abs(&self, order: Ordering) -> $float_type
            where
                &'a Self: HasFetchAbs,
            {
                unimplemented!()
            }
            } // cfg_no_atomic_cas_or_amo32!
        }
        } // cfg_no_atomic_cas!
    };
}

cfg_has_atomic_ptr! {
    #[cfg(target_pointer_width = "16")]
    atomic_int!(AtomicIsize, isize, 2, cfg_has_atomic_cas_or_amo8, cfg_no_atomic_cas_or_amo8);
    #[cfg(target_pointer_width = "16")]
    atomic_int!(AtomicUsize, usize, 2, cfg_has_atomic_cas_or_amo8, cfg_no_atomic_cas_or_amo8);
    #[cfg(target_pointer_width = "32")]
    atomic_int!(AtomicIsize, isize, 4, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32);
    #[cfg(target_pointer_width = "32")]
    atomic_int!(AtomicUsize, usize, 4, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32);
    #[cfg(target_pointer_width = "64")]
    atomic_int!(AtomicIsize, isize, 8, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32);
    #[cfg(target_pointer_width = "64")]
    atomic_int!(AtomicUsize, usize, 8, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32);
    #[cfg(target_pointer_width = "128")]
    atomic_int!(AtomicIsize, isize, 16, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32);
    #[cfg(target_pointer_width = "128")]
    atomic_int!(AtomicUsize, usize, 16, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32);
}

cfg_has_atomic_8! {
    atomic_int!(AtomicI8, i8, 1, cfg_has_atomic_cas_or_amo8, cfg_no_atomic_cas_or_amo8);
    atomic_int!(AtomicU8, u8, 1, cfg_has_atomic_cas_or_amo8, cfg_no_atomic_cas_or_amo8);
}
cfg_has_atomic_16! {
    atomic_int!(AtomicI16, i16, 2, cfg_has_atomic_cas_or_amo8, cfg_no_atomic_cas_or_amo8);
    atomic_int!(AtomicU16, u16, 2, cfg_has_atomic_cas_or_amo8, cfg_no_atomic_cas_or_amo8,
        #[cfg(all(feature = "float", portable_atomic_unstable_f16))] AtomicF16, f16);
}
cfg_has_atomic_32! {
    atomic_int!(AtomicI32, i32, 4, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32);
    atomic_int!(AtomicU32, u32, 4, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32,
        #[cfg(feature = "float")] AtomicF32, f32);
}
cfg_has_atomic_64! {
    atomic_int!(AtomicI64, i64, 8, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32);
    atomic_int!(AtomicU64, u64, 8, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32,
        #[cfg(feature = "float")] AtomicF64, f64);
}
cfg_has_atomic_128! {
    atomic_int!(AtomicI128, i128, 16, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32);
    atomic_int!(AtomicU128, u128, 16, cfg_has_atomic_cas_or_amo32, cfg_no_atomic_cas_or_amo32,
        #[cfg(all(feature = "float", portable_atomic_unstable_f128))] AtomicF128, f128);
}

// See https://github.com/taiki-e/portable-atomic/issues/180
#[cfg(not(feature = "require-cas"))]
cfg_no_atomic_cas! {
cfg_no_atomic_cas_or_amo32! {
#[cfg(feature = "float")]
use self::diagnostic_helper::HasFetchAbs;
use self::diagnostic_helper::{
    HasAnd, HasBitClear, HasBitSet, HasBitToggle, HasFetchAnd, HasFetchByteAdd, HasFetchByteSub,
    HasFetchNot, HasFetchOr, HasFetchPtrAdd, HasFetchPtrSub, HasFetchXor, HasNot, HasOr, HasXor,
};
} // cfg_no_atomic_cas_or_amo32!
cfg_no_atomic_cas_or_amo8! {
use self::diagnostic_helper::{HasAdd, HasSub, HasSwap};
} // cfg_no_atomic_cas_or_amo8!
#[cfg_attr(not(feature = "float"), allow(unused_imports))]
use self::diagnostic_helper::{
    HasCompareExchange, HasCompareExchangeWeak, HasFetchAdd, HasFetchMax, HasFetchMin,
    HasFetchNand, HasFetchNeg, HasFetchSub, HasFetchUpdate, HasNeg,
};
#[cfg_attr(
    any(
        all(
            portable_atomic_no_atomic_load_store,
            not(any(
                target_arch = "avr",
                target_arch = "bpf",
                target_arch = "msp430",
                target_arch = "riscv32",
                target_arch = "riscv64",
                feature = "critical-section",
            )),
        ),
        not(feature = "float"),
    ),
    allow(dead_code, unreachable_pub)
)]
#[allow(unknown_lints, unnameable_types)] // Not public API. unnameable_types is available on Rust 1.79+
mod diagnostic_helper {
    cfg_no_atomic_cas_or_amo8! {
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`swap` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasSwap {}
    } // cfg_no_atomic_cas_or_amo8!
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`compare_exchange` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasCompareExchange {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`compare_exchange_weak` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasCompareExchangeWeak {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_add` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchAdd {}
    cfg_no_atomic_cas_or_amo8! {
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`add` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasAdd {}
    } // cfg_no_atomic_cas_or_amo8!
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_sub` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchSub {}
    cfg_no_atomic_cas_or_amo8! {
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`sub` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasSub {}
    } // cfg_no_atomic_cas_or_amo8!
    cfg_no_atomic_cas_or_amo32! {
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_ptr_add` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchPtrAdd {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_ptr_sub` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchPtrSub {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_byte_add` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchByteAdd {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_byte_sub` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchByteSub {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_and` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchAnd {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`and` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasAnd {}
    } // cfg_no_atomic_cas_or_amo32!
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_nand` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchNand {}
    cfg_no_atomic_cas_or_amo32! {
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_or` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchOr {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`or` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasOr {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_xor` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchXor {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`xor` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasXor {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_not` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchNot {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`not` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasNot {}
    } // cfg_no_atomic_cas_or_amo32!
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_neg` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchNeg {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`neg` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasNeg {}
    cfg_no_atomic_cas_or_amo32! {
    #[cfg(feature = "float")]
    #[cfg_attr(target_pointer_width = "16", allow(dead_code, unreachable_pub))]
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_abs` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchAbs {}
    } // cfg_no_atomic_cas_or_amo32!
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_min` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchMin {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_max` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchMax {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`fetch_update` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasFetchUpdate {}
    cfg_no_atomic_cas_or_amo32! {
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`bit_set` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasBitSet {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`bit_clear` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasBitClear {}
    #[doc(hidden)]
    #[cfg_attr(
        not(portable_atomic_no_diagnostic_namespace),
        diagnostic::on_unimplemented(
            message = "`bit_toggle` requires atomic CAS but not available on this target by default",
            label = "this associated function is not available on this target by default",
            note = "consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features",
            note = "see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more."
        )
    )]
    pub trait HasBitToggle {}
    } // cfg_no_atomic_cas_or_amo32!
}
} // cfg_no_atomic_cas!
