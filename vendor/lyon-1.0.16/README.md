# Lyon

A path tessellation library written in rust for GPU-based 2D graphics rendering.

<p align="center">
<img src="https://nical.github.io/lyon-doc/lyon-logo.svg" alt="Project logo">
</p>

<p align="center">
  <a href="https://crates.io/crates/lyon">
      <img src="https://img.shields.io/crates/v/lyon.svg" alt="crates.io">
  </a>
  <a href="https://github.com/nical/lyon/actions">
      <img src="https://github.com/nical/lyon/actions/workflows/main.yml/badge.svg" alt="Build Status">
  </a>
  <a href="https://docs.rs/lyon">
      <img src="https://docs.rs/lyon/badge.svg" alt="documentation">
  </a>

  <a href="https://gitter.im/lyon-rs/Lobby">
    <img src="https://img.shields.io/badge/GITTER-join%20chat-green.svg" alt="Gitter Chat">
  </a>

</p>

## Example

```rust
extern crate lyon;
use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::*;

fn main() {
    // Build a Path.
    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(1.0, 0.0));
    builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
    builder.cubic_bezier_to(point(1.0, 1.0), point(0.0, 1.0), point(0.0, 0.0));
    builder.end(true);
    let path = builder.build();
    // Let's use our own custom vertex type instead of the default one.
    #[derive(Copy, Clone, Debug)]
    struct MyVertex { position: [f32; 2] };
    // Will contain the result of the tessellation.
    let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();
    {
        // Compute the tessellation.
        tessellator.tessellate_path(
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                MyVertex {
                    position: vertex.position().to_array(),
                }
            }),
        ).unwrap();
    }
    // The tessellated geometry is ready to be uploaded to the GPU.
    println!(" -- {} vertices {} indices",
        geometry.vertices.len(),
        geometry.indices.len()
    );
}
```
