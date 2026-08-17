# ArcSwap

[![Actions Status](https://github.com/vorner/arc-swap/workflows/test/badge.svg)](https://github.com/vorner/arc-swap/actions)
[![Codecov](https://codecov.io/gh/vorner/arc-swap/branch/master/graph/badge.svg?token=3KA3R2D9fV)](https://codecov.io/gh/vorner/arc-swap)
[![Docs](https://docs.rs/arc-swap/badge.svg)](https://docs.rs/arc-swap)

This provides something similar to what `RwLock<Arc<T>>` is or what
`Atomic<Arc<T>>` would be if it existed, optimized for read-mostly write-seldom
scenarios, with consistent performance characteristics.

Read [the documentation](https://docs.rs/arc-swap) before using.

## Rust version policy

The 1. version will build on any edition 2018 capable compiler. This does not
include:

* Tests. Tests build and run on recent compilers, mostly because of
  dependencies.
* Additional feature flags. Most feature flags are guaranteed to build since the
  version they are introduced. Experimental features are without any guarantees.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/license/mit)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`AtomicPtr`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html
[`ArcSwap`]: https://docs.rs/arc-swap/*/arc_swap/type.ArcSwap.html
