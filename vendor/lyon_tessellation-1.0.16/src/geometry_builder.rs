//! Tools to help with generating vertex and index buffers.
//!
//! ## Overview
//!
//! While it would be possible for the tessellation algorithms to manually generate vertex
//! and index buffers with a certain layout, it would mean that most code using the tessellators
//! have to copy and convert all generated vertices in order to have their own vertex
//! layout, or de-interleaved vertex formats, which is a very common use-case.
//!
//! In order to flexibly and efficiently build geometry of various flavors, this module contains
//! a number of builder interfaces that centered around the idea of building vertex and index
//! buffers without having to know about the final vertex and index types.
//!
//! See:
//!
//! * [`GeometryBuilder`](trait.GeometryBuilder.html)
//! * [`FillGeometryBuilder`](trait.FillGeometryBuilder.html)
//! * [`StrokeGeometryBuilder`](trait.StrokeGeometryBuilder.html)
//!
//! The traits above are what the tessellators interface with. It is very common to push
//! vertices and indices into a pair of vectors, so to facilitate this pattern this module
//! also provides:
//!
//! * The struct [`VertexBuffers`](struct.VertexBuffers.html) is a simple pair of vectors of
//!   indices and vertices (generic parameters).
//! * The struct [`BuffersBuilder`](struct.BuffersBuilder.html) which writes into a
//!   [`VertexBuffers`](struct.VertexBuffers.html) and implements the various geometry
//!   builder traits. It takes care of filling the buffers while producing vertices is
//!   delegated to a vertex constructor.
//! * The traits [`FillVertexConstructor`](trait.FillVertexConstructor.html),
//!   [`StrokeVertexConstructor`](trait.StrokeVertexConstructor.html) and
//!   [`BuffersBuilder`](struct.BuffersBuilder.html) in order to generate any vertex type. In the
//!   first example below, a struct `WithColor` implements the `FillVertexConstructor` trait in order to
//!   create vertices composed of a 2d position and a color value from an input 2d position.
//!   This separates the construction of vertex values from the assembly of the vertex buffers.
//!   Another, simpler example of vertex constructor is the [`Positions`](struct.Positions.html)
//!   constructor which just returns the vertex position untransformed.
//!
//! Geometry builders are a practical way to add one last step to the tessellation pipeline,
//! such as applying a transform or clipping the geometry.
//!
//! While this is module designed to facilitate the generation of vertex buffers and index
//! buffers, nothing prevents a given GeometryBuilder implementation to only generate a
//! vertex buffer without indices, or write into a completely different format.
//! These builder traits are at the end of the tessellation pipelines and are meant for
//! users of this crate to be able to adapt the output of the tessellators to their own
//! needs.
//!
//! ## Do I need to implement geometry builders or vertex constructors?
//!
//! If you only generate a vertex buffer and an index buffer (as a pair of standard `Vec`),
//! then the simplest option is to work with custom vertex constructors and use
//! `VertexBuffers` and `BuffersBuilder`.
//!
//! For more specific or elaborate use cases where control over where the vertices as written
//! is needed such as building de-interleaved vertex buffers or writing directly into a mapped
//! GPU buffer, implementing custom geometry builders is the right thing to do.
//!
//! Which of the vertex constructor or geometry builder traits to implement (fill/stroke/basic
//! variants), depends on which tessellators the builder or constructor will interface with.
//!
//! ## Examples
//!
//! ### Generating custom vertices
//!
//! The example below implements the `FillVertexConstructor` trait in order to use a custom
//! vertex type `MyVertex` (containing position and color), storing the tessellation in a
//! `VertexBuffers<MyVertex, u16>`, and tessellates two shapes with different colors.
//!
//! ```
//! extern crate lyon_tessellation as tess;
//! use tess::{FillVertexConstructor, VertexBuffers, BuffersBuilder, FillOptions, FillTessellator, FillVertex};
//! use tess::math::{Point, point};
//!
//! // Our custom vertex.
//! #[derive(Copy, Clone, Debug)]
//! pub struct MyVertex {
//!   position: [f32; 2],
//!   color: [f32; 4],
//! }
//!
//! // The vertex constructor. This is the object that will be used to create the custom
//! // vertices from the information provided by the tessellators.
//! struct WithColor([f32; 4]);
//!
//! impl FillVertexConstructor<MyVertex> for WithColor {
//!     fn new_vertex(&mut self, vertex: FillVertex) -> MyVertex {
//!         MyVertex {
//!             position: vertex.position().to_array(),
//!             color: self.0,
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let mut output: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
//!     let mut tessellator = FillTessellator::new();
//!     // Tessellate a red and a green circle.
//!     tessellator.tessellate_circle(
//!         point(0.0, 0.0),
//!         10.0,
//!         &FillOptions::tolerance(0.05),
//!         &mut BuffersBuilder::new(
//!             &mut output,
//!             WithColor([1.0, 0.0, 0.0, 1.0])
//!         ),
//!     );
//!     tessellator.tessellate_circle(
//!         point(10.0, 0.0),
//!         5.0,
//!         &FillOptions::tolerance(0.05),
//!         &mut BuffersBuilder::new(
//!             &mut output,
//!             WithColor([0.0, 1.0, 0.0, 1.0])
//!         ),
//!     );
//!
//!     println!(" -- {} vertices, {} indices", output.vertices.len(), output.indices.len());
//! }
//! ```
//!
//! ### Generating a completely custom output
//!
//! Using `VertexBuffers<T>` is convenient and probably fits a lot of use cases, but
//! what if we do not want to write the geometry in a pair of vectors?
//! Perhaps we want to write the geometry in a different data structure or directly
//! into gpu-accessible buffers mapped on the CPU?
//!
//! ```
//! extern crate lyon_tessellation as tess;
//! use tess::{StrokeTessellator, GeometryBuilder, StrokeGeometryBuilder, StrokeOptions, GeometryBuilderError, StrokeVertex, VertexId};
//! use tess::math::{Point, point};
//! use tess::path::polygon::Polygon;
//! use std::fmt::Debug;
//! use std::u32;
//!
//! // A geometry builder that writes the result of the tessellation to stdout instead
//! // of filling vertex and index buffers.
//! pub struct ToStdOut {
//!     vertices: u32,
//!     indices: u32,
//! }
//!
//! impl ToStdOut {
//!      pub fn new() -> Self { ToStdOut { vertices: 0, indices: 0 } }
//! }
//!
//! impl GeometryBuilder for ToStdOut {
//!     fn begin_geometry(&mut self) {
//!         // Reset the vertex in index counters.
//!         self.vertices = 0;
//!         self.indices = 0;
//!         println!(" -- begin geometry");
//!     }
//!
//!     fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId) {
//!         println!("triangle ({}, {}, {})", a.offset(), b.offset(), c.offset());
//!         self.indices += 3;
//!     }
//!
//!     fn abort_geometry(&mut self) {
//!         println!(" -- oops!");
//!     }
//! }
//!
//! impl StrokeGeometryBuilder for ToStdOut {
//!     fn add_stroke_vertex(&mut self, vertex: StrokeVertex) -> Result<VertexId, GeometryBuilderError> {
//!         println!("vertex {:?}", vertex.position());
//!         if self.vertices >= u32::MAX {
//!             return Err(GeometryBuilderError::TooManyVertices);
//!         }
//!         self.vertices += 1;
//!         Ok(VertexId(self.vertices as u32 - 1))
//!     }
//! }
//!
//! fn main() {
//!     let mut output = ToStdOut::new();
//!     let mut tessellator = StrokeTessellator::new();
//!     tessellator.tessellate_polygon(
//!         Polygon {
//!             points: &[point(0.0, 0.0), point(10.0, 0.0), point(5.0, 5.0)],
//!             closed: true,
//!         },
//!         &StrokeOptions::default(),
//!         &mut output,
//!     );
//! }
//! ```
//!

pub use crate::error::GeometryBuilderError;
use crate::math::Point;
use crate::{FillVertex, Index, StrokeVertex, VertexId};

use alloc::vec::Vec;
use core::convert::From;
use core::ops::Add;

/// An interface separating tessellators and other geometry generation algorithms from the
/// actual vertex construction.
///
/// Depending on which tessellator a geometry builder interfaces with, it also has to
/// implement one or several of the following traits (Which contain the hooks to generate
/// vertices):
///  - [`FillGeometryBuilder`](trait.FillGeometryBuilder.html)
///  - [`StrokeGeometryBuilder`](trait.StrokeGeometryBuilder.html)
///
/// See the [`geometry_builder`](index.html) module documentation for more detailed explanation.
pub trait GeometryBuilder {
    /// Called at the beginning of a generation.
    ///
    /// end_geometry must be called before begin_geometry is called again.
    fn begin_geometry(&mut self) {}

    /// Called at the end of a generation.
    /// Returns the number of vertices and indices added since the last time begin_geometry was
    /// called.
    fn end_geometry(&mut self) {}

    /// Insert a triangle made of vertices that were added after the last call to begin_geometry.
    ///
    /// This method can only be called between begin_geometry and end_geometry.
    fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId);

    /// abort_geometry is called instead of end_geometry if an error occurred while producing
    /// the geometry and we won't be able to finish.
    ///
    /// The implementation is expected to discard the geometry that was generated since the last
    /// time begin_geometry was called, and to remain in a usable state.
    fn abort_geometry(&mut self) {}
}

/// A Geometry builder to interface with the [`FillTessellator`](../struct.FillTessellator.html).
///
/// Types implementing this trait must also implement the [`GeometryBuilder`](trait.GeometryBuilder.html) trait.
pub trait FillGeometryBuilder: GeometryBuilder {
    /// Inserts a vertex, providing its position, and optionally a normal.
    /// Returns a vertex id that is only valid between begin_geometry and end_geometry.
    ///
    /// This method can only be called between begin_geometry and end_geometry.
    fn add_fill_vertex(&mut self, vertex: FillVertex) -> Result<VertexId, GeometryBuilderError>;
}

/// A Geometry builder to interface with the [`StrokeTessellator`](../struct.StrokeTessellator.html).
///
/// Types implementing this trait must also implement the [`GeometryBuilder`](trait.GeometryBuilder.html) trait.
pub trait StrokeGeometryBuilder: GeometryBuilder {
    /// Inserts a vertex, providing its position, and optionally a normal.
    /// Returns a vertex id that is only valid between begin_geometry and end_geometry.
    ///
    /// This method can only be called between begin_geometry and end_geometry.
    fn add_stroke_vertex(&mut self, vertex: StrokeVertex)
        -> Result<VertexId, GeometryBuilderError>;
}

/// Structure that holds the vertex and index data.
///
/// Usually written into though temporary `BuffersBuilder` objects.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct VertexBuffers<OutputVertex, OutputIndex> {
    pub vertices: Vec<OutputVertex>,
    pub indices: Vec<OutputIndex>,
}

impl<OutputVertex, OutputIndex> VertexBuffers<OutputVertex, OutputIndex> {
    /// Constructor
    pub fn new() -> Self {
        VertexBuffers::with_capacity(512, 1024)
    }

    /// Constructor
    pub fn with_capacity(num_vertices: usize, num_indices: usize) -> Self {
        VertexBuffers {
            vertices: Vec::with_capacity(num_vertices),
            indices: Vec::with_capacity(num_indices),
        }
    }

    /// Empty the buffers without freeing memory, for reuse without reallocation.
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}

/// A temporary view on a `VertexBuffers` object which facilitate the population of vertex and index
/// data.
///
/// `BuffersBuilders` record the vertex offset from when they are created so that algorithms using
/// them don't need to worry about offsetting indices if some geometry was added beforehand. This
/// means that from the point of view of a `BuffersBuilder` user, the first added vertex is at always
/// offset at the offset 0 and `VertexBuilder` takes care of translating indices adequately.
///
/// Often, algorithms are built to generate vertex positions without knowledge of eventual other
/// vertex vertex. The `VertexConstructor` does the translation from generic `Input` to `OutputVertex`.
/// If your logic generates the actual vertex type directly, you can use the `SimpleBuffersBuilder`
/// convenience typedef.
pub struct BuffersBuilder<'l, OutputVertex: 'l, OutputIndex: 'l, Ctor> {
    buffers: &'l mut VertexBuffers<OutputVertex, OutputIndex>,
    first_vertex: Index,
    first_index: Index,
    vertex_offset: Index,
    vertex_constructor: Ctor,
}

impl<'l, OutputVertex: 'l, OutputIndex: 'l, Ctor>
    BuffersBuilder<'l, OutputVertex, OutputIndex, Ctor>
{
    pub fn new(buffers: &'l mut VertexBuffers<OutputVertex, OutputIndex>, ctor: Ctor) -> Self {
        let first_vertex = buffers.vertices.len() as Index;
        let first_index = buffers.indices.len() as Index;
        BuffersBuilder {
            buffers,
            first_vertex,
            first_index,
            vertex_offset: 0,
            vertex_constructor: ctor,
        }
    }

    pub fn with_vertex_offset(mut self, offset: Index) -> Self {
        self.vertex_offset = offset;

        self
    }

    /// Consumes self and returns a builder with opposite triangle face winding.
    pub fn with_inverted_winding(self) -> InvertWinding<Self> {
        InvertWinding(self)
    }

    pub fn buffers<'a, 'b: 'a>(&'b self) -> &'a VertexBuffers<OutputVertex, OutputIndex> {
        self.buffers
    }
}

/// A wrapper for stroke and fill geometry builders that inverts the triangle face winding.
pub struct InvertWinding<B>(B);

impl<B: GeometryBuilder> GeometryBuilder for InvertWinding<B> {
    fn begin_geometry(&mut self) {
        self.0.begin_geometry();
    }

    fn end_geometry(&mut self) {
        self.0.end_geometry()
    }

    fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId) {
        // Invert the triangle winding by flipping b and c.
        self.0.add_triangle(a, c, b);
    }

    fn abort_geometry(&mut self) {
        self.0.abort_geometry();
    }
}

impl<B: FillGeometryBuilder> FillGeometryBuilder for InvertWinding<B> {
    #[inline]
    fn add_fill_vertex(&mut self, vertex: FillVertex) -> Result<VertexId, GeometryBuilderError> {
        self.0.add_fill_vertex(vertex)
    }
}

impl<B: StrokeGeometryBuilder> StrokeGeometryBuilder for InvertWinding<B> {
    #[inline]
    fn add_stroke_vertex(
        &mut self,
        vertex: StrokeVertex,
    ) -> Result<VertexId, GeometryBuilderError> {
        self.0.add_stroke_vertex(vertex)
    }
}

/// A trait specifying how to create vertex values.
pub trait FillVertexConstructor<OutputVertex> {
    fn new_vertex(&mut self, vertex: FillVertex) -> OutputVertex;
}

/// A trait specifying how to create vertex values.
pub trait StrokeVertexConstructor<OutputVertex> {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> OutputVertex;
}

/// A simple vertex constructor that just takes the position.
pub struct Positions;

impl FillVertexConstructor<Point> for Positions {
    fn new_vertex(&mut self, vertex: FillVertex) -> Point {
        vertex.position()
    }
}

impl StrokeVertexConstructor<Point> for Positions {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Point {
        vertex.position()
    }
}

impl<F, OutputVertex> FillVertexConstructor<OutputVertex> for F
where
    F: Fn(FillVertex) -> OutputVertex,
{
    fn new_vertex(&mut self, vertex: FillVertex) -> OutputVertex {
        self(vertex)
    }
}

impl<F, OutputVertex> StrokeVertexConstructor<OutputVertex> for F
where
    F: Fn(StrokeVertex) -> OutputVertex,
{
    fn new_vertex(&mut self, vertex: StrokeVertex) -> OutputVertex {
        self(vertex)
    }
}

/// A `BuffersBuilder` that takes the actual vertex type as input.
pub type SimpleBuffersBuilder<'l> = BuffersBuilder<'l, Point, u16, Positions>;

/// Creates a `SimpleBuffersBuilder`.
pub fn simple_builder(buffers: &mut VertexBuffers<Point, u16>) -> SimpleBuffersBuilder {
    let first_vertex = buffers.vertices.len() as Index;
    let first_index = buffers.indices.len() as Index;
    BuffersBuilder {
        buffers,
        first_vertex,
        first_index,
        vertex_offset: 0,
        vertex_constructor: Positions,
    }
}

impl<'l, OutputVertex, OutputIndex, Ctor> GeometryBuilder
    for BuffersBuilder<'l, OutputVertex, OutputIndex, Ctor>
where
    OutputVertex: 'l,
    OutputIndex: Add + From<VertexId> + MaxIndex,
{
    fn begin_geometry(&mut self) {
        self.first_vertex = self.buffers.vertices.len() as Index;
        self.first_index = self.buffers.indices.len() as Index;
    }

    fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId) {
        #[cfg(feature = "std")]
        if a == b || a == c || b == c {
            std::println!("bad triangle {a:?} {b:?} {c:?}");
        }
        debug_assert!(a != b);
        debug_assert!(a != c);
        debug_assert!(b != c);
        debug_assert!(a != VertexId::INVALID);
        debug_assert!(b != VertexId::INVALID);
        debug_assert!(c != VertexId::INVALID);
        self.buffers.indices.push((a + self.vertex_offset).into());
        self.buffers.indices.push((b + self.vertex_offset).into());
        self.buffers.indices.push((c + self.vertex_offset).into());
    }

    fn abort_geometry(&mut self) {
        self.buffers.vertices.truncate(self.first_vertex as usize);
        self.buffers.indices.truncate(self.first_index as usize);
    }
}

impl<'l, OutputVertex, OutputIndex, Ctor> FillGeometryBuilder
    for BuffersBuilder<'l, OutputVertex, OutputIndex, Ctor>
where
    OutputVertex: 'l,
    OutputIndex: Add + From<VertexId> + MaxIndex,
    Ctor: FillVertexConstructor<OutputVertex>,
{
    fn add_fill_vertex(&mut self, vertex: FillVertex) -> Result<VertexId, GeometryBuilderError> {
        self.buffers
            .vertices
            .push(self.vertex_constructor.new_vertex(vertex));
        let len = self.buffers.vertices.len();
        if len > OutputIndex::MAX {
            return Err(GeometryBuilderError::TooManyVertices);
        }
        Ok(VertexId((len - 1) as Index))
    }
}

impl<'l, OutputVertex, OutputIndex, Ctor> StrokeGeometryBuilder
    for BuffersBuilder<'l, OutputVertex, OutputIndex, Ctor>
where
    OutputVertex: 'l,
    OutputIndex: Add + From<VertexId> + MaxIndex,
    Ctor: StrokeVertexConstructor<OutputVertex>,
{
    fn add_stroke_vertex(&mut self, v: StrokeVertex) -> Result<VertexId, GeometryBuilderError> {
        self.buffers
            .vertices
            .push(self.vertex_constructor.new_vertex(v));
        let len = self.buffers.vertices.len();
        if len > OutputIndex::MAX {
            return Err(GeometryBuilderError::TooManyVertices);
        }
        Ok(VertexId((len - 1) as Index))
    }
}

/// A geometry builder that does not output any geometry.
///
/// Mostly useful for testing.
pub struct NoOutput {
    next_vertex: u32,
}

impl NoOutput {
    pub fn new() -> Self {
        NoOutput { next_vertex: 0 }
    }
}

impl Default for NoOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl GeometryBuilder for NoOutput {
    fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId) {
        debug_assert!(a != b);
        debug_assert!(a != c);
        debug_assert!(b != c);
    }
}

impl FillGeometryBuilder for NoOutput {
    fn add_fill_vertex(&mut self, _vertex: FillVertex) -> Result<VertexId, GeometryBuilderError> {
        if self.next_vertex == u32::MAX {
            return Err(GeometryBuilderError::TooManyVertices);
        }
        self.next_vertex += 1;
        Ok(VertexId(self.next_vertex - 1))
    }
}

impl StrokeGeometryBuilder for NoOutput {
    fn add_stroke_vertex(&mut self, _: StrokeVertex) -> Result<VertexId, GeometryBuilderError> {
        if self.next_vertex == u32::MAX {
            return Err(GeometryBuilderError::TooManyVertices);
        }
        self.next_vertex += 1;
        Ok(VertexId(self.next_vertex - 1))
    }
}

/// Provides the maximum value of an index.
///
/// This should be the maximum value representable by the index type up
/// to u32::MAX because the tessellators can't internally represent more
/// than u32::MAX indices.
pub trait MaxIndex {
    const MAX: usize;
}

impl MaxIndex for u8 {
    const MAX: usize = u8::MAX as usize;
}
impl MaxIndex for i8 {
    const MAX: usize = i8::MAX as usize;
}
impl MaxIndex for u16 {
    const MAX: usize = u16::MAX as usize;
}
impl MaxIndex for i16 {
    const MAX: usize = i16::MAX as usize;
}
impl MaxIndex for u32 {
    const MAX: usize = u32::MAX as usize;
}
impl MaxIndex for i32 {
    const MAX: usize = i32::MAX as usize;
}
// The tessellators internally use u32 indices so we can't have more than u32::MAX
impl MaxIndex for u64 {
    const MAX: usize = u32::MAX as usize;
}
impl MaxIndex for i64 {
    const MAX: usize = u32::MAX as usize;
}
impl MaxIndex for usize {
    const MAX: usize = u32::MAX as usize;
}
impl MaxIndex for isize {
    const MAX: usize = u32::MAX as usize;
}
