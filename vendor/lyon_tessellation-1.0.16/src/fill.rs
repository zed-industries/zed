use crate::event_queue::*;
use crate::geom::LineSegment;
use crate::math::*;
use crate::monotone::*;
use crate::path::polygon::Polygon;
use crate::path::traits::{Build, PathBuilder};
use crate::path::{
    builder::NoAttributes, AttributeStore, Attributes, EndpointId, FillRule, IdEvent, PathEvent,
    PathSlice, PositionStore, Winding, NO_ATTRIBUTES,
};
use crate::{FillGeometryBuilder, Orientation, VertexId};
use crate::{
    FillOptions, InternalError, SimpleAttributeStore, TessellationError, TessellationResult,
    UnsupportedParamater, VertexSource,
};
use float_next_after::NextAfter;
use core::cmp::Ordering;
use core::f32::consts::FRAC_1_SQRT_2;
use core::mem;
use core::ops::Range;
use alloc::boxed::Box;
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use num_traits::Float;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Side {
    Left,
    Right,
}

impl Side {
    pub fn opposite(self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    pub fn is_left(self) -> bool {
        self == Side::Left
    }

    pub fn is_right(self) -> bool {
        self == Side::Right
    }
}

type SpanIdx = i32;
type ActiveEdgeIdx = usize;

// It's a bit odd but this consistently performs a bit better than f32::max, probably
// because the latter deals with NaN.
#[inline(always)]
fn fmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

fn slope(v: Vector) -> f32 {
    v.x / (v.y.max(f32::MIN))
}

#[cfg(all(debug_assertions, feature = "std"))]
macro_rules! tess_log {
    ($obj:ident, $fmt:expr) => (
        if $obj.log {
            std::println!($fmt);
        }
    );
    ($obj:ident, $fmt:expr, $($arg:tt)*) => (
        if $obj.log {
            std::println!($fmt, $($arg)*);
        }
    );
}

#[cfg(not(all(debug_assertions, feature = "std")))]
macro_rules! tess_log {
    ($obj:ident, $fmt:expr) => {};
    ($obj:ident, $fmt:expr, $($arg:tt)*) => {};
}

#[derive(Copy, Clone, Debug)]
struct WindingState {
    span_index: SpanIdx,
    number: i16,
    is_in: bool,
}

impl WindingState {
    fn new() -> Self {
        // The span index starts at -1 so that entering the first span (of index 0) increments
        // it to zero.
        WindingState {
            span_index: -1,
            number: 0,
            is_in: false,
        }
    }

    fn update(&mut self, fill_rule: FillRule, edge_winding: i16) {
        self.number += edge_winding;
        self.is_in = fill_rule.is_in(self.number);
        if self.is_in {
            self.span_index += 1;
        }
    }
}

struct ActiveEdgeScan {
    vertex_events: Vec<(SpanIdx, Side)>,
    edges_to_split: Vec<ActiveEdgeIdx>,
    spans_to_end: Vec<SpanIdx>,
    merge_event: bool,
    split_event: bool,
    merge_split_event: bool,
    above: Range<ActiveEdgeIdx>,
    winding_before_point: WindingState,
}

impl ActiveEdgeScan {
    fn new() -> Self {
        ActiveEdgeScan {
            vertex_events: Vec::new(),
            edges_to_split: Vec::new(),
            spans_to_end: Vec::new(),
            merge_event: false,
            split_event: false,
            merge_split_event: false,
            above: 0..0,
            winding_before_point: WindingState::new(),
        }
    }

    fn reset(&mut self) {
        self.vertex_events.clear();
        self.edges_to_split.clear();
        self.spans_to_end.clear();
        self.merge_event = false;
        self.split_event = false;
        self.merge_split_event = false;
        self.above = 0..0;
        self.winding_before_point = WindingState::new();
    }
}

#[derive(Copy, Clone, Debug)]
struct ActiveEdge {
    from: Point,
    to: Point,

    winding: i16,
    is_merge: bool,

    from_id: VertexId,
    src_edge: TessEventId,

    range_end: f32,
}

#[test]
fn active_edge_size() {
    // We want to be careful about the size of the struct.
    assert_eq!(std::mem::size_of::<ActiveEdge>(), 32);
}

impl ActiveEdge {
    #[inline(always)]
    fn min_x(&self) -> f32 {
        self.from.x.min(self.to.x)
    }

    #[inline(always)]
    fn max_x(&self) -> f32 {
        fmax(self.from.x, self.to.x)
    }
}

impl ActiveEdge {
    fn solve_x_for_y(&self, y: f32) -> f32 {
        // Because of float precision hazard, solve_x_for_y can
        // return something slightly out of the min/max range which
        // causes the ordering to be inconsistent with the way the
        // scan phase uses the min/max range.
        LineSegment {
            from: self.from,
            to: self.to,
        }
        .solve_x_for_y(y)
        .max(self.min_x())
        .min(self.max_x())
    }
}

struct ActiveEdges {
    edges: Vec<ActiveEdge>,
}

struct Span {
    /// We store `MonotoneTessellator` behind a `Box` for performance purposes.
    /// For more info, see [Issue #621](https://github.com/nical/lyon/pull/621).
    tess: Option<Box<MonotoneTessellator>>,
}

impl Span {
    fn tess(&mut self) -> &mut MonotoneTessellator {
        // this should only ever be called on a "live" span.
        match self.tess.as_mut() {
            None => {
                debug_assert!(false);
                unreachable!();
            }
            Some(tess) => tess,
        }
    }
}

struct Spans {
    spans: Vec<Span>,

    /// We store `MonotoneTessellator` behind a `Box` for performance purposes.
    /// For more info, see [Issue #621](https://github.com/nical/lyon/pull/621).
    #[allow(clippy::vec_box)]
    pool: Vec<Box<MonotoneTessellator>>,
}

impl Spans {
    fn begin_span(&mut self, span_idx: SpanIdx, position: &Point, vertex: VertexId) {
        let mut tess = self
            .pool
            .pop()
            .unwrap_or_else(|| Box::new(MonotoneTessellator::new()));
        tess.begin(*position, vertex);

        self.spans
            .insert(span_idx as usize, Span { tess: Some(tess) });
    }

    fn end_span(
        &mut self,
        span_idx: SpanIdx,
        position: &Point,
        id: VertexId,
        output: &mut dyn FillGeometryBuilder,
    ) {
        let idx = span_idx as usize;

        let span = &mut self.spans[idx];
        if let Some(mut tess) = span.tess.take() {
            tess.end(*position, id);
            tess.flush(output);
            // Recycle the allocations for future use.
            self.pool.push(tess);
        } else {
            debug_assert!(false);
            unreachable!();
        }
    }

    fn merge_spans(
        &mut self,
        left_span_idx: SpanIdx,
        current_position: &Point,
        current_vertex: VertexId,
        merge_position: &Point,
        merge_vertex: VertexId,
        output: &mut dyn FillGeometryBuilder,
    ) {
        //  \...\ /.
        //   \...x..  <-- merge vertex
        //    \./...  <-- active_edge
        //     x....  <-- current vertex

        let right_span_idx = left_span_idx + 1;

        self.spans[left_span_idx as usize].tess().vertex(
            *merge_position,
            merge_vertex,
            Side::Right,
        );

        self.spans[right_span_idx as usize].tess().vertex(
            *merge_position,
            merge_vertex,
            Side::Left,
        );

        self.end_span(left_span_idx, current_position, current_vertex, output);
    }

    fn cleanup_spans(&mut self) {
        // Get rid of the spans that were marked for removal.
        self.spans.retain(|span| span.tess.is_some());
    }
}

#[derive(Copy, Clone, Debug)]
struct PendingEdge {
    to: Point,
    sort_key: f32,
    // Index in events.edge_data
    src_edge: TessEventId,
    winding: i16,
    range_end: f32,
}

/// A Context object that can tessellate fill operations for complex paths.
///
/// <svg version="1.1" viewBox="0 0 400 200" height="200" width="400">
///   <g transform="translate(0,-852.36216)">
///     <path style="fill:#aad400;stroke:none;" transform="translate(0,852.36216)" d="M 20 20 L 20 180 L 180.30273 180 L 180.30273 20 L 20 20 z M 100 55 L 145 145 L 55 145 L 100 55 z "/>
///     <path style="fill:#aad400;fill-rule:evenodd;stroke:#000000;stroke-width:1px;stroke-linecap:butt;stroke-linejoin:miter;stroke-" d="m 219.75767,872.36216 0,160.00004 160.30273,0 0,-160.00004 -160.30273,0 z m 80,35 45,90 -90,0 45,-90 z"/>
///     <path style="fill:none;stroke:#000000;stroke-linecap:round;stroke-linejoin:round;stroke-" d="m 220,1032.3622 35,-35.00004 125,35.00004 -35,-35.00004 35,-125 -80,35 -80,-35 35,125"/>
///     <circle r="5" cy="872.36218" cx="20" style="color:#000000;;fill:#ff6600;fill-;stroke:#000000;" />
///     <circle r="5" cx="180.10918" cy="872.61475" style="fill:#ff6600;stroke:#000000;"/>
///     <circle r="5" cy="1032.2189" cx="180.10918" style="fill:#ff6600;stroke:#000000;"/>
///     <circle r="5" cx="20.505075" cy="1032.4714" style="fill:#ff6600;stroke:#000000;"/>
///     <circle r="5" cy="907.21252" cx="99.802048" style="fill:#ff6600;stroke:#000000;"/>
///     <circle r="5" cx="55.102798" cy="997.36865" style="fill:#ff6600;stroke:#000000;"/>
///     <circle r="5" cy="997.62122" cx="145.25891" style="fill:#ff6600;stroke:#000000;"/>
///   </g>
/// </svg>
///
/// ## Overview
///
/// The most important structure is [`FillTessellator`](struct.FillTessellator.html).
/// It implements the path fill tessellation algorithm which is by far the most advanced
/// feature in all lyon crates.
///
/// The `FillTessellator` takes a description of the input path and
/// [`FillOptions`](struct.FillOptions.html) as input. The description of the path can be an
/// `PathEvent` iterator, or an iterator of `IdEvent` with an implementation of`PositionStore`
/// to retrieve positions form endpoint and control point ids, and optionally an `AttributeStore`
/// providing custom endpoint attributes that the tessellator can hand over to the geometry builder.
///
/// The output of the tessellator is produced by the
/// [`FillGeometryBuilder`](geometry_builder/trait.FillGeometryBuilder.html) (see the
/// [`geometry_builder` documentation](geometry_builder/index.html) for more details about
/// how tessellators produce their output geometry, and how to generate custom vertex layouts).
///
/// The [tessellator's wiki page](https://github.com/nical/lyon/wiki/Tessellator) is a good place
/// to learn more about how the tessellator's algorithm works. The source code also contains
/// inline documentation for the adventurous who want to delve into more details.
///
/// The tessellator does not handle `NaN` values in any of its inputs.
///
/// ## Associating custom attributes with vertices.
///
/// It is sometimes useful to be able to link vertices generated by the tessellator back
/// with the path's original data, for example to be able to add attributes that the tessellator
/// does not know about (vertex color, texture coordinates, etc.).
///
/// The fill tessellator has two mechanisms to help with these advanced use cases. One is
/// simple to use and one that, while more complicated to use, can cover advanced scenarios.
///
/// Before going delving into these mechanisms, it is important to understand that the
/// vertices generated by the tessellator don't always correspond to the vertices existing
/// in the original path.
/// - Self-intersections, for example, introduce a new vertex where two edges meet.
/// - When several vertices are at the same position, they are merged into a single vertex
///   from the point of view of the tessellator.
/// - The tessellator does not handle curves, and uses an approximation that introduces a
///   number of line segments and therefore endpoints between the original endpoints of any
///   quadratic or cubic bézier curve.
///
/// This complicates the task of adding extra data to vertices without loosing the association
/// during tessellation.
///
/// ### Vertex sources
///
/// This is the complicated, but most powerful mechanism. The tessellator keeps track of where
/// each vertex comes from in the original path, and provides access to this information via
/// an iterator of [`VertexSource`](enum.VertexSource.html) in `FillVertex::sources`.
///
/// It is most common for the vertex source iterator to yield a single `VertexSource::Endpoint`
/// source, which happens when the vertex directly corresponds to an endpoint of the original path.
/// More complicated cases can be expressed.
/// For example if a vertex is inserted at an intersection halfway in the edge AB and two thirds
/// of the way through edge BC, the source for this new vertex is `VertexSource::Edge { from: A, to: B, t: 0.5 }`
/// and `VertexSource::Edge { from: C, to: D, t: 0.666666 }` where A, B, C and D are endpoint IDs.
///
/// To use this feature, make sure to use `FillTessellator::tessellate_with_ids` instead of
/// `FillTessellator::tessellate`.
///
/// ### Interpolated float attributes
///
/// Having to iterate over potentially several sources for each vertex can be cumbersome, in addition
/// to having to deal with generating proper values for the attributes of vertices that were introduced
/// at intersections or along curves.
///
/// In many scenarios, vertex attributes are made of floating point numbers and the most reasonable
/// way to generate new attributes is to linearly interpolate these numbers between the endpoints
/// of the edges they originate from.
///
/// Custom endpoint attributes are represented as `&[f32]` slices accessible via
/// `FillVertex::interpolated_attributes`. All vertices, whether they originate from a single
/// endpoint or some more complex source, have exactly the same number of attributes.
/// Without having to know about the meaning of attributes, the tessellator can either
/// forward the slice of attributes from a provided `AttributeStore` when possible or
/// generate the values via linear interpolation.
///
/// To use this feature, make sure to use `FillTessellator::tessellate_path` or
/// `FillTessellator::tessellate_with_ids` instead of `FillTessellator::tessellate`.
///
/// Attributes are lazily computed when calling `FillVertex::interpolated_attributes`.
/// In other words they don't add overhead when not used, however it is best to avoid calling
/// interpolated_attributes several times per vertex.
///
/// # Examples
///
/// ```
/// # extern crate lyon_tessellation as tess;
/// # use tess::path::Path;
/// # use tess::path::builder::*;
/// # use tess::path::iterator::*;
/// # use tess::math::{Point, point};
/// # use tess::geometry_builder::{VertexBuffers, simple_builder};
/// # use tess::*;
/// # fn main() {
/// // Create a simple path.
/// let mut path_builder = Path::builder();
/// path_builder.begin(point(0.0, 0.0));
/// path_builder.line_to(point(1.0, 2.0));
/// path_builder.line_to(point(2.0, 0.0));
/// path_builder.line_to(point(1.0, 1.0));
/// path_builder.end(true);
/// let path = path_builder.build();
///
/// // Create the destination vertex and index buffers.
/// let mut buffers: VertexBuffers<Point, u16> = VertexBuffers::new();
///
/// {
///     let mut vertex_builder = simple_builder(&mut buffers);
///
///     // Create the tessellator.
///     let mut tessellator = FillTessellator::new();
///
///     // Compute the tessellation.
///     let result = tessellator.tessellate_path(
///         &path,
///         &FillOptions::default(),
///         &mut vertex_builder
///     );
///     assert!(result.is_ok());
/// }
///
/// println!("The generated vertices are: {:?}.", &buffers.vertices[..]);
/// println!("The generated indices are: {:?}.", &buffers.indices[..]);
///
/// # }
/// ```
///
/// ```
/// # extern crate lyon_tessellation as tess;
/// # use tess::path::Path;
/// # use tess::path::builder::*;
/// # use tess::path::iterator::*;
/// # use tess::math::{Point, point};
/// # use tess::geometry_builder::{VertexBuffers, simple_builder};
/// # use tess::*;
/// # fn main() {
/// // Create a path with three custom endpoint attributes.
/// let mut path_builder = Path::builder_with_attributes(3);
/// path_builder.begin(point(0.0, 0.0), &[0.0, 0.1, 0.5]);
/// path_builder.line_to(point(1.0, 2.0), &[1.0, 1.0, 0.1]);
/// path_builder.line_to(point(2.0, 0.0), &[1.0, 0.0, 0.8]);
/// path_builder.line_to(point(1.0, 1.0), &[0.1, 0.3, 0.5]);
/// path_builder.end(true);
/// let path = path_builder.build();
///
/// struct MyVertex {
///     x: f32, y: f32,
///     r: f32, g: f32, b: f32, a: f32,
/// }
/// // A custom vertex constructor, see the geometry_builder module.
/// struct Ctor;
/// impl FillVertexConstructor<MyVertex> for Ctor {
///     fn new_vertex(&mut self, mut vertex: FillVertex) -> MyVertex {
///         let position = vertex.position();
///         let attrs = vertex.interpolated_attributes();
///         MyVertex {
///             x: position.x,
///             y: position.y,
///             r: attrs[0],
///             g: attrs[1],
///             b: attrs[2],
///             a: 1.0,
///         }
///     }
/// }
///
/// // Create the destination vertex and index buffers.
/// let mut buffers: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
///
/// {
///     // We use our custom vertex constructor here.
///     let mut vertex_builder = BuffersBuilder::new(&mut buffers, Ctor);
///
///     // Create the tessellator.
///     let mut tessellator = FillTessellator::new();
///
///     // Compute the tessellation. Here we use tessellate_with_ids
///     // which has a slightly more complicated interface. The provides
///     // the iterator as well as storage for positions and attributes at
///     // the same time.
///     let result = tessellator.tessellate_with_ids(
///         path.id_iter(), // Iterator over ids in the path
///         &path,          // PositionStore
///         Some(&path),    // AttributeStore
///         &FillOptions::default(),
///         &mut vertex_builder
///     );
///     assert!(result.is_ok());
/// }
///
/// # }
/// ```
pub struct FillTessellator {
    current_position: Point,
    current_vertex: VertexId,
    current_event_id: TessEventId,
    active: ActiveEdges,
    edges_below: Vec<PendingEdge>,
    fill_rule: FillRule,
    orientation: Orientation,
    tolerance: f32,
    fill: Spans,
    log: bool,
    assume_no_intersection: bool,
    attrib_buffer: Vec<f32>,

    scan: ActiveEdgeScan,
    events: EventQueue,
}

impl Default for FillTessellator {
    fn default() -> Self {
        Self::new()
    }
}

impl FillTessellator {
    /// Constructor.
    pub fn new() -> Self {
        #[cfg(all(debug_assertions, feature = "std"))]
        let log = std::env::var("LYON_FORCE_LOGGING").is_ok();
        #[cfg(not(all(debug_assertions, feature = "std")))]
        let log = false;

        FillTessellator {
            current_position: point(f32::MIN, f32::MIN),
            current_vertex: VertexId::INVALID,
            current_event_id: INVALID_EVENT_ID,
            active: ActiveEdges { edges: Vec::new() },
            edges_below: Vec::new(),
            fill_rule: FillRule::EvenOdd,
            orientation: Orientation::Vertical,
            tolerance: FillOptions::DEFAULT_TOLERANCE,
            fill: Spans {
                spans: Vec::new(),
                pool: Vec::new(),
            },
            log,
            assume_no_intersection: false,
            attrib_buffer: Vec::new(),

            scan: ActiveEdgeScan::new(),
            events: EventQueue::new(),
        }
    }

    /// Compute the tessellation from a path iterator.
    pub fn tessellate(
        &mut self,
        path: impl IntoIterator<Item = PathEvent>,
        options: &FillOptions,
        output: &mut dyn FillGeometryBuilder,
    ) -> TessellationResult {
        let event_queue = core::mem::take(&mut self.events);
        let mut queue_builder = event_queue.into_builder(options.tolerance);

        queue_builder.set_path(
            options.tolerance,
            options.sweep_orientation,
            path,
        );

        self.events = queue_builder.build();

        self.tessellate_impl(options, None, output)
    }

    /// Compute the tessellation using an iterator over endpoint and control
    /// point ids, storage for the positions and, optionally, storage for
    /// custom endpoint attributes.
    pub fn tessellate_with_ids(
        &mut self,
        path: impl IntoIterator<Item = IdEvent>,
        positions: &impl PositionStore,
        custom_attributes: Option<&dyn AttributeStore>,
        options: &FillOptions,
        output: &mut dyn FillGeometryBuilder,
    ) -> TessellationResult {
        let event_queue = core::mem::take(&mut self.events);
        let mut queue_builder = event_queue.into_builder(options.tolerance);

        queue_builder.set_path_with_ids(
            options.tolerance,
            options.sweep_orientation,
            path,
            positions,
        );

        self.events = queue_builder.build();

        self.tessellate_impl(options, custom_attributes, output)
    }

    /// Compute the tessellation from a path slice.
    ///
    /// The tessellator will internally only track vertex sources and interpolated
    /// attributes if the path has interpolated attributes.
    pub fn tessellate_path<'l>(
        &'l mut self,
        path: impl Into<PathSlice<'l>>,
        options: &'l FillOptions,
        builder: &'l mut dyn FillGeometryBuilder,
    ) -> TessellationResult {
        let path = path.into();

        if path.num_attributes() > 0 {
            self.tessellate_with_ids(path.id_iter(), &path, Some(&path), options, builder)
        } else {
            self.tessellate(path.iter(), options, builder)
        }
    }

    /// Tessellate a `Polygon`.
    pub fn tessellate_polygon(
        &mut self,
        polygon: Polygon<Point>,
        options: &FillOptions,
        output: &mut dyn FillGeometryBuilder,
    ) -> TessellationResult {
        self.tessellate(polygon.path_events(), options, output)
    }

    /// Tessellate an axis-aligned rectangle.
    pub fn tessellate_rectangle(
        &mut self,
        rect: &Box2D,
        _options: &FillOptions,
        output: &mut dyn FillGeometryBuilder,
    ) -> TessellationResult {
        crate::basic_shapes::fill_rectangle(rect, output)
    }

    /// Tessellate a circle.
    pub fn tessellate_circle(
        &mut self,
        center: Point,
        radius: f32,
        options: &FillOptions,
        output: &mut dyn FillGeometryBuilder,
    ) -> TessellationResult {
        crate::basic_shapes::fill_circle(center, radius, options, output)
    }

    /// Tessellate an ellipse.
    pub fn tessellate_ellipse(
        &mut self,
        center: Point,
        radii: Vector,
        x_rotation: Angle,
        winding: Winding,
        options: &FillOptions,
        output: &mut dyn FillGeometryBuilder,
    ) -> TessellationResult {
        let options = (*options).with_intersections(false);

        let mut builder = self.builder(&options, output);
        builder.add_ellipse(center, radii, x_rotation, winding);

        builder.build()
    }

    /// Tessellate directly from a sequence of `PathBuilder` commands, without
    /// creating an intermediate path data structure.
    ///
    /// The returned builder implements the [`lyon_path::traits::PathBuilder`] trait,
    /// is compatible with the all `PathBuilder` adapters.
    /// It also has all requirements documented in `PathBuilder` (All sub-paths must be
    /// wrapped in a `begin`/`end` pair).
    ///
    /// # Example
    ///
    /// ```rust
    /// use lyon_tessellation::{FillTessellator, FillOptions};
    /// use lyon_tessellation::geometry_builder::{simple_builder, VertexBuffers};
    /// use lyon_tessellation::math::{Point, point};
    ///
    /// let mut buffers: VertexBuffers<Point, u16> = VertexBuffers::new();
    /// let mut vertex_builder = simple_builder(&mut buffers);
    /// let mut tessellator = FillTessellator::new();
    /// let options = FillOptions::default();
    ///
    /// // Create a temporary builder (borrows the tessellator).
    /// let mut builder = tessellator.builder(&options, &mut vertex_builder);
    ///
    /// // Build the path directly in the tessellator, skipping an intermediate data
    /// // structure.
    /// builder.begin(point(0.0, 0.0));
    /// builder.line_to(point(10.0, 0.0));
    /// builder.line_to(point(10.0, 10.0));
    /// builder.line_to(point(0.0, 10.0));
    /// builder.end(true);
    ///
    /// // Finish the tessellation and get the result.
    /// let result = builder.build();
    /// ```
    ///
    /// [`lyon_path::traits::PathBuilder`]: https://docs.rs/lyon_path/*/lyon_path/traits/trait.PathBuilder.html
    pub fn builder<'l>(
        &'l mut self,
        options: &'l FillOptions,
        output: &'l mut dyn FillGeometryBuilder,
    ) -> NoAttributes<FillBuilder<'l>> {
        NoAttributes::wrap(FillBuilder::new(0, self, options, output))
    }

    /// Tessellate directly from a sequence of `PathBuilder` commands, without
    /// creating an intermediate path data structure.
    ///
    /// Similar to `FillTessellator::builder` with custom attributes.
    pub fn builder_with_attributes<'l>(
        &'l mut self,
        num_attributes: usize,
        options: &'l FillOptions,
        output: &'l mut dyn FillGeometryBuilder,
    ) -> FillBuilder<'l> {
        FillBuilder::new(num_attributes, self, options, output)
    }

    fn tessellate_impl(
        &mut self,
        options: &FillOptions,
        attrib_store: Option<&dyn AttributeStore>,
        builder: &mut dyn FillGeometryBuilder,
    ) -> TessellationResult {
        if options.tolerance.is_nan() || options.tolerance <= 0.0 {
            return Err(TessellationError::UnsupportedParamater(
                UnsupportedParamater::ToleranceIsNaN,
            ));
        }

        self.reset();

        if let Some(store) = attrib_store {
            self.attrib_buffer.resize(store.num_attributes(), 0.0);
        } else {
            self.attrib_buffer.clear();
        }

        self.fill_rule = options.fill_rule;
        self.orientation = options.sweep_orientation;
        self.tolerance = options.tolerance * 0.5;
        self.assume_no_intersection = !options.handle_intersections;

        builder.begin_geometry();

        let mut scan = mem::replace(&mut self.scan, ActiveEdgeScan::new());

        let result = self.tessellator_loop(attrib_store, &mut scan, builder);

        mem::swap(&mut self.scan, &mut scan);

        if let Err(e) = result {
            tess_log!(self, "Tessellation failed with error: {}.", e);
            builder.abort_geometry();

            return Err(e);
        }

        if !self.assume_no_intersection {
            debug_assert!(self.active.edges.is_empty());
            debug_assert!(self.fill.spans.is_empty());
        }

        // There shouldn't be any span left after the tessellation ends.
        // If for whatever reason (bug) there are, flush them so that we don't
        // miss the triangles they contain.
        for span in &mut self.fill.spans {
            if let Some(tess) = span.tess.as_mut() {
                tess.flush(builder);
            }
        }

        self.fill.spans.clear();

        builder.end_geometry();

        Ok(())
    }

    /// Enable/disable some verbose logging during the tessellation, for
    /// debugging purposes.
    pub fn set_logging(&mut self, is_enabled: bool) {
        #[cfg(all(debug_assertions, feature = "std"))]
        let forced = std::env::var("LYON_FORCE_LOGGING").is_ok();

        #[cfg(not(all(debug_assertions, feature = "std")))]
        let forced = false;

        self.log = is_enabled || forced;
    }

    #[cfg_attr(feature = "profiling", inline(never))]
    fn tessellator_loop(
        &mut self,
        attrib_store: Option<&dyn AttributeStore>,
        scan: &mut ActiveEdgeScan,
        output: &mut dyn FillGeometryBuilder,
    ) -> Result<(), TessellationError> {
        log_svg_preamble(self);

        let mut _prev_position = point(f32::MIN, f32::MIN);
        self.current_event_id = self.events.first_id();
        while self.events.valid_id(self.current_event_id) {
            self.initialize_events(attrib_store, output)?;

            debug_assert!(is_after(self.current_position, _prev_position));
            _prev_position = self.current_position;

            if let Err(e) = self.process_events(scan, output) {
                // Something went wrong, attempt to salvage the state of the sweep
                // line
                self.recover_from_error(e, output);
                // ... and try again.
                self.process_events(scan, output)?
            }

            #[cfg(debug_assertions)]
            self.check_active_edges();

            self.current_event_id = self.events.next_id(self.current_event_id);
        }

        Ok(())
    }

    fn initialize_events(
        &mut self,
        attrib_store: Option<&dyn AttributeStore>,
        output: &mut dyn FillGeometryBuilder,
    ) -> Result<(), TessellationError> {
        let current_event = self.current_event_id;

        tess_log!(
            self,
            "\n\n<!--         event #{}          -->",
            current_event
        );

        self.current_position = self.events.position(current_event);

        if self.current_position.x.is_nan() || self.current_position.y.is_nan() {
            return Err(TessellationError::UnsupportedParamater(
                UnsupportedParamater::PositionIsNaN,
            ));
        }

        let position = match self.orientation {
            Orientation::Vertical => self.current_position,
            Orientation::Horizontal => reorient(self.current_position),
        };

        self.current_vertex = output.add_fill_vertex(FillVertex {
            position,
            events: &self.events,
            current_event,
            attrib_store,
            attrib_buffer: &mut self.attrib_buffer,
        })?;

        let mut current_sibling = current_event;
        while self.events.valid_id(current_sibling) {
            let edge = &self.events.edge_data[current_sibling as usize];
            // We insert "fake" edges when there are end events
            // to make sure we process that vertex even if it has
            // no edge below.
            if edge.is_edge {
                let to = edge.to;
                debug_assert!(is_after(to, self.current_position));
                self.edges_below.push(PendingEdge {
                    to,
                    sort_key: slope(to - self.current_position), //.angle_from_x_axis().radians,
                    src_edge: current_sibling,
                    winding: edge.winding,
                    range_end: edge.range.end,
                });
            }

            current_sibling = self.events.next_sibling_id(current_sibling);
        }

        Ok(())
    }

    /// An iteration of the sweep line algorithm.
    #[cfg_attr(feature = "profiling", inline(never))]
    fn process_events(
        &mut self,
        scan: &mut ActiveEdgeScan,
        output: &mut dyn FillGeometryBuilder,
    ) -> Result<(), InternalError> {
        tess_log!(self, "<!--");
        tess_log!(
            self,
            "     events at {:?} {:?}         {} edges below",
            self.current_position,
            self.current_vertex,
            self.edges_below.len(),
        );

        tess_log!(self, "edges below (initially): {:#?}", self.edges_below);

        // Step 1 - Scan the active edge list, deferring processing and detecting potential
        // ordering issues in the active edges.
        self.scan_active_edges(scan)?;

        // Step 2 - Do the necessary processing on edges that end at the current point.
        self.process_edges_above(scan, output);

        // Step 3 - Do the necessary processing on edges that start at the current point.
        self.process_edges_below(scan);

        // Step 4 - Insert/remove edges to the active edge as necessary and handle
        // potential self-intersections.
        self.update_active_edges(scan);

        tess_log!(self, "-->");

        #[cfg(debug_assertions)]
        self.log_active_edges();

        Ok(())
    }

    #[cfg(debug_assertions)]
    fn log_active_edges(&self) {
        tess_log!(self, r#"<g class="active-edges">"#);
        tess_log!(
            self,
            r#"<path d="M 0 {} L 1000 {}" class="sweep-line"/>"#,
            self.current_position.y,
            self.current_position.y
        );
        tess_log!(self, "<!-- active edges: -->");
        for e in &self.active.edges {
            if e.is_merge {
                tess_log!(
                    self,
                    r#"  <circle cx="{}" cy="{}" r="3px" class="merge"/>"#,
                    e.from.x,
                    e.from.y
                );
            } else {
                tess_log!(
                    self,
                    r#"  <path d="M {:.5?} {:.5?} L {:.5?} {:.5?}" class="edge", winding="{:>2}"/>"#,
                    e.from.x,
                    e.from.y,
                    e.to.x,
                    e.to.y,
                    e.winding,
                );
            }
        }
        tess_log!(self, "<!-- spans: {}-->", self.fill.spans.len());
        tess_log!(self, "</g>");
    }

    #[cfg(debug_assertions)]
    fn check_active_edges(&self) {
        let mut winding = WindingState::new();
        for (idx, edge) in self.active.edges.iter().enumerate() {
            winding.update(self.fill_rule, edge.winding);
            if edge.is_merge {
                assert!(self.fill_rule.is_in(winding.number));
            } else {
                assert!(
                    !is_after(self.current_position, edge.to),
                    "error at edge {}, position {:.6?} (current: {:.6?}",
                    idx,
                    edge.to,
                    self.current_position,
                );
            }
        }
        assert_eq!(winding.number, 0);
        let expected_span_count = (winding.span_index + 1) as usize;
        assert_eq!(self.fill.spans.len(), expected_span_count);
    }

    /// Scan the active edges to find the information we will need for the tessellation, without
    /// modifying the state of the sweep line and active spans.
    ///
    /// During this scan we also check that the ordering of the active edges is correct.
    /// If an error is detected we bail out of the scan which will cause us to sort the active
    /// edge list and try to scan again (this is why have to defer any modification to after
    /// the scan).
    ///
    /// The scan happens in three steps:
    /// - 1) Loop over the edges on the left of the current point to compute the winding number.
    /// - 2) Loop over the edges that connect with the current point to determine what processing
    ///      is needed (for example end events or right events).
    /// - 3) Loop over the edges on the right of the current point to detect potential edges that should
    ///      have been handled in the previous phases.
    #[cfg_attr(feature = "profiling", inline(never))]
    fn scan_active_edges(&self, scan: &mut ActiveEdgeScan) -> Result<(), InternalError> {
        scan.reset();

        let current_x = self.current_position.x;
        let mut connecting_edges = false;
        let mut active_edge_idx = 0;
        let mut winding = WindingState::new();
        let mut previous_was_merge = false;

        // Step 1 - Iterate over edges *before* the current point.
        for active_edge in &self.active.edges {
            if active_edge.is_merge {
                // \.....\ /...../
                //  \.....x...../   <--- merge vertex
                //   \....:..../
                // ---\---:---/----  <-- sweep line
                //     \..:../

                // An unresolved merge vertex implies the left and right spans are
                // adjacent and there is no transition between the two which means
                // we need to bump the span index manually.
                winding.span_index += 1;
                active_edge_idx += 1;
                previous_was_merge = true;

                continue;
            }

            let edge_is_before_current_point =
                if points_are_equal(self.current_position, active_edge.to) {
                    // We just found our first edge that connects with the current point.
                    // We might find other ones in the next iterations.
                    connecting_edges = true;
                    false
                } else if active_edge.max_x() < current_x {
                    true
                } else if active_edge.min_x() > current_x {
                    tess_log!(
                        self,
                        "min_x({:?}) > current_x({:?})",
                        active_edge.min_x(),
                        current_x
                    );
                    false
                } else if active_edge.from.y == active_edge.to.y {
                    connecting_edges = true;
                    false
                } else {
                    let ex = active_edge.solve_x_for_y(self.current_position.y);

                    if (ex - current_x).abs() <= self.tolerance {
                        connecting_edges = true;
                        false
                    } else if ex > current_x {
                        tess_log!(self, "ex({:?}) > current_x({:?})", ex, current_x);
                        false
                    } else {
                        true
                    }
                };

            if !edge_is_before_current_point {
                break;
            }

            winding.update(self.fill_rule, active_edge.winding);
            previous_was_merge = false;
            active_edge_idx += 1;

            tess_log!(
                self,
                " > span: {}, in: {}",
                winding.span_index,
                winding.is_in
            );
        }

        scan.above.start = active_edge_idx;
        scan.winding_before_point = winding;

        if previous_was_merge {
            scan.winding_before_point.span_index -= 1;
            scan.above.start -= 1;

            // First connecting edge is a merge.
            //  ...:./.      ...:...
            //  ...:/..  or  ...:...
            //  ...X...      ...X...
            //
            // The span on the left does not end here but it has a vertex
            // on its right side.
            //
            // The next loop can now assume that merge edges can't make the first
            // transition connecting with the current vertex,

            if !connecting_edges {
                // There are no edges left and right of the merge that connect with
                // the current vertex. In other words the merge is the only edge
                // connecting and there must be a split event formed by two edges
                // below the current vertex.
                //
                // In this case we don't end any span and we skip splitting. The merge
                // and the split cancel each-other out.
                //
                //  ...:...
                //  ...:...
                //  ...x...
                //  ../ \..
                scan.vertex_events
                    .push((winding.span_index - 1, Side::Right));
                scan.vertex_events.push((winding.span_index, Side::Left));
                scan.merge_split_event = true;
                tess_log!(self, "split+merge");
            }
        }

        //  .......
        //  ...x...
        //  ../ \..
        scan.split_event = !connecting_edges && winding.is_in && !scan.merge_split_event;

        // Step 2 - Iterate over edges connecting with the current point.

        tess_log!(
            self,
            "connecting_edges {} | edge {} | span {}",
            connecting_edges,
            active_edge_idx,
            winding.span_index
        );
        if connecting_edges {
            let in_before_vertex = winding.is_in;
            let mut first_connecting_edge = !previous_was_merge;

            for active_edge in &self.active.edges[active_edge_idx..] {
                if active_edge.is_merge {
                    if !winding.is_in {
                        return Err(InternalError::MergeVertexOutside);
                    }

                    // Merge above the current vertex to resolve.
                    //
                    // Resolving a merge usually leads to a span adjacent to the merge
                    // ending.
                    //
                    // If there was already an edge connecting with the current vertex
                    // just left of the merge edge, we can end the span between that edge
                    // and the merge.
                    //
                    //    |
                    //    v
                    //  \...:...
                    //  .\..:...
                    //  ..\.:...
                    //  ...\:...
                    //  ....X...
                    scan.spans_to_end.push(winding.span_index);

                    // To deal with the right side of the merge, we simply pretend it
                    // transitioned into the shape. Next edge that transitions out (if any)
                    // will close out the span as if it was surrounded be regular edges.
                    //
                    //       |
                    //       v
                    //  ...:.../
                    //  ...:../
                    //  ...:./
                    //  ...:/
                    //  ...X

                    winding.span_index += 1;
                    active_edge_idx += 1;
                    first_connecting_edge = false;

                    continue;
                }

                if !self.is_edge_connecting(active_edge, active_edge_idx, scan)? {
                    break;
                }

                if !first_connecting_edge && winding.is_in {
                    // End event.
                    //
                    //  \.../
                    //   \./
                    //    x
                    //
                    scan.spans_to_end.push(winding.span_index);
                }

                winding.update(self.fill_rule, active_edge.winding);

                tess_log!(
                    self,
                    " x span: {} in: {}",
                    winding.span_index,
                    winding.is_in
                );

                if winding.is_in && winding.span_index >= self.fill.spans.len() as i32 {
                    return Err(InternalError::InsufficientNumberOfSpans);
                }

                active_edge_idx += 1;
                first_connecting_edge = false;
            }

            let in_after_vertex = winding.is_in;

            let vertex_is_merge_event = in_before_vertex
                && in_after_vertex
                && self.edges_below.is_empty()
                && scan.edges_to_split.is_empty();

            if vertex_is_merge_event {
                //  .\   /.      .\ |./ /.
                //  ..\ /..      ..\|//...
                //  ...x...  or  ...x.....  (etc.)
                //  .......      .........
                scan.merge_event = true;
            }

            if in_before_vertex {
                //   ...|         ..\ /..
                //   ...x    or   ...x...  (etc.)
                //   ...|         ...:...
                let first_span_index = scan.winding_before_point.span_index;
                scan.vertex_events.push((first_span_index, Side::Right));
            }

            if in_after_vertex {
                //    |...        ..\ /..
                //    x...   or   ...x...  (etc.)
                //    |...        ...:...
                scan.vertex_events.push((winding.span_index, Side::Left));
            }
        }

        tess_log!(self, "edges after | {}", active_edge_idx);

        scan.above.end = active_edge_idx;

        // Step 3 - Now Iterate over edges after the current point.
        // We only do this to detect errors.
        self.check_remaining_edges(active_edge_idx, current_x)
    }

    #[cfg_attr(feature = "profiling", inline(never))]
    #[cfg_attr(not(feature = "profiling"), inline(always))]
    fn check_remaining_edges(
        &self,
        active_edge_idx: usize,
        current_x: f32,
    ) -> Result<(), InternalError> {
        // This function typically takes about 2.5% ~ 3% of the profile, so not necessarily the best
        // target for optimization. That said all of the work done here is only robustness checks
        // so we could add an option to skip it.
        for active_edge in &self.active.edges[active_edge_idx..] {
            if active_edge.is_merge {
                continue;
            }

            if active_edge.max_x() < current_x {
                return Err(InternalError::IncorrectActiveEdgeOrder(1));
            }

            if points_are_equal(self.current_position, active_edge.to) {
                return Err(InternalError::IncorrectActiveEdgeOrder(2));
            }

            if active_edge.min_x() < current_x
                && active_edge.solve_x_for_y(self.current_position.y) < current_x
            {
                return Err(InternalError::IncorrectActiveEdgeOrder(3));
            }
        }

        Ok(())
    }

    // Returns Ok(true) if the edge connects with the current vertex, Ok(false) otherwise.
    // Returns Err if the active edge order is wrong.
    fn is_edge_connecting(
        &self,
        active_edge: &ActiveEdge,
        active_edge_idx: usize,
        scan: &mut ActiveEdgeScan,
    ) -> Result<bool, InternalError> {
        if points_are_equal(self.current_position, active_edge.to) {
            return Ok(true);
        }

        let current_x = self.current_position.x;
        let threshold = self.tolerance;

        let min_x = active_edge.min_x();
        let max_x = active_edge.max_x();

        if max_x + threshold < current_x || active_edge.to.y < self.current_position.y {
            return Err(InternalError::IncorrectActiveEdgeOrder(4));
        }

        if min_x > current_x {
            return Ok(false);
        }

        let ex = if active_edge.from.y != active_edge.to.y {
            active_edge.solve_x_for_y(self.current_position.y)
        } else if max_x >= current_x && min_x <= current_x {
            current_x
        } else {
            active_edge.to.x
        };

        if (ex - current_x).abs() <= threshold {
            tess_log!(
                self,
                "vertex on an edge! {:?} -> {:?}",
                active_edge.from,
                active_edge.to
            );
            scan.edges_to_split.push(active_edge_idx);
            return Ok(true);
        }

        if ex < current_x {
            return Err(InternalError::IncorrectActiveEdgeOrder(5));
        }

        tess_log!(self, "ex = {:?} (diff={})", ex, ex - current_x);

        Ok(false)
    }

    #[cfg_attr(feature = "profiling", inline(never))]
    fn process_edges_above(
        &mut self,
        scan: &mut ActiveEdgeScan,
        output: &mut dyn FillGeometryBuilder,
    ) {
        for &(span_index, side) in &scan.vertex_events {
            tess_log!(
                self,
                "   -> Vertex event, span: {:?} / {:?} / id: {:?}",
                span_index,
                side,
                self.current_vertex
            );
            self.fill.spans[span_index as usize].tess().vertex(
                self.current_position,
                self.current_vertex,
                side,
            );
        }

        for &span_index in &scan.spans_to_end {
            tess_log!(self, "   -> End span {:?}", span_index);
            self.fill.end_span(
                span_index,
                &self.current_position,
                self.current_vertex,
                output,
            );
        }

        self.fill.cleanup_spans();

        for &edge_idx in &scan.edges_to_split {
            let active_edge = &mut self.active.edges[edge_idx];
            let to = active_edge.to;

            self.edges_below.push(PendingEdge {
                to,
                sort_key: slope(to - self.current_position),
                src_edge: active_edge.src_edge,
                winding: active_edge.winding,
                range_end: active_edge.range_end,
            });
            tess_log!(
                self,
                "split {:?}, add edge below {:?} -> {:?} ({:?})",
                edge_idx,
                self.current_position,
                self.edges_below.last().unwrap().to,
                active_edge.winding,
            );

            active_edge.to = self.current_position;
        }

        if scan.merge_event {
            // Merge event.
            //
            //  ...\   /...
            //  ....\ /....
            //  .....x.....
            //
            let edge = &mut self.active.edges[scan.above.start];
            edge.is_merge = true;
            edge.from = edge.to;
            edge.winding = 0;
            edge.from_id = self.current_vertex;

            // take the merge edge out of the range so that it isn't removed later.
            scan.above.start += 1;
        }
    }

    #[cfg_attr(feature = "profiling", inline(never))]
    fn process_edges_below(&mut self, scan: &mut ActiveEdgeScan) {
        let mut winding = scan.winding_before_point;

        tess_log!(
            self,
            "connecting edges: {}..{} in: {:?}",
            scan.above.start,
            scan.above.end,
            winding.is_in
        );
        tess_log!(self, "winding state before point: {:?}", winding);
        tess_log!(self, "edges below: {:#?}", self.edges_below);

        self.sort_edges_below();

        self.handle_coincident_edges_below();

        if scan.split_event {
            // Split event.
            //
            //  ...........
            //  .....x.....
            //  ..../ \....
            //  .../   \...
            //

            tess_log!(self, "split event");

            let left_enclosing_edge_idx = scan.above.start - 1;
            self.split_event(left_enclosing_edge_idx, winding.span_index);
        }

        // Go through the edges that start at the current point and emit
        // start events for each time an in-out pair is found.

        let mut first_pending_edge = true;
        for pending_edge in &self.edges_below {
            if !first_pending_edge && winding.is_in {
                // Start event.
                //
                //      x
                //     /.\
                //    /...\
                //

                tess_log!(
                    self,
                    " begin span {} ({})",
                    winding.span_index,
                    self.fill.spans.len()
                );

                self.fill.begin_span(
                    winding.span_index,
                    &self.current_position,
                    self.current_vertex,
                );
            }
            winding.update(self.fill_rule, pending_edge.winding);

            tess_log!(
                self,
                "edge below: span: {}, in: {}",
                winding.span_index,
                winding.is_in
            );

            first_pending_edge = false;
        }
    }

    #[cfg_attr(feature = "profiling", inline(never))]
    fn update_active_edges(&mut self, scan: &ActiveEdgeScan) {
        let above = scan.above.start..scan.above.end;

        tess_log!(
            self,
            " remove {} edges ({}..{})",
            above.end - above.start,
            above.start,
            above.end
        );

        if !self.assume_no_intersection {
            self.handle_intersections(above.clone());
        }

        #[cfg(debug_assertions)]
        for active_edge in &self.active.edges[above.clone()] {
            debug_assert!(active_edge.is_merge || !is_after(self.current_position, active_edge.to));
        }

        let from = self.current_position;
        let from_id = self.current_vertex;
        self.active.edges.splice(
            above,
            self.edges_below.drain(..).map(|edge| ActiveEdge {
                from,
                to: edge.to,
                winding: edge.winding,
                is_merge: false,
                from_id,
                src_edge: edge.src_edge,
                range_end: edge.range_end,
            }),
        );
    }

    fn split_event(&mut self, left_enclosing_edge_idx: ActiveEdgeIdx, left_span_idx: SpanIdx) {
        let right_enclosing_edge_idx = left_enclosing_edge_idx + 1;

        let upper_left = self.active.edges[left_enclosing_edge_idx].from;
        let upper_right = self.active.edges[right_enclosing_edge_idx].from;

        let right_span_idx = left_span_idx + 1;

        let (upper_position, upper_id, new_span_idx) = if is_after(upper_left, upper_right) {
            //                |.....
            // upper_left --> x.....
            //               /.:....
            //              /...x... <-- current split vertex
            //             /.../ \..
            (
                upper_left,
                self.active.edges[left_enclosing_edge_idx].from_id,
                left_span_idx,
            )
        } else {
            //                          .....|
            //                          .....x <-- upper_right
            //                          ....:.\
            // current split vertex --> ...x...\
            //                          ../ \...\
            (
                upper_right,
                self.active.edges[right_enclosing_edge_idx].from_id,
                right_span_idx,
            )
        };

        self.fill
            .begin_span(new_span_idx, &upper_position, upper_id);

        self.fill.spans[left_span_idx as usize].tess().vertex(
            self.current_position,
            self.current_vertex,
            Side::Right,
        );
        self.fill.spans[right_span_idx as usize].tess().vertex(
            self.current_position,
            self.current_vertex,
            Side::Left,
        );
    }

    #[cfg_attr(feature = "profiling", inline(never))]
    fn handle_intersections(&mut self, skip_range: Range<usize>) {
        // Do intersection checks for all of the new edges against already active edges.
        //
        // If several intersections are found on the same edges we only keep the top-most.
        // the active and new edges are then truncated at the intersection position and the
        // lower parts are added to the event queue.
        //
        // In order to not break invariants of the sweep line we need to ensure that:
        // - the intersection position is never ordered before the current position,
        // - after truncation, edges continue being oriented downwards,
        //
        // Floating-point precision (or the lack thereof) prevent us from taking the
        // above properties from granted even though they make sense from a purely
        // geometrical perspective. Therefore we have to take great care in checking
        // whether these invariants aren't broken by the insertion of the intersection,
        // manually fixing things up if need be and making sure to not break more
        // invariants in doing so.

        let mut edges_below = mem::take(&mut self.edges_below);
        for edge_below in &mut edges_below {
            let below_min_x = self.current_position.x.min(edge_below.to.x);
            let below_max_x = fmax(self.current_position.x, edge_below.to.x);

            let below_segment = LineSegment {
                from: self.current_position.to_f64(),
                to: edge_below.to.to_f64(),
            };

            let mut tb_min = 1.0;
            let mut intersection = None;
            for (i, active_edge) in self.active.edges.iter().enumerate() {
                if skip_range.contains(&i) {
                    continue;
                }
                if active_edge.is_merge || below_min_x > active_edge.max_x() {
                    continue;
                }

                if below_max_x < active_edge.min_x() {
                    // We can't early out because there might be edges further on the right
                    // that extend further on the left which would be missed.
                    //
                    // sweep line -> =o===/==/==
                    //                |\ /  /
                    //                | o  /
                    //  edge below -> |   /
                    //                |  /
                    //                | / <- missed active edge
                    //                |/
                    //                x <- missed intersection
                    //               /|
                    continue;
                }

                let active_segment = LineSegment {
                    from: active_edge.from.to_f64(),
                    to: active_edge.to.to_f64(),
                };

                if let Some((ta, tb)) = active_segment.intersection_t(&below_segment) {
                    if tb < tb_min && tb > 0.0 && ta > 0.0 && ta <= 1.0 {
                        // we only want the closest intersection;
                        tb_min = tb;
                        intersection = Some((ta, tb, i));
                    }
                }
            }

            if let Some((ta, tb, active_edge_idx)) = intersection {
                self.process_intersection(ta, tb, active_edge_idx, edge_below, &below_segment);
            }
        }
        self.edges_below = edges_below;

        //self.log_active_edges();
    }

    #[inline(never)]
    fn process_intersection(
        &mut self,
        ta: f64,
        tb: f64,
        active_edge_idx: usize,
        edge_below: &mut PendingEdge,
        below_segment: &LineSegment<f64>,
    ) {
        let mut intersection_position = below_segment.sample(tb).to_f32();
        tess_log!(
            self,
            "-> intersection at: {:?} t={:?}|{:?}",
            intersection_position,
            ta,
            tb
        );
        tess_log!(
            self,
            "   from {:?}->{:?} and {:?}->{:?}",
            self.active.edges[active_edge_idx].from,
            self.active.edges[active_edge_idx].to,
            self.current_position,
            edge_below.to,
        );

        let active_edge = &mut self.active.edges[active_edge_idx];

        if self.current_position == intersection_position {
            active_edge.from = intersection_position;
            let src_range = &mut self.events.edge_data[active_edge.src_edge as usize].range;
            let remapped_ta = remap_t_in_range(ta as f32, src_range.start..active_edge.range_end);
            src_range.start = remapped_ta;

            return;
        }

        if !is_after(intersection_position, self.current_position) {
            tess_log!(self, "fixup the intersection");
            intersection_position.y = self.current_position.y.next_after(f32::INFINITY);
        }

        assert!(
            is_after(intersection_position, self.current_position),
            "!!! {:.9?} {:.9?}",
            intersection_position,
            self.current_position
        );

        if is_near(intersection_position, edge_below.to) {
            tess_log!(self, "intersection near below.to");
            intersection_position = edge_below.to;
        } else if is_near(intersection_position, active_edge.to) {
            tess_log!(self, "intersection near active_edge.to");
            intersection_position = active_edge.to;
        }

        let a_src_edge_data = self.events.edge_data[active_edge.src_edge as usize].clone();
        let b_src_edge_data = self.events.edge_data[edge_below.src_edge as usize].clone();

        let mut inserted_evt = None;
        let mut flipped_active = false;

        if active_edge.to != intersection_position && active_edge.from != intersection_position {
            let remapped_ta = remap_t_in_range(
                ta as f32,
                a_src_edge_data.range.start..active_edge.range_end,
            );

            if is_after(active_edge.to, intersection_position) {
                // Should take this branch most of the time.
                inserted_evt = Some(self.events.insert_sorted(
                    intersection_position,
                    EdgeData {
                        range: remapped_ta..active_edge.range_end,
                        winding: active_edge.winding,
                        to: active_edge.to,
                        is_edge: true,
                        ..a_src_edge_data
                    },
                    self.current_event_id,
                ));
            } else {
                tess_log!(self, "flip active edge after intersection");
                flipped_active = true;
                self.events.insert_sorted(
                    active_edge.to,
                    EdgeData {
                        range: active_edge.range_end..remapped_ta,
                        winding: -active_edge.winding,
                        to: intersection_position,
                        is_edge: true,
                        ..a_src_edge_data
                    },
                    self.current_event_id,
                );
            }

            active_edge.to = intersection_position;
            active_edge.range_end = remapped_ta;
        }

        if edge_below.to != intersection_position && self.current_position != intersection_position
        {
            let remapped_tb =
                remap_t_in_range(tb as f32, b_src_edge_data.range.start..edge_below.range_end);

            if is_after(edge_below.to, intersection_position) {
                let edge_data = EdgeData {
                    range: remapped_tb..edge_below.range_end,
                    winding: edge_below.winding,
                    to: edge_below.to,
                    is_edge: true,
                    ..b_src_edge_data
                };

                if let Some(idx) = inserted_evt {
                    // Should take this branch most of the time.
                    self.events
                        .insert_sibling(idx, intersection_position, edge_data);
                } else {
                    self.events.insert_sorted(
                        intersection_position,
                        edge_data,
                        self.current_event_id,
                    );
                }
            } else {
                tess_log!(self, "flip edge below after intersection");
                self.events.insert_sorted(
                    edge_below.to,
                    EdgeData {
                        range: edge_below.range_end..remapped_tb,
                        winding: -edge_below.winding,
                        to: intersection_position,
                        is_edge: true,
                        ..b_src_edge_data
                    },
                    self.current_event_id,
                );

                if flipped_active {
                    // It is extremely rare but if we end up flipping both of the
                    // edges that are inserted in the event queue, then we created a
                    // merge event which means we have to insert a vertex event into
                    // the queue, otherwise the tessellator will skip over the end of
                    // these two edges.
                    self.events.vertex_event_sorted(
                        intersection_position,
                        b_src_edge_data.to_id,
                        self.current_event_id,
                    );
                }
            }

            edge_below.to = intersection_position;
            edge_below.range_end = remapped_tb;
        }
    }

    fn sort_active_edges(&mut self) {
        // Merge edges are a little subtle when it comes to sorting.
        // They are points rather than edges and the best we can do is
        // keep their relative ordering with their previous or next edge.
        // Unfortunately this can cause merge vertices to end up outside of
        // the shape.
        // After sorting we go through the active edges and rearrange merge
        // vertices to prevent that.

        let y = self.current_position.y;

        let mut keys = Vec::with_capacity(self.active.edges.len());

        let mut has_merge_vertex = false;
        let mut prev_x = f32::NAN;
        for (i, edge) in self.active.edges.iter().enumerate() {
            if edge.is_merge {
                debug_assert!(!prev_x.is_nan());
                has_merge_vertex = true;
                keys.push((prev_x, i));
            } else {
                debug_assert!(!is_after(self.current_position, edge.to));

                let eq_to = edge.to.y == y;
                let eq_from = edge.from.y == y;

                let x = if eq_to && eq_from {
                    let current_x = self.current_position.x;
                    if edge.max_x() >= current_x && edge.min_x() <= current_x {
                        self.current_position.x
                    } else {
                        edge.min_x()
                    }
                } else if eq_from {
                    edge.from.x
                } else if eq_to {
                    edge.to.x
                } else {
                    edge.solve_x_for_y(y)
                };

                keys.push((fmax(x, edge.min_x()), i));
                prev_x = x;
            }
        }

        keys.sort_by(|a, b| match a.0.partial_cmp(&b.0).unwrap() {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let a = &self.active.edges[a.1];
                let b = &self.active.edges[b.1];
                match (a.is_merge, b.is_merge) {
                    (false, false) => {
                        let slope_a = slope(a.to - a.from);
                        let slope_b = slope(b.to - b.from);
                        slope_b.partial_cmp(&slope_a).unwrap_or(Ordering::Equal)
                    }
                    (true, false) => Ordering::Greater,
                    (false, true) => Ordering::Less,
                    (true, true) => Ordering::Equal,
                }
            }
        });

        let mut new_active_edges = Vec::with_capacity(self.active.edges.len());
        for &(_, idx) in &keys {
            new_active_edges.push(self.active.edges[idx]);
        }

        self.active.edges = new_active_edges;

        if !has_merge_vertex {
            return;
        }

        let mut winding_number = 0;
        for i in 0..self.active.edges.len() {
            let needs_swap = {
                let edge = &self.active.edges[i];
                if edge.is_merge {
                    !self.fill_rule.is_in(winding_number)
                } else {
                    winding_number += edge.winding;
                    false
                }
            };

            if needs_swap {
                let mut w = winding_number;
                tess_log!(self, "Fixing up merge vertex after sort.");
                let mut idx = i;
                loop {
                    // Roll back previous edge winding and swap.
                    w -= self.active.edges[idx - 1].winding;
                    self.active.edges.swap(idx, idx - 1);

                    if self.fill_rule.is_in(w) {
                        break;
                    }

                    idx -= 1;
                }
            }
        }
    }

    #[inline(never)]
    fn recover_from_error(&mut self, _error: InternalError, output: &mut dyn FillGeometryBuilder) {
        tess_log!(self, "Attempt to recover error {:?}", _error);

        self.sort_active_edges();

        debug_assert!(self
            .active
            .edges
            .first()
            .map(|e| !e.is_merge)
            .unwrap_or(true));
        // This can only happen if we ignore self-intersections,
        // so we are in a pretty broken state already.
        // There isn't a fully correct solution for this (other
        // than properly detecting self intersections and not
        // getting into this situation), but the rest of the code
        // doesn't deal with merge edges being at the last position
        // so we artificially move them to avoid that.
        // TODO: with self-intersections properly handled it may make more sense
        // to turn this into an assertion.
        let len = self.active.edges.len();
        if len > 1 && self.active.edges[len - 1].is_merge {
            self.active.edges.swap(len - 1, len - 2);
        }

        let mut winding = WindingState::new();

        for edge in &self.active.edges {
            if edge.is_merge {
                winding.span_index += 1;
            } else {
                winding.update(self.fill_rule, edge.winding);
            }

            if winding.span_index >= self.fill.spans.len() as i32 {
                self.fill
                    .begin_span(winding.span_index, &edge.from, edge.from_id);
            }
        }

        while self.fill.spans.len() > (winding.span_index + 1) as usize {
            self.fill.spans.last_mut().unwrap().tess().flush(output);
            self.fill.spans.pop();
        }

        tess_log!(self, "-->");

        #[cfg(debug_assertions)]
        self.log_active_edges();
    }

    #[cfg_attr(feature = "profiling", inline(never))]
    fn sort_edges_below(&mut self) {
        self.edges_below
            .sort_unstable_by(|a, b| a.sort_key.partial_cmp(&b.sort_key).unwrap_or(Ordering::Equal));
    }

    #[cfg_attr(feature = "profiling", inline(never))]
    fn handle_coincident_edges_below(&mut self) {
        if self.edges_below.len() < 2 {
            return;
        }

        for idx in (0..(self.edges_below.len() - 1)).rev() {
            let a_idx = idx;
            let b_idx = idx + 1;

            let a_slope = self.edges_below[a_idx].sort_key;
            let b_slope = self.edges_below[b_idx].sort_key;

            const THRESHOLD: f32 = 0.00005;

            // The slope function preserves the ordering for sorting but isn't a very good approximation
            // of the angle as edges get closer to horizontal.
            // When edges are larger in x than y, comparing the inverse is a better approximation.
            let angle_is_close = if a_slope.abs() <= 1.0 {
                (a_slope - b_slope).abs() < THRESHOLD
            } else {
                (1.0 / a_slope - 1.0 / b_slope).abs() < THRESHOLD
            };

            if angle_is_close {
                self.merge_coincident_edges(a_idx, b_idx);
            }
        }
    }

    #[cold]
    fn merge_coincident_edges(&mut self, a_idx: usize, b_idx: usize) {
        let a_to = self.edges_below[a_idx].to;
        let b_to = self.edges_below[b_idx].to;

        let (lower_idx, upper_idx, split) = match compare_positions(a_to, b_to) {
            Ordering::Greater => (a_idx, b_idx, true),
            Ordering::Less => (b_idx, a_idx, true),
            Ordering::Equal => (a_idx, b_idx, false),
        };

        tess_log!(
            self,
            "coincident edges {:?} -> {:?} / {:?}",
            self.current_position,
            a_to,
            b_to
        );

        tess_log!(
            self,
            "update winding: {:?} -> {:?}",
            self.edges_below[upper_idx].winding,
            self.edges_below[upper_idx].winding + self.edges_below[lower_idx].winding
        );
        self.edges_below[upper_idx].winding += self.edges_below[lower_idx].winding;
        let split_point = self.edges_below[upper_idx].to;

        tess_log!(
            self,
            "remove coincident edge {:?}, split:{:?}",
            a_idx,
            split
        );
        let edge = self.edges_below.remove(lower_idx);

        if !split {
            return;
        }

        let src_edge_data = self.events.edge_data[edge.src_edge as usize].clone();

        let t = LineSegment {
            from: self.current_position,
            to: edge.to,
        }
        .solve_t_for_y(split_point.y);

        let src_range = src_edge_data.range.start..edge.range_end;
        let t_remapped = remap_t_in_range(t, src_range);

        let edge_data = EdgeData {
            range: t_remapped..edge.range_end,
            winding: edge.winding,
            to: edge.to,
            is_edge: true,
            ..src_edge_data
        };

        self.events
            .insert_sorted(split_point, edge_data, self.current_event_id);
    }

    fn reset(&mut self) {
        self.current_position = point(f32::MIN, f32::MIN);
        self.current_vertex = VertexId::INVALID;
        self.current_event_id = INVALID_EVENT_ID;
        self.active.edges.clear();
        self.edges_below.clear();
        self.fill.spans.clear();
    }
}

pub(crate) fn points_are_equal(a: Point, b: Point) -> bool {
    a == b
}

pub(crate) fn compare_positions(a: Point, b: Point) -> Ordering {
    // This function is somewhat hot during the sorting phase but it might be that inlining
    // moves the cost of fetching the positions here.
    // The y coordinates are rarely equal (typically less than 7% of the time) but it's
    // unclear whether moving the x comparison out into a cold function helps in practice.
    if a.y > b.y {
        return Ordering::Greater;
    }
    if a.y < b.y {
        return Ordering::Less;
    }
    if a.x > b.x {
        return Ordering::Greater;
    }
    if a.x < b.x {
        return Ordering::Less;
    }

    Ordering::Equal
}

#[inline]
pub(crate) fn is_after(a: Point, b: Point) -> bool {
    a.y > b.y || (a.y == b.y && a.x > b.x)
}

#[inline]
pub(crate) fn is_near(a: Point, b: Point) -> bool {
    (a - b).square_length() < 0.000000001
}

#[inline]
fn reorient(p: Point) -> Point {
    point(p.y, -p.x)
}

/// Extra vertex information from the `FillTessellator`, accessible when building vertices.
pub struct FillVertex<'l> {
    pub(crate) position: Point,
    pub(crate) events: &'l EventQueue,
    pub(crate) current_event: TessEventId,
    pub(crate) attrib_buffer: &'l mut [f32],
    pub(crate) attrib_store: Option<&'l dyn AttributeStore>,
}

impl<'l> FillVertex<'l> {
    pub fn position(&self) -> Point {
        self.position
    }

    /// Return an iterator over the sources of the vertex.
    pub fn sources(&self) -> VertexSourceIterator {
        VertexSourceIterator {
            events: self.events,
            id: self.current_event,
            prev: None,
        }
    }

    /// Returns the first endpoint that this vertex is on, if any.
    ///
    /// This is meant to be used only in very simple cases where self-intersections,
    /// overlapping vertices and curves are unexpected.
    /// This will return `None` at self-intersections and between the endpoints of
    /// a flattened curve. If two endpoints are at the same position only one of
    /// them is returned.
    ///
    /// See also: `FillVertex::sources`.
    pub fn as_endpoint_id(&self) -> Option<EndpointId> {
        let mut current = self.current_event;
        while self.events.valid_id(current) {
            let edge = &self.events.edge_data[current as usize];
            let t = edge.range.start;
            if t == 0.0 {
                return Some(edge.from_id);
            }
            if t == 1.0 {
                return Some(edge.to_id);
            }

            current = self.events.next_sibling_id(current)
        }

        None
    }

    /// Fetch or interpolate the custom attribute values at this vertex.
    pub fn interpolated_attributes(&mut self) -> Attributes {
        if self.attrib_store.is_none() {
            return NO_ATTRIBUTES;
        }

        let store = self.attrib_store.unwrap();

        let mut sources = VertexSourceIterator {
            events: self.events,
            id: self.current_event,
            prev: None,
        };

        let num_attributes = store.num_attributes();

        let first = sources.next().unwrap();
        let mut next = sources.next();

        // Fast path for the single-source-single-endpoint common case.
        if next.is_none() {
            if let VertexSource::Endpoint { id } = first {
                return store.get(id);
            }
        }

        // First source taken out of the loop to avoid initializing the buffer.
        match first {
            VertexSource::Endpoint { id } => {
                let a = store.get(id);
                assert_eq!(a.len(), num_attributes);
                assert_eq!(self.attrib_buffer.len(), num_attributes);
                self.attrib_buffer[..num_attributes].clone_from_slice(&a[..num_attributes]);
            }
            VertexSource::Edge { from, to, t } => {
                let a = store.get(from);
                let b = store.get(to);
                assert_eq!(a.len(), num_attributes);
                assert_eq!(b.len(), num_attributes);
                assert_eq!(self.attrib_buffer.len(), num_attributes);
                for i in 0..num_attributes {
                    self.attrib_buffer[i] = a[i] * (1.0 - t) + b[i] * t;
                }
            }
        }

        let mut div = 1.0;
        loop {
            match next {
                Some(VertexSource::Endpoint { id }) => {
                    let a = store.get(id);
                    assert_eq!(a.len(), num_attributes);
                    assert_eq!(self.attrib_buffer.len(), num_attributes);
                    for (i, &att) in a.iter().enumerate() {
                        self.attrib_buffer[i] += att;
                    }
                }
                Some(VertexSource::Edge { from, to, t }) => {
                    let a = store.get(from);
                    let b = store.get(to);
                    assert_eq!(a.len(), num_attributes);
                    assert_eq!(b.len(), num_attributes);
                    assert_eq!(self.attrib_buffer.len(), num_attributes);
                    for i in 0..num_attributes {
                        self.attrib_buffer[i] += a[i] * (1.0 - t) + b[i] * t;
                    }
                }
                None => {
                    break;
                }
            }
            div += 1.0;
            next = sources.next();
        }

        if div > 1.0 {
            for attribute in self.attrib_buffer.iter_mut() {
                *attribute /= div;
            }
        }

        self.attrib_buffer
    }
}

/// An iterator over the sources of a given vertex.
#[derive(Clone)]
pub struct VertexSourceIterator<'l> {
    events: &'l EventQueue,
    id: TessEventId,
    prev: Option<VertexSource>,
}

impl<'l> Iterator for VertexSourceIterator<'l> {
    type Item = VertexSource;
    #[inline]
    fn next(&mut self) -> Option<VertexSource> {
        let mut src;
        loop {
            if self.id == INVALID_EVENT_ID {
                return None;
            }

            let edge = &self.events.edge_data[self.id as usize];

            self.id = self.events.next_sibling_id(self.id);

            let t = edge.range.start;

            src = if t == 0.0 {
                Some(VertexSource::Endpoint { id: edge.from_id })
            } else if t == 1.0 {
                Some(VertexSource::Endpoint { id: edge.to_id })
            } else {
                Some(VertexSource::Edge {
                    from: edge.from_id,
                    to: edge.to_id,
                    t,
                })
            };

            if src != self.prev {
                break;
            }
        }

        self.prev = src;
        src
    }
}

fn remap_t_in_range(val: f32, range: Range<f32>) -> f32 {
    if range.end > range.start {
        let d = range.end - range.start;
        range.start + val * d
    } else {
        let d = range.start - range.end;
        range.end + (1.0 - val) * d
    }
}

pub struct FillBuilder<'l> {
    events: EventQueueBuilder,
    next_id: EndpointId,
    first_id: EndpointId,
    first_position: Point,
    horizontal_sweep: bool,
    attrib_store: SimpleAttributeStore,
    tessellator: &'l mut FillTessellator,
    output: &'l mut dyn FillGeometryBuilder,
    options: &'l FillOptions,
}

impl<'l> FillBuilder<'l> {
    fn new(
        num_attributes: usize,
        tessellator: &'l mut FillTessellator,
        options: &'l FillOptions,
        output: &'l mut dyn FillGeometryBuilder,
    ) -> Self {
        let events = core::mem::take(&mut tessellator.events)
            .into_builder(options.tolerance);

        FillBuilder {
            events,
            next_id: EndpointId(0),
            first_id: EndpointId(0),
            horizontal_sweep: options.sweep_orientation == Orientation::Horizontal,
            first_position: point(0.0, 0.0),
            tessellator,
            options,
            output,
            attrib_store: SimpleAttributeStore::new(num_attributes),
        }
    }

    #[inline]
    fn position(&self, p: Point) -> Point {
        if self.horizontal_sweep {
            point(-p.y, p.x)
        } else {
            p
        }
    }

    pub fn num_attributes(&self) -> usize {
        self.attrib_store.num_attributes()
    }

    pub fn begin(&mut self, at: Point, attributes: Attributes) -> EndpointId {
        let at = self.position(at);
        let id = self.attrib_store.add(attributes);
        self.first_id = id;
        self.first_position = at;
        self.events.begin(at, id);

        id
    }

    pub fn end(&mut self, _close: bool) {
        self.events.end(self.first_position, self.first_id);
    }

    pub fn line_to(&mut self, to: Point, attributes: Attributes) -> EndpointId {
        let to = self.position(to);
        let id = self.attrib_store.add(attributes);
        self.events.line_segment(to, id, 0.0, 1.0);

        id
    }

    pub fn quadratic_bezier_to(
        &mut self,
        ctrl: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        let ctrl = self.position(ctrl);
        let to = self.position(to);
        let id = self.attrib_store.add(attributes);
        self.events.quadratic_bezier_segment(ctrl, to, id);

        id
    }

    pub fn cubic_bezier_to(
        &mut self,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        let ctrl1 = self.position(ctrl1);
        let ctrl2 = self.position(ctrl2);
        let to = self.position(to);
        let id = self.attrib_store.add(attributes);
        self.events.cubic_bezier_segment(ctrl1, ctrl2, to, id);

        id
    }

    pub fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.attrib_store.reserve(endpoints);
        self.events.reserve(endpoints + ctrl_points * 2);
    }

    pub fn add_circle(
        &mut self,
        center: Point,
        radius: f32,
        winding: Winding,
        attributes: Attributes,
    ) {
        // This specialized routine extracts the curves into separate sub-paths
        // to nudge the tessellator towards putting them in their own monotonic
        // spans. This avoids generating thin triangles from one side of the circle
        // to the other.
        // We can do this because we know shape is convex and we don't need to trace
        // the outline.

        let radius = radius.abs();
        let dir = match winding {
            Winding::Positive => 1.0,
            Winding::Negative => -1.0,
        };

        self.reserve(16, 8);

        let tan_pi_over_8 = 0.41421357;
        let d = radius * tan_pi_over_8;

        let start = center + vector(-radius, 0.0);
        self.begin(start, attributes);
        let ctrl_0 = center + vector(-radius, -d * dir);
        let mid_0 = center + vector(-1.0, -dir) * radius * FRAC_1_SQRT_2;
        let ctrl_1 = center + vector(-d, -radius * dir);
        let mid_1 = center + vector(0.0, -radius * dir);
        self.quadratic_bezier_to(ctrl_0, mid_0, attributes);
        self.end(false);
        self.begin(mid_0, attributes);
        self.quadratic_bezier_to(ctrl_1, mid_1, attributes);
        self.end(false);

        self.begin(mid_1, attributes);
        let ctrl_0 = center + vector(d, -radius * dir);
        let mid_2 = center + vector(1.0, -dir) * radius * FRAC_1_SQRT_2;
        let ctrl_1 = center + vector(radius, -d * dir);
        let mid_3 = center + vector(radius, 0.0);
        self.quadratic_bezier_to(ctrl_0, mid_2, attributes);
        self.end(false);
        self.begin(mid_2, attributes);
        self.quadratic_bezier_to(ctrl_1, mid_3, attributes);
        self.end(false);

        self.begin(mid_3, attributes);
        let ctrl_0 = center + vector(radius, d * dir);
        let mid_4 = center + vector(1.0, dir) * radius * FRAC_1_SQRT_2;
        let ctrl_1 = center + vector(d, radius * dir);
        let mid_5 = center + vector(0.0, radius * dir);
        self.quadratic_bezier_to(ctrl_0, mid_4, attributes);
        self.end(false);
        self.begin(mid_4, attributes);
        self.quadratic_bezier_to(ctrl_1, mid_5, attributes);
        self.end(false);

        self.begin(mid_5, attributes);
        let ctrl_0 = center + vector(-d, radius * dir);
        let mid_6 = center + vector(-1.0, dir) * radius * FRAC_1_SQRT_2;
        let ctrl_1 = center + vector(-radius, d * dir);
        self.quadratic_bezier_to(ctrl_0, mid_6, attributes);
        self.end(false);
        self.begin(mid_6, attributes);
        self.quadratic_bezier_to(ctrl_1, start, attributes);
        self.end(false);

        self.begin(start, attributes);
        self.line_to(mid_0, attributes);
        self.line_to(mid_1, attributes);
        self.line_to(mid_2, attributes);
        self.line_to(mid_3, attributes);
        self.line_to(mid_4, attributes);
        self.line_to(mid_5, attributes);
        self.line_to(mid_6, attributes);
        self.close();
    }

    pub fn build(self) -> TessellationResult {
        let mut event_queue = self.events.build();
        core::mem::swap(&mut self.tessellator.events, &mut event_queue);

        let attrib_store = if self.attrib_store.num_attributes > 0 {
            Some(&self.attrib_store as &dyn AttributeStore)
        } else {
            None
        };

        self.tessellator
            .tessellate_impl(self.options, attrib_store, self.output)
    }
}

impl<'l> PathBuilder for FillBuilder<'l> {
    fn num_attributes(&self) -> usize {
        self.attrib_store.num_attributes()
    }

    fn begin(&mut self, at: Point, attributes: Attributes) -> EndpointId {
        self.begin(at, attributes)
    }

    fn end(&mut self, close: bool) {
        self.end(close)
    }

    fn line_to(&mut self, to: Point, attributes: Attributes) -> EndpointId {
        self.line_to(to, attributes)
    }

    fn quadratic_bezier_to(
        &mut self,
        ctrl: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        self.quadratic_bezier_to(ctrl, to, attributes)
    }

    fn cubic_bezier_to(
        &mut self,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
        attributes: Attributes,
    ) -> EndpointId {
        self.cubic_bezier_to(ctrl1, ctrl2, to, attributes)
    }

    fn reserve(&mut self, endpoints: usize, ctrl_points: usize) {
        self.reserve(endpoints, ctrl_points)
    }

    fn add_circle(&mut self, center: Point, radius: f32, winding: Winding, attributes: Attributes) {
        self.add_circle(center, radius, winding, attributes)
    }
}

impl<'l> Build for FillBuilder<'l> {
    type PathType = TessellationResult;

    #[inline]
    fn build(self) -> TessellationResult {
        self.build()
    }
}

fn log_svg_preamble(_tess: &FillTessellator) {
    tess_log!(
        _tess,
        r#"
<svg viewBox="0 0 1000 1000">

<style type="text/css">
<![CDATA[
  path.sweep-line {{
    stroke: red;
    fill: none;
  }}

  path.edge {{
    stroke: blue;
    fill: none;
  }}

  path.edge.select {{
    stroke: green;
    fill: none;
  }}

  circle.merge {{
    fill: yellow;
    stroke: orange;
    fill-opacity: 1;
  }}

  circle.current {{
    fill: white;
    stroke: grey;
    fill-opacity: 1;
  }}

  g.active-edges {{
    opacity: 0;
  }}

  g.active-edges.select {{
    opacity: 1;
  }}
]]>
</style>
"#
    );
}

#[cfg(test)]
use crate::geometry_builder::*;

#[cfg(test)]
fn eq(a: Point, b: Point) -> bool {
    (a.x - b.x).abs() < 0.00001 && (a.y - b.y).abs() < 0.00001
}

#[cfg(test)]
fn at_endpoint(src: &VertexSource, endpoint: EndpointId) -> bool {
    match src {
        VertexSource::Edge { .. } => false,
        VertexSource::Endpoint { id } => *id == endpoint,
    }
}

#[cfg(test)]
fn on_edge(src: &VertexSource, from_id: EndpointId, to_id: EndpointId, d: f32) -> bool {
    match src {
        VertexSource::Edge { t, from, to, .. } => {
            *from == from_id
                && *to == to_id
                && ((d - *t).abs() < 0.00001 || (1.0 - d - *t).abs() <= 0.00001)
        }
        VertexSource::Endpoint { .. } => false,
    }
}

#[test]
fn fill_vertex_source_01() {
    use crate::path::commands::PathCommands;
    use crate::path::AttributeSlice;

    let endpoints: &[Point] = &[point(0.0, 0.0), point(1.0, 1.0), point(0.0, 2.0)];

    let attributes = &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

    let mut cmds = PathCommands::builder();
    cmds.begin(EndpointId(0));
    cmds.line_to(EndpointId(1));
    cmds.line_to(EndpointId(2));
    cmds.end(true);

    let cmds = cmds.build();

    let mut tess = FillTessellator::new();
    tess.tessellate_with_ids(
        cmds.iter(),
        &(endpoints, endpoints),
        Some(&AttributeSlice::new(attributes, 3)),
        &FillOptions::default(),
        &mut CheckVertexSources { next_vertex: 0 },
    )
    .unwrap();

    struct CheckVertexSources {
        next_vertex: u32,
    }

    impl GeometryBuilder for CheckVertexSources {
        fn add_triangle(&mut self, _: VertexId, _: VertexId, _: VertexId) {}
        fn abort_geometry(&mut self) {}
    }

    impl FillGeometryBuilder for CheckVertexSources {
        fn add_fill_vertex(
            &mut self,
            mut vertex: FillVertex,
        ) -> Result<VertexId, GeometryBuilderError> {
            let pos = vertex.position();
            for src in vertex.sources() {
                if eq(pos, point(0.0, 0.0)) {
                    assert!(at_endpoint(&src, EndpointId(0)))
                } else if eq(pos, point(1.0, 1.0)) {
                    assert!(at_endpoint(&src, EndpointId(1)))
                } else if eq(pos, point(0.0, 2.0)) {
                    assert!(at_endpoint(&src, EndpointId(2)))
                } else {
                    panic!()
                }
            }

            if eq(pos, point(0.0, 0.0)) {
                assert_eq!(vertex.interpolated_attributes(), &[1.0, 0.0, 0.0])
            } else if eq(pos, point(1.0, 1.0)) {
                assert_eq!(vertex.interpolated_attributes(), &[0.0, 1.0, 0.0])
            } else if eq(pos, point(0.0, 2.0)) {
                assert_eq!(vertex.interpolated_attributes(), &[0.0, 0.0, 1.0])
            }

            let id = self.next_vertex;
            self.next_vertex += 1;

            Ok(VertexId(id))
        }
    }
}

#[test]
fn fill_vertex_source_02() {
    // Check the vertex sources of a simple self-intersecting shape.
    //    _
    //  _|_|_
    // | | | |
    // |_|_|_|
    //   |_|
    //

    let mut path = crate::path::Path::builder_with_attributes(3);
    let a = path.begin(point(1.0, 0.0), &[1.0, 0.0, 1.0]);
    let b = path.line_to(point(2.0, 0.0), &[2.0, 0.0, 1.0]);
    let c = path.line_to(point(2.0, 4.0), &[3.0, 0.0, 1.0]);
    let d = path.line_to(point(1.0, 4.0), &[4.0, 0.0, 1.0]);
    path.end(true);
    let e = path.begin(point(0.0, 1.0), &[0.0, 1.0, 2.0]);
    let f = path.line_to(point(0.0, 3.0), &[0.0, 2.0, 2.0]);
    let g = path.line_to(point(3.0, 3.0), &[0.0, 3.0, 2.0]);
    let h = path.line_to(point(3.0, 1.0), &[0.0, 4.0, 2.0]);
    path.end(true);

    let path = path.build();

    let mut tess = FillTessellator::new();
    tess.tessellate_with_ids(
        path.id_iter(),
        &path,
        Some(&path),
        &FillOptions::default(),
        &mut CheckVertexSources {
            next_vertex: 0,
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
        },
    )
    .unwrap();

    struct CheckVertexSources {
        next_vertex: u32,
        a: EndpointId,
        b: EndpointId,
        c: EndpointId,
        d: EndpointId,
        e: EndpointId,
        f: EndpointId,
        g: EndpointId,
        h: EndpointId,
    }

    impl GeometryBuilder for CheckVertexSources {
        fn add_triangle(&mut self, _: VertexId, _: VertexId, _: VertexId) {}
    }

    impl FillGeometryBuilder for CheckVertexSources {
        fn add_fill_vertex(
            &mut self,
            mut vertex: FillVertex,
        ) -> Result<VertexId, GeometryBuilderError> {
            let pos = vertex.position();
            for src in vertex.sources() {
                if eq(pos, point(1.0, 0.0)) {
                    assert!(at_endpoint(&src, self.a));
                } else if eq(pos, point(2.0, 0.0)) {
                    assert!(at_endpoint(&src, self.b));
                } else if eq(pos, point(2.0, 4.0)) {
                    assert!(at_endpoint(&src, self.c));
                } else if eq(pos, point(1.0, 4.0)) {
                    assert!(at_endpoint(&src, self.d));
                } else if eq(pos, point(0.0, 1.0)) {
                    assert!(at_endpoint(&src, self.e));
                } else if eq(pos, point(0.0, 3.0)) {
                    assert!(at_endpoint(&src, self.f));
                } else if eq(pos, point(3.0, 3.0)) {
                    assert!(at_endpoint(&src, self.g));
                } else if eq(pos, point(3.0, 1.0)) {
                    assert!(at_endpoint(&src, self.h));
                } else if eq(pos, point(1.0, 1.0)) {
                    assert!(
                        on_edge(&src, self.h, self.e, 2.0 / 3.0)
                            || on_edge(&src, self.d, self.a, 3.0 / 4.0)
                    );
                } else if eq(pos, point(2.0, 1.0)) {
                    assert!(
                        on_edge(&src, self.h, self.e, 1.0 / 3.0)
                            || on_edge(&src, self.b, self.c, 1.0 / 4.0)
                    );
                } else if eq(pos, point(1.0, 3.0)) {
                    assert!(
                        on_edge(&src, self.f, self.g, 1.0 / 3.0)
                            || on_edge(&src, self.d, self.a, 1.0 / 4.0)
                    );
                } else if eq(pos, point(2.0, 3.0)) {
                    assert!(
                        on_edge(&src, self.f, self.g, 2.0 / 3.0)
                            || on_edge(&src, self.b, self.c, 3.0 / 4.0)
                    );
                } else {
                    panic!()
                }
            }

            fn assert_attr(a: Attributes, b: Attributes) {
                for i in 0..a.len() {
                    let are_equal = (a[i] - b[i]).abs() < 0.001;
                    #[cfg(feature = "std")]
                    if !are_equal {
                        std::println!("{a:?} != {b:?}");
                    }
                    assert!(are_equal);
                }

                assert_eq!(a.len(), b.len());
            }

            let pos = vertex.position();
            let attribs = vertex.interpolated_attributes();
            if eq(pos, point(1.0, 0.0)) {
                assert_attr(attribs, &[1.0, 0.0, 1.0]);
            } else if eq(pos, point(2.0, 0.0)) {
                assert_attr(attribs, &[2.0, 0.0, 1.0]);
            } else if eq(pos, point(2.0, 4.0)) {
                assert_attr(attribs, &[3.0, 0.0, 1.0]);
            } else if eq(pos, point(1.0, 4.0)) {
                assert_attr(attribs, &[4.0, 0.0, 1.0]);
            } else if eq(pos, point(0.0, 1.0)) {
                assert_attr(attribs, &[0.0, 1.0, 2.0]);
            } else if eq(pos, point(0.0, 3.0)) {
                assert_attr(attribs, &[0.0, 2.0, 2.0]);
            } else if eq(pos, point(3.0, 3.0)) {
                assert_attr(attribs, &[0.0, 3.0, 2.0]);
            } else if eq(pos, point(3.0, 1.0)) {
                assert_attr(attribs, &[0.0, 4.0, 2.0]);
            } else if eq(pos, point(1.0, 1.0)) {
                assert_attr(attribs, &[0.875, 1.0, 1.5]);
            } else if eq(pos, point(2.0, 1.0)) {
                assert_attr(attribs, &[1.125, 1.5, 1.5]);
            } else if eq(pos, point(1.0, 3.0)) {
                assert_attr(attribs, &[1.625, 1.16666, 1.5]);
            } else if eq(pos, point(2.0, 3.0)) {
                assert_attr(attribs, &[1.375, 1.33333, 1.5]);
            }

            let id = self.next_vertex;
            self.next_vertex += 1;

            Ok(VertexId(id))
        }
    }
}

#[test]
fn fill_vertex_source_03() {
    use crate::path::commands::PathCommands;
    use crate::path::AttributeSlice;

    // x---x
    //  \ /
    //   x  <---
    //  / \
    // x---x
    //
    // check that the attribute interpolation is weighted correctly at
    // start events.

    let endpoints: &[Point] = &[
        point(0.0, 0.0),
        point(2.0, 0.0),
        point(1.0, 1.0),
        point(0.0, 2.0),
        point(2.0, 2.0),
        point(1.0, 1.0),
    ];

    let attributes = &[0.0, 0.0, 1.0, 0.0, 0.0, 2.0];

    let mut cmds = PathCommands::builder();
    cmds.begin(EndpointId(0));
    cmds.line_to(EndpointId(1));
    cmds.line_to(EndpointId(2));
    cmds.end(true);
    cmds.begin(EndpointId(3));
    cmds.line_to(EndpointId(4));
    cmds.line_to(EndpointId(5));
    cmds.end(true);

    let cmds = cmds.build();

    let mut tess = FillTessellator::new();
    tess.tessellate_with_ids(
        cmds.iter(),
        &(endpoints, endpoints),
        Some(&AttributeSlice::new(attributes, 1)),
        &FillOptions::default(),
        &mut CheckVertexSources { next_vertex: 0 },
    )
    .unwrap();

    struct CheckVertexSources {
        next_vertex: u32,
    }

    impl GeometryBuilder for CheckVertexSources {
        fn add_triangle(&mut self, _: VertexId, _: VertexId, _: VertexId) {}
    }

    impl FillGeometryBuilder for CheckVertexSources {
        fn add_fill_vertex(
            &mut self,
            mut vertex: FillVertex,
        ) -> Result<VertexId, GeometryBuilderError> {
            if eq(vertex.position(), point(1.0, 1.0)) {
                assert_eq!(vertex.interpolated_attributes(), &[1.5]);
                assert_eq!(vertex.sources().count(), 2);
            } else {
                assert_eq!(vertex.interpolated_attributes(), &[0.0]);
                assert_eq!(vertex.sources().count(), 1);
            }

            let id = self.next_vertex;
            self.next_vertex += 1;

            Ok(VertexId(id))
        }
    }
}

#[test]
fn fill_builder_vertex_source() {
    let mut tess = FillTessellator::new();
    let options = FillOptions::default();

    let mut check = CheckVertexSources { next_vertex: 0 };
    let mut builder = tess.builder(&options, &mut check);

    assert_eq!(builder.begin(point(0.0, 0.0)), EndpointId(0));
    assert_eq!(builder.line_to(point(1.0, 1.0)), EndpointId(1));
    assert_eq!(builder.line_to(point(0.0, 2.0)), EndpointId(2));
    builder.end(true);

    builder.build().unwrap();

    struct CheckVertexSources {
        next_vertex: u32,
    }

    impl GeometryBuilder for CheckVertexSources {
        fn add_triangle(&mut self, _: VertexId, _: VertexId, _: VertexId) {}
    }

    impl FillGeometryBuilder for CheckVertexSources {
        fn add_fill_vertex(
            &mut self,
            vertex: FillVertex,
        ) -> Result<VertexId, GeometryBuilderError> {
            let pos = vertex.position();
            for src in vertex.sources() {
                if eq(pos, point(0.0, 0.0)) {
                    assert!(at_endpoint(&src, EndpointId(0)))
                } else if eq(pos, point(1.0, 1.0)) {
                    assert!(at_endpoint(&src, EndpointId(1)))
                } else if eq(pos, point(0.0, 2.0)) {
                    assert!(at_endpoint(&src, EndpointId(2)))
                } else {
                    panic!()
                }
            }

            let id = self.next_vertex;
            self.next_vertex += 1;

            Ok(VertexId(id))
        }
    }
}
