struct QuadVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) background_color: vec4<f32>,
    @location(1) @interpolate(flat) border_color: vec4<f32>,
    @location(2) @interpolate(flat) quad_id: u32,
    @location(3) clip_distance: vec4<f32>,
};

/*
struct QuadFragmentInput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) background_color: vec4<f32>,
    @location(1) @interpolate(flat) border_color: vec4<f32>,
    @location(2) @interpolate(flat) quad_id: u32,
    @location(3) clip_distance: vec4<f32>,
};
*/

struct ViewId {
    low_bits: u32,
    high_bits: u32,
};

alias LayerId = u32;

alias DrawOrder = u32;

alias ScaledPixels = f32;
alias DevicePixels = i32;

struct Point_ScaledPixels {
    x: ScaledPixels,
    y: ScaledPixels,
};

struct Size_ScaledPixels {
    width: ScaledPixels,
    height: ScaledPixels,
};

struct Size_DevicePixels {
    width: DevicePixels,
    height: DevicePixels,
};

struct Bounds_ScaledPixels {
    origin: Point_ScaledPixels,
    size: Size_ScaledPixels,
};

struct ContentMask_ScaledPixels {
    bounds: Bounds_ScaledPixels,
};

struct Corners_ScaledPixels {
    top_left: ScaledPixels,
    top_right: ScaledPixels,
    bottom_right: ScaledPixels,
    bottom_left: ScaledPixels,
};

struct Edges_ScaledPixels {
    top: ScaledPixels,
    right: ScaledPixels,
    bottom: ScaledPixels,
    left: ScaledPixels,
}

struct Hsla {
    h: f32,
    s: f32,
    l: f32,
    a: f32,
};

struct Quad {
    view_id: ViewId,
    layer_id: LayerId,
    order: DrawOrder,
    bounds: Bounds_ScaledPixels,
    content_mask: ContentMask_ScaledPixels,
    background: Hsla,
    border_color: Hsla,
    corner_radii: Corners_ScaledPixels,
    border_widths: Edges_ScaledPixels,
};

@group(0) @binding(0)
var<storage> unit_vertices: array<vec2<f32>>;

@group(0) @binding(1)
var<storage> quads: array<Quad>;

@group(0) @binding(2)
var<storage> viewport_size: Size_DevicePixels;


@vertex
fn vertex(
    @builtin(vertex_index) unit_vertex_id: u32,
    @builtin(instance_index) quad_id: u32,
) -> QuadVertexOutput {
    let unit_vertex = unit_vertices[unit_vertex_id];
    let quad = quads[quad_id];
    let device_position = to_device_position(unit_vertex, quad.bounds);
/*
    float4 clip_distance = distance_from_clip_rect(
        unit_vertex,
        quad.bounds,
        quad.content_mask.bounds
    );
  float4 background_color = hsla_to_rgba(quad.background);
  float4 border_color = hsla_to_rgba(quad.border_color);
  return QuadVertexOutput {
      device_position,
      background_color,
      border_color,
      quad_id,
      {clip_distance.x, clip_distance.y, clip_distance.z, clip_distance.w}
    };
*/

    return QuadVertexOutput();
}

fn to_device_position(
    unit_vertex: vec2<f32>,
    bounds: Bounds_ScaledPixels,
) -> vec4<f32> {
    let position = unit_vertex
        * vec2<f32>(bounds.size.width, bounds.size.height)
        + vec2<f32>(bounds.origin.x, bounds.origin.y);

    let viewport_size = vec2<f32>(f32(viewport_size.width), f32(viewport_size.height));
    let device_position = position / viewport_size * vec2<f32>(2., -2.) + vec2<f32>(-1., 1.);

    return vec4<f32>(device_position, 0., 1.);
}

