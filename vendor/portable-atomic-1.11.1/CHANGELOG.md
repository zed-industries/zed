# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

Releases may yanked if there is a security bug, a soundness bug, or a regression.

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [1.11.1] - 2025-06-06

- Fix build error when building non-x86 targets for Miri or ThreadSanitizer since nightly-2025-05-31.

- aarch64: Optimize atomic floats when FEAT_LSFE is enabled. ([#201](https://github.com/taiki-e/portable-atomic/pull/201))

- Improve compile-time detection of RISC-V Zacas extension. ([b7634e2](https://github.com/taiki-e/portable-atomic/commit/b7634e2cd808ea118266d12f99fd8877a92e3d31))

- Improve run-time detection on linux-musl. ([7fdad7f](https://github.com/taiki-e/portable-atomic/commit/7fdad7f7dd32e32ece7bd0eaf565db657b3406bb))

## [1.11.0] - 2025-02-24

- Work around [nightly-2025-02-24 rustc regression causing "cannot use value of type `*mut T` for inline assembly" error](https://github.com/rust-lang/rust/issues/137512) on RISC-V without A extension, MSP430, and pre-v6 no-std Arm targets. ([eeb0235](https://github.com/taiki-e/portable-atomic/commit/eeb0235b9fda4c28a56ee5a9ffe0d7fb884a50ab))

- Support `AtomicF16` and `AtomicF128` for [unstable `f16` and `f128`](https://github.com/rust-lang/rust/issues/116909) under unstable cfgs. ([#200](https://github.com/taiki-e/portable-atomic/pull/200))

- RISC-V Zacas extension support is no longer experimental. ([#206](https://github.com/taiki-e/portable-atomic/pull/206))

- Improve support of run-time detection and outline-atomics:
  - riscv: Enable run-time detection of Zacas extension by default on Linux/Android. ([#207](https://github.com/taiki-e/portable-atomic/pull/207))
  - aarch64: Support run-time detection of FEAT_LRCPC3/FEAT_LSE128 on FreeBSD. ([6a5075d](https://github.com/taiki-e/portable-atomic/commit/6a5075d43543875cf38d6114f2951047e2e64f1a))
  - powerpc64: Support run-time detection of quadword-atomics on AIX (currently disabled by default because detection support for AIX is experimental). ([#102](https://github.com/taiki-e/portable-atomic/pull/102))

## [1.10.0] - 2024-11-23

- Update to stabilized [s390x](https://github.com/rust-lang/rust/pull/131258) and [Arm64EC](https://github.com/rust-lang/rust/pull/131781) inline assembly. ([97645c1](https://github.com/taiki-e/portable-atomic/commit/97645c1b2b938249f16eacb0fe696d4c7bb96754), [e1d1a97](https://github.com/taiki-e/portable-atomic/commit/e1d1a97cd1ab4bd04b45962c44ca1e9f0f9e1456))

- Make `get_mut` `const fn` on Rust 1.83+. ([0dea68c](https://github.com/taiki-e/portable-atomic/commit/0dea68c26e2bba3a83a849e2f137a2da445fc014))

- Make `from_ptr` `const fn` on Rust 1.83+. (align to the [std atomic change in Rust 1.84](https://github.com/rust-lang/rust/pull/131717)) ([50532d8](https://github.com/taiki-e/portable-atomic/commit/50532d8ce92408976307431a2f1a94000c630b23))

- Various optimizations:
  - RISC-V without A-extension: Optimize 16-bit fetch_not when Zabha enabled. ([a487a09](https://github.com/taiki-e/portable-atomic/commit/a487a094ab99fdf8ef3413b943c9c154a352d8c4))
  - s390x: Optimize 128-bit CAS/RMW. ([fba028d](https://github.com/taiki-e/portable-atomic/commit/fba028d7618e47b3de5b352c1789ea52f45298b4), [33ab2c1](https://github.com/taiki-e/portable-atomic/commit/33ab2c19719b29c0a30df8d011d0cd4003495dc8))
  - PowerPC64: Optimize 128-bit Acquire/AcqRel/SeqCst CAS/RMW and 128-bit CAS with Relaxed failure ordering. ([33ab2c1](https://github.com/taiki-e/portable-atomic/commit/33ab2c19719b29c0a30df8d011d0cd4003495dc8))
  - AVR: Optimize 8-bit load/store. ([33ab2c1](https://github.com/taiki-e/portable-atomic/commit/33ab2c19719b29c0a30df8d011d0cd4003495dc8))

- Improve support of run-time detection and outline-atomics:
  - Enable run-time detection by default on powerpc64 and aarch64 linux-uclibc. ([#193](https://github.com/taiki-e/portable-atomic/pull/193))
  - Improve run-time detection of powerpc64 quadword-atomics. ([1e3bfda](https://github.com/taiki-e/portable-atomic/commit/1e3bfda29bf6eb06a359f24554f19fc080ae57eb))
  - Improve run-time detection of Zhaoxin CPU. ([f283d2a](https://github.com/taiki-e/portable-atomic/commit/f283d2a1deb94e84676a2c1ee678439f507482db))

- Support RISC-V Zacas extension on pre-1.82 rustc. ([#194](https://github.com/taiki-e/portable-atomic/pull/194))

- Improve compile-time detection of RISC-V Zaamo/Zabha extensions. ([673137a](https://github.com/taiki-e/portable-atomic/commit/673137afb87b731ed19a1b0717059468712fad1d))

- Respect [`RUSTC_BOOTSTRAP=-1` recently added in nightly](https://github.com/rust-lang/rust/pull/132993) in rustc version detection. ([5b2847a](https://github.com/taiki-e/portable-atomic/commit/5b2847a8b99aa2a57a6c80f5a47327b2764f08cc))

## [1.9.0] - 2024-09-28

- RISC-V without A-extension: Support RMW when Zaamo extension enabled (even when `unsafe-assume-single-core` disabled). ([#185](https://github.com/taiki-e/portable-atomic/pull/185), [9983a8b](https://github.com/taiki-e/portable-atomic/commit/9983a8b9ad66efe4303b95678014369a56839aef))
  See "operations don't require disabling interrupts" list in [`interrupt` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/interrupt/README.md) for the operations provided.

- Support run-time detection of RISC-V Zacas extension (currently disabled by default). ([#183](https://github.com/taiki-e/portable-atomic/pull/183))

- Support 128-bit atomics on Arm64EC (currently nightly-only) ([#184](https://github.com/taiki-e/portable-atomic/pull/184))

- Improve compile-time detection of powerpc64 quadword-atomics. ([3eb8507](https://github.com/taiki-e/portable-atomic/commit/3eb8507f91c1ab382d9f455b2311a8664a4c33ff))

## [1.8.0] - 2024-09-20

- Improve diagnostics when method that requires CAS is unavailable. ([#181](https://github.com/taiki-e/portable-atomic/pull/181))

  Before:

  ```text
  error[E0599]: no method named `compare_exchange` found for struct `portable_atomic::AtomicUsize` in the current scope
    --> src/race.rs:60:24
     |
  60 |             self.inner.compare_exchange(0, value.get(), Ordering::AcqRel, Ordering::Acquire);
     |                        ^^^^^^^^^^^^^^^^ method not found in `AtomicUsize`
  ```

  After:

  ```text
  error[E0277]: `compare_exchange` requires atomic CAS but not available on this target by default
      --> src/race.rs:60:24
       |
  60   |             self.inner.compare_exchange(0, value.get(), Ordering::AcqRel, Ordering::Acquire);
       |                        ^^^^^^^^^^^^^^^^ this associated function is not available on this target by default
       |
       = help: the trait `HasCompareExchange` is not implemented for `&portable_atomic::AtomicUsize`
       = note: consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features
       = note: see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more.
  ```

- Improve compile error messages for some other cases ([19716ac](https://github.com/taiki-e/portable-atomic/commit/19716ac1d3b6082a8cb838af532ccab871041249), [61dcaaa](https://github.com/taiki-e/portable-atomic/commit/61dcaaa320cb347ab799c6c4f4480600692de2ad))

- Various improvements to RISC-V.
  - riscv64: Support 128-bit atomics when Zacas extension enabled. ([173](https://github.com/taiki-e/portable-atomic/pull/173)) This is currently marked as experimental because LLVM marking the corresponding target feature as experimental.
  - riscv32: Support 64-bit atomics when Zacas extension enabled. ([173](https://github.com/taiki-e/portable-atomic/pull/173)) This is currently marked as experimental because LLVM marking the corresponding target feature as experimental.
  - Improvements for RISC-V without A-extension:
    - Support zaamo target feature. When building for single-core RISC-V without A-extension, this is equivalent to force-amo feature  ([8abba4b](https://github.com/taiki-e/portable-atomic/commit/8abba4b0ea920d23799c2d7b6985617e6392d176))
    - Support zabha target feature. ([694364a](https://github.com/taiki-e/portable-atomic/commit/694364a179441f723e429516132436c46f5857b4))
    - Strengthen SeqCst store to improve compatibility with code that uses atomic instruction mapping that differs from LLVM and GCC. ([5b10b15](https://github.com/taiki-e/portable-atomic/commit/5b10b1516e056b16b851e498a271f751152feed9))

- Improve support of run-time detection and outline-atomics:
  - aarch64: Support run-time detection of FEAT_LRCPC3/FEAT_LSE128 for load/store. ([#174](https://github.com/taiki-e/portable-atomic/pull/174))
  - aarch64: Support run-time detection of FEAT_LSE2 on OpenBSD. ([4f8c735](https://github.com/taiki-e/portable-atomic/commit/4f8c7350fd80d005233d85191707e4329a5ef484))
  - aarch64: Support run-time detection of FEAT_LSE/FEAT_LSE2 on illumos (currently disabled by default because illumos AArch64 port is experimental). ([#175](https://github.com/taiki-e/portable-atomic/pull/175))
  - powerpc64: Support run-time detection on OpenBSD 7.6+ (currently disabled by default for compatibility with old versions). ([09a967b](https://github.com/taiki-e/portable-atomic/commit/09a967b59c217dc9ebb4d78ec114758ef9941fc8))

- Support AArch64 FEAT_LRCPC3/FEAT_LSE128 with pre-16 LLVM. ([#178](https://github.com/taiki-e/portable-atomic/pull/178))

- Improve compile-time detection of AArch64 FEAT_LSE2/FEAT_LRCPC3/FEAT_LSE128. ([10d47de](https://github.com/taiki-e/portable-atomic/commit/10d47def74b9c13fd864436d6118e902f118c028))

- Relax minimal version of `serde` (supported via optional feature) to 1.0.60.

## [1.7.0] - 2024-07-19

- Support run-time detection for cmpxchg16b on x86_64 on pre-1.69 rustc. ([#154](https://github.com/taiki-e/portable-atomic/pull/154))

- Make `into_inner` `const fn` on Rust 1.56+. (align to the [std atomic change in Rust 1.79](https://github.com/rust-lang/rust/pull/123522)) ([dee1f89](https://github.com/taiki-e/portable-atomic/commit/dee1f89739594271e4f5b5d3f122d2762fcbbd7d))

- Work around [rustc_codegen_gcc bug on x86_64](https://github.com/rust-lang/rustc_codegen_gcc/issues/485). ([d938f77](https://github.com/taiki-e/portable-atomic/commit/d938f77f3f2f353cbfb525ac23e63e880cd583df))

- Optimize x86_64 atomics.
  - Optimize 128-bit load/store on Zhaoxin CPU with AVX. ([86cee8f](https://github.com/taiki-e/portable-atomic/commit/86cee8fccf7deedb5b8cbcb3868ec2e43aca575f))
  - Optimize 128-bit SeqCst store on Intel/AMD/Zhaoxin CPU with AVX. ([#156](https://github.com/taiki-e/portable-atomic/pull/156), [0483042](https://github.com/taiki-e/portable-atomic/commit/0483042671fad4ccd0bdb691424a967fc2a5cb8e))
  - Remove needless test in CAS. ([573e025](https://github.com/taiki-e/portable-atomic/commit/573e02586f933a257c8e7383dcf64408b4708b85))

- Make rustc version detection robust for custom toolchains. ([f8ea85e](https://github.com/taiki-e/portable-atomic/commit/f8ea85e1aa46fa00bc865633fb40b05f8a0c823b))

- Respect `RUSTC_WRAPPER` in rustc version detection.

- Our build script is now less likely to be [re-run unnecessarily](https://github.com/taiki-e/portable-atomic/issues/151) in versions where the cargo bug fix is available (cargo 1.79+). ([52c277b](https://github.com/taiki-e/portable-atomic/commit/52c277bc9a9a8031175aaed54987260da1750af4))

## [1.6.0] - 2023-12-06

- Add `cfg_{has,no}_atomic_{8,16,32,64,128,ptr}` macros to enable code when the corresponding atomic implementation is available/unavailable.

- Add `cfg_{has,no}_atomic_cas` macros to enable code when atomic CAS/RMW implementation is available/unavailable.

- Improve support for RISC-V targets without atomic CAS.

## [1.5.1] - 2023-10-29

- Fix bug in `i{8,16}` `fetch_{or,xor}` on RISC-V without A-extension where `unsafe-assume-single-core` and `force-amo` are enabled.

- Optimize `swap` for targets that do not have native atomic CAS instructions.

## [1.5.0] - 2023-10-23

**Note:** This release has been yanked due to a bug fixed in 1.5.1.

- Add `from_ptr`.

- Add `force-amo` feature (`portable_atomic_force_amo` cfg) for single-core RISC-V without A-extension. ([#124](https://github.com/taiki-e/portable-atomic/pull/124))

- Support run-time detection on AArch64 on pre-1.61 rustc. ([#98](https://github.com/taiki-e/portable-atomic/pull/98))

  This also solves [a compatibility issue with rustc_codegen_cranelift](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1400).

- Support run-time detection of FEAT_LSE2. ([#126](https://github.com/taiki-e/portable-atomic/pull/126))

- Support run-time detection of FEAT_LSE on AArch64 NetBSD. ([#66](https://github.com/taiki-e/portable-atomic/pull/66))

- Acknowledge ESP-IDF targets' 64-bit atomics are not lock-free. See [#122](https://github.com/taiki-e/portable-atomic/issues/122) for more.

- Optimize 128-bit weak CAS on powerpc64.

- Optimize interrupt disable on no-std pre-v6 Arm where `unsafe-assume-single-core` and `disable-fiq` are enabled. ([771c45d](https://github.com/taiki-e/portable-atomic/commit/771c45da2d2afc4f83df033dd4bdf3f976d14a74))

- Improve detection of Apple hardware. ([5c3a43b](https://github.com/taiki-e/portable-atomic/commit/5c3a43b53f1c4188f9dd597599633bc1a315bf44))

- Improve compatibility with the future version of Miri.

## [1.4.3] - 2023-08-25

- Optimize AArch64 128-bit atomic store/swap/fetch_and/fetch_or when the `lse128` target feature is enabled at compile-time. ([#68](https://github.com/taiki-e/portable-atomic/pull/68))

- Optimize AArch64 128-bit atomic load/store when the `rcpc3` target feature is enabled at compile-time. ([#68](https://github.com/taiki-e/portable-atomic/pull/68))

- Optimize inline assemblies on Arm, AArch64, and MSP430.

## [1.4.2] - 2023-07-27

- Optimize `AtomicBool` on RISC-V/LoongArch64. This is the same as [rust-lang/rust#114034](https://github.com/rust-lang/rust/pull/114034), but is available for all rustc versions.

## [1.4.1] - 2023-07-15

- Improve compatibility with the future version of Miri.

## [1.4.0] - 2023-07-11

- Allow using embedded-related cfgs as Cargo features. ([#94](https://github.com/taiki-e/portable-atomic/pull/94), thanks @Dirbaio)

  Originally, we were providing these as cfgs instead of features, but based on a strong request from the embedded ecosystem, we have agreed to provide them as features as well. See [#94](https://github.com/taiki-e/portable-atomic/pull/94) for more.

  cfgs are kept and can be used as aliases for features.

- Acknowledge all x86_64 Apple targets support 128-bit atomics.

  Our code already recognizes this via `cfg(target_feature)`, so this only affects docs and users using pre-1.69 stable rustc.

  See also [rust-lang/rust#112150](https://github.com/rust-lang/rust/pull/112150).

- Optimize 128-bit atomics on AArch64/s390x.

## [1.3.3] - 2023-05-31

- Fix build error on AArch64 ILP32 ABI targets (tier 3).

- Optimize 128-bit atomics on s390x.

## [1.3.2] - 2023-05-09

- Fix bug in powerpc64/s390x 128-bit atomic RMWs on old nightly.

- Optimize 128-bit atomics on powerpc64/s390x.

## [1.3.1] - 2023-05-07

- Documentation improvements.

## [1.3.0] - 2023-05-06

- Add `require-cas` feature. ([#100](https://github.com/taiki-e/portable-atomic/pull/100))

  If your crate supports no-std environment and requires atomic CAS, enabling this feature will allow the `portable-atomic` to display helpful error messages to users on targets requiring additional action on the user side to provide atomic CAS.

  ```toml
  [dependencies]
  portable-atomic = { version = "1.3", default-features = false, features = ["require-cas"] }
  ```

  See [#100](https://github.com/taiki-e/portable-atomic/pull/100) for more.

- Support `portable_atomic_unsafe_assume_single_core` cfg on Xtensa targets without atomic CAS. ([#86](https://github.com/taiki-e/portable-atomic/pull/86))

- Fix bug in AArch64 128-bit SeqCst load when FEAT_LSE2 is enabled at compile-time. This is [the same bug that was fixed in the recently released GCC 13.1](https://gcc.gnu.org/bugzilla/show_bug.cgi?id=108891). LLVM also has the same bug, which had not yet been fixed when the patch was created; I will open a bug report if necessary after looking into the situation in LLVM. ([a29154b](https://github.com/taiki-e/portable-atomic/commit/a29154b21da270e90cb86f6865b591ab36eade7d))

- Fix compile error on `bpf{eb,el}-unknown-none` (tier 3) and `mipsel-sony-psx` (tier 3) when `critical-section` feature is disabled.

- Various optimizations
  - Optimize x86_64 128-bit outline-atomics. This improves performance by up to 15% in concurrent RMW/store for cases where the `cmpxchg16b` target feature is not available at compile-time. ([40c4cd4](https://github.com/taiki-e/portable-atomic/commit/40c4cd4f682f1cb153f18d4d6a88795bafaf5667))
  - Optimize x86_64 128-bit load that uses cmpxchg16b. ([40c4cd4](https://github.com/taiki-e/portable-atomic/commit/40c4cd4f682f1cb153f18d4d6a88795bafaf5667))
  - Optimize AArch64 128-bit load that uses FEAT_LSE. ([40c4cd4](https://github.com/taiki-e/portable-atomic/commit/40c4cd4f682f1cb153f18d4d6a88795bafaf5667))
  - Optimize pre-Armv6 Linux/Android 64-bit atomics. ([efacc89](https://github.com/taiki-e/portable-atomic/commit/efacc89c210d7a34ef5e879821112189da5d1901))
  - Support outline-atomics for powerpc64 128-bit atomics. This is currently disabled by default, and can be enabled by `--cfg portable_atomic_outline_atomics`. ([#90](https://github.com/taiki-e/portable-atomic/pull/90))
  - Optimize AArch64 outline-atomics on linux-musl. On linux-musl, outline-atomics is enabled by default only when dynamic linking is enabled. When static linking is enabled, this can be enabled by `--cfg portable_atomic_outline_atomics`. See the [`atomic128` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/atomic128/README.md#run-time-feature-detection) for more. ([8418235](https://github.com/taiki-e/portable-atomic/commit/84182354e4a149074e28bda4683d538e5fb617ce), [31d0862](https://github.com/taiki-e/portable-atomic/commit/31d08623d4e21af207ff2343f5553b9b5a030452))

## [1.2.0] - 2023-03-25

- Make 64-bit atomics lock-free on Arm Linux/Android targets that do not have 64-bit atomics (e.g., armv5te-unknown-linux-gnueabi, arm-linux-androideabi, etc.) when the kernel version is 3.1 or later. ([#82](https://github.com/taiki-e/portable-atomic/pull/82))

- Fix AArch64 128-bit atomics performance regression on Apple hardware. ([#89](https://github.com/taiki-e/portable-atomic/pull/89))

- Optimize 128-bit atomics on AArch64, x86_64, powerpc64, and s390x.

## [1.1.0] - 2023-03-24

- Add `Atomic{I,U}*::bit_{set,clear,toggle}` and `AtomicPtr::bit_{set,clear,toggle}`. ([#72](https://github.com/taiki-e/portable-atomic/pull/72))

  They correspond to x86's `lock bt{s,r,c}`, and the implementation calls them on x86/x86_64.

- Add `AtomicU*::{fetch_neg,neg}` methods. Previously it was only available on `AtomicI*` and `AtomicF*`.

- Add `as_ptr` method to all atomic types. ([#79](https://github.com/taiki-e/portable-atomic/pull/79))

- Make `AtomicF{32,64}::as_bits` `const fn` on Rust 1.58+. ([#79](https://github.com/taiki-e/portable-atomic/pull/79))

- Relax ordering in `Serialize` impl to reflect the [upstream change](https://github.com/serde-rs/serde/pull/2263).

- Optimize x86_64 outline-atomics for 128-bit atomics.
  - Support outline-atomics for cmpxchg16b on Rust 1.69+ (i.e., on Rust 1.69+, x86_64 128-bit atomics is lock-free on all Intel chips and almost all AMD chips, even if cmpxchg16b is not available at compile-time.). Previously it was only nightly. ([#80](https://github.com/taiki-e/portable-atomic/pull/80))
  - portable-atomic no longer enables outline-atomics on target where run-time CPU feature detection is not available. ([#80](https://github.com/taiki-e/portable-atomic/pull/80))

- Optimize AArch64 outline-atomics for 128-bit atomics.
  - Support more targets and improve performance. ([#63](https://github.com/taiki-e/portable-atomic/pull/63), [#64](https://github.com/taiki-e/portable-atomic/pull/64), [#67](https://github.com/taiki-e/portable-atomic/pull/67), [#69](https://github.com/taiki-e/portable-atomic/pull/69), [#75](https://github.com/taiki-e/portable-atomic/pull/75), [#76](https://github.com/taiki-e/portable-atomic/pull/76), [#77](https://github.com/taiki-e/portable-atomic/pull/77))
    See the [`atomic128` module's readme](https://github.com/taiki-e/portable-atomic/blob/HEAD/src/imp/atomic128/README.md#run-time-feature-detection) for a list of platforms that support outline-atomics.
    Most of these improvements have already been [submitted and accepted in rust-lang/stdarch](https://github.com/rust-lang/stdarch/pulls?q=is%3Apr+author%3Ataiki-e+std_detect) and will soon be available in `std::arch::is_aarch64_feature_detected`.
  - portable-atomic no longer enables outline-atomics on target where run-time CPU feature detection is not available.

- Performance improvements. ([#70](https://github.com/taiki-e/portable-atomic/pull/70), [#81](https://github.com/taiki-e/portable-atomic/pull/81), [6c189ae](https://github.com/taiki-e/portable-atomic/commit/6c189ae1792ce0c08b4f56b6e6c256c223475ce2), [13c92b0](https://github.com/taiki-e/portable-atomic/commit/13c92b015a8e8646a4b885229157547354d03b9e), etc.)

- Improve support for old nightly. ([#73](https://github.com/taiki-e/portable-atomic/pull/73), [872feb9](https://github.com/taiki-e/portable-atomic/commit/872feb9d7f3a4ca7cf9b63935265d46498fcae99))

- Documentation improvements.

## [1.0.1] - 2023-01-21

- Optimize `Atomic{I,U}*::{fetch_not,not}` methods. ([#62](https://github.com/taiki-e/portable-atomic/pull/62))

## [1.0.0] - 2023-01-15

- Add `critical-section` feature to use [critical-section](https://github.com/rust-embedded/critical-section) on targets where atomic CAS is not natively available. ([#51](https://github.com/taiki-e/portable-atomic/pull/51), thanks @Dirbaio)

  This is useful to get atomic CAS when `--cfg portable_atomic_unsafe_assume_single_core` can't be used, such as multi-core targets, unprivileged code running under some RTOS, or environments where disabling interrupts needs extra care due to e.g. real-time requirements.

  See [documentation](https://github.com/taiki-e/portable-atomic#optional-features-critical-section) for more.

- Remove `outline-atomics` feature. This was no-op since 0.3.19.

- Documentation improvements.

## [0.3.20] - 2023-05-07

The latest version of portable-atomic is 1.x. This release makes portable-atomic 0.3 is built on top of portable-atomic 1.x to make bug fixes and improvements such as [support for new targets](https://github.com/taiki-e/portable-atomic/pull/86) in 1.x available to the ecosystem that depends on older portable-atomic. portable-atomic 0.3 is still maintained passively, but upgrading to portable-atomic 1.x is recommended. (There are no breaking changes from 0.3, except that a deprecated no-op `outline-atomics` Cargo feature has been removed.) ([#99](https://github.com/taiki-e/portable-atomic/pull/99))

## [0.3.19] - 2022-12-25

- Add `AtomicI*::{fetch_neg,neg}` and `AtomicF*::fetch_neg` methods. ([#54](https://github.com/taiki-e/portable-atomic/pull/54))

  `AtomicI*::neg` are equivalent to the corresponding `fetch_*` methods, but do not return the previous value. They are intended for optimization on platforms that have atomic instructions for the corresponding operation, such as x86's `lock neg`.

  Currently, optimizations by these methods (`neg`) are only guaranteed for x86/x86_64.

- Add `Atomic{I,U}*::{fetch_not,not}` methods. ([#54](https://github.com/taiki-e/portable-atomic/pull/54))

  `Atomic{I,U}*::not` are equivalent to the corresponding `fetch_*` methods, but do not return the previous value. They are intended for optimization on platforms that have atomic instructions for the corresponding operation, such as x86's `lock not`, MSP430's `inv`.

  Currently, optimizations by these methods (`not`) are only guaranteed for x86/x86_64 and MSP430.

  (Note: `AtomicBool` already has `fetch_not` and `not` methods.)

- Enable outline-atomics for 128-bit atomics by default. ([#57](https://github.com/taiki-e/portable-atomic/pull/57)) See [#57](https://github.com/taiki-e/portable-atomic/pull/57) for more.

- Improve support for old nightly compilers.

## [0.3.18] - 2022-12-15

- Fix build error when not using `portable_atomic_unsafe_assume_single_core` cfg on AVR and MSP430 custom targets. ([#50](https://github.com/taiki-e/portable-atomic/pull/50))

  Since 0.3.11, atomic CAS was supported without the cfg on AVR and MSP430 builtin targets, but that change was not applied to custom targets.

## [0.3.17] - 2022-12-14

- Optimize x86_64 128-bit atomic load/store on AMD CPU with AVX. ([#49](https://github.com/taiki-e/portable-atomic/pull/49))

- Improve support for custom targets on old rustc.

## [0.3.16] - 2022-12-09

- Add `Atomic{I,U}*::{add,sub,and,or,xor}` and `AtomicBool::{and,or,xor}` methods. ([#47](https://github.com/taiki-e/portable-atomic/pull/47))

  They are equivalent to the corresponding `fetch_*` methods, but do not return the previous value. They are intended for optimization on platforms that implement atomics using inline assembly, such as the MSP430.

  Currently, optimizations by these methods (`add`,`sub`,`and`,`or`,`xor`) are only guaranteed for MSP430; on x86/x86_64, LLVM can optimize in most cases, so cases, where this would improve things, should be rare.

- Various improvements to `portable_atomic_unsafe_assume_single_core` cfg. ([#44](https://github.com/taiki-e/portable-atomic/pull/44), [#40](https://github.com/taiki-e/portable-atomic/pull/40))

  - Support disabling FIQs on pre-v6 Arm under `portable_atomic_disable_fiq` cfg.
  - Support RISC-V supervisor mode under `portable_atomic_s_mode` cfg.
  - Optimize interrupt restore on AVR and MSP430. ([#40](https://github.com/taiki-e/portable-atomic/pull/40))
  - Documentation improvements.

  See [#44](https://github.com/taiki-e/portable-atomic/pull/44) for more.

## [0.3.15] - 2022-09-09

- Implement workaround for std cpuid bug due to LLVM bug ([rust-lang/rust#101346](https://github.com/rust-lang/rust/issues/101346), [llvm/llvm-project#57550](https://github.com/llvm/llvm-project/issues/57550)).

  - Our use case is likely not affected, but we implement this just in case.
  - We've confirmed that the uses of inline assembly in this crate are not affected by this LLVM bug.

## [0.3.14] - 2022-09-04

- Optimize atomic load/store on no-std pre-v6 Arm when `portable_atomic_unsafe_assume_single_core` cfg is used. ([#36](https://github.com/taiki-e/portable-atomic/pull/36))

- Support pre-power8 powerpc64le. powerpc64le's default cpu version is power8, but you can technically compile it for the old cpu using the unsafe `-C target-cpu` rustc flag.

## [0.3.13] - 2022-08-15

- Use track_caller when debug assertions are enabled on Rust 1.46+.

- Make powerpc64 128-bit atomics compatible with Miri and ThreadSanitizer on LLVM 15+.

- Document that 128-bit atomics are compatible with Miri and ThreadSanitizer on recent nightly.

## [0.3.12] - 2022-08-13

- Support atomic CAS on no-std pre-v6 Arm targets (e.g., thumbv4t-none-eabi) under unsafe cfg `portable_atomic_unsafe_assume_single_core`. ([#28](https://github.com/taiki-e/portable-atomic/pull/28))

## [0.3.11] - 2022-08-12

- Always provide atomic CAS for MSP430 and AVR. ([#31](https://github.com/taiki-e/portable-atomic/pull/31))

  This previously required unsafe cfg `portable_atomic_unsafe_assume_single_core`, but since all MSP430 and AVR are single-core, we can safely provide atomic CAS based on disabling interrupts.

- Support `fence` and `compiler_fence` on MSP430. (On MSP430, the standard library's fences are currently unavailable due to LLVM errors.)

- Update safety requirements for unsafe cfg `portable_atomic_unsafe_assume_single_core` to mention use of privileged instructions to disable interrupts.

- Atomic operations based on disabling interrupts on single-core systems are now considered lock-free.

  The previous behavior was inconsistent because we consider the pre-v6 Arm Linux's atomic operations provided in a similar way by the Linux kernel to be lock-free.

- Respect `-Z allow-features`.

## [0.3.10] - 2022-08-03

- Optimize AArch64 128-bit atomic load when the `lse` target feature is enabled at compile-time. ([#20](https://github.com/taiki-e/portable-atomic/pull/20))

## [0.3.9] - 2022-08-03

- Fix build error on old Miri.

- Documentation improvements.

## [0.3.8] - 2022-08-02

- Make AArch64 and s390x 128-bit atomics compatible with Miri and ThreadSanitizer.

## [0.3.7] - 2022-07-31

- Provide stable equivalent of [`#![feature(strict_provenance_atomic_ptr)]`](https://github.com/rust-lang/rust/issues/99108). ([#23](https://github.com/taiki-e/portable-atomic/pull/23))

  - `AtomicPtr::fetch_ptr_{add,sub}`
  - `AtomicPtr::fetch_byte_{add,sub}`
  - `AtomicPtr::fetch_{or,and,xor}`

  These APIs are compatible with strict-provenance on `cfg(miri)`. Otherwise, they are compatible with permissive-provenance.
  Once `#![feature(strict_provenance_atomic_ptr)]` is stabilized, these APIs will be strict-provenance compatible in all cases from the version in which it is stabilized.

- Provide stable equivalent of [`#![feature(atomic_bool_fetch_not)]`](https://github.com/rust-lang/rust/issues/98485). ([#24](https://github.com/taiki-e/portable-atomic/pull/24))

  - `AtomicBool::fetch_not`

- Optimize x86_64 128-bit RMWs. ([#22](https://github.com/taiki-e/portable-atomic/pull/22))

- Optimize x86_64 outline-atomics.

- Optimize inline assemblies on Arm and AArch64.

- Revert [thumbv6m atomic load/store changes made in 0.3.5](https://github.com/taiki-e/portable-atomic/pull/18). This is because [rust-lang/rust#99595](https://github.com/rust-lang/rust/pull/99595) has been reverted, so this is no longer needed.

## [0.3.6] - 2022-07-26

- Fix build failure due to the existence of the `specs` directory.

- Documentation improvements.

- Optimize inline assemblies on x86_64, RISC-V, and MSP430.

## [0.3.5] - 2022-07-23

**Note:** This release has been yanked due to a bug fixed in 0.3.6.

- Provide thumbv6m atomic load/store which is planned to be removed from the standard library in [rust-lang/rust#99595](https://github.com/rust-lang/rust/pull/99595). ([#18](https://github.com/taiki-e/portable-atomic/pull/18))

- Optimize inline assemblies on AArch64, RISC-V, and powerpc64.

## [0.3.4] - 2022-06-25

- Optimize x86_64 128-bit atomic store.

## [0.3.3] - 2022-06-24

- Allow CAS failure ordering stronger than success ordering. ([#17](https://github.com/taiki-e/portable-atomic/pull/17))

## [0.3.2] - 2022-06-19

- Optimize x86_64 128-bit atomic load/store on Intel CPU with AVX. ([#16](https://github.com/taiki-e/portable-atomic/pull/16))

- Support native 128-bit atomic operations for powerpc64 (le or pwr8+, currently nightly-only).

- Fix behavior differences between stable and nightly. ([#15](https://github.com/taiki-e/portable-atomic/pull/15))

## [0.3.1] - 2022-06-16

- Optimize AArch64 128-bit atomic load/store when the `lse2` target feature is enabled at compile-time. ([#11](https://github.com/taiki-e/portable-atomic/pull/11))

- Relax ordering in `Debug` impl to reflect std changes. ([#12](https://github.com/taiki-e/portable-atomic/pull/12))

## [0.3.0] - 2022-03-25

- Support native 128-bit atomic operations for s390x (currently nightly-only).

- Add `AtomicF{32,64}::fetch_abs`.

- Add `#[must_use]` to constructors.

- Use 128-bit atomic operation mappings same as LLVM on AArch64.

- Remove `parking_lot` optional feature to allow the use of this crate within global allocators.

## [0.2.1] - 2022-03-17

- Implement AArch64 outline-atomics.

## [0.2.0] - 2022-03-10

- Remove `i128` feature. `Atomic{I,U}128` are now always enabled.

- Add `outline-atomics` feature. Currently, this is the same as the 0.1's `i128-dynamic`, except that `fallback` feature is not implicitly enabled.

- Remove `i128-dynamic` feature in favor of `outline-atomics` feature.

- Add `AtomicF{32,64}::as_bits`.

## [0.1.4] - 2022-03-02

- Support native 128-bit atomic operations for AArch64 at Rust 1.59+. This was previously supported only on nightly. ([#6](https://github.com/taiki-e/portable-atomic/pull/6))

## [0.1.3] - 2022-02-28

- Fix inline assembly for RISC-V without A-extension.

## [0.1.2] - 2022-02-26

**Note:** This release has been yanked due to a bug fixed in 0.1.3.

- Add `parking_lot` feature to use parking_lot in global locks of fallback implementation.

- Fix bug in cmpxchg16b support. ([#5](https://github.com/taiki-e/portable-atomic/pull/5))

## [0.1.1] - 2022-02-25

**Note:** This release has been yanked due to a bug fixed in 0.1.3.

- Fix doc cfg on `Atomic{I,U}128`.

## [0.1.0] - 2022-02-24

**Note:** This release has been yanked due to a bug fixed in 0.1.3.

Initial release

[Unreleased]: https://github.com/taiki-e/portable-atomic/compare/v1.11.1...HEAD
[1.11.1]: https://github.com/taiki-e/portable-atomic/compare/v1.11.0...v1.11.1
[1.11.0]: https://github.com/taiki-e/portable-atomic/compare/v1.10.0...v1.11.0
[1.10.0]: https://github.com/taiki-e/portable-atomic/compare/v1.9.0...v1.10.0
[1.9.0]: https://github.com/taiki-e/portable-atomic/compare/v1.8.0...v1.9.0
[1.8.0]: https://github.com/taiki-e/portable-atomic/compare/v1.7.0...v1.8.0
[1.7.0]: https://github.com/taiki-e/portable-atomic/compare/v1.6.0...v1.7.0
[1.6.0]: https://github.com/taiki-e/portable-atomic/compare/v1.5.1...v1.6.0
[1.5.1]: https://github.com/taiki-e/portable-atomic/compare/v1.5.0...v1.5.1
[1.5.0]: https://github.com/taiki-e/portable-atomic/compare/v1.4.3...v1.5.0
[1.4.3]: https://github.com/taiki-e/portable-atomic/compare/v1.4.2...v1.4.3
[1.4.2]: https://github.com/taiki-e/portable-atomic/compare/v1.4.1...v1.4.2
[1.4.1]: https://github.com/taiki-e/portable-atomic/compare/v1.4.0...v1.4.1
[1.4.0]: https://github.com/taiki-e/portable-atomic/compare/v1.3.3...v1.4.0
[1.3.3]: https://github.com/taiki-e/portable-atomic/compare/v1.3.2...v1.3.3
[1.3.2]: https://github.com/taiki-e/portable-atomic/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/taiki-e/portable-atomic/compare/v1.3.0...v1.3.1
[1.3.0]: https://github.com/taiki-e/portable-atomic/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/taiki-e/portable-atomic/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/taiki-e/portable-atomic/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/taiki-e/portable-atomic/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/taiki-e/portable-atomic/compare/v0.3.19...v1.0.0
[0.3.20]: https://github.com/taiki-e/portable-atomic/compare/v0.3.19...v0.3.20
[0.3.19]: https://github.com/taiki-e/portable-atomic/compare/v0.3.18...v0.3.19
[0.3.18]: https://github.com/taiki-e/portable-atomic/compare/v0.3.17...v0.3.18
[0.3.17]: https://github.com/taiki-e/portable-atomic/compare/v0.3.16...v0.3.17
[0.3.16]: https://github.com/taiki-e/portable-atomic/compare/v0.3.15...v0.3.16
[0.3.15]: https://github.com/taiki-e/portable-atomic/compare/v0.3.14...v0.3.15
[0.3.14]: https://github.com/taiki-e/portable-atomic/compare/v0.3.13...v0.3.14
[0.3.13]: https://github.com/taiki-e/portable-atomic/compare/v0.3.12...v0.3.13
[0.3.12]: https://github.com/taiki-e/portable-atomic/compare/v0.3.11...v0.3.12
[0.3.11]: https://github.com/taiki-e/portable-atomic/compare/v0.3.10...v0.3.11
[0.3.10]: https://github.com/taiki-e/portable-atomic/compare/v0.3.9...v0.3.10
[0.3.9]: https://github.com/taiki-e/portable-atomic/compare/v0.3.8...v0.3.9
[0.3.8]: https://github.com/taiki-e/portable-atomic/compare/v0.3.7...v0.3.8
[0.3.7]: https://github.com/taiki-e/portable-atomic/compare/v0.3.6...v0.3.7
[0.3.6]: https://github.com/taiki-e/portable-atomic/compare/v0.3.5...v0.3.6
[0.3.5]: https://github.com/taiki-e/portable-atomic/compare/v0.3.4...v0.3.5
[0.3.4]: https://github.com/taiki-e/portable-atomic/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/taiki-e/portable-atomic/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/taiki-e/portable-atomic/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/taiki-e/portable-atomic/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/taiki-e/portable-atomic/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/taiki-e/portable-atomic/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/portable-atomic/compare/v0.1.4...v0.2.0
[0.1.4]: https://github.com/taiki-e/portable-atomic/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/taiki-e/portable-atomic/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/portable-atomic/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/portable-atomic/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/portable-atomic/releases/tag/v0.1.0
