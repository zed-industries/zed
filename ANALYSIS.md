# Crash Analysis: SIGILL in wasmtime during WASM extension execution

## Crash Summary
- **Sentry Issue:** ZED-47 (https://sentry.io/organizations/zed-dev/issues/6807884895/)
- **Error:** SIGILL / ILL_ILLOPN / 0x0 (Illegal Instruction)
- **Crash Site:** wasmtime JIT-compiled code via `wasmtime::runtime::vm::traphandlers::catch_traps`
- **First Seen:** 2025-08-13
- **Last Seen:** 2026-02-14
- **Event Count:** 3050 events

## Environment
- **OS:** Linux (Ubuntu)
- **GPU:** Intel Alder Lake-N [UHD Graphics]
- **CPU:** Intel Alder Lake-N (low-power mobile/embedded processor)
- **Channel:** Preview
- **Version:** 0.203.2

## Root Cause

### Analysis
The crash is a SIGILL (Illegal Instruction) occurring inside wasmtime's JIT-compiled WebAssembly extension code. The stacktrace shows the crash happening during:

1. `wasmtime_fiber::unix::fiber_start` - Extension fiber execution starts
2. `wasmtime::runtime::vm::traphandlers::catch_traps` - Wasmtime's trap handler catches the signal
3. The actual crash occurs inside JIT-generated code (first 13 stack frames are "unknown")

### Why This Happens

The Intel Alder Lake-N is a low-power variant of the Alder Lake architecture. While standard Alder Lake processors support all modern x86-64 instruction sets (AVX, AVX2, AVX-512, etc.), the **Alder Lake-N (N-series) uses only E-cores (Gracemont) and may lack some SIMD instruction support** that the JIT compiler assumes is available.

Specifically, the Gracemont E-cores in Alder Lake-N have limited SIMD capabilities compared to the P-cores:
- They support SSE4.2 and AVX2 but **not** AVX-512
- Some specific instruction variants may behave differently

The wasmtime cranelift JIT compiler infers CPU features from the host system at runtime using `cranelift-native`. However, there can be mismatches between:
1. What the CPUID reports as available
2. What instructions the kernel allows (e.g., GDS/Downfall mitigations may disable AVX features)
3. What actually works on the specific CPU variant

According to wasmtime issue #3809, certain WASM SIMD operations require at minimum SSE4.1 support, and the WASM SIMD proposal uses SSE4.1 as the baseline for x86-64. However, there may be edge cases where:
1. The CPU feature detection incorrectly reports a feature as available
2. Kernel mitigations (like GDS/Downfall) disable certain instructions at runtime
3. The specific instruction encoding generated is not supported on the E-core variant

### Data Flow
1. User loads a WASM extension in Zed
2. `WasmHost::load_extension` compiles the WASM bytes using wasmtime's cranelift backend
3. The cranelift backend generates native x86-64 code using detected CPU features
4. When the extension is called, the JIT code executes
5. A specific instruction in the JIT code triggers SIGILL because:
   - The instruction isn't actually supported on this CPU variant, OR
   - A kernel mitigation has disabled the instruction at runtime

## Potential Solutions

### Option A: Conservative CPU Feature Detection (Recommended)
Explicitly disable advanced CPU features that may cause issues on edge-case hardware. The wasmtime `Config` provides `cranelift_flag_set` to override feature detection.

```rust
config.cranelift_flag_set("has_avx512bitalg", "false")?;
config.cranelift_flag_set("has_avx512dq", "false")?;
config.cranelift_flag_set("has_avx512f", "false")?;
config.cranelift_flag_set("has_avx512vl", "false")?;
// etc. for AVX-512 variants that may be problematic
```

**Pros:** Simple, targeted fix
**Cons:** May slightly reduce performance on systems that could use AVX-512

### Option B: Catch and Handle SIGILL in Extension Execution
The crash is already being caught by wasmtime's trap handler infrastructure. The issue is that it's being treated as a fatal crash rather than a recoverable error. We could improve error handling to:
1. Detect SIGILL during extension execution
2. Disable the extension gracefully
3. Report the issue to the user

**Pros:** Handles any future instruction compatibility issues
**Cons:** More complex, extension still won't work

### Option C: Use wasmtime's Pulley Interpreter
Wasmtime has a portable interpreter called "Pulley" that doesn't use JIT compilation. This could be used as a fallback when JIT causes issues.

**Pros:** Complete solution for CPU compatibility
**Cons:** Performance impact, additional complexity

## Reproduction

This crash is difficult to reproduce in tests because:
1. It requires specific CPU hardware (Intel Alder Lake-N)
2. It may also require specific kernel configurations (GDS mitigations)
3. The exact WASM extension and operation that triggers it is unknown

A minimal reproduction would require:
1. Running on an Alder Lake-N CPU
2. Loading a WASM extension that uses SIMD operations
3. Executing the extension

For testing purposes, we can verify that the fix (disabling problematic CPU features) is properly applied to the wasmtime configuration.

## Suggested Fix

The recommended fix is **Option A**: Add conservative CPU feature settings to the wasmtime configuration. This specifically disables AVX-512 features which are known to cause issues on certain CPUs, especially when:
- The CPU variant doesn't fully support them (E-cores in hybrid CPUs)
- Kernel mitigations have disabled them at runtime

The fix should be applied in `crates/extension_host/src/wasm_host.rs` in the `wasm_engine` function.

## Test Command

```
cargo check -p extension_host
cargo test -p extension_host test_wasm_engine_creates_successfully
```

## Implementation Notes

The fix adds a `#[cfg(target_arch = "x86_64")]` block in the `wasm_engine` function that disables all AVX-512 feature flags before creating the wasmtime Engine:

```rust
#[cfg(target_arch = "x86_64")]
{
    // SAFETY: These flags disable CPU features rather than enable them,
    // which cannot cause unsoundness - it only affects performance.
    unsafe {
        config.cranelift_flag_set("has_avx512bitalg", "false");
        config.cranelift_flag_set("has_avx512dq", "false");
        config.cranelift_flag_set("has_avx512f", "false");
        config.cranelift_flag_set("has_avx512vbmi", "false");
        config.cranelift_flag_set("has_avx512vl", "false");
    }
}
```

This is a conservative fix that trades a small amount of potential performance for improved compatibility across a wider range of x86-64 CPUs.
