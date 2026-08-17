# Circular Buffer for Rust

[![Crate](https://img.shields.io/crates/v/circular-buffer)](https://crates.io/crates/circular-buffer) [![Documentation](https://img.shields.io/docsrs/circular-buffer)](https://docs.rs/circular-buffer/latest/circular_buffer/) [![License](https://img.shields.io/crates/l/circular-buffer)](https://choosealicense.com/licenses/bsd-3-clause/)

This is a Rust crate that implements a [circular buffer], also known as cyclic
buffer, circular queue or ring.

This circular buffer has a fixed maximum capacity, does not automatically grow,
and once its maximum capacity is reached, elements at the start of the buffer
are overwritten. It's useful for implementing fast FIFO (_first in, first out_)
and LIFO (_last in, first out_) queues with a fixed memory capacity.

For more information and examples, check out the [documentation]!

[circular buffer]: https://en.wikipedia.org/wiki/Circular_buffer
[documentation]: https://docs.rs/circular-buffer/latest/circular_buffer/

# Changelog

For a full list of changes between releases, visit
[GitHub](https://github.com/andreacorbellini/rust-circular-buffer/releases).

# Example

```rust
use circular_buffer::CircularBuffer;

// Initialize a new, empty circular buffer with a capacity of 5 elements
let mut buf = CircularBuffer::<5, u32>::new();

// Add a few elements
buf.push_back(1);
buf.push_back(2);
buf.push_back(3);
assert_eq!(buf, [1, 2, 3]);

// Add more elements to fill the buffer capacity completely
buf.push_back(4);
buf.push_back(5);
assert_eq!(buf, [1, 2, 3, 4, 5]);

// Adding more elements than the buffer can contain causes the front elements to be
// automatically dropped
buf.push_back(6);
assert_eq!(buf, [2, 3, 4, 5, 6]); // `1` got dropped to make room for `6`
```
