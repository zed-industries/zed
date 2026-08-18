#include "alpha_correction.hlsl"

cbuffer GlobalParams: register(b0) {
    float4 gamma_ratios;
    float2 global_viewport_size;
    float grayscale_enhanced_contrast;
    float subpixel_enhanced_contrast;
    uint is_bgr;
    uint3 global_pad;
};

cbuffer BatchParams: register(b1) {
    uint batch_start_index;
    uint3 batch_pad;
};

Texture2D<float4> t_sprite: register(t0);
SamplerState s_sprite: register(s0);

struct SubpixelSpriteFragmentOutput {
    float4 foreground : SV_Target0;
    float4 alpha : SV_Target1;
};

struct Bounds {
    float2 origin;
    float2 size;
};

struct Corners {
    float top_left;
    float top_right;
    float bottom_right;
    float bottom_left;
};

struct Edges {
    float top;
    float right;
    float bottom;
    float left;
};

struct Hsla {
    float h;
    float s;
    float l;
    float a;
};

struct LinearColorStop {
    Hsla color;
    float percentage;
};

struct Background {
    // 0u is Solid
    // 1u is LinearGradient
    // 2u is PatternSlash
    uint tag;
    // 0u is sRGB color
    // 1u is Oklab color
    uint color_space;
    Hsla solid;
    float gradient_angle_or_pattern_height;
    LinearColorStop colors[2];
    uint pad;
};

struct PreparedBackground {
    nointerpolation float4 color0: COLOR1;
    nointerpolation float4 color1: COLOR2;
    // Per-mode coefficients, measured from `pivot`:
    //   linear gradient: (direction.xy, offset, unused)
    //   pattern slash:   (cos, sin, period, threshold)
    //   checkerboard:    (cell size, unused, unused, unused)
    nointerpolation float4 basis: COLOR3;
    nointerpolation float2 pivot: TEXCOORD1;
    nointerpolation uint2 kind: TEXCOORD2;
};

struct AtlasTextureId {
    uint index;
    uint kind;
};

struct AtlasBounds {
    int2 origin;
    int2 size;
};

struct AtlasTile {
    AtlasTextureId texture_id;
    uint tile_id;
    uint padding;
    AtlasBounds bounds;
};

struct TransformationMatrix {
    float2x2 rotation_scale;
    float2 translation;
};

static const float M_PI_F = 3.141592653f;
static const float3 GRAYSCALE_FACTORS = float3(0.2126f, 0.7152f, 0.0722f);

float4 to_device_position_impl(float2 position) {
    float2 device_position = position / global_viewport_size * float2(2.0, -2.0) + float2(-1.0, 1.0);
    return float4(device_position, 0., 1.);
}

float4 to_device_position(float2 unit_vertex, Bounds bounds) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    return to_device_position_impl(position);
}

float4 distance_from_clip_rect_impl(float2 position, Bounds clip_bounds) {
    float2 tl = position - clip_bounds.origin;
    float2 br = clip_bounds.origin + clip_bounds.size - position;
    return float4(tl.x, br.x, tl.y, br.y);
}

float4 distance_from_clip_rect_transformed(float2 unit_vertex, Bounds bounds, Bounds clip_bounds, TransformationMatrix transformation) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    float2 transformed = mul(position, transformation.rotation_scale) + transformation.translation;
    return distance_from_clip_rect_impl(transformed, clip_bounds);
}

struct ClippedVertex {
    float2 position;
    // The corner as a fraction of the original bounds.
    float2 unit_vertex;
};

ClippedVertex clip_to_mask(float2 unit_vertex, Bounds bounds, Bounds mask) {
    float2 origin = max(bounds.origin, mask.origin);
    float2 corner = min(bounds.origin + bounds.size, mask.origin + mask.size);
    float2 size = max(corner - origin, float2(0.0, 0.0));

    ClippedVertex result;
    result.position = origin + unit_vertex * size;
    result.unit_vertex = (result.position - bounds.origin) / bounds.size;
    return result;
}

// Convert linear RGB to sRGB
float3 linear_to_srgb(float3 color) {
    return pow(color, float3(1.0 / 2.2, 1.0 / 2.2, 1.0 / 2.2));
}

// Convert sRGB to linear RGB
float3 srgb_to_linear(float3 color) {
    return pow(color, float3(2.2, 2.2, 2.2));
}

/// Hsla to sRGB conversion.
float4 hsla_to_rgba(Hsla hsla) {
    float h = hsla.h * 6.0; // Now, it's an angle but scaled in [0, 6) range
    float s = hsla.s;
    float l = hsla.l;
    float a = hsla.a;

    float c = (1.0 - abs(2.0 * l - 1.0)) * s;
    float x = c * (1.0 - abs(fmod(h, 2.0) - 1.0));
    float m = l - c / 2.0;

    float r = 0.0;
    float g = 0.0;
    float b = 0.0;

    if (h >= 0.0 && h < 1.0) {
        r = c;
        g = x;
        b = 0.0;
    } else if (h >= 1.0 && h < 2.0) {
        r = x;
        g = c;
        b = 0.0;
    } else if (h >= 2.0 && h < 3.0) {
        r = 0.0;
        g = c;
        b = x;
    } else if (h >= 3.0 && h < 4.0) {
        r = 0.0;
        g = x;
        b = c;
    } else if (h >= 4.0 && h < 5.0) {
        r = x;
        g = 0.0;
        b = c;
    } else {
        r = c;
        g = 0.0;
        b = x;
    }

    float4 rgba;
    rgba.x = (r + m);
    rgba.y = (g + m);
    rgba.z = (b + m);
    rgba.w = a;
    return rgba;
}

// Converts a sRGB color to the Oklab color space.
// Reference: https://bottosson.github.io/posts/oklab/#converting-from-linear-srgb-to-oklab
float4 srgb_to_oklab(float4 color) {
    // Convert non-linear sRGB to linear sRGB
    color = float4(srgb_to_linear(color.rgb), color.a);

    float l = 0.4122214708 * color.r + 0.5363325363 * color.g + 0.0514459929 * color.b;
    float m = 0.2119034982 * color.r + 0.6806995451 * color.g + 0.1073969566 * color.b;
    float s = 0.0883024619 * color.r + 0.2817188376 * color.g + 0.6299787005 * color.b;

    float l_ = pow(l, 1.0/3.0);
    float m_ = pow(m, 1.0/3.0);
    float s_ = pow(s, 1.0/3.0);

    return float4(
        0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
        1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
        0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
        color.a
    );
}

// Converts an Oklab color to the sRGB color space.
float4 oklab_to_srgb(float4 color) {
    float l_ = color.r + 0.3963377774 * color.g + 0.2158037573 * color.b;
    float m_ = color.r - 0.1055613458 * color.g - 0.0638541728 * color.b;
    float s_ = color.r - 0.0894841775 * color.g - 1.2914855480 * color.b;

    float l = l_ * l_ * l_;
    float m = m_ * m_ * m_;
    float s = s_ * s_ * s_;

    float3 linear_rgb = float3(
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s
    );

    // Convert linear sRGB to non-linear sRGB
    return float4(linear_to_srgb(linear_rgb), color.a);
}

// This approximates the error function, needed for the gaussian integral
float2 erf(float2 x) {
    float2 s = sign(x);
    float2 a = abs(x);
    x = 1. + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
    x *= x;
    return s - s / (x * x);
}

float blur_along_x(float x, float y, float sigma, float corner, float2 half_size) {
    float delta = min(half_size.y - corner - abs(y), 0.);
    float curved = half_size.x - corner + sqrt(max(0., corner * corner - delta * delta));
    float2 integral = 0.5 + 0.5 * erf((x + float2(-curved, curved)) * (sqrt(0.5) / sigma));
    return integral.y - integral.x;
}

// A standard gaussian function, used for weighting samples
float gaussian(float x, float sigma) {
    return exp(-(x * x) / (2. * sigma * sigma)) / (sqrt(2. * M_PI_F) * sigma);
}

float4 over(float4 below, float4 above) {
    float4 result;
    float alpha = above.a + below.a * (1.0 - above.a);
    result.rgb = (above.rgb * above.a + below.rgb * below.a * (1.0 - above.a)) / alpha;
    result.a = alpha;
    return result;
}

float2 to_tile_position(float2 unit_vertex, AtlasTile tile) {
    float2 atlas_size;
    t_sprite.GetDimensions(atlas_size.x, atlas_size.y);
    return (float2(tile.bounds.origin) + unit_vertex * float2(tile.bounds.size)) / atlas_size;
}

// Selects corner radius based on quadrant.
float pick_corner_radius(float2 center_to_point, Corners corner_radii) {
    if (center_to_point.x < 0.) {
        if (center_to_point.y < 0.) {
            return corner_radii.top_left;
        } else {
            return corner_radii.bottom_left;
        }
    } else {
        if (center_to_point.y < 0.) {
            return corner_radii.top_right;
        } else {
            return corner_radii.bottom_right;
        }
    }
}

float4 to_device_position_transformed(float2 unit_vertex, Bounds bounds,
                                      TransformationMatrix transformation) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    float2 transformed = mul(position, transformation.rotation_scale) + transformation.translation;
    float2 device_position = transformed / global_viewport_size * float2(2.0, -2.0) + float2(-1.0, 1.0);
    return float4(device_position, 0.0, 1.0);
}

// Implementation of quad signed distance field
float quad_sdf_impl(float2 corner_center_to_point, float corner_radius) {
    if (corner_radius == 0.0) {
        // Fast path for unrounded corners
        return max(corner_center_to_point.x, corner_center_to_point.y);
    } else {
        // Signed distance of the point from a quad that is inset by corner_radius
        // It is negative inside this quad, and positive outside
        float signed_distance_to_inset_quad =
            // 0 inside the inset quad, and positive outside
            length(max(float2(0.0, 0.0), corner_center_to_point)) +
            // 0 outside the inset quad, and negative inside
            min(0.0, max(corner_center_to_point.x, corner_center_to_point.y));

        return signed_distance_to_inset_quad - corner_radius;
    }
}

float quad_sdf(float2 pt, Bounds bounds, Corners corner_radii) {
    float2 half_size = bounds.size / 2.;
    float2 center = bounds.origin + half_size;
    float2 center_to_point = pt - center;
    float corner_radius = pick_corner_radius(center_to_point, corner_radii);
    float2 corner_to_point = abs(center_to_point) - half_size;
    float2 corner_center_to_point = corner_to_point + corner_radius;
    return quad_sdf_impl(corner_center_to_point, corner_radius);
}

PreparedBackground prepare_background(Background background, Bounds bounds) {
    PreparedBackground output;
    output.kind = uint2(background.tag, background.color_space);
    output.color0 = float4(0.0, 0.0, 0.0, 0.0);
    output.color1 = float4(0.0, 0.0, 0.0, 0.0);
    output.basis = float4(0.0, 0.0, 0.0, 0.0);
    output.pivot = bounds.origin;

    switch (background.tag) {
        case 1: {
            output.color0 = hsla_to_rgba(background.colors[0].color);
            output.color1 = hsla_to_rgba(background.colors[1].color);
            if (background.color_space == 1) {
                output.color0 = srgb_to_oklab(output.color0);
                output.color1 = srgb_to_oklab(output.color1);
            }

            // -90 degrees to match the CSS gradient angle.
            float radians = (fmod(background.gradient_angle_or_pattern_height, 360.0) - 90.0)
                * (M_PI_F / 180.0);
            float2 direction = float2(cos(radians), sin(radians));

            // Expand the short side to be the same as the long side
            if (bounds.size.x > bounds.size.y) {
                direction.y *= bounds.size.y / bounds.size.x;
            } else {
                direction.x *= bounds.size.x / bounds.size.y;
            }

            float extent = abs(direction.x) > abs(direction.y)
                ? bounds.size.x
                : bounds.size.y;
            float direction_length = length(direction);
            float span = background.colors[1].percentage - background.colors[0].percentage;
            // The half-extent term is exactly half of `extent`, so the
            // position-independent part reduces to this.
            float offset = 0.5 - background.colors[0].percentage;

            if (direction_length == 0.0 || extent == 0.0) {
                output.basis = float4(0.0, 0.0, 0.0, 0.0);
            } else if (span == 0.0) {
                const float STEP = 1e30;
                output.basis = float4(direction / (direction_length * extent) * STEP,
                                     offset * STEP, 0.0);
            } else {
                output.basis = float4(direction / (direction_length * extent * span),
                                     offset / span, 0.0);
            }
            output.pivot = bounds.origin + bounds.size * 0.5;
            break;
        }
        case 2: {
            output.color0 = hsla_to_rgba(background.solid);
            float pattern_height_encoded = background.gradient_angle_or_pattern_height;
            float pattern_width = (pattern_height_encoded / 65535.0) / 255.0;
            float pattern_interval = fmod(pattern_height_encoded, 65535.0) / 255.0;
            float pattern_height = pattern_width + pattern_interval;
            // Only the rotated x coordinate is used, and a row-vector multiply
            // by the rotation reduces it to dot(v, float2(cos a, sin a)).
            float stripe_angle = M_PI_F / 4.0;
            float2 stripe_direction = float2(cos(stripe_angle), sin(stripe_angle));
            float pattern_period = pattern_height * sin(stripe_angle);
            float threshold = pattern_period * (pattern_width / pattern_height) / 2.0;
            output.basis = float4(stripe_direction, pattern_period, threshold);
            break;
        }
        case 3: {
            output.color0 = hsla_to_rgba(background.solid);
            output.basis = float4(background.gradient_angle_or_pattern_height, 0.0, 0.0, 0.0);
            break;
        }
        default:
            output.color0 = hsla_to_rgba(background.solid);
            break;
    }

    return output;
}

// Ordered (Bayer 4x4) dither threshold, remapped to [-1, +1].
float dither_offset(float2 position) {
    uint2 cell = uint2(position) & 3u;
    uint z = cell.x ^ cell.y;
    uint threshold = ((z & 1u) << 3)
        | ((cell.y & 1u) << 2)
        | (z & 2u)
        | ((cell.y & 2u) >> 1);
    return (float(threshold) + 0.5) * (1.0 / 8.0) - 1.0;
}

float4 background_color(PreparedBackground background, float2 position) {
    float2 relative_position = position - background.pivot;
    float4 color = background.color0;

    switch (background.kind.x) {
        case 1: {
            float t = saturate(dot(relative_position, background.basis.xy) + background.basis.z);
            color = lerp(background.color0, background.color1, t);
            if (background.kind.y == 1) {
                color = oklab_to_srgb(color);
            }

            // Dither to reduce banding in gradients (especially dark/alpha).
            float dither = dither_offset(position);
            color.rgb += dither * 1.0 / 255.0;
            color.a   += dither * 1.5 / 255.0;
            break;
        }
        case 2: {
            float period = background.basis.z;
            float pattern = fmod(dot(relative_position, background.basis.xy), period);
            float distance = min(pattern, period - pattern) - background.basis.w;
            color.a *= saturate(0.5 - distance);
            break;
        }
        case 3: {
            // checkerboard
            float cell_size = background.basis.x;
            float x_index = floor(relative_position.x / cell_size);
            float y_index = floor(relative_position.y / cell_size);
            color.a *= saturate((x_index + y_index) % 2.0);
            break;
        }
    }

    return color;
}

// Returns the dash velocity of a corner given the dash velocity of the two
// sides, by returning the slower velocity (larger dashes).
//
// Since 0 is used for dash velocity when the border width is 0 (instead of
// +inf), this returns the other dash velocity in that case.
//
// An alternative to this might be to appropriately interpolate the dash
// velocity around the corner, but that seems overcomplicated.
float corner_dash_velocity(float dv1, float dv2) {
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
float dash_alpha(
    float t, float period, float length, float dash_velocity,
    float antialias_threshold
) {
    float half_period = period / 2.0;
    float half_length = length / 2.0;
    // Value in [-half_period, half_period]
    // The dash is in [-half_length, half_length]
    float centered = fmod(t + half_period - half_length, period) - half_period;
    // Signed distance for the dash, negative values are inside the dash
    float signed_distance = abs(centered) - half_length;
    // Antialiased alpha based on the signed distance
    return saturate(antialias_threshold - signed_distance / dash_velocity);
}

// This approximates distance to the nearest point to a quarter ellipse in a way
// that is sufficient for anti-aliasing when the ellipse is not very eccentric.
// The components of `point` are expected to be positive.
//
// Negative on the outside and positive on the inside.
float quarter_ellipse_sdf(float2 pt, float2 radii) {
    // Scale the space to treat the ellipse like a unit circle
    float2 circle_vec = pt / radii;
    float unit_circle_sdf = length(circle_vec) - 1.0;
    // Approximate up-scaling of the length by using the average of the radii.
    //
    // TODO: A better solution would be to use the gradient of the implicit
    // function for an ellipse to approximate a scaling factor.
    return unit_circle_sdf * (radii.x + radii.y) * -0.5;
}

/*
**
**              Quads
**
*/

struct Quad {
    uint order;
    uint border_style;
    Bounds bounds;
    Bounds content_mask;
    Background background;
    Hsla border_color;
    Corners corner_radii;
    Edges border_widths;
};

bool has_rounded_corners(Corners corner_radii) {
    return corner_radii.top_left != 0.0
        || corner_radii.top_right != 0.0
        || corner_radii.bottom_right != 0.0
        || corner_radii.bottom_left != 0.0;
}

bool has_border(Edges border_widths) {
    return border_widths.top != 0.0
        || border_widths.right != 0.0
        || border_widths.bottom != 0.0
        || border_widths.left != 0.0;
}

struct QuadFragmentInput {
    nointerpolation uint quad_id: TEXCOORD0;
    float4 position: SV_Position;
    nointerpolation float4 border_color: COLOR0;
    nointerpolation uint is_simple: TEXCOORD3;
    PreparedBackground background;
};

StructuredBuffer<Quad> quads: register(t1);

QuadFragmentInput quad_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    uint quad_id = batch_start_index + instance_id;
    Quad quad = quads[quad_id];
    ClippedVertex vertex = clip_to_mask(unit_vertex, quad.bounds, quad.content_mask);
    float4 device_position = to_device_position_impl(vertex.position);

    float4 border_color = hsla_to_rgba(quad.border_color);

    bool is_simple = !has_rounded_corners(quad.corner_radii)
        && !has_border(quad.border_widths);

    QuadFragmentInput output;
    output.position = device_position;
    output.border_color = border_color;
    output.quad_id = quad_id;
    output.is_simple = is_simple ? 1u : 0u;
    output.background = prepare_background(quad.background, quad.bounds);
    return output;
}

float4 quad_fragment(QuadFragmentInput input): SV_Target {
    float4 background = background_color(input.background, input.position.xy);
    if (input.is_simple != 0u) {
        return background;
    }

    Quad quad = quads[input.quad_id];
    bool unrounded = !has_rounded_corners(quad.corner_radii);

    float2 size = quad.bounds.size;
    float2 half_size = size / 2.;
    float2 the_point = input.position.xy - quad.bounds.origin;
    float2 center_to_point = the_point - half_size;

    // Signed distance field threshold for inclusion of pixels. 0.5 is the
    // minimum distance between the center of the pixel and the edge.
    const float antialias_threshold = 0.5;

    // Radius of the nearest corner
    float corner_radius = pick_corner_radius(center_to_point, quad.corner_radii);

    float2 border = float2(
        center_to_point.x < 0.0 ? quad.border_widths.left : quad.border_widths.right,
        center_to_point.y < 0.0 ? quad.border_widths.top : quad.border_widths.bottom
    );

    // 0-width borders are reduced so that `inner_sdf >= antialias_threshold`.
    // The purpose of this is to not draw antialiasing pixels in this case.
    float2 reduced_border = float2(
        border.x == 0.0 ? -antialias_threshold : border.x,
        border.y == 0.0 ? -antialias_threshold : border.y
    );

    // Vector from the corner of the quad bounds to the point, after mirroring
    // the point into the bottom right quadrant. Both components are <= 0.
    float2 corner_to_point = abs(center_to_point) - half_size;

    // Vector from the point to the center of the rounded corner's circle, also
    // mirrored into bottom right quadrant.
    float2 corner_center_to_point = corner_to_point + corner_radius;

    // Whether the nearest point on the border is rounded
    bool is_near_rounded_corner =
        corner_center_to_point.x >= 0.0 &&
        corner_center_to_point.y >= 0.0;

    // Vector from straight border inner corner to point.
    //
    // 0-width borders are turned into width -1 so that inner_sdf is > 1.0 near
    // the border. Without this, antialiasing pixels would be drawn.
    float2 straight_border_inner_corner_to_point = corner_to_point + reduced_border;

    // Whether the point is beyond the inner edge of the straight border
    bool is_beyond_inner_straight_border =
        straight_border_inner_corner_to_point.x > 0.0 ||
        straight_border_inner_corner_to_point.y > 0.0;

    // Whether the point is far enough inside the quad, such that the pixels are
    // not affected by the straight border.
    bool is_within_inner_straight_border =
        straight_border_inner_corner_to_point.x < -antialias_threshold &&
        straight_border_inner_corner_to_point.y < -antialias_threshold;

    // Fast path for points that must be part of the background
    if (is_within_inner_straight_border && !is_near_rounded_corner) {
        return background;
    }

    // Signed distance of the point to the outside edge of the quad's border
    float outer_sdf = quad_sdf_impl(corner_center_to_point, corner_radius);

    // Approximate signed distance of the point to the inside edge of the quad's
    // border. It is negative outside this edge (within the border), and
    // positive inside.
    //
    // This is not always an accurate signed distance:
    // * The rounded portions with varying border width use an approximation of
    //   nearest-point-on-ellipse.
    // * When it is quickly known to be outside the edge, -1.0 is used.
    float inner_sdf = 0.0;
    if (corner_center_to_point.x <= 0.0 || corner_center_to_point.y <= 0.0) {
        // Fast paths for straight borders
        inner_sdf = -max(straight_border_inner_corner_to_point.x,
                        straight_border_inner_corner_to_point.y);
    } else if (is_beyond_inner_straight_border) {
        // Fast path for points that must be outside the inner edge
        inner_sdf = -1.0;
    } else if (reduced_border.x == reduced_border.y) {
        // Fast path for circular inner edge.
        inner_sdf = -(outer_sdf + reduced_border.x);
    } else {
        float2 ellipse_radii = max(float2(0.0, 0.0), float2(corner_radius, corner_radius) - reduced_border);
        inner_sdf = quarter_ellipse_sdf(corner_center_to_point, ellipse_radii);
    }

    // Negative when inside the border
    float border_sdf = max(inner_sdf, outer_sdf);

    float4 color = background;
    if (border_sdf < antialias_threshold) {
        float4 border_color = input.border_color;
        // Dashed border logic when border_style == 1
        if (quad.border_style == 1) {
            // Position along the perimeter in "dash space", where each dash
            // period has length 1
            float t = 0.0;

            // Total number of dash periods, so that the dash spacing can be
            // adjusted to evenly divide it
            float max_t = 0.0;

            // Border width is proportional to dash size. This is the behavior
            // used by browsers, but also avoids dashes from different segments
            // overlapping when dash size is smaller than the border width.
            //
            // Dash pattern: (2 * border width) dash, (1 * border width) gap
            const float dash_length_per_width = 2.0;
            const float dash_gap_per_width = 1.0;
            const float dash_period_per_width = dash_length_per_width + dash_gap_per_width;

            // Since the dash size is determined by border width, the density of
            // dashes varies. Multiplying a pixel distance by this returns a
            // position in dash space - it has units (dash period / pixels). So
            // a dash velocity of (1 / 10) is 1 dash every 10 pixels.
            float dash_velocity = 0.0;

            // Dividing this by the border width gives the dash velocity
            const float dv_numerator = 1.0 / dash_period_per_width;

            if (unrounded) {
                // When corners aren't rounded, the dashes are separately laid
                // out on each straight line, rather than around the whole
                // perimeter. This way each line starts and ends with a dash.
                bool is_horizontal = corner_center_to_point.x < corner_center_to_point.y;
                // Choosing the right border width for dashed borders.
                // TODO: A better solution exists taking a look at the whole file.
                // this does not fix single dashed borders at the corners
                float2 dashed_border = float2(
                    max(quad.border_widths.bottom, quad.border_widths.top),
                    max(quad.border_widths.right, quad.border_widths.left)
                );
                float border_width = is_horizontal ? dashed_border.x : dashed_border.y;
                dash_velocity = dv_numerator / border_width;
                t = is_horizontal ? the_point.x : the_point.y;
                t *= dash_velocity;
                max_t = is_horizontal ? size.x : size.y;
                max_t *= dash_velocity;
            } else {
                // When corners are rounded, the dashes are laid out clockwise
                // around the whole perimeter.

                float r_tr = quad.corner_radii.top_right;
                float r_br = quad.corner_radii.bottom_right;
                float r_bl = quad.corner_radii.bottom_left;
                float r_tl = quad.corner_radii.top_left;

                float w_t = quad.border_widths.top;
                float w_r = quad.border_widths.right;
                float w_b = quad.border_widths.bottom;
                float w_l = quad.border_widths.left;

                // Straight side dash velocities
                float dv_t = w_t <= 0.0 ? 0.0 : dv_numerator / w_t;
                float dv_r = w_r <= 0.0 ? 0.0 : dv_numerator / w_r;
                float dv_b = w_b <= 0.0 ? 0.0 : dv_numerator / w_b;
                float dv_l = w_l <= 0.0 ? 0.0 : dv_numerator / w_l;

                // Straight side lengths in dash space
                float s_t = (size.x - r_tl - r_tr) * dv_t;
                float s_r = (size.y - r_tr - r_br) * dv_r;
                float s_b = (size.x - r_br - r_bl) * dv_b;
                float s_l = (size.y - r_bl - r_tl) * dv_l;

                float corner_dash_velocity_tr = corner_dash_velocity(dv_t, dv_r);
                float corner_dash_velocity_br = corner_dash_velocity(dv_b, dv_r);
                float corner_dash_velocity_bl = corner_dash_velocity(dv_b, dv_l);
                float corner_dash_velocity_tl = corner_dash_velocity(dv_t, dv_l);

                // Corner lengths in dash space
                float c_tr = r_tr * (M_PI_F / 2.0) * corner_dash_velocity_tr;
                float c_br = r_br * (M_PI_F / 2.0) * corner_dash_velocity_br;
                float c_bl = r_bl * (M_PI_F / 2.0) * corner_dash_velocity_bl;
                float c_tl = r_tl * (M_PI_F / 2.0) * corner_dash_velocity_tl;

                // Cumulative dash space upto each segment
                float upto_tr = s_t;
                float upto_r = upto_tr + c_tr;
                float upto_br = upto_r + s_r;
                float upto_b = upto_br + c_br;
                float upto_bl = upto_b + s_b;
                float upto_l = upto_bl + c_bl;
                float upto_tl = upto_l + s_l;
                max_t = upto_tl + c_tl;

                if (is_near_rounded_corner) {
                    float radians = atan2(corner_center_to_point.y, corner_center_to_point.x);
                    float corner_t = radians * corner_radius;

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
                            // Subtracted because radians is pi/1 to 0 when
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
                    bool is_horizontal = corner_center_to_point.x < corner_center_to_point.y;
                    if (is_horizontal) {
                        if (center_to_point.y < 0.0) {
                            dash_velocity = dv_t;
                            t = (the_point.x - r_tl) * dash_velocity;
                        } else {
                            dash_velocity = dv_b;
                            t = upto_bl - (the_point.x - r_bl) * dash_velocity;
                        }
                    } else {
                        if (center_to_point.x < 0.0) {
                            dash_velocity = dv_l;
                            t = upto_tl - (the_point.y - r_tl) * dash_velocity;
                        } else {
                            dash_velocity = dv_r;
                            t = upto_r + (the_point.y - r_tr) * dash_velocity;
                        }
                    }
                }
            }
            float dash_length = dash_length_per_width / dash_period_per_width;
            float desired_dash_gap = dash_gap_per_width / dash_period_per_width;

            // Straight borders should start and end with a dash, so max_t is
            // reduced to cause this.
            max_t -= unrounded ? dash_length : 0.0;
            if (max_t >= 1.0) {
                // Adjust dash gap to evenly divide max_t
                float dash_count = floor(max_t);
                float dash_period = max_t / dash_count;
                border_color.a *= dash_alpha(t, dash_period, dash_length, dash_velocity, antialias_threshold);
            } else if (unrounded) {
                // When there isn't enough space for the full gap between the
                // two start / end dashes of a straight border, reduce gap to
                // make them fit.
                float dash_gap = max_t - dash_length;
                if (dash_gap > 0.0) {
                    float dash_period = dash_length + dash_gap;
                    border_color.a *= dash_alpha(t, dash_period, dash_length, dash_velocity, antialias_threshold);
                }
            }
        }

        // Blend the border on top of the background and then linearly interpolate
        // between the two as we slide inside the background.
        float4 blended_border = over(background, border_color);
        color = lerp(background, blended_border,
                    saturate(antialias_threshold - inner_sdf));
    }

    return color * float4(1.0, 1.0, 1.0, saturate(antialias_threshold - outer_sdf));
}

/*
**
**              Shadows
**
*/

struct Shadow {
    uint order;
    float blur_radius;
    Bounds bounds;
    Corners corner_radii;
    Bounds content_mask;
    Hsla color;
    Bounds element_bounds;
    Corners element_corner_radii;
    uint inset;
    uint pad; // align to 8 bytes
};

struct ShadowFragmentInput {
    nointerpolation uint shadow_id: TEXCOORD0;
    float4 position: SV_Position;
    nointerpolation float4 color: COLOR;
};

StructuredBuffer<Shadow> shadows: register(t1);

ShadowFragmentInput shadow_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    uint shadow_id = batch_start_index + instance_id;
    Shadow shadow = shadows[shadow_id];

    Bounds bounds;
    if (shadow.inset != 0u) {
        bounds = shadow.element_bounds;
    } else {
        // Leave room for the gaussian tail outside the shadow rect.
        float margin = 3.0 * shadow.blur_radius;
        bounds = shadow.bounds;
        bounds.origin -= margin;
        bounds.size += 2.0 * margin;
    }

    ClippedVertex vertex = clip_to_mask(unit_vertex, bounds, shadow.content_mask);
    float4 device_position = to_device_position_impl(vertex.position);
    float4 color = hsla_to_rgba(shadow.color);

    ShadowFragmentInput output;
    output.position = device_position;
    output.color = color;
    output.shadow_id = shadow_id;

    return output;
}

float4 shadow_fragment(ShadowFragmentInput input): SV_TARGET {
    Shadow shadow = shadows[input.shadow_id];

    float2 half_size = shadow.bounds.size / 2.;
    float2 center = shadow.bounds.origin + half_size;
    float2 point0 = input.position.xy - center;
    float corner_radius = pick_corner_radius(point0, shadow.corner_radii);

    float alpha;
    if (shadow.blur_radius == 0.) {
        float distance = quad_sdf(input.position.xy, shadow.bounds, shadow.corner_radii);
        alpha = saturate(0.5 - distance);
    } else {
        // The signal is only non-zero in a limited range, so don't waste samples
        float low = point0.y - half_size.y;
        float high = point0.y + half_size.y;
        float start = clamp(-3. * shadow.blur_radius, low, high);
        float end = clamp(3. * shadow.blur_radius, low, high);

        // Accumulate samples (we can get away with surprisingly few samples)
        float step = (end - start) / 4.;
        float y = start + step * 0.5;
        alpha = 0.;
        for (int i = 0; i < 4; i++) {
            alpha += blur_along_x(point0.x, point0.y - y, shadow.blur_radius,
                                corner_radius, half_size) *
                    gaussian(y, shadow.blur_radius) * step;
            y += step;
        }
    }

    if (shadow.inset != 0u) {
        // The inset shadow is the complement of the (blurred) hole rect, clipped to the element.
        // `saturate(0.5 - d)` gives a 1-pixel antialiased edge: d <= -0.5 -> 1, d >= 0.5 -> 0.
        alpha = 1.0 - alpha;
        float element_distance = quad_sdf(input.position.xy, shadow.element_bounds,
                                          shadow.element_corner_radii);
        alpha *= saturate(0.5 - element_distance);
    }

    return input.color * float4(1., 1., 1., alpha);
}

/*
**
**              Path Tiles
**
*/

// One element of a tile's curve list: a whole curve, plus the two facts that
// depend on which tile is looking at it. Stored inline rather than indexed,
// so the fragment loop is one sequential run of reads whose addresses are
// known before it starts, instead of a dependent index-then-curve chase on
// every iteration.
struct TileCurve {
    float2 p0;
    float2 p1;
    // Quadratic coefficients (ax, ay); exactly zero for line curves.
    float2 a;
    // Linear coefficients (bx, by).
    float2 b;
    float leg_y;
    // Bit 0: winding sign is negative. Bits 1-2: sx + 1. Bit 3: the curve
    // crosses this tile's downward leg below the sample point.
    uint flags;
};

static const float PATH_TILE_SIZE = 8.0;

struct PathTile {
    uint paint;
    uint curve_start;
    uint curve_count;
    int backdrop;
    float2 corner;
    uint run;
};

struct PathPaint {
    Bounds bounds;
    Bounds content_mask;
    Background color;
    uint even_odd;
};

StructuredBuffer<PathTile> path_tiles: register(t1);
StructuredBuffer<TileCurve> tile_curves: register(t2);
StructuredBuffer<PathPaint> path_paints: register(t3);

// A tile is one wave's worth of pixels, so every interpolant is paid once per
// wave with nothing to amortize it against. Passing the tile and its paint
// down measured -32% wave occupancy on Adreno and was slower despite issuing
// 71% fewer buffer loads, so both stay in memory and the fragment reads them.
struct PathTileFragmentInput {
    float4 position: SV_Position;
    nointerpolation uint tile_id: TEXCOORD0;
};

PathTileFragmentInput path_tile_vertex(uint vertex_id: SV_VertexID, uint tile_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    uint tile_index = batch_start_index + tile_id;
    PathTile tile = path_tiles[tile_index];
    PathPaint paint = path_paints[tile.paint];

    Bounds quad;
    quad.origin = tile.corner;
    quad.size = float2(PATH_TILE_SIZE * float(tile.run), PATH_TILE_SIZE);
    ClippedVertex vertex = clip_to_mask(unit_vertex, quad, paint.content_mask);

    PathTileFragmentInput output;
    output.position = to_device_position_impl(vertex.position);
    output.tile_id = tile_index;
    return output;
}

// Solve for a*t^2 + b*t + c = 0, returning the root within [0, 1].
// `direction` is the sign of the solved component's derivative over the
// monotone piece (+1 increasing, -1 decreasing) and selects the in-range
// root in closed form: at that root b^2 - 4ac = (b + 2at)^2, so
// b + direction * sqrt(...) equals 2(b + at) with both terms carrying
// direction's sign -- the citardauq quotient -2c / (b + direction * sqrt)
// is the monotone root with no cancellation and no candidate selection.
float monotone_quadratic_root(float a, float b, float c, float direction) {
    if (abs(a) < 1e-6) {
        return abs(b) < 1e-12 ? 0.0 : saturate(-c / b);
    }
    float discriminant = max(b * b - 4.0 * a * c, 0.0);
    float denominator = b + direction * sqrt(discriminant);
    if (abs(denominator) < 1e-12) {
        return 0.0;
    }
    return saturate(-2.0 * c / denominator);
}

// Definite integral over [t0, t1] of (ax*t^2 + bx*t + cx) * (2*ay*t + by):
// the integrand is cubic, so the midpoint value plus the dt^2/12
// second-derivative correction is exact, and it avoids differencing a
// quartic antiderivative at two nearby points.
float coverage_integral(float ax, float bx, float cx, float ay, float by, float t0, float t1) {
    float dt = t1 - t0;
    float m = 0.5 * (t0 + t1);
    float x_m = (ax * m + bx) * m + cx;
    float dx_m = 2.0 * ax * m + bx;
    float dy_m = 2.0 * ay * m + by;
    return dt * (x_m * dy_m + dt * dt / 12.0 * (ax * dy_m + 2.0 * ay * dx_m));
}

// Exact area of the part of the pixel column [px, px+1] left of a line
// over a y-window of the given height: integral of clamp(x(y), 0, 1) dy
// with x affine in y from x0 to x1 (column-relative). Phi(x) =
// 0.5*max(x,0)^2 - 0.5*max(x-1,0)^2 is the antiderivative of clamp in x,
// so the area is height * (Phi(x1) - Phi(x0)) / (x1 - x0), with the
// constant-x limit when the window is vertical. The squared differences
// are factored so that small dx stays exact (m1 - m0 equals dx exactly
// inside the column) instead of cancelling inside Phi.
float line_column_area(float height, float x0, float x1) {
    float dx = x1 - x0;
    if (dx == 0.0) {
        return height * saturate(x0);
    }
    float m0 = max(x0, 0.0);
    float m1 = max(x1, 0.0);
    float n0 = max(x0 - 1.0, 0.0);
    float n1 = max(x1 - 1.0, 0.0);
    return height * 0.5 * ((m1 - m0) * (m1 + m0) - (n1 - n0) * (n1 + n0)) / dx;
}

// Exact area of the part of the pixel column [px, px+1] left of a
// downward-monotone quadratic curve over the window [ya, yb]:
//     integral over [ya, yb] of clamp(x(y) - px, 0, 1) dy
// Specialized to its one call site: the caller guarantees the window lies
// inside the curve's y-span (so no constant extensions are needed) and
// passes the window-end roots ta/tb and column-relative offsets xa/xb
// (= x - px) it already computed.
float curve_column_area(float ax, float bx, float cx, float ay, float by, float p0y,
                        float ta, float tb, float xa, float xb) {
    // Split the parameter interval where the curve crosses the column's
    // boundaries; each crossing is another single-root monotone solve.
    if (xb >= xa) {
        float s0 = xa >= 0.0 ? ta : (xb <= 0.0 ? tb : clamp(monotone_quadratic_root(ax, bx, cx, 1.0), ta, tb));
        float s1 = xb <= 1.0 ? tb : (xa >= 1.0 ? ta : clamp(monotone_quadratic_root(ax, bx, cx - 1.0, 1.0), ta, tb));
        float y_s1 = (ay * s1 + by) * s1 + p0y;
        float y_tb = (ay * tb + by) * tb + p0y;
        return (y_tb - y_s1) + coverage_integral(ax, bx, cx, ay, by, s0, s1);
    } else {
        float s1 = xa <= 1.0 ? ta : (xb >= 1.0 ? tb : clamp(monotone_quadratic_root(ax, bx, cx - 1.0, -1.0), ta, tb));
        float s0 = xb >= 0.0 ? tb : (xa <= 0.0 ? ta : clamp(monotone_quadratic_root(ax, bx, cx, -1.0), ta, tb));
        float y_s1 = (ay * s1 + by) * s1 + p0y;
        float y_ta = (ay * ta + by) * ta + p0y;
        return (y_s1 - y_ta) + coverage_integral(ax, bx, cx, ay, by, s1, s0);
    }
}

// Mean winding this curve contributes to the pixel box, along the L-shaped
// route from the tile's backdrop sample point: down to the sample's row,
// then right to the sample. `crosses_downward_leg` and `leg_y` are the
// CPU's booking of whether this curve's crossing of the tile's left-edge
// line belongs to the leg, and where that crossing is; both are constant
// over the tile and are never re-derived here.
//
// A leg crossing counts with the sign of cross(leg direction, contour
// tangent). For the rightward leg that is sign(dy) = curve.sign; for the
// downward leg it is -sign(dx) = -curve.sign * curve.sx. The rightward leg
// matches what the CPU counted along the grid row line, so the backdrop and
// these corrections compose into the true winding.
float curve_winding(TileCurve curve, float2 corner, float2 pixel) {
    float ax = curve.a.x;
    float bx = curve.b.x;
    float ay = curve.a.y;
    float by = curve.b.y;
    float leg_y = curve.leg_y;
    float sign = 1.0 - 2.0 * float(curve.flags & 1u);
    float sx = float((curve.flags >> 1u) & 3u) - 1.0;
    bool crosses_downward_leg = (curve.flags & 8u) != 0u;
    bool is_line = (curve.flags & 16u) != 0u;
    float winding = 0.0;

    // Rightward leg, from the tile's left edge to the sample. Clamping to
    // the curve's y-span first is what makes the column integral usable
    // here: it extends its boundary with constant x outside the span, but a
    // winding crossing exists only inside the span.
    float ya = max(pixel.y, curve.p0.y);
    float yb = min(pixel.y + 1.0, curve.p1.y);
    if (yb > ya) {
        float window = yb - ya;
        // A line's x is affine in y, so the window ends are two multiply-adds
        // against the uploaded slope rather than two root solves. The branch
        // is per-curve and every lane of a tile is on the same curve at the
        // same iteration, so it is wave-uniform.
        float ta = 0.0;
        float tb = 0.0;
        float xa, xb;
        if (is_line) {
            xa = curve.p0.x + (ya - curve.p0.y) * ax;
            xb = curve.p0.x + (yb - curve.p0.y) * ax;
        } else {
            ta = monotone_quadratic_root(ay, by, curve.p0.y - ya, 1.0);
            tb = monotone_quadratic_root(ay, by, curve.p0.y - yb, 1.0);
            xa = (ax * ta + bx) * ta + curve.p0.x;
            xb = (ax * tb + bx) * tb + curve.p0.x;
        }

        // By monotonicity the curve's x-extent over the window is exactly
        // [min(xa, xb), max(xa, xb)], which classifies most pixels without
        // the column integral: a curve entirely right of this pixel's
        // column contributes zero (the pixel is wholly left of the
        // boundary), and one entirely left of it contributes the whole
        // window. Only the one or two columns the curve actually passes
        // through pay for the exact area.
        if (min(xa, xb) < pixel.x + 1.0) {
            // A crossing left of the tile's left edge is not on the leg and
            // must contribute zero for that y -- not a full pixel width.
            // x-monotonicity means x_c(y) meets corner.x at most once, at
            // the uploaded height, so the window splits into a live part
            // and a dead one with no solve. Every discrete gate on both
            // sides evaluates the same one-sided limit, "sample at
            // corner + 0+": dead means at-or-left of the corner, matching
            // the CPU's ceil-minus-one booking, under which a crossing
            // exactly on the edge is strictly left of the perturbed sample
            // and backdrop-owned -- a vertical curve lying exactly on the
            // tile's left edge is dead for every pixel of the tile. Only
            // the crossing-count height is clipped; the area integral
            // keeps the full window, because over the dead part the curve
            // is at-or-left of corner.x where this pixel's column
            // integrand is zero anyway (exactly zero off the leftmost
            // column), which is what lets the clipped end keep its
            // already-solved parameter bounds.
            bool live = true;
            if (max(xa, xb) <= corner.x) {
                live = false;
            } else if (min(xa, xb) < corner.x) {
                float y_c = clamp(leg_y, ya, yb);
                if (xa < corner.x) {
                    ya = y_c;
                } else {
                    yb = y_c;
                }
                live = yb > ya;
            }
            if (live) {
                if (max(xa, xb) <= pixel.x) {
                    winding += sign * (yb - ya);
                } else if (is_line) {
                    // Lines carry exact-zero quadratic coefficients (set,
                    // not derived, in MonotoneCurve::scaled), so this branch
                    // is uniform across every lane processing the same
                    // curve. Like the general integral, the area spans the
                    // full window, not the leg-clipped [ya, yb]: the clip
                    // only shortens the crossing height, and the clipped
                    // part's integrand is zero. The line formula couples
                    // height and x-range through the slope, so it must see
                    // matching full-window values for both.
                    winding += sign * ((yb - ya)
                        - line_column_area(window, xa - pixel.x, xb - pixel.x));
                } else {
                    // clamp(px + 1 - x_c, 0, 1) = 1 - clamp(x_c - px, 0, 1).
                    winding += sign * ((yb - ya)
                        - curve_column_area(ax, bx, curve.p0.x - pixel.x, ay, by, curve.p0.y,
                                            ta, tb, xa - pixel.x, xb - pixel.x));
                }
            }
        }
    }

    // Downward leg, from the sample point down to the sample's row. Both
    // the decision and the crossing height were made once, on the CPU,
    // alongside the backdrop this correction must complement; re-deriving
    // either here would repeat per-tile work at every pixel, and
    // re-deriving the decision would reopen the possibility of the two
    // sides disagreeing. Per pixel only the box-filter weight remains. No
    // lower gate is needed; the weight zeroes crossings below the pixel by
    // itself.
    if (crosses_downward_leg) {
        winding -= sign * sx * clamp(pixel.y + 1.0 - leg_y, 0.0, 1.0);
    }

    return winding;
}

float4 path_tile_fragment(PathTileFragmentInput input): SV_Target {
    PathTile tile = path_tiles[input.tile_id];
    float2 pixel = floor(input.position.xy);

    // The tile's curve list is sorted by each curve's leftmost x, so the
    // first curve entirely right of this pixel's column ends the loop:
    // every later one is too, and such curves contribute nothing — the
    // rightward-leg gate rejects them, and a downward-leg booking
    // requires straddling the tile's left edge, which lies left of every
    // pixel's right edge. The break costs divergence only where neighbors
    // within a wave straddle a curve's leftmost x.
    float winding = float(tile.backdrop);
    [loop]
    for (uint i = 0u; i < tile.curve_count; i++) {
        TileCurve curve = tile_curves[tile.curve_start + i];
        if (min(curve.p0.x, curve.p1.x) >= pixel.x + 1.0) {
            break;
        }
        winding += curve_winding(curve, tile.corner, pixel);
    }

    float coverage = path_paints[tile.paint].even_odd != 0u
        // Distance to the nearest even integer (FreeType's fold).
        ? abs(winding - 2.0 * round(winding * 0.5))
        // Distance from zero, clamped.
        : min(abs(winding), 1.0);

    // Wholly-exterior pixels of boundary tiles land on exactly zero (their
    // winding is the untouched integer backdrop), and with straight-alpha
    // blending an all-zero output is a no-op, so skip paint evaluation —
    // gradient direction math, Oklab conversion, and dithering — for them.
    if (coverage == 0.0) {
        return float4(0.0, 0.0, 0.0, 0.0);
    }

    // Only reached by pixels with ink, so the paint — the cold half of
    // `PathPaint` — is read behind this branch rather than up front.
    PathPaint paint = path_paints[tile.paint];
    float4 color = background_color(prepare_background(paint.color, paint.bounds),
        input.position.xy);
    return float4(color.rgb, color.a * coverage);
}

/*
**
**              Underlines
**
*/

struct Underline {
    uint order;
    uint pad;
    Bounds bounds;
    Bounds content_mask;
    Hsla color;
    float thickness;
    uint wavy;
};

struct UnderlineFragmentInput {
  nointerpolation uint underline_id: TEXCOORD0;
  float4 position: SV_Position;
  nointerpolation float4 color: COLOR;
  // Straight underlines are the common case and need nothing but the color, so
  // this rides along to spare them the instance read entirely.
  nointerpolation uint wavy: TEXCOORD1;
};

StructuredBuffer<Underline> underlines: register(t1);

UnderlineFragmentInput underline_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    uint underline_id = batch_start_index + instance_id;
    Underline underline = underlines[underline_id];
    ClippedVertex vertex = clip_to_mask(unit_vertex, underline.bounds, underline.content_mask);
    float4 device_position = to_device_position_impl(vertex.position);
    float4 color = hsla_to_rgba(underline.color);

    UnderlineFragmentInput output;
    output.position = device_position;
    output.color = color;
    output.underline_id = underline_id;
    output.wavy = underline.wavy;
    return output;
}

float4 underline_fragment(UnderlineFragmentInput input): SV_Target {
    const float WAVE_FREQUENCY = 2.0;
    const float WAVE_HEIGHT_RATIO = 0.8;

    if (input.wavy == 0u) {
        return input.color;
    }

    Underline underline = underlines[input.underline_id];
    float half_thickness = underline.thickness * 0.5;
    float2 origin = underline.bounds.origin;

    float2 st = ((input.position.xy - origin) / underline.bounds.size.y) - float2(0., 0.5);
    float frequency = (M_PI_F * WAVE_FREQUENCY * underline.thickness) / underline.bounds.size.y;
    float amplitude = (underline.thickness * WAVE_HEIGHT_RATIO) / underline.bounds.size.y;

    float sine = sin(st.x * frequency) * amplitude;
    float dSine = cos(st.x * frequency) * amplitude * frequency;
    float distance = (st.y - sine) / sqrt(1. + dSine * dSine);
    float distance_in_pixels = distance * underline.bounds.size.y;
    float distance_from_top_border = distance_in_pixels - half_thickness;
    float distance_from_bottom_border = distance_in_pixels + half_thickness;
    float alpha = saturate(
        0.5 - max(-distance_from_bottom_border, distance_from_top_border));
    return input.color * float4(1., 1., 1., alpha);
}

/*
**
**              Monochrome sprites
**
*/

struct MonochromeSprite {
    uint order;
    uint pad;
    Bounds bounds;
    Bounds content_mask;
    Hsla color;
    AtlasTile tile;
    TransformationMatrix transformation;
};

struct MonochromeSpriteFragmentInput {
    float4 position: SV_Position;
    float2 tile_position: POSITION;
    nointerpolation float4 color: COLOR;
    float4 clip_distance: SV_ClipDistance;
};

StructuredBuffer<MonochromeSprite> mono_sprites: register(t1);

MonochromeSpriteFragmentInput monochrome_sprite_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    uint sprite_id = batch_start_index + instance_id;
    MonochromeSprite sprite = mono_sprites[sprite_id];
    float4 device_position =
        to_device_position_transformed(unit_vertex, sprite.bounds, sprite.transformation);
    float4 clip_distance = distance_from_clip_rect_transformed(unit_vertex, sprite.bounds, sprite.content_mask, sprite.transformation);
    float2 tile_position = to_tile_position(unit_vertex, sprite.tile);
    float4 color = hsla_to_rgba(sprite.color);

    MonochromeSpriteFragmentInput output;
    output.position = device_position;
    output.tile_position = tile_position;
    output.color = color;
    output.clip_distance = clip_distance;
    return output;
}

float4 monochrome_sprite_fragment(MonochromeSpriteFragmentInput input): SV_Target {
    float sample = t_sprite.Sample(s_sprite, input.tile_position).r;
    float alpha_corrected = apply_contrast_and_gamma_correction(sample, input.color.rgb, grayscale_enhanced_contrast, gamma_ratios);
    return float4(input.color.rgb, input.color.a * alpha_corrected);
}

MonochromeSpriteFragmentInput subpixel_sprite_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    return monochrome_sprite_vertex(vertex_id, instance_id);
}

SubpixelSpriteFragmentOutput subpixel_sprite_fragment(MonochromeSpriteFragmentInput input) {
    float3 sample = t_sprite.Sample(s_sprite, input.tile_position).rgb;
    if (is_bgr) {
        sample = sample.bgr;
    }
    float3 alpha_corrected = apply_contrast_and_gamma_correction3(sample, input.color.rgb, subpixel_enhanced_contrast, gamma_ratios);

    SubpixelSpriteFragmentOutput output;
    output.foreground = float4(input.color.rgb, 1.0f);
    output.alpha = float4(input.color.a * alpha_corrected, 1.0f);
    return output;
}

/*
**
**              Polychrome sprites
**
*/

struct PolychromeSprite {
    uint order;
    uint pad;
    uint grayscale;
    float opacity;
    Bounds bounds;
    Bounds content_mask;
    Corners corner_radii;
    AtlasTile tile;
};

struct PolychromeSpriteFragmentInput {
    nointerpolation uint sprite_id: TEXCOORD0;
    float4 position: SV_Position;
    float2 tile_position: POSITION;
};

StructuredBuffer<PolychromeSprite> poly_sprites: register(t1);

PolychromeSpriteFragmentInput polychrome_sprite_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    uint sprite_id = batch_start_index + instance_id;
    PolychromeSprite sprite = poly_sprites[sprite_id];
    ClippedVertex vertex = clip_to_mask(unit_vertex, sprite.bounds, sprite.content_mask);
    float4 device_position = to_device_position_impl(vertex.position);
    float2 tile_position = to_tile_position(vertex.unit_vertex, sprite.tile);

    PolychromeSpriteFragmentInput output;
    output.position = device_position;
    output.tile_position = tile_position;
    output.sprite_id = sprite_id;
    return output;
}

float4 polychrome_sprite_fragment(PolychromeSpriteFragmentInput input): SV_Target {
    PolychromeSprite sprite = poly_sprites[input.sprite_id];
    float4 sample = t_sprite.Sample(s_sprite, input.tile_position);
    float distance = quad_sdf(input.position.xy, sprite.bounds, sprite.corner_radii);

    float4 color = sample;
    if (sprite.grayscale != 0u) {
        float3 grayscale = dot(color.rgb, GRAYSCALE_FACTORS);
        color = float4(grayscale, sample.a);
    }
    color.a *= sprite.opacity * saturate(0.5 - distance);
    return color;
}
