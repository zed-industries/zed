# Linked Issues for ZED-47 (SIGILL in wasmtime extension execution)

## High Confidence

### wasmtime Issue #3809: Minimum x86_64 feature support required for SIMD proposal
**URL:** https://github.com/bytecodealliance/wasmtime/issues/3809
**Relevance:** Directly relevant - discusses the minimum CPU feature requirements for WASM SIMD operations and how cranelift may panic when features aren't available.
**Key Quote:** "Currently in Wasmtime it appears that at least for some instructions the SSE 4.1 extensions are required. If those are not present then two crashes found so far are... Cannot emit inst for target; failed to match ISA requirements"

### wasmtime Issue #10199: x64 Wide operations used for some ALU ops
**URL:** https://github.com/bytecodealliance/wasmtime/issues/10199
**Relevance:** Related to x86-64 code generation decisions in cranelift that could cause issues on specific CPU variants.

## Medium Confidence

### GitHub gemini-cli Issue #13046: SIGILL crash on processors without AES-NI support
**URL:** https://github.com/google-gemini/gemini-cli/issues/13046
**Relevance:** Similar pattern - SIGILL crash due to compiled code using CPU instructions not supported by the target processor. Different root cause (AES-NI vs potential AVX/SIMD) but same symptom.

### wasmtime Issue #8898: Unable to link a binary with wasmtime and musl
**URL:** https://github.com/bytecodealliance/wasmtime/issues/8898
**Relevance:** Related to CPU feature detection issues when using wasmtime on specific architectures.

### GitHub element-desktop Issue #1385: Illegal instruction on Skylake CPUs with GDS/Downfall mitigation
**URL:** https://github.com/element-hq/element-desktop/issues/1385
**Relevance:** Similar pattern - SIGILL crash caused by kernel mitigations disabling AVX instructions at runtime while the code still attempts to use them.

### wasmtime Issue #10283: Help debugging a seemingly random segfault in wasmtime
**URL:** https://github.com/bytecodealliance/wasmtime/issues/10283
**Relevance:** Reports intermittent crashes in wasmtime after version upgrades, similar to the pattern seen here.

## Low Confidence

### Zed Issue #26143: Linux session crash after Zed launch - GPU hung
**URL:** https://github.com/zed-industries/zed/issues/26143
**Relevance:** Linux crash on similar hardware (Intel graphics) but appears to be GPU-related rather than WASM extension related.

### Zed Issue #14554: Crashing when starting on Wayland with Intel GPU
**URL:** https://github.com/zed-industries/zed/issues/14554
**Relevance:** Crash on Intel GPU on Linux, but specifically Wayland/Vulkan related, not WASM.

### Zed extensions Issue #346: Local extension doesn't load
**URL:** https://github.com/zed-industries/extensions/issues/346
**Relevance:** Extension loading issues, but appears to be file parsing related not SIGILL.

### wasmtime Issue #11958: Wasmtime crashes in release mode with Rust 1.91
**URL:** https://github.com/bytecodealliance/wasmtime/issues/11958
**Relevance:** Wasmtime crash, but memory allocation failure rather than SIGILL.

## Summary

The most relevant upstream issues are:
1. **wasmtime #3809** - Establishes that WASM SIMD requires SSE4.1 minimum and documents similar crashes
2. **element-desktop #1385** - Shows that kernel mitigations (GDS/Downfall) can cause SIGILL even when CPUID reports feature support

The crash pattern suggests that either:
1. The Intel Alder Lake-N E-cores don't fully support some instruction the JIT generates
2. Kernel mitigations are disabling instructions at runtime after CPUID detection
3. There's a specific instruction encoding issue on this CPU variant
