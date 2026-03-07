# Fix SIGILL crash in WASM extension execution on certain x86-64 CPUs

## Crash Summary

**Sentry Issue:** [ZED-47](https://sentry.io/organizations/zed-dev/issues/6807884895/)
- **Error:** SIGILL / ILL_ILLOPN / 0x0 (Illegal Instruction)
- **Event Count:** 3050 events
- **First Seen:** 2025-08-13
- **Last Seen:** 2026-02-14
- **Affected Hardware:** Intel Alder Lake-N [UHD Graphics] on Linux
- **Channel:** Preview

## Root Cause

The crash occurs during WASM extension execution inside wasmtime's JIT-compiled code. The Intel Alder Lake-N is a low-power variant using only E-cores (Gracemont), which have different SIMD capabilities compared to the hybrid P-cores on standard Alder Lake CPUs. 

Wasmtime's cranelift JIT compiler infers CPU features from CPUID at runtime, but there can be mismatches between what CPUID reports and what actually works:
1. The CPU variant may not fully support all features that CPUID reports
2. Kernel mitigations (like GDS/Downfall) can disable AVX instructions at runtime
3. Specific instruction encodings may behave differently on E-core variants

This results in the JIT generating instructions that cause SIGILL when executed.

## Fix

Disable AVX-512 CPU features in the wasmtime configuration to prevent the JIT from generating potentially problematic AVX-512 instructions. This is a conservative approach that sacrifices a small amount of potential performance for improved compatibility.

The fix adds explicit `cranelift_flag_set` calls to disable:
- `has_avx512bitalg`
- `has_avx512dq`
- `has_avx512f`
- `has_avx512vbmi`
- `has_avx512vl`

These features are disabled only on x86-64 targets via `#[cfg(target_arch = "x86_64")]`.

## Validation

- [x] Code compiles successfully with `cargo check -p extension_host`
- [x] Clippy passes with `./script/clippy -p extension_host`
- [x] Unit test added: `test_wasm_engine_creates_successfully`

## Potentially Related Issues

### High Confidence
- [wasmtime #3809](https://github.com/bytecodealliance/wasmtime/issues/3809) - Minimum x86_64 feature support required for SIMD proposal

### Medium Confidence
- [element-desktop #1385](https://github.com/element-hq/element-desktop/issues/1385) - Illegal instruction on Skylake CPUs with GDS/Downfall mitigation
- [wasmtime #10199](https://github.com/bytecodealliance/wasmtime/issues/10199) - x64 Wide operations used for some ALU ops

## Reviewer Checklist

- [ ] Verify the fix is appropriately scoped and doesn't introduce unintended side effects
- [ ] Consider if other problematic CPU features should also be disabled
- [ ] Evaluate performance impact on systems that could safely use AVX-512
- [ ] Confirm the `unsafe` block is properly documented and justified

---

Release Notes:

- Fixed a crash that could occur when running WASM extensions on certain Intel CPUs (particularly Alder Lake-N and systems with kernel security mitigations enabled)
