# Étagère

<p align="center">
  <a href="https://crates.io/crates/etagere">
      <img src="https://img.shields.io/crates/v/etagere.svg" alt="crates.io">
  </a>
  <a href="https://docs.rs/etagere">
      <img src="https://docs.rs/etagere/badge.svg" alt="documentation">
  </a>

</p>

A dynamic texture atlas allocator using the shelf packing algorithm.

## Motivation

The ability to dynamically batch textures together is important for some graphics rendering scenarios (for example [WebRender](https://github.com/servo/webrender)).

The shelf packing algorithm works very well when there is a high number of items with similar sizes, for example for dynamic glyph atlases.

See also [guillotière](https://github.com/nical/guillotiere), another dynamic atlas allocator based on a different algorithm, with different packing and performance characteristics.

## Example

```rust
use etagere::*;

let mut atlas = AtlasAllocator::new(size2(1000, 1000));

let a = atlas.allocate(size2(100, 100)).unwrap();
let b = atlas.allocate(size2(900, 200)).unwrap();

atlas.deallocate(a.id);

let c = atlas.allocate(size2(300, 200)).unwrap();

assert_eq!(c.rectangle, atlas.get(c.id));

atlas.deallocate(c.id);
atlas.deallocate(b.id);
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
