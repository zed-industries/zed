# Helpers to write more compact simd code

## Implemented so far
- [x] `cold_for_target_arch` mark a function cold for certain arches only
- [ ] `cold_for_target_feature`

## Example
``` toml
[dependencies]
simd_helpers = "0.1"
```
``` rust
use simd_helpers::cold_for_target_arch;

// On arm and power it is the main, impl for x86_64 there is a asm-optimized variant
#[cold_for_target_arch("x86_64")]
fn fallback_simple_impl() { ... }
```
