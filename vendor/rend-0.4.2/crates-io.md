rend is a library that provides endian-aware primitives for Rust.

---

## rend in action

```rust
use rend::*;

let little_int = i32_le::new(0x12345678);
// Internal representation is little-endian
assert_eq!([0x78, 0x56, 0x34, 0x12], unsafe { ::core::mem::transmute::<_, [u8; 4]>(little_int) });

// Can also be made with `.into()`
let little_int: i32_le = 0x12345678.into();
// Still formats correctly
assert_eq!("305419896", format!("{}", little_int));
assert_eq!("0x12345678", format!("0x{:x}", little_int));

let big_int = i32_be::new(0x12345678);
// Internal representation is big-endian
assert_eq!([0x12, 0x34, 0x56, 0x78], unsafe { ::core::mem::transmute::<_, [u8; 4]>(big_int) });

// Can also be made with `.into()`
let big_int: i32_be = 0x12345678.into();
// Still formats correctly
assert_eq!("305419896", format!("{}", big_int));
assert_eq!("0x12345678", format!("0x{:x}", big_int));
```