struct Bounds {
    origin: vec2<f32>,
    size: vec2<f32>,
}
struct Corners {
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
}
struct Edges {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}
struct Hsla {
    h: f32,
    s: f32,
    l: f32,
    a: f32,
}

struct Quad {
    view_id: vec2<u32>,
    layer_id: u32,
    order: u32,
    bounds: Bounds,
    content_mask: Bounds,
    background: Hsla,
    border_color: Hsla,
    corner_radii: Corners,
    border_widths: Edges,
}

struct Globals {
    viewport_size: vec2<f32>,
    pad: vec2<u32>,
}

var<uniform> globals: Globals;
var<storage, read> quads: array<Quad>;

struct QuadsVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) background_color: vec4<f32>,
    @location(1) @interpolate(flat) border_color: vec4<f32>,
    @location(2) @interpolate(flat) quad_id: u32,
    //TODO: use `clip_distance` once Naga supports it
    @location(3) clip_distances: vec4<f32>,
}

fn to_device_position(unit_vertex: vec2<f32>, bounds: Bounds) -> vec4<f32> {
    let position = unit_vertex * vec2<f32>(bounds.size) + bounds.origin;
    let device_position = position / globals.viewport_size * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0);
    return vec4<f32>(device_position, 0.0, 1.0);
}

fn distance_from_clip_rect(unit_vertex: vec2<f32>, bounds: Bounds, clip_bounds: Bounds) -> vec4<f32> {
    let position = unit_vertex * vec2<f32>(bounds.size) + bounds.origin;
    let tl = position - clip_bounds.origin;
    let br = clip_bounds.origin + clip_bounds.size - position;
    return vec4<f32>(tl.x, br.x, tl.y, br.y);
}

fn hsla_to_rgba(hsla: Hsla) -> vec4<f32> {
    let h = hsla.h * 6.0; // Now, it's an angle but scaled in [0, 6) range
    let s = hsla.s;
    let l = hsla.l;
    let a = hsla.a;

    let c = (1.0 - abs(2.0 * l - 1.0)) * s;
    let x = c * (1.0 - abs(h % 2.0 - 1.0));
    let m = l - c / 2.0;

    var color = vec4<f32>(m, m, m, a);

    if (h >= 0.0 && h < 1.0) {
        color.r += c;
        color.g += x;
    } else if (h >= 1.0 && h < 2.0) {
        color.r += x;
        color.g += c;
    } else if (h >= 2.0 && h < 3.0) {
        color.g += c;
        color.b += x;
    } else if (h >= 3.0 && h < 4.0) {
        color.g += x;
        color.b += c;
    } else if (h >= 4.0 && h < 5.0) {
        color.r += x;
        color.b += c;
    } else {
        color.r += c;
        color.b += x;
    }

    return color;
}

fn over(below: vec4<f32>, above: vec4<f32>) -> vec4<f32> {
  let alpha = above.a + below.a * (1.0 - above.a);
  let color = (above.rgb * above.a + below.rgb * below.a * (1.0 - above.a)) / alpha;
  return vec4<f32>(color, alpha);
}

@vertex
fn vs_quads(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> QuadsVarying {
    let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
    let quad = quads[instance_id];

    var out = QuadsVarying();
    out.position = to_device_position(unit_vertex, quad.bounds);
    out.background_color = hsla_to_rgba(quad.background);
    out.border_color = hsla_to_rgba(quad.border_color);
    out.quad_id = instance_id;
    out.clip_distances = distance_from_clip_rect(unit_vertex, quad.bounds, quad.content_mask);
    return out;
}

@fragment
fn fs_quads(input: QuadsVarying) -> @location(0) vec4<f32> {
    // Alpha clip first, since we don't have `clip_distance`.
    let min_distance = min(
        min(input.clip_distances.x, input.clip_distances.y),
        min(input.clip_distances.z, input.clip_distances.w)
    );
    if min_distance <= 0.0 {
        return vec4<f32>(0.0);
    }

    let quad = quads[input.quad_id];
    let half_size = quad.bounds.size / 2.0;
    let center = quad.bounds.origin + half_size;
    let center_to_point = input.position.xy - center;

    var corner_radius = 0.0;
    if (center_to_point.x < 0.0) {
        if (center_to_point.y < 0.0) {
            corner_radius = quad.corner_radii.top_left;
        } else {
            corner_radius = quad.corner_radii.bottom_left;
        }
    } else {
        if (center_to_point.y < 0.) {
            corner_radius = quad.corner_radii.top_right;
        } else {
            corner_radius = quad.corner_radii.bottom_right;
        }
    }

    let rounded_edge_to_point = abs(center_to_point) - half_size + corner_radius;
    let distance =
      length(max(vec2<f32>(0.0), rounded_edge_to_point)) +
      min(0.0, max(rounded_edge_to_point.x, rounded_edge_to_point.y)) -
      corner_radius;

    let vertical_border = select(quad.border_widths.left, quad.border_widths.right, center_to_point.x > 0.0);
    let horizontal_border = select(quad.border_widths.top, quad.border_widths.bottom, center_to_point.y > 0.0);
    let inset_size = half_size - corner_radius - vec2<f32>(vertical_border, horizontal_border);
    let point_to_inset_corner = abs(center_to_point) - inset_size;

    var border_width = 0.0;
    if (point_to_inset_corner.x < 0.0 && point_to_inset_corner.y < 0.0) {
        border_width = 0.0;
    } else if (point_to_inset_corner.y > point_to_inset_corner.x) {
        border_width = horizontal_border;
    } else {
        border_width = vertical_border;
    }

    var color = input.background_color;
    if (border_width > 0.0) {
        let inset_distance = distance + border_width;
        // Blend the border on top of the background and then linearly interpolate
        // between the two as we slide inside the background.
        let blended_border = over(input.background_color, input.border_color);
        color = mix(blended_border, input.background_color,
                    saturate(0.5 - inset_distance));
    }

    return color * vec4<f32>(1.0, 1.0, 1.0, saturate(0.5 - distance));
}