# 128-bit atomic implementations on 64-bit architectures

(See the [`atomic64` module](../atomic64) for 64-bit atomic implementations on 32-bit architectures.)

## 128-bit atomic instructions

Here is the table of targets that support 128-bit atomics and the instructions used:

| target_arch | load | store | CAS | RMW | note |
| ----------- | ---- | ----- | --- | --- | ---- |
| x86_64 | cmpxchg16b or vmovdqa | cmpxchg16b or vmovdqa | cmpxchg16b | cmpxchg16b | Requires `cmpxchg16b` target feature (enabled by default on Apple, Windows (except Windows 7, since Rust 1.78), and Fuchsia (since Rust 1.87) targets). vmovdqa requires Intel, AMD, or Zhaoxin CPU with AVX. <br> Both compile-time and run-time detection are supported for cmpxchg16b. vmovdqa is currently run-time detection only. <br> Requires rustc 1.59+ |
| aarch64/arm64ec | ldxp/stxp or casp or ldp/ldiapp | ldxp/stxp or casp or stp/stilp/swpp | ldxp/stxp or casp | ldxp/stxp or casp/swpp/ldclrp/ldsetp | casp requires `lse` target feature, ldp/stp requires `lse2` target feature, ldiapp/stilp requires `lse2` and `rcpc3` target features, swpp/ldclrp/ldsetp requires `lse128` target feature. <br> Both compile-time and run-time detection are supported. <br> Requires rustc 1.59+ (aarch64) / 1.84+ (arm64ec) |
| riscv64 | amocas.q | amocas.q | amocas.q | amocas.q | Requires `zacas` target feature. Both compile-time and run-time detection are supported. <br> Requires rustc 1.59+ |
| powerpc64 | lq | stq | lqarx/stqcx. | lqarx/stqcx. | Requires `quadword-atomics` target feature (enabled by default on powerpc64le). Both compile-time and run-time detection are supported. <br> Requires nightly |
| s390x | lpq | stpq | cdsg | cdsg | Requires rustc 1.84+ |
| loongarch64 | sc.q | sc.q | sc.q | sc.q | Unimplemented. Requires `scq` target feature.  |
| mips64r6 | lldp | lldp/scdp | lldp/scdp | lldp/scdp | Unimplemented (unsupported in LLVM). Requires Release 6 Paired LL/SC family of instructions |
| nvptx64 | ld.b128 | st.b128 | atom.cas.b128 | atom.exch.b128/atom.cas.b128 | Unimplemented. Requires `ptx83` and `sm_90`. |

On compiler versions or platforms where these are not supported, the fallback implementation is used.

See [aarch64.rs](aarch64.rs) module-level comments for more details on the instructions used on AArch64.

## Comparison with core::intrinsics::atomic_\* (core::sync::atomic::Atomic{I,U}128)

This directory has target-specific implementations with inline assembly ([x86_64.rs](x86_64.rs), [aarch64.rs](aarch64.rs), [riscv64.rs](riscv64.rs), [powerpc64.rs](powerpc64.rs), [s390x.rs](s390x.rs)) and an implementation without inline assembly ([intrinsics.rs](intrinsics.rs)). The latter currently always needs nightly compilers and is only used for Miri and ThreadSanitizer, which do not support inline assembly.

Implementations with inline assembly generate assemblies almost equivalent to the `core::intrinsics::atomic_*` (used in `core::sync::atomic::Atomic{I,U}128`) for many operations, but some operations may or may not generate more efficient code. For example:

- On x86_64 and AArch64, implementation with inline assembly contains additional optimizations (e.g., [#16](https://github.com/taiki-e/portable-atomic/pull/16), [#126](https://github.com/taiki-e/portable-atomic/pull/126)) and is much faster for some operations.
- On AArch64, implementation with inline assembly supports outline-atomics on more operating systems, and may be faster in environments where outline-atomics can improve performance.
- On powerpc64, LLVM does not support generating some 128-bit atomic operations (see [intrinsics.rs](intrinsics.rs) module-level comments), and we use CAS loop to implement them, so implementation with inline assembly may be faster for those operations.
- In implementations without inline assembly, the compiler may reuse condition flags that have changed as a result of the operation, or use immediate values instead of registers, depending on the situation.

As 128-bit atomics-related APIs stabilize in the standard library, implementations with inline assembly are planned to be updated to get the benefits of both.

## Run-time CPU feature detection

See the [`detect` module's readme](../detect/README.md) for run-time CPU feature detection.
