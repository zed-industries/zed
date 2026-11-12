/* Functions useful for debugging:

// A heat map color for debugging (blue -> cyan -> green -> yellow -> red).
fn heat_map_color(value: f32, minValue: f32, maxValue: f32, position: vec2<f32>) -> vec4<f32> {
    // Normalize value to 0-1 range
    let t = clamp((value - minValue) / (maxValue - minValue), 0.0, 1.0);

    // Heat map color calculation
    let r = t * t;
    let g = 4.0 * t * (1.0 - t);
    let b = (1.0 - t) * (1.0 - t);
    let heat_color = vec3<f32>(r, g, b);

    // Create a checkerboard pattern (black and white)
    let sum = floor(position.x / 3) + floor(position.y / 3);
    let is_odd = fract(sum * 0.5); // 0.0 for even, 0.5 for odd
    let checker_value = is_odd * 2.0; // 0.0 for even, 1.0 for odd
    let checker_color = vec3<f32>(checker_value);

    // Determine if value is in range (1.0 if in range, 0.0 if out of range)
    let in_range = step(minValue, value) * step(value, maxValue);

    // Mix checkerboard and heat map based on whether value is in range
    let final_color = mix(checker_color, heat_color, in_range);

    return vec4<f32>(final_color, 1.0);
}

*/

// Contrast and gamma correction adapted from https://github.com/microsoft/terminal/blob/1283c0f5b99a2961673249fa77c6b986efb5086c/src/renderer/atlas/dwrite.hlsl
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.
fn color_brightness(color: vec3<f32>) -> f32 {
    // REC. 601 luminance coefficients for perceived brightness
    return dot(color, vec3<f32>(0.30, 0.59, 0.11));
}

fn light_on_dark_contrast(enhancedContrast: f32, color: vec3<f32>) -> f32 {
    let brightness = color_brightness(color);
    let multiplier = saturate(4.0 * (0.75 - brightness));
    return enhancedContrast * multiplier;
}

fn enhance_contrast(alpha: f32, k: f32) -> f32 {
    return alpha * (k + 1.0) / (alpha * k + 1.0);
}

fn enhance_contrast3(alpha: vec3<f32>, k: f32) -> vec3<f32> {
    return alpha * (k + 1.0) / (alpha * k + 1.0);
}

fn apply_alpha_correction(a: f32, b: f32, g: vec4<f32>) -> f32 {
    let brightness_adjustment = g.x * b + g.y;
    let correction = brightness_adjustment * a + (g.z * b + g.w);
    return a + a * (1.0 - a) * correction;
}

fn apply_alpha_correction3(a: vec3<f32>, b: vec3<f32>, g: vec4<f32>) -> vec3<f32> {
    let brightness_adjustment = g.x * b + g.y;
    let correction = brightness_adjustment * a + (g.z * b + g.w);
    return a + a * (1.0 - a) * correction;
}

fn apply_contrast_and_gamma_correction(sample: f32, color: vec3<f32>, enhanced_contrast_factor: f32, gamma_ratios: vec4<f32>) -> f32 {
    let enhanced_contrast = light_on_dark_contrast(enhanced_contrast_factor, color);
    let brightness = color_brightness(color);

    let contrasted = enhance_contrast(sample, enhanced_contrast);
    return apply_alpha_correction(contrasted, brightness, gamma_ratios);
}

fn apply_contrast_and_gamma_correction3(sample: vec3<f32>, color: vec3<f32>, enhanced_contrast_factor: f32, gamma_ratios: vec4<f32>) -> vec3<f32> {
    let enhanced_contrast = light_on_dark_contrast(enhanced_contrast_factor, color);

    let contrasted = enhance_contrast3(sample, enhanced_contrast);
    return apply_alpha_correction3(contrasted, color, gamma_ratios);
}

struct GlobalParams {
    viewport_size: vec2<f32>,
    path_paint_base: u32,
    path_curve_base: u32,
    depth_scale: f32,
    quad_count: u32,
    pad: vec2<u32>,
}

override PREMULTIPLIED_ALPHA: bool = false;

struct GammaParams {
    gamma_ratios: vec4<f32>,
    grayscale_enhanced_contrast: f32,
    subpixel_enhanced_contrast: f32,
    is_bgr: u32,
    pad: u32,
}

@group(0) @binding(0) var<uniform> globals: GlobalParams;
@group(0) @binding(1) var<uniform> gamma_params: GammaParams;
@group(2) @binding(0) var t_sprite: texture_2d<f32>;
@group(2) @binding(1) var s_sprite: sampler;

const M_PI_F: f32 = 3.1415926;
const GRAYSCALE_FACTORS: vec3<f32> = vec3<f32>(0.2126, 0.7152, 0.0722);

const SIMPLE_PRIMITIVE_FLAG: u32 = 0x80000000u;

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

struct LinearColorStop {
    color: Hsla,
    percentage: f32,
}

struct Background {
    // 0u is Solid
    // 1u is LinearGradient
    // 2u is PatternSlash
    // 3u is Checkerboard
    tag: u32,
    // 0u is sRGB color
    // 1u is Oklab color
    color_space: u32,
    solid: Hsla,
    gradient_angle_or_pattern_height: f32,
    colors: array<LinearColorStop, 2>,
    pad: u32,
}

struct AtlasTextureId {
    index: u32,
    kind: u32,
}

struct AtlasBounds {
    origin: vec2<i32>,
    size: vec2<i32>,
}

struct AtlasTile {
    texture_id: AtlasTextureId,
    tile_id: u32,
    padding: u32,
    bounds: AtlasBounds,
}

struct TransformationMatrix {
    rotation_scale: mat2x2<f32>,
    translation: vec2<f32>,
}

fn to_device_position_impl(position: vec2<f32>) -> vec4<f32> {
    let device_position = position / globals.viewport_size * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0);
    return vec4<f32>(device_position, 0.0, 1.0);
}

fn to_device_position(unit_vertex: vec2<f32>, bounds: Bounds) -> vec4<f32> {
    let position = unit_vertex * vec2<f32>(bounds.size) + bounds.origin;
    return to_device_position_impl(position);
}

fn to_device_position_transformed(unit_vertex: vec2<f32>, bounds: Bounds, transform: TransformationMatrix) -> vec4<f32> {
    let position = unit_vertex * vec2<f32>(bounds.size) + bounds.origin;
    //Note: Rust side stores it as row-major, so transposing here
    let transformed = transpose(transform.rotation_scale) * position + transform.translation;
    return to_device_position_impl(transformed);
}

fn to_tile_position(unit_vertex: vec2<f32>, tile: AtlasTile) -> vec2<f32> {
  let atlas_size = vec2<f32>(textureDimensions(t_sprite, 0));
  return (vec2<f32>(tile.bounds.origin) + unit_vertex * vec2<f32>(tile.bounds.size)) / atlas_size;
}

fn distance_from_clip_rect_impl(position: vec2<f32>, clip_bounds: Bounds) -> vec4<f32> {
    let tl = position - clip_bounds.origin;
    let br = clip_bounds.origin + clip_bounds.size - position;
    return vec4<f32>(tl.x, br.x, tl.y, br.y);
}

fn distance_from_clip_rect_transformed(unit_vertex: vec2<f32>, bounds: Bounds, clip_bounds: Bounds, transform: TransformationMatrix) -> vec4<f32> {
    let position = unit_vertex * vec2<f32>(bounds.size) + bounds.origin;
    let transformed = transpose(transform.rotation_scale) * position + transform.translation;
    return distance_from_clip_rect_impl(transformed, clip_bounds);
}

struct ClippedVertex {
    position: vec2<f32>,
    // The corner as a fraction of the original bounds.
    unit_vertex: vec2<f32>,
}

fn clip_to_mask(unit_vertex: vec2<f32>, bounds: Bounds, mask: Bounds) -> ClippedVertex {
    let origin = max(bounds.origin, mask.origin);
    let corner = min(bounds.origin + bounds.size, mask.origin + mask.size);
    let size = max(corner - origin, vec2<f32>(0.0));

    var result: ClippedVertex;
    result.position = origin + unit_vertex * size;
    result.unit_vertex = (result.position - bounds.origin) / bounds.size;
    return result;
}

// https://gamedev.stackexchange.com/questions/92015/optimized-linear-to-srgb-glsl
fn srgb_to_linear(srgb: vec3<f32>) -> vec3<f32> {
    let cutoff = srgb < vec3<f32>(0.04045);
    let higher = pow((srgb + vec3<f32>(0.055)) / vec3<f32>(1.055), vec3<f32>(2.4));
    let lower = srgb / vec3<f32>(12.92);
    return select(higher, lower, cutoff);
}

fn srgb_to_linear_component(a: f32) -> f32 {
    let cutoff = a < 0.04045;
    let higher = pow((a + 0.055) / 1.055, 2.4);
    let lower = a / 12.92;
    return select(higher, lower, cutoff);
}

fn linear_to_srgb(linear: vec3<f32>) -> vec3<f32> {
    let cutoff = linear < vec3<f32>(0.0031308);
    let higher = vec3<f32>(1.055) * pow(linear, vec3<f32>(1.0 / 2.4)) - vec3<f32>(0.055);
    let lower = linear * vec3<f32>(12.92);
    return select(higher, lower, cutoff);
}

/// Convert a linear color to sRGBA space.
fn linear_to_srgba(color: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(linear_to_srgb(color.rgb), color.a);
}

/// Convert a sRGBA color to linear space.
fn srgba_to_linear(color: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(srgb_to_linear(color.rgb), color.a);
}

/// Hsla to linear RGBA conversion.
fn hsla_to_rgba(hsla: Hsla) -> vec4<f32> {
    let h = hsla.h * 6.0; // Now, it's an angle but scaled in [0, 6) range
    let s = hsla.s;
    let l = hsla.l;
    let a = hsla.a;

    let c = (1.0 - abs(2.0 * l - 1.0)) * s;
    let x = c * (1.0 - abs(h % 2.0 - 1.0));
    let m = l - c / 2.0;
    var color = vec3<f32>(m);

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

    return vec4<f32>(color, a);
}

/// Convert a linear sRGB to Oklab space.
/// Reference: https://bottosson.github.io/posts/oklab/#converting-from-linear-srgb-to-oklab
fn linear_srgb_to_oklab(color: vec4<f32>) -> vec4<f32> {
	let l = 0.4122214708 * color.r + 0.5363325363 * color.g + 0.0514459929 * color.b;
	let m = 0.2119034982 * color.r + 0.6806995451 * color.g + 0.1073969566 * color.b;
	let s = 0.0883024619 * color.r + 0.2817188376 * color.g + 0.6299787005 * color.b;

	let l_ = pow(l, 1.0 / 3.0);
	let m_ = pow(m, 1.0 / 3.0);
	let s_ = pow(s, 1.0 / 3.0);

	return vec4<f32>(
		0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
		1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
		0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
		color.a
	);
}

/// Convert an Oklab color to linear sRGB space.
fn oklab_to_linear_srgb(color: vec4<f32>) -> vec4<f32> {
	let l_ = color.r + 0.3963377774 * color.g + 0.2158037573 * color.b;
	let m_ = color.r - 0.1055613458 * color.g - 0.0638541728 * color.b;
	let s_ = color.r - 0.0894841775 * color.g - 1.2914855480 * color.b;

	let l = l_ * l_ * l_;
	let m = m_ * m_ * m_;
	let s = s_ * s_ * s_;

	return vec4<f32>(
		4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
		-1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
		-0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
		color.a
	);
}

fn over(below: vec4<f32>, above: vec4<f32>) -> vec4<f32> {
    let alpha = above.a + below.a * (1.0 - above.a);
    let color = (above.rgb * above.a + below.rgb * below.a * (1.0 - above.a)) / alpha;
    return vec4<f32>(color, alpha);
}

// Selects corner radius based on quadrant.
fn pick_corner_radius(center_to_point: vec2<f32>, radii: Corners) -> f32 {
    if (center_to_point.x < 0.0) {
        if (center_to_point.y < 0.0) {
            return radii.top_left;
        } else {
            return radii.bottom_left;
        }
    } else {
        if (center_to_point.y < 0.0) {
            return radii.top_right;
        } else {
            return radii.bottom_right;
        }
    }
}

// Signed distance of the point to the quad's border - positive outside the
// border, and negative inside.
//
// See comments on similar code using `quad_sdf_impl` in `fs_quad` for
// explanation.
fn quad_sdf(point: vec2<f32>, bounds: Bounds, corner_radii: Corners) -> f32 {
    let half_size = bounds.size / 2.0;
    let center = bounds.origin + half_size;
    let center_to_point = point - center;
    let corner_radius = pick_corner_radius(center_to_point, corner_radii);
    let corner_to_point = abs(center_to_point) - half_size;
    let corner_center_to_point = corner_to_point + corner_radius;
    return quad_sdf_impl(corner_center_to_point, corner_radius);
}

fn quad_sdf_impl(corner_center_to_point: vec2<f32>, corner_radius: f32) -> f32 {
    if (corner_radius == 0.0) {
        // Fast path for unrounded corners.
        return max(corner_center_to_point.x, corner_center_to_point.y);
    } else {
        // Signed distance of the point from a quad that is inset by corner_radius.
        // It is negative inside this quad, and positive outside.
        let signed_distance_to_inset_quad =
            // 0 inside the inset quad, and positive outside.
            length(max(vec2<f32>(0.0), corner_center_to_point)) +
            // 0 outside the inset quad, and negative inside.
            min(0.0, max(corner_center_to_point.x, corner_center_to_point.y));

        return signed_distance_to_inset_quad - corner_radius;
    }
}

// Abstract away the final color transformation based on the
// target alpha compositing mode.
fn blend_color(color: vec4<f32>, alpha_factor: f32) -> vec4<f32> {
    let alpha = color.a * alpha_factor;
    let multiplier = select(1.0, alpha, PREMULTIPLIED_ALPHA);
    return vec4<f32>(color.rgb * multiplier, alpha);
}


struct PreparedBackground {
    color0: vec4<f32>,
    color1: vec4<f32>,
    tag: u32,
}

fn prepare_background(background: Background) -> PreparedBackground {
    var result: PreparedBackground;
    result.tag = background.tag;
    result.color1 = vec4<f32>(0.0);

    if (background.tag == 1u) {
        // `hsla_to_rgba` returns linear sRGB. Converting the endpoints to the
        // interpolation space once avoids converting at every stop evaluation.
        result.color0 = hsla_to_rgba(background.colors[0].color);
        result.color1 = hsla_to_rgba(background.colors[1].color);
        if (background.color_space == 0u) {
            result.color0 = linear_to_srgba(result.color0);
            result.color1 = linear_to_srgba(result.color1);
        } else if (background.color_space == 1u) {
            result.color0 = linear_srgb_to_oklab(result.color0);
            result.color1 = linear_srgb_to_oklab(result.color1);
        }
    } else {
        result.color0 = hsla_to_rgba(background.solid);
    }

    return result;
}

// Ordered (Bayer 4x4) dither threshold, remapped to [-1, +1].
fn dither_offset(position: vec2<f32>) -> f32 {
    let cell = vec2<u32>(position) & vec2<u32>(3u);
    let z = cell.x ^ cell.y;
    let threshold = ((z & 1u) << 3u)
        | ((cell.y & 1u) << 2u)
        | (z & 2u)
        | ((cell.y & 2u) >> 1u);
    return (f32(threshold) + 0.5) * (1.0 / 8.0) - 1.0;
}

fn background_color(prepared: PreparedBackground, background: Background, bounds: Bounds,
    position: vec2<f32>) -> vec4<f32> {
    var color = prepared.color0;
    if (prepared.tag == 0u) {
        return color;
    }

    switch (prepared.tag) {
        case 1u: {
            // -90 degrees to match the CSS gradient angle.
            let radians = (background.gradient_angle_or_pattern_height % 360.0 - 90.0)
                * M_PI_F / 180.0;
            var direction = vec2<f32>(cos(radians), sin(radians));

            // Expand the short side to be the same as the long side
            if (bounds.size.x > bounds.size.y) {
                direction.y *= bounds.size.y / bounds.size.x;
            } else {
                direction.x *= bounds.size.x / bounds.size.y;
            }

            let extent = select(bounds.size.y, bounds.size.x, abs(direction.x) > abs(direction.y));
            let direction_length = length(direction);
            let span = background.colors[1].percentage - background.colors[0].percentage;
            // The half-extent term is exactly half of `extent`, so the
            // position-independent part reduces to this.
            let offset = 0.5 - background.colors[0].percentage;

            // t = dot(position - center, gradient.xy) + gradient.z
            var gradient = vec3<f32>(0.0);
            if (direction_length != 0.0 && extent != 0.0) {
                if (span == 0.0) {
                    const STEP: f32 = 1e30;
                    gradient = vec3<f32>(direction / (direction_length * extent) * STEP, offset * STEP);
                } else {
                    gradient = vec3<f32>(direction / (direction_length * extent * span), offset / span);
                }
            }
            let center = bounds.origin + bounds.size * 0.5;
            let t = saturate(dot(position - center, gradient.xy) + gradient.z);
            color = mix(prepared.color0, prepared.color1, t);
            if (background.color_space == 1u) {
                color = oklab_to_linear_srgb(color);
            } else {
                color = srgba_to_linear(color);
            }

            // Dither to reduce banding in gradients (especially dark/alpha).
            let dither = dither_offset(position);
            color = vec4<f32>(color.rgb + dither * 1.0 / 255.0, color.a + dither * 1.5 / 255.0);
        }
        case 2u: {
            // pattern slash
            let pattern_height_encoded = background.gradient_angle_or_pattern_height;
            let pattern_width = (pattern_height_encoded / 65535.0) / 255.0;
            let pattern_interval = (pattern_height_encoded % 65535.0) / 255.0;
            let pattern_height = pattern_width + pattern_interval;
            // Only the rotated x coordinate is used, and multiplying by the
            // rotation reduces it to dot(v, vec2(cos a, sin a)).
            let stripe_angle = M_PI_F / 4.0;
            let stripe_direction = vec2<f32>(cos(stripe_angle), sin(stripe_angle));
            let pattern_period = pattern_height * sin(stripe_angle);
            let threshold = pattern_period * (pattern_width / pattern_height) / 2.0;
            let pattern = dot(position - bounds.origin, stripe_direction) % pattern_period;
            let distance = min(pattern, pattern_period - pattern) - threshold;
            color.a *= saturate(0.5 - distance);
        }
        case 3u: {
            // checkerboard
            let cell_size = background.gradient_angle_or_pattern_height;
            let relative_position = position - bounds.origin;
            let x_index = floor(relative_position.x / cell_size);
            let y_index = floor(relative_position.y / cell_size);
            color.a *= saturate((x_index + y_index) % 2.0);
        }
        default: {}
    }

    return color;
}

// --- quads --- //

struct Quad {
    order: u32,
    border_style: u32,
    bounds: Bounds,
    content_mask: Bounds,
    background: Background,
    border_color: Hsla,
    corner_radii: Corners,
    border_widths: Edges,
}

fn has_rounded_corners(corner_radii: Corners) -> bool {
    return corner_radii.top_left != 0.0
        || corner_radii.top_right != 0.0
        || corner_radii.bottom_right != 0.0
        || corner_radii.bottom_left != 0.0;
}

fn has_border(border_widths: Edges) -> bool {
    return border_widths.top != 0.0
        || border_widths.right != 0.0
        || border_widths.bottom != 0.0
        || border_widths.left != 0.0;
}

fn is_shaped(quad: Quad) -> bool {
    return has_rounded_corners(quad.corner_radii) || has_border(quad.border_widths);
}

const CULLED_POSITION: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);

fn quad_depth(quad_id: u32) -> f32 {
    return saturate(f32(quad_id + 1u) * globals.depth_scale);
}

// Opaque rectangles are drawn by the opaque pass instead of the quad pass.
fn is_opaque_rectangle(quad: Quad) -> bool {
    return quad.background.tag == 0u
        && quad.background.solid.a >= 1.0
        && !has_rounded_corners(quad.corner_radii)
        && !has_border(quad.border_widths);
}

struct QuadVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) border_color: vec4<f32>,
    @location(1) @interpolate(flat) quad_id: u32,
    // Inter-stage values can't be structs, so `PreparedBackground` is flattened.
    @location(2) @interpolate(flat) background_color0: vec4<f32>,
    @location(3) @interpolate(flat) background_color1: vec4<f32>,
    @location(4) @interpolate(flat) background_tag: u32,
}

@vertex
fn vs_quad(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> QuadVarying {
    let quad = load_quad(instance_id);

    var out = QuadVarying();
    if (is_opaque_rectangle(quad)) {
        out.position = CULLED_POSITION;
        return out;
    }

    let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
    let clipped = clip_to_mask(unit_vertex, quad.bounds, quad.content_mask);
    out.position = to_device_position_impl(clipped.position);
    out.position.z = quad_depth(instance_id);

    out.border_color = hsla_to_rgba(quad.border_color);
    let simple = !is_shaped(quad) && quad.background.tag == 0u;
    out.quad_id = instance_id | select(0u, SIMPLE_PRIMITIVE_FLAG, simple);
    let background = prepare_background(quad.background);
    out.background_color0 = background.color0;
    out.background_color1 = background.color1;
    out.background_tag = background.tag;
    return out;
}

@fragment
fn fs_quad(input: QuadVarying) -> @location(0) vec4<f32> {
    if ((input.quad_id & SIMPLE_PRIMITIVE_FLAG) != 0u) {
        return blend_color(input.background_color0, 1.0);
    }

    let quad = load_quad(input.quad_id);
    let prepared = PreparedBackground(input.background_color0, input.background_color1,
        input.background_tag);
    let background = background_color(prepared, quad.background, quad.bounds, input.position.xy);
    let unrounded = !has_rounded_corners(quad.corner_radii);

    let size = quad.bounds.size;
    let half_size = size / 2.0;
    let point = input.position.xy - quad.bounds.origin;
    let center_to_point = point - half_size;

    // Signed distance field threshold for inclusion of pixels. 0.5 is the
    // minimum distance between the center of the pixel and the edge.
    let antialias_threshold = 0.5;

    // Radius of the nearest corner
    let corner_radius = pick_corner_radius(center_to_point, quad.corner_radii);

    // Width of the nearest borders
    let border = vec2<f32>(
        select(
            quad.border_widths.right,
            quad.border_widths.left,
            center_to_point.x < 0.0),
        select(
            quad.border_widths.bottom,
            quad.border_widths.top,
            center_to_point.y < 0.0));

    // 0-width borders are reduced so that `inner_sdf >= antialias_threshold`.
    // The purpose of this is to not draw antialiasing pixels in this case.
    let reduced_border =
        vec2<f32>(select(border.x, -antialias_threshold, border.x == 0.0),
                  select(border.y, -antialias_threshold, border.y == 0.0));

    // Vector from the corner of the quad bounds to the point, after mirroring
    // the point into the bottom right quadrant. Both components are <= 0.
    let corner_to_point = abs(center_to_point) - half_size;

    // Vector from the point to the center of the rounded corner's circle, also
    // mirrored into bottom right quadrant.
    let corner_center_to_point = corner_to_point + corner_radius;

    // Whether the nearest point on the border is rounded
    let is_near_rounded_corner =
            corner_center_to_point.x >= 0 &&
            corner_center_to_point.y >= 0;

    // Vector from straight border inner corner to point.
    let straight_border_inner_corner_to_point = corner_to_point + reduced_border;

    // Whether the point is beyond the inner edge of the straight border.
    let is_beyond_inner_straight_border =
            straight_border_inner_corner_to_point.x > 0 ||
            straight_border_inner_corner_to_point.y > 0;

    // Whether the point is far enough inside the quad, such that the pixels are
    // not affected by the straight border.
    let is_within_inner_straight_border =
        straight_border_inner_corner_to_point.x < -antialias_threshold &&
        straight_border_inner_corner_to_point.y < -antialias_threshold;

    // Fast path for points that must be part of the background.
    //
    // This could be optimized further for large rounded corners by including
    // points in an inscribed rectangle, or some other quick linear check.
    // However, that might negatively impact performance in the case of
    // reasonable sizes for rounded corners.
    if (is_within_inner_straight_border && !is_near_rounded_corner) {
        return blend_color(background, 1.0);
    }

    // Signed distance of the point to the outside edge of the quad's border. It
    // is positive outside this edge, and negative inside.
    let outer_sdf = quad_sdf_impl(corner_center_to_point, corner_radius);

    // Approximate signed distance of the point to the inside edge of the quad's
    // border. It is negative outside this edge (within the border), and
    // positive inside.
    //
    // This is not always an accurate signed distance:
    // * The rounded portions with varying border width use an approximation of
    //   nearest-point-on-ellipse.
    // * When it is quickly known to be outside the edge, -1.0 is used.
    var inner_sdf = 0.0;
    if (corner_center_to_point.x <= 0 || corner_center_to_point.y <= 0) {
        // Fast paths for straight borders.
        inner_sdf = -max(straight_border_inner_corner_to_point.x,
                         straight_border_inner_corner_to_point.y);
    } else if (is_beyond_inner_straight_border) {
        // Fast path for points that must be outside the inner edge.
        inner_sdf = -1.0;
    } else if (reduced_border.x == reduced_border.y) {
        // Fast path for circular inner edge.
        inner_sdf = -(outer_sdf + reduced_border.x);
    } else {
        let ellipse_radii = max(vec2<f32>(0.0), corner_radius - reduced_border);
        inner_sdf = quarter_ellipse_sdf(corner_center_to_point, ellipse_radii);
    }

    // Negative when inside the border
    let border_sdf = max(inner_sdf, outer_sdf);

    var color = background;
    if (border_sdf < antialias_threshold) {
        var border_color = input.border_color;

        // Dashed border logic when border_style == 1
        if (quad.border_style == 1) {
            // Position along the perimeter in "dash space", where each dash
            // period has length 1
            var t = 0.0;

            // Total number of dash periods, so that the dash spacing can be
            // adjusted to evenly divide it
            var max_t = 0.0;

            // Border width is proportional to dash size. This is the behavior
            // used by browsers, but also avoids dashes from different segments
            // overlapping when dash size is smaller than the border width.
            //
            // Dash pattern: (2 * border width) dash, (1 * border width) gap
            let dash_length_per_width = 2.0;
            let dash_gap_per_width = 1.0;
            let dash_period_per_width = dash_length_per_width + dash_gap_per_width;

            // Since the dash size is determined by border width, the density of
            // dashes varies. Multiplying a pixel distance by this returns a
            // position in dash space - it has units (dash period / pixels). So
            // a dash velocity of (1 / 10) is 1 dash every 10 pixels.
            var dash_velocity = 0.0;

            // Dividing this by the border width gives the dash velocity
            let dv_numerator = 1.0 / dash_period_per_width;

            if (unrounded) {
                // When corners aren't rounded, the dashes are separately laid
                // out on each straight line, rather than around the whole
                // perimeter. This way each line starts and ends with a dash.
                let is_horizontal =
                        corner_center_to_point.x <
                        corner_center_to_point.y;

                // When applying dashed borders to just some, not all, the sides.
                // The way we chose border widths above sometimes comes with a 0 width value.
                // So we choose again to avoid division by zero.
                // TODO: A better solution exists taking a look at the whole file.
                // this does not fix single dashed borders at the corners
                let dashed_border = vec2<f32>(
                        max(
                            quad.border_widths.bottom,
                            quad.border_widths.top,
                        ),
                        max(
                            quad.border_widths.right,
                            quad.border_widths.left,
                        )
                   );

                let border_width = select(dashed_border.y, dashed_border.x, is_horizontal);
                dash_velocity = dv_numerator / border_width;
                t = select(point.y, point.x, is_horizontal) * dash_velocity;
                max_t = select(size.y, size.x, is_horizontal) * dash_velocity;
            } else {
                // When corners are rounded, the dashes are laid out clockwise
                // around the whole perimeter.

                let r_tr = quad.corner_radii.top_right;
                let r_br = quad.corner_radii.bottom_right;
                let r_bl = quad.corner_radii.bottom_left;
                let r_tl = quad.corner_radii.top_left;

                let w_t = quad.border_widths.top;
                let w_r = quad.border_widths.right;
                let w_b = quad.border_widths.bottom;
                let w_l = quad.border_widths.left;

                // Straight side dash velocities
                let dv_t = select(dv_numerator / w_t, 0.0, w_t <= 0.0);
                let dv_r = select(dv_numerator / w_r, 0.0, w_r <= 0.0);
                let dv_b = select(dv_numerator / w_b, 0.0, w_b <= 0.0);
                let dv_l = select(dv_numerator / w_l, 0.0, w_l <= 0.0);

                // Straight side lengths in dash space
                let s_t = (size.x - r_tl - r_tr) * dv_t;
                let s_r = (size.y - r_tr - r_br) * dv_r;
                let s_b = (size.x - r_br - r_bl) * dv_b;
                let s_l = (size.y - r_bl - r_tl) * dv_l;

                let corner_dash_velocity_tr = corner_dash_velocity(dv_t, dv_r);
                let corner_dash_velocity_br = corner_dash_velocity(dv_b, dv_r);
                let corner_dash_velocity_bl = corner_dash_velocity(dv_b, dv_l);
                let corner_dash_velocity_tl = corner_dash_velocity(dv_t, dv_l);

                // Corner lengths in dash space
                let c_tr = r_tr * (M_PI_F / 2.0) * corner_dash_velocity_tr;
                let c_br = r_br * (M_PI_F / 2.0) * corner_dash_velocity_br;
                let c_bl = r_bl * (M_PI_F / 2.0) * corner_dash_velocity_bl;
                let c_tl = r_tl * (M_PI_F / 2.0) * corner_dash_velocity_tl;

                // Cumulative dash space upto each segment
                let upto_tr = s_t;
                let upto_r = upto_tr + c_tr;
                let upto_br = upto_r + s_r;
                let upto_b = upto_br + c_br;
                let upto_bl = upto_b + s_b;
                let upto_l = upto_bl + c_bl;
                let upto_tl = upto_l + s_l;
                max_t = upto_tl + c_tl;

                if (is_near_rounded_corner) {
                    let radians = atan2(corner_center_to_point.y,
                                        corner_center_to_point.x);
                    let corner_t = radians * corner_radius;

                    if (center_to_point.x >= 0.0) {
                        if (center_to_point.y < 0.0) {
                            dash_velocity = corner_dash_velocity_tr;
                            // Subtracted because radians is pi/2 to 0 when
                            // going clockwise around the top right corner,
                            // since the y axis has been flipped
                            t = upto_r - corner_t * dash_velocity;
                        } else {
                            dash_velocity = corner_dash_velocity_br;
                            // Added because radians is 0 to pi/2 when going
                            // clockwise around the bottom-right corner
                            t = upto_br + corner_t * dash_velocity;
                        }
                    } else {
                        if (center_to_point.y >= 0.0) {
                            dash_velocity = corner_dash_velocity_bl;
                            // Subtracted because radians is pi/2 to 0 when
                            // going clockwise around the bottom-left corner,
                            // since the x axis has been flipped
                            t = upto_l - corner_t * dash_velocity;
                        } else {
                            dash_velocity = corner_dash_velocity_tl;
                            // Added because radians is 0 to pi/2 when going
                            // clockwise around the top-left corner, since both
                            // axis were flipped
                            t = upto_tl + corner_t * dash_velocity;
                        }
                    }
                } else {
                    // Straight borders
                    let is_horizontal =
                            corner_center_to_point.x <
                            corner_center_to_point.y;
                    if (is_horizontal) {
                        if (center_to_point.y < 0.0) {
                            dash_velocity = dv_t;
                            t = (point.x - r_tl) * dash_velocity;
                        } else {
                            dash_velocity = dv_b;
                            t = upto_bl - (point.x - r_bl) * dash_velocity;
                        }
                    } else {
                        if (center_to_point.x < 0.0) {
                            dash_velocity = dv_l;
                            t = upto_tl - (point.y - r_tl) * dash_velocity;
                        } else {
                            dash_velocity = dv_r;
                            t = upto_r + (point.y - r_tr) * dash_velocity;
                        }
                    }
                }
            }

            let dash_length = dash_length_per_width / dash_period_per_width;
            let desired_dash_gap = dash_gap_per_width / dash_period_per_width;

            // Straight borders should start and end with a dash, so max_t is
            // reduced to cause this.
            max_t -= select(0.0, dash_length, unrounded);
            if (max_t >= 1.0) {
                // Adjust dash gap to evenly divide max_t.
                let dash_count = floor(max_t);
                let dash_period = max_t / dash_count;
                border_color.a *= dash_alpha(
                    t,
                    dash_period,
                    dash_length,
                    dash_velocity,
                    antialias_threshold);
            } else if (unrounded) {
                // When there isn't enough space for the full gap between the
                // two start / end dashes of a straight border, reduce gap to
                // make them fit.
                let dash_gap = max_t - dash_length;
                if (dash_gap > 0.0) {
                    let dash_period = dash_length + dash_gap;
                    border_color.a *= dash_alpha(
                        t,
                        dash_period,
                        dash_length,
                        dash_velocity,
                        antialias_threshold);
                }
            }
        }

        // Blend the border on top of the background and then linearly interpolate
        // between the two as we slide inside the background.
        let blended_border = over(background, border_color);
        color = mix(background, blended_border,
                    saturate(antialias_threshold - inner_sdf));
    }

    return blend_color(color, saturate(antialias_threshold - outer_sdf));
}

// Returns the dash velocity of a corner given the dash velocity of the two
// sides, by returning the slower velocity (larger dashes).
//
// Since 0 is used for dash velocity when the border width is 0 (instead of
// +inf), this returns the other dash velocity in that case.
//
// An alternative to this might be to appropriately interpolate the dash
// velocity around the corner, but that seems overcomplicated.
fn corner_dash_velocity(dv1: f32, dv2: f32) -> f32 {
    if (dv1 == 0.0) {
        return dv2;
    } else if (dv2 == 0.0) {
        return dv1;
    } else {
        return min(dv1, dv2);
    }
}

// Returns alpha used to render antialiased dashes.
// `t` is within the dash when `fmod(t, period) < length`.
fn dash_alpha(t: f32, period: f32, length: f32, dash_velocity: f32, antialias_threshold: f32) -> f32 {
    let half_period = period / 2;
    let half_length = length / 2;
    // Value in [-half_period, half_period].
    // The dash is in [-half_length, half_length].
    let centered = fmod(t + half_period - half_length, period) - half_period;
    // Signed distance for the dash, negative values are inside the dash.
    let signed_distance = abs(centered) - half_length;
    // Antialiased alpha based on the signed distance.
    return saturate(antialias_threshold - signed_distance / dash_velocity);
}

// This approximates distance to the nearest point to a quarter ellipse in a way
// that is sufficient for anti-aliasing when the ellipse is not very eccentric.
// The components of `point` are expected to be positive.
//
// Negative on the outside and positive on the inside.
fn quarter_ellipse_sdf(point: vec2<f32>, radii: vec2<f32>) -> f32 {
    // Scale the space to treat the ellipse like a unit circle.
    let circle_vec = point / radii;
    let unit_circle_sdf = length(circle_vec) - 1.0;
    // Approximate up-scaling of the length by using the average of the radii.
    //
    // TODO: A better solution would be to use the gradient of the implicit
    // function for an ellipse to approximate a scaling factor.
    return unit_circle_sdf * (radii.x + radii.y) * -0.5;
}

// Modulus that has the same sign as `a`.
fn fmod(a: f32, b: f32) -> f32 {
    return a - b * trunc(a / b);
}

// --- opaque quads --- //

struct OpaqueQuadVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec4<f32>,
}

@vertex
fn vs_opaque_quad(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> OpaqueQuadVarying {
    let quad_id = globals.quad_count - 1u - instance_id;
    let quad = load_quad(quad_id);

    var out = OpaqueQuadVarying();
    if (!is_opaque_rectangle(quad)) {
        out.position = CULLED_POSITION;
        return out;
    }

    let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
    let clipped = clip_to_mask(unit_vertex, quad.bounds, quad.content_mask);
    out.position = to_device_position_impl(clipped.position);
    out.position.z = quad_depth(quad_id);
    out.color = blend_color(hsla_to_rgba(quad.background.solid), 1.0);
    return out;
}

@fragment
fn fs_opaque_quad(input: OpaqueQuadVarying) -> @location(0) vec4<f32> {
    return input.color;
}

// --- shadows --- //

struct Shadow {
    order: u32,
    blur_radius: f32,
    // The area the shadow paints: the blurred extent of a drop shadow, or the element
    // that an inset shadow is clipped to.
    bounds: Bounds,
    corner_radii: Corners,
    content_mask: Bounds,
    color: Hsla,
    // The shape that is blurred: the silhouette of a drop shadow, or the hole of an
    // inset shadow.
    shape_bounds: Bounds,
    shape_corner_radii: Corners,
    // 0 = drop shadow, 1 = inset shadow.
    inset: u32,
    pad: u32, // align to 8 bytes
}

struct ShadowVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec4<f32>,
    @location(1) @interpolate(flat) shadow_id: u32,
    @location(2) @interpolate(flat) blur_shape: vec4<f32>,
    @location(3) @interpolate(flat) blur_r1: vec4<f32>,
    @location(4) @interpolate(flat) blur_exponent: vec4<f32>,
    @location(5) @interpolate(flat) blur_scale: vec4<f32>,
}

struct PreparedBlurredRoundedRect {
    shape: vec4<f32>,
    r1: vec4<f32>,
    exponent: vec4<f32>,
    scale: vec4<f32>,
}

fn erf7(input: f32) -> f32 {
    let x = clamp(input, -10.0, 10.0) * (2.0 / sqrt(M_PI_F));
    let xx = x * x;
    let y = x + (0.24295 + (0.03395 + 0.0104 * xx) * xx) * (x * xx);
    return y / sqrt(1.0 + y * y);
}

// Raph Levien's blurred rounded rectangle approximation, whose blur s is sigma * sqrt(2):
// https://raphlinus.github.io/graphics/2020/04/21/blurred-rounded-rects.html
fn prepare_blurred_rounded_rect(size: vec2<f32>, corner_radii: Corners, sigma: f32) -> PreparedBlurredRoundedRect {
    let s = max(sigma * sqrt(2.0), 1e-6);
    let s_inv = 1.0 / s;
    let min_edge = min(size.x, size.y);
    let r_max = 0.5 * min_edge;
    let radii = vec4<f32>(corner_radii.top_left, corner_radii.top_right,
                          corner_radii.bottom_right, corner_radii.bottom_left);
    let r0 = min(sqrt(radii * radii + 1.15 * 1.15 * s * s), vec4<f32>(r_max));
    let r1 = min(sqrt(radii * radii + 4.0 * s * s), vec4<f32>(r_max));

    let half_extent = 0.5 * s_inv * size;
    let delta = 1.25 * s * (exp(-half_extent.x * half_extent.x) - exp(-half_extent.y * half_extent.y));
    let adjusted_size = vec2<f32>(size.x + min(delta, 0.0), size.y - max(delta, 0.0));
    let peak = s_inv * 0.5 * (max(adjusted_size.x, adjusted_size.y) - 0.5 * radii);

    var output: PreparedBlurredRoundedRect;
    output.shape = vec4<f32>(0.5 * adjusted_size, s_inv, min_edge);
    output.r1 = r1;
    output.exponent = 2.0 * r1 / r0;
    output.scale = 0.5 * vec4<f32>(erf7(peak.x), erf7(peak.y), erf7(peak.z), erf7(peak.w));
    return output;
}

fn pick_corner(center_to_point: vec2<f32>, values: vec4<f32>) -> f32 {
    return pick_corner_radius(center_to_point, Corners(values.x, values.y, values.z, values.w));
}

fn blurred_rounded_rect(center_to_point: vec2<f32>, prepared: PreparedBlurredRoundedRect) -> f32 {
    let r1 = pick_corner(center_to_point, prepared.r1);
    let exponent = pick_corner(center_to_point, prepared.exponent);
    let scale = pick_corner(center_to_point, prepared.scale);
    let s_inv = prepared.shape.z;
    let min_edge = prepared.shape.w;

    let p0 = abs(center_to_point) - (prepared.shape.xy - r1);
    let p1 = max(p0, vec2<f32>(0.0));
    var d_pos: f32;
    if (p1.x > 0.0 && p1.y > 0.0) {
        d_pos = pow(pow(p1.x, exponent) + pow(p1.y, exponent), 1.0 / exponent);
    } else {
        d_pos = p1.x + p1.y;
    }

    let d_neg = min(max(p0.x, p0.y), 0.0);
    let d = d_pos + d_neg - r1;
    return scale * (erf7(s_inv * (min_edge + d)) - erf7(s_inv * d));
}

@vertex
fn vs_shadow(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> ShadowVarying {
    let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
    var shadow = load_shadow(instance_id);

    var out = ShadowVarying();
    let clipped = clip_to_mask(unit_vertex, shadow.bounds, shadow.content_mask);
    out.position = to_device_position_impl(clipped.position);
    out.color = hsla_to_rgba(shadow.color);
    out.shadow_id = instance_id;
    let blur = prepare_blurred_rounded_rect(shadow.shape_bounds.size, shadow.shape_corner_radii,
                                            shadow.blur_radius);
    out.blur_shape = blur.shape;
    out.blur_r1 = blur.r1;
    out.blur_exponent = blur.exponent;
    out.blur_scale = blur.scale;
    return out;
}

@fragment
fn fs_shadow(input: ShadowVarying) -> @location(0) vec4<f32> {
    let shadow = load_shadow(input.shadow_id);

    var alpha: f32;
    if (shadow.blur_radius == 0.0) {
        let distance = quad_sdf(input.position.xy, shadow.shape_bounds, shadow.shape_corner_radii);
        alpha = saturate(0.5 - distance);
    } else {
        let center = shadow.shape_bounds.origin + shadow.shape_bounds.size / 2.0;
        let blur = PreparedBlurredRoundedRect(input.blur_shape, input.blur_r1,
                                              input.blur_exponent, input.blur_scale);
        alpha = blurred_rounded_rect(input.position.xy - center, blur);
    }

    if (shadow.inset != 0u) {
        // The inset shadow is the complement of the (blurred) hole rect, clipped to the element.
        // `saturate(0.5 - d)` gives a 1-pixel antialiased edge: d <= -0.5 -> 1, d >= 0.5 -> 0.
        alpha = 1.0 - alpha;
        let element_distance = quad_sdf(input.position.xy, shadow.bounds, shadow.corner_radii);
        alpha *= saturate(0.5 - element_distance);
    }

    return blend_color(input.color, alpha);
}

// --- path tiles --- //

struct PathTile {
    paint: u32,
    curve_start: u32,
    curve_count: u32,
    backdrop: i32,
    top_left: vec2<f32>,
    bottom_right: vec2<f32>,
    reference: vec2<f32>,
    curves_top_left: vec2<f32>,
    curves_bottom_right: vec2<f32>,
}

struct PathCurve {
    p0: vec2<f32>,
    p1: vec2<f32>,
    a: vec2<f32>,
    b: vec2<f32>,
    sign: f32,
    pad: u32,
}

struct PathPaint {
    bounds: Bounds,
    content_mask: Bounds,
    color: Background,
    even_odd: u32,
    pad: u32,
}

struct PathTileVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) tile_id: u32,
    // Inter-stage values can't be structs, so `PreparedBackground` is flattened.
    @location(1) @interpolate(flat) background_color0: vec4<f32>,
    @location(2) @interpolate(flat) background_color1: vec4<f32>,
    @location(3) @interpolate(flat) background_tag: u32,
    @location(4) @interpolate(flat) curve_box: vec4<f32>,
}

@vertex
fn vs_path_tile(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> PathTileVarying {
    let tile = load_path_tile(instance_id);
    let paint = load_path_paint(tile.paint);

    let mask_top_left = paint.content_mask.origin;
    let mask_bottom_right = paint.content_mask.origin + paint.content_mask.size;
    let top_left = clamp(tile.top_left, mask_top_left, mask_bottom_right);
    let bottom_right = clamp(tile.bottom_right, top_left, mask_bottom_right);
    let position = vec2<f32>(
        select(top_left.x, bottom_right.x, (vertex_id & 1u) != 0u),
        select(top_left.y, bottom_right.y, (vertex_id & 2u) != 0u),
    );

    var out = PathTileVarying();
    out.position = to_device_position_impl(position);
    out.tile_id = instance_id;
    out.curve_box = vec4<f32>(tile.curves_top_left, tile.curves_bottom_right);
    let background = prepare_background(paint.color);
    out.background_color0 = background.color0;
    out.background_color1 = background.color1;
    out.background_tag = background.tag;
    return out;
}

// Solves a*t^2 + b*t + c = 0 for the root in [0, 1] on a monotone piece.
fn monotone_quadratic_root(a: f32, b: f32, c: f32, direction: f32) -> f32 {
    let discriminant = max(b * b - 4.0 * a * c, 0.0);
    let denominator = b + direction * sqrt(discriminant);
    if (denominator == 0.0) {
        return 0.0;
    }
    return saturate(-2.0 * c / denominator);
}

// Coverage estimate adapted from https://github.com/EricLengyel/Slug/blob/be3c13eb7d63f9e8aa5c583e42d92c374cb91d98/SlugPixelShader.hlsl
// Copyright (c) 2017 Eric Lengyel.
// Licensed under the MIT license.

// Accumulates the curve's winding along the pixel's leftward and upward rays. Like Slug, a crossing
// within half a pixel of the center counts fractionally.
fn add_curve_winding(curve: PathCurve, center: vec2<f32>, winding: ptr<function, vec2<f32>>,
    weight: ptr<function, vec2<f32>>) {
    let min_x = min(curve.p0.x, curve.p1.x);
    let max_x = max(curve.p0.x, curve.p1.x);

    if (curve.p0.y <= center.y && center.y < curve.p1.y) {
        if (max_x <= center.x - 0.5) {
            (*winding).x += curve.sign;
        } else {
            let t = monotone_quadratic_root(curve.a.y, curve.b.y, curve.p0.y - center.y, 1.0);
            let x = (curve.a.x * t + curve.b.x) * t + curve.p0.x;
            let distance = center.x - x;
            (*winding).x += curve.sign * saturate(distance + 0.5);
            (*weight).x = max((*weight).x, saturate(1.0 - 2.0 * abs(distance)));
        }
    }

    if (min_x <= center.x && center.x < max_x) {
        let direction = select(-1.0, 1.0, curve.p1.x > curve.p0.x);
        if (curve.p1.y <= center.y - 0.5) {
            (*winding).y -= curve.sign * direction;
        } else if (curve.p0.y < center.y + 0.5) {
            let t = monotone_quadratic_root(curve.a.x, curve.b.x, curve.p0.x - center.x, direction);
            let y = (curve.a.y * t + curve.b.y) * t + curve.p0.y;
            let distance = center.y - y;
            (*winding).y -= curve.sign * direction * saturate(distance + 0.5);
            (*weight).y = max((*weight).y, saturate(1.0 - 2.0 * abs(distance)));
        }
    }
}

// The backdrop gives the unlisted curves' winding along a leftward ray from the tile's reference
// corner. We need it at the current pixel, along both leftward and upward rays. Because paths are
// closed, chains of unlisted curves can only begin or end where a listed curve does. So, we can
// compute the winding at the current pixel by correcting the backdrop for each listed endpoint
// between its ray and the pixel.
fn correct_for_endpoint(endpoint: vec2<f32>, step: f32, reference: vec2<f32>, center: vec2<f32>,
    winding: ptr<function, vec2<f32>>) {
    if (endpoint.x <= reference.x) {
        (*winding).x -= step * (f32(endpoint.y <= center.y) - f32(endpoint.y <= reference.y));
    }
    if (endpoint.y <= reference.y && endpoint.x <= center.x) {
        (*winding).y += step;
    }
}

@fragment
fn fs_path_tile(input: PathTileVarying) -> @location(0) vec4<f32> {
    let center = floor(input.position.xy) + 0.5;
    if (input.background_tag == 0u
        && (any(center < input.curve_box.xy) || any(center > input.curve_box.zw))) {
        return blend_color(input.background_color0, 1.0);
    }
    let tile = load_path_tile(input.tile_id);
    let paint = load_path_paint(tile.paint);

    var winding = vec2<f32>(f32(tile.backdrop));
    var weight = vec2<f32>(0.0);
    for (var i = 0u; i < tile.curve_count; i++) {
        let curve = load_path_curve(tile.curve_start + i);
        if (min(curve.p0.x, curve.p1.x) >= center.x + 0.5) {
            break;
        }
        if (curve.p0.y < center.y + 0.5) {
            add_curve_winding(curve, center, &winding, &weight);
            correct_for_endpoint(curve.p0, curve.sign, tile.reference, center, &winding);
            correct_for_endpoint(curve.p1, -curve.sign, tile.reference, center, &winding);
        }
    }

    // Weight the winding computed by each ray by its nearest crossing's distance from the pixel center,
    // using Slug's technique.
    var coverage = max(
        abs(dot(winding, weight)) / max(weight.x + weight.y, 1.0 / 65536.0),
        min(abs(winding.x), abs(winding.y)));
    if (paint.even_odd != 0u) {
        coverage = 1.0 - abs(1.0 - fract(coverage * 0.5) * 2.0);
    } else {
        coverage = saturate(coverage);
    }

    if (coverage == 0.0) {
        return vec4<f32>(0.0);
    }

    let prepared = PreparedBackground(input.background_color0, input.background_color1,
        input.background_tag);
    let color = background_color(prepared, paint.color, paint.bounds, input.position.xy);
    return blend_color(color, coverage);
}

// --- underlines --- //

struct Underline {
    order: u32,
    pad: u32,
    bounds: Bounds,
    content_mask: Bounds,
    color: Hsla,
    thickness: f32,
    wavy: u32,
}


struct UnderlineVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec4<f32>,
    @location(1) @interpolate(flat) underline_id: u32,
}

@vertex
fn vs_underline(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> UnderlineVarying {
    let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
    let underline = load_underline(instance_id);

    var out = UnderlineVarying();
    let clipped = clip_to_mask(unit_vertex, underline.bounds, underline.content_mask);
    out.position = to_device_position_impl(clipped.position);
    out.color = hsla_to_rgba(underline.color);
    out.underline_id = instance_id | select(0u, SIMPLE_PRIMITIVE_FLAG, underline.wavy == 0u);
    return out;
}

@fragment
fn fs_underline(input: UnderlineVarying) -> @location(0) vec4<f32> {
    const WAVE_FREQUENCY: f32 = 2.0;
    const WAVE_HEIGHT_RATIO: f32 = 0.8;

    if ((input.underline_id & SIMPLE_PRIMITIVE_FLAG) != 0u) {
        return blend_color(input.color, input.color.a);
    }

    let underline = load_underline(input.underline_id);
    let half_thickness = underline.thickness * 0.5;

    let st = (input.position.xy - underline.bounds.origin) / underline.bounds.size.y - vec2<f32>(0.0, 0.5);
    let frequency = M_PI_F * WAVE_FREQUENCY * underline.thickness / underline.bounds.size.y;
    let amplitude = (underline.thickness * WAVE_HEIGHT_RATIO) / underline.bounds.size.y;

    let sine = sin(st.x * frequency) * amplitude;
    let dSine = cos(st.x * frequency) * amplitude * frequency;
    let distance = (st.y - sine) / sqrt(1.0 + dSine * dSine);
    let distance_in_pixels = distance * underline.bounds.size.y;
    let distance_from_top_border = distance_in_pixels - half_thickness;
    let distance_from_bottom_border = distance_in_pixels + half_thickness;
    let alpha = saturate(0.5 - max(-distance_from_bottom_border, distance_from_top_border));
    return blend_color(input.color, alpha * input.color.a);
}

// --- monochrome sprites --- //

struct MonochromeSprite {
    order: u32,
    pad: u32,
    tile: AtlasTile,
    bounds: Bounds,
    content_mask: Bounds,
    color: Hsla,
    transformation: TransformationMatrix,
}


struct MonoSpriteVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) tile_position: vec2<f32>,
    @location(1) @interpolate(flat) color: vec4<f32>,
    @location(3) clip_distances: vec4<f32>,
}

@vertex
fn vs_mono_sprite(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> MonoSpriteVarying {
    let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
    let sprite = load_mono_sprite(instance_id);

    var out = MonoSpriteVarying();
    out.position = to_device_position_transformed(unit_vertex, sprite.bounds, sprite.transformation);

    out.tile_position = to_tile_position(unit_vertex, sprite.tile);
    out.color = hsla_to_rgba(sprite.color);
    out.clip_distances = distance_from_clip_rect_transformed(unit_vertex, sprite.bounds, sprite.content_mask, sprite.transformation);
    return out;
}

@fragment
fn fs_mono_sprite(input: MonoSpriteVarying) -> @location(0) vec4<f32> {
    let sample = textureSample(t_sprite, s_sprite, input.tile_position).r;
    let alpha_corrected = apply_contrast_and_gamma_correction(sample, input.color.rgb, gamma_params.grayscale_enhanced_contrast, gamma_params.gamma_ratios);

    // Alpha clip after using the derivatives.
    if (any(input.clip_distances < vec4<f32>(0.0))) {
        return vec4<f32>(0.0);
    }

    return blend_color(input.color, alpha_corrected);
}

// --- polychrome sprites --- //

struct PolychromeSprite {
    order: u32,
    pad: u32,
    tile: AtlasTile,
    grayscale: u32,
    opacity: f32,
    bounds: Bounds,
    content_mask: Bounds,
    corner_radii: Corners,
}


struct PolySpriteVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) tile_position: vec2<f32>,
    @location(1) @interpolate(flat) sprite_id: u32,
    @location(2) @interpolate(flat) opacity: f32,
}

@vertex
fn vs_poly_sprite(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> PolySpriteVarying {
    let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
    let sprite = load_poly_sprite(instance_id);

    var out = PolySpriteVarying();
    let clipped = clip_to_mask(unit_vertex, sprite.bounds, sprite.content_mask);
    out.position = to_device_position_impl(clipped.position);
    out.tile_position = to_tile_position(clipped.unit_vertex, sprite.tile);
    let simple = !has_rounded_corners(sprite.corner_radii) && sprite.grayscale == 0u;
    out.sprite_id = instance_id | select(0u, SIMPLE_PRIMITIVE_FLAG, simple);
    out.opacity = sprite.opacity;
    return out;
}

@fragment
fn fs_poly_sprite(input: PolySpriteVarying) -> @location(0) vec4<f32> {
    var color = textureSample(t_sprite, s_sprite, input.tile_position);
    if ((input.sprite_id & SIMPLE_PRIMITIVE_FLAG) != 0u) {
        return blend_color(color, input.opacity);
    }

    let sprite = load_poly_sprite(input.sprite_id);
    if (sprite.grayscale != 0u) {
        color = vec4<f32>(vec3<f32>(dot(color.rgb, GRAYSCALE_FACTORS)), color.a);
    }
    let distance = quad_sdf(input.position.xy, sprite.bounds, sprite.corner_radii);
    return blend_color(color, input.opacity * saturate(0.5 - distance));
}

// --- surfaces --- //

struct SurfaceParams {
    bounds: Bounds,
    content_mask: Bounds,
}

@group(1) @binding(0) var<uniform> surface_locals: SurfaceParams;
@group(1) @binding(1) var t_y: texture_2d<f32>;
@group(1) @binding(2) var t_cb_cr: texture_2d<f32>;
@group(1) @binding(3) var s_surface: sampler;

const ycbcr_to_RGB = mat4x4<f32>(
    vec4<f32>( 1.0000f,  1.0000f,  1.0000f, 0.0),
    vec4<f32>( 0.0000f, -0.3441f,  1.7720f, 0.0),
    vec4<f32>( 1.4020f, -0.7141f,  0.0000f, 0.0),
    vec4<f32>(-0.7010f,  0.5291f, -0.8860f, 1.0),
);

struct SurfaceVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_position: vec2<f32>,
}

@vertex
fn vs_surface(@builtin(vertex_index) vertex_id: u32) -> SurfaceVarying {
    let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));

    var out = SurfaceVarying();
    let clipped = clip_to_mask(unit_vertex, surface_locals.bounds, surface_locals.content_mask);
    out.position = to_device_position_impl(clipped.position);
    // The whole texture is copied, so sample at the vertex's position within the unclipped bounds.
    out.texture_position = clipped.unit_vertex;
    return out;
}

@fragment
fn fs_surface(input: SurfaceVarying) -> @location(0) vec4<f32> {
    let y_cb_cr = vec4<f32>(
        textureSampleLevel(t_y, s_surface, input.texture_position, 0.0).r,
        textureSampleLevel(t_cb_cr, s_surface, input.texture_position, 0.0).rg,
        1.0);

    return ycbcr_to_RGB * y_cb_cr;
}
