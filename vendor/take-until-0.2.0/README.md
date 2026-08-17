# Take Until

[![Actions Status](https://github.com/hdevalke/take-until/workflows/Rust/badge.svg)](https://github.com/hdevalke/take-until/actions)


This crate adds the `take_until` method as an extension for iterators.

## Examples

### Parsing the next base 128 varint from a byte slice.

```rust
use take_until::TakeUntilExt;

let varint = &[0b1010_1100u8, 0b0000_0010, 0b1000_0001];
let int: u32 = varint
    .iter()
    .take_until(|b| (**b & 0b1000_0000) == 0)
    .enumerate()
    .fold(0, |acc, (i, b)| {
        acc | ((*b & 0b0111_1111) as u32) << (i * 7)
     });
assert_eq!(300, int);
```

### Take Until vs Take While (from Standard Library)
```rust
use take_until::TakeUntilExt;

fn main() {
    let items = [1, 2, 3, 4, -5, -6, -7, -8];
    let filtered_take_while = items
        .into_iter()
        .take_while(|x| *x > 0)
        .collect::<Vec<i32>>();
    let filtered_take_until = items
        .into_iter()
        .take_until(|x| *x <= 0)
        .collect::<Vec<i32>>();
    assert_eq!([1, 2, 3, 4], filtered_take_while.as_slice());
    assert_eq!([1, 2, 3, 4, -5], filtered_take_until.as_slice());
}
```

## MSRV
The MSRV is `1.56.1` stable.
