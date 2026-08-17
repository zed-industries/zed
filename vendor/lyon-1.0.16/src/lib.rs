#![doc(html_logo_url = "https://nical.github.io/lyon-doc/lyon-logo.svg")]
#![deny(bare_trait_objects)]

//! GPU-based 2D graphics rendering tools in Rust using path tessellation.
//!
//! ![logo](https://nical.github.io/lyon-doc/lyon-logo.svg)
//!
//! [![crate](https://img.shields.io/crates/v/lyon.svg)](https://crates.io/crates/lyon)
//! [![ci](https://github.com/nical/lyon/actions/workflows/main.yml/badge.svg)](https://github.com/nical/lyon/actions)
//!
//! # Crates
//!
//! This meta-crate (`lyon`) reexports the following sub-crates for convenience:
//!
//! * [![crate](https://img.shields.io/crates/v/lyon_tessellation.svg)](https://crates.io/crates/lyon_tessellation)
//!   [![doc](https://docs.rs/lyon_tessellation/badge.svg)](https://docs.rs/lyon_tessellation) -
//!   **lyon_tessellation** - Path tessellation routines.
//! * [![crate](https://img.shields.io/crates/v/lyon_path.svg)](https://crates.io/crates/lyon_path)
//!   [![doc](https://docs.rs/lyon_path/badge.svg)](https://docs.rs/lyon_path) -
//!   **lyon_path** - Tools to build and iterate over paths.
//! * [![crate](https://img.shields.io/crates/v/lyon_algorithms.svg)](https://crates.io/crates/lyon_algorithms)
//!   [![doc](https://docs.rs/lyon_algorithms/badge.svg)](https://docs.rs/lyon_algorithms) -
//!   **lyon_algorithms** - Various 2d path related algorithms.
//! * [![crate](https://img.shields.io/crates/v/lyon_geom.svg)](https://crates.io/crates/lyon_geom)
//!   [![doc](https://docs.rs/lyon_geom/badge.svg)](https://docs.rs/lyon_geom) -
//!   **lyon_geom** - 2d utilities for cubic and quadratic bézier curves, arcs and more.
//! * [![crate](https://img.shields.io/crates/v/lyon_extra.svg)](https://crates.io/crates/lyon_extra)
//!   [![doc](https://docs.rs/lyon_extra/badge.svg)](https://docs.rs/lyon_extra) -
//!   **lyon_extra** - Additional testing and debugging tools.
//!
//! Each `lyon_<name>` crate is reexported as a `<name>` module in `lyon`. For example:
//!
//! ```ignore
//! extern crate lyon_tessellation;
//! use lyon_tessellation::FillTessellator;
//! ```
//!
//! Is equivalent to:
//!
//! ```ignore
//! extern crate lyon;
//! use lyon::tessellation::FillTessellator;
//! ```
//!
//! # Feature flags
//!
//! serialization using serde can be enabled on each crate using the
//! `serialization` feature flag (disabled by default).
//!
//! When using the main crate `lyon`, `lyon_extra` dependencies is disabled by default.
//! It can be added with the feature flags `extra`.
//!
//! # Additional documentation and links
//!
//! * [wgpu example](https://github.com/nical/lyon/tree/main/examples/wgpu).
//! * [wgpu_svg example](https://github.com/nical/lyon/tree/main/examples/wgpu_svg) similar to the first example,
//!   can render a very small subset of SVG.
//! * There is some useful documentation on the project's [wiki](https://github.com/nical/lyon/wiki).
//! * The source code is available on the project's [git repository](https://github.com/nical/lyon).
//! * Interested in contributing? Pull requests are welcome. If you would like to help but don't know
//!   what to do specifically, have a look at the [github issues](https://github.com/nical/lyon/issues),
//!   some of which are tagged as [easy](https://github.com/nical/lyon/issues?q=is%3Aissue+is%3Aopen+label%3Aeasy).
//!
//! # Examples
//!
//! ## Tessellating a rounded rectangle
//!
//! The `lyon_tessellation` crate provides a collection of tessellation routines
//! for common shapes such as rectangles and circles. Let's have a look at how
//! to obtain the fill tessellation a rectangle with rounded corners:
//!
//! ```
//! use lyon::math::{Box2D, Point, point};
//! use lyon::path::{Winding, builder::BorderRadii};
//! use lyon::tessellation::{FillTessellator, FillOptions, VertexBuffers};
//! use lyon::tessellation::geometry_builder::simple_builder;
//!
//! fn main() {
//!     let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();
//!     let mut geometry_builder = simple_builder(&mut geometry);
//!     let options = FillOptions::tolerance(0.1);
//!     let mut tessellator = FillTessellator::new();
//!
//!     let mut builder = tessellator.builder(
//!         &options,
//!         &mut geometry_builder,
//!     );
//!
//!     builder.add_rounded_rectangle(
//!         &Box2D { min: point(0.0, 0.0), max: point(100.0, 50.0) },
//!         &BorderRadii {
//!             top_left: 10.0,
//!             top_right: 5.0,
//!             bottom_left: 20.0,
//!             bottom_right: 25.0,
//!         },
//!         Winding::Positive,
//!     );
//!
//!     builder.build();
//!
//!     // The tessellated geometry is ready to be uploaded to the GPU.
//!     println!(" -- {} vertices {} indices",
//!         geometry.vertices.len(),
//!         geometry.indices.len()
//!     );
//! }
//!
//! ```
//!
//! ## Building and tessellating an arbitrary path
//!
//! ```
//! extern crate lyon;
//! use lyon::math::{point, Point};
//! use lyon::path::Path;
//! use lyon::path::builder::*;
//! use lyon::tessellation::*;
//!
//! fn main() {
//!     // Build a Path.
//!     let mut builder = Path::builder();
//!     builder.begin(point(0.0, 0.0));
//!     builder.line_to(point(1.0, 0.0));
//!     builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
//!     builder.cubic_bezier_to(point(1.0, 1.0), point(0.0, 1.0), point(0.0, 0.0));
//!     builder.close();
//!     let path = builder.build();
//!
//!     // Let's use our own custom vertex type instead of the default one.
//!     #[derive(Copy, Clone, Debug)]
//!     struct MyVertex { position: [f32; 2] };
//!
//!     // Will contain the result of the tessellation.
//!     let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
//!
//!     let mut tessellator = FillTessellator::new();
//!
//!     {
//!         // Compute the tessellation.
//!         tessellator.tessellate_path(
//!             &path,
//!             &FillOptions::default(),
//!             &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
//!                 MyVertex {
//!                     position: vertex.position().to_array(),
//!                 }
//!             }),
//!         ).unwrap();
//!     }
//!
//!     // The tessellated geometry is ready to be uploaded to the GPU.
//!     println!(" -- {} vertices {} indices",
//!         geometry.vertices.len(),
//!         geometry.indices.len()
//!     );
//! }
//! ```
//!
//! ## What is the tolerance variable in these examples?
//!
//! The tessellator operates on flattened paths (that only contains line segments)
//! so we have to approximate the curves segments with sequences of line segments.
//! To do so we pick a tolerance threshold which is the maximum distance allowed
//! between the curve and its approximation.
//! The documentation of the [lyon_geom](https://docs.rs/lyon_geom) crate provides
//! more detailed explanations about this tolerance parameter.
//!
//! ## Rendering the tessellated geometry
//!
//! Lyon does not provide with any GPU abstraction or rendering backend (for now).
//! It is up to the user of this crate to decide whether to use OpenGL, vulkan, gfx-rs,
//! wgpu, glium, or any low level graphics API and how to render it.
//! The [wgpu example](https://github.com/nical/lyon/tree/main/examples/wgpu)
//! can be used to get an idea of how to render the geometry (in this case
//! using wgpu).

pub extern crate lyon_algorithms;
#[cfg(feature = "extra")]
pub extern crate lyon_extra;
pub extern crate lyon_tessellation;

pub use lyon_algorithms as algorithms;
#[cfg(feature = "extra")]
pub use lyon_extra as extra;
pub use lyon_tessellation as tessellation;
pub use tessellation::geom;
pub use tessellation::path;

pub use path::math;
