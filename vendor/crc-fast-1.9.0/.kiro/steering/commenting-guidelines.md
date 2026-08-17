---
inclusion: always
---

# Code Commenting Guidelines

## Comment Philosophy

- Only add comments when they explain WHY something is done a particular way
- NEVER explain WHAT the code is doing unless it's hard to understand without a comment
- Code should be self-documenting through clear naming and structure
- Comments should provide context, reasoning, or non-obvious implications

## Examples

### Good Comments (WHY)
```rust
// Use 512KiB chunks because benchmarks showed this was fastest on Apple M2 Ultra
let chunk_size = chunk_size.unwrap_or(524288);

// Remove xorout since it's already been applied and needs to be re-added on final output
self.state = combine::checksums(
    self.state ^ self.params.xorout,
    other_crc,
    other.amount,
    self.params,
) ^ self.params.xorout;
```

### Avoid (WHAT)
```rust
// Set chunk_size to 524288 if None
let chunk_size = chunk_size.unwrap_or(524288);

// XOR the state with xorout
self.state = self.state ^ self.params.xorout;
```

### Exception: Complex Logic
Comments explaining WHAT are acceptable when the code logic is genuinely hard to follow:
```rust
// Fold 8 bytes at a time using SIMD, then handle remainder with scalar operations
```