[[stage(vertex)]]
fn quad_vertex(input: QuadVertexInput) -> QuadVertexOutput {
    var output: QuadVertexOutput;

        // Apply clip bounds
    input.bounds_origin = max(input.bounds_origin, input.clip_bounds.xy);
    input.bounds_size = min(input.bounds_size, input.clip_bounds.zw);

    var ndc: vec2<f32> = (input.bounds_origin / uniforms.window_size) * 2.0 - 1.0;

    output.position = vec4<f32>(ndc, 0.0, 1.0);
    output.position.y = -output.position.y; // Inverting since NDC's origin is at the center and y is up
    output.color = input.color;
    output.bounds = input.bounds_size / uniforms.window_size; // Convert size to NDC
    output.corner_radii = input.corner_radii / uniforms.window_size; // Convert corner radii to NDC
    output.clip_bounds = input.clip_bounds / uniforms.window_size; // Convert clip bounds to NDC
    output.clip_corner_radii = input.corner_radii / uniforms.window_size; // Convert clip corner radii to NDC

    return output;
}

// #[derive(Debug, Clone, Copy)]
// #[repr(C)]
// pub struct gpui3::scene::Quad {
//     pub order: f32,
//     pub bounds: Bounds<Pixels>,
//     pub clip_bounds: Bounds<Pixels>,
//     pub clip_corner_radii: Corners<Pixels>,
//     pub background: Hsla,
//     pub border_color: Hsla,
//     pub corner_radii: Corners<Pixels>,
//     pub border_widths: Edges<Pixels>,
// }

struct QuadVertexInput {
    [[location(0)]] order: f32;
    [[location(1)]] bounds: vec4<f32>; // Bounds<Pixels>
    [[location(2)]] clip_bounds: vec4<f32>; // Bounds<Pixels>
    [[location(3)]] clip_corner_radii: vec4<f32>; // Corners<Pixels>
    [[location(4)]] color: vec4<f32>; // Hsla
    [[location(5)]] border_color: vec4<f32>; // Hsla
    [[location(6)]] corner_radii: vec4<f32>; // Corners<Pixels>
    [[location(7)]] border_widths: vec4<f32>; // Edges<Pixels>
};

[[block]]
struct Uniforms {
    viewport: vec2<f32>;
};

[[binding(0), group(0)]] var<uniform> uniforms: Uniforms;

struct QuadVertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] border_color: vec4<f32>;
    [[location(2)]] bounds: vec4<f32>;
    [[location(3)]] corner_radii: vec4<f32>; // assuming topLeft, topRight, bottomRight, bottomLeft
    [[location(4)]] clip_bounds: vec4<f32>;
    [[location(5)]] clip_corner_radii: vec4<f32>;
    [[location(6)]] border_widths: vec4<f32>;
};

[[stage(fragment)]]
fn quad_fragment(input: QuadVertexOutput) -> [[location(0)]] vec4<f32> {
    var output_color: vec4<f32>;
    var sdf = rounded_quad_sdf(input.position, input.bounds, input.corner_radii);
    var alpha = clamp(1.0 - sdf, 0.0, 1.0);
    var border_color: vec4<f32> = input.border_color;
    var mix_factor: f32 = 1.0 - clamp(sdf, 0.0, 1.0); // Mixing factor dependent on sdf distance

     var border_width_factor: vec4<f32> = normalize(input.border_widths); // Normalizing the border width to account for directional widths

        output_color = mix(input.color, border_color, mix_factor * border_width_factor); // Modulate the mix_factor with the border_width_factor to handle different border widths

    output_color.a = alpha;

    return output_color;
}

[[stage(fragment)]]
fn rounded_quad_sdf(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    var rx: vec2<f32>;
    rx = select(r.xy, r.zw, greaterThan(p.x, 0.0));
    rx.x = select(rx.x, rx.y, greaterThan(p.y, 0.0));
    var q: vec2<f32> = abs(p)-b+rx.x;
    return min(max(q.x,q.y),0.0) + length(max(q, vec2<f32>(0.0, 0.0))) - rx.x;
}
