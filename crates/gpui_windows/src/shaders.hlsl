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
    // 0u is sRGB linear color
    // 1u is Oklab color
    uint color_space;
    Hsla solid;
    float gradient_angle_or_pattern_height;
    LinearColorStop colors[2];
    uint pad;
};

struct GradientColor {
  float4 solid;
  float4 color0;
  float4 color1;
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

float4 distance_from_clip_rect(float2 unit_vertex, Bounds bounds, Bounds clip_bounds) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    return distance_from_clip_rect_impl(position, clip_bounds);
}

float4 distance_from_clip_rect_transformed(float2 unit_vertex, Bounds bounds, Bounds clip_bounds, TransformationMatrix transformation) {
    float2 position = unit_vertex * bounds.size + bounds.origin;
    float2 transformed = mul(position, transformation.rotation_scale) + transformation.translation;
    return distance_from_clip_rect_impl(transformed, clip_bounds);
}

// Convert linear RGB to sRGB
float3 linear_to_srgb(float3 color) {
    return pow(color, float3(2.2, 2.2, 2.2));
}

// Convert sRGB to linear RGB
float3 srgb_to_linear(float3 color) {
    return pow(color, float3(1.0 / 2.2, 1.0 / 2.2, 1.0 / 2.2));
}

/// Hsla to linear RGBA conversion.
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

float4 corner_values(Corners corner_radii) {
    return float4(
        corner_radii.top_left,
        corner_radii.top_right,
        corner_radii.bottom_right,
        corner_radii.bottom_left
    );
}

// Match the superellipse's diagonal inset to that of the public circular
// radius while its edge reach grows by (1 + smoothing).
static const float SUPERELLIPSE_DIAGONAL_INSET = 0.2928932188134524;

float normalized_superellipse_power(float corner_smoothing) {
    float smoothing = clamp(corner_smoothing, 0.0, 1.0);
    float normalized_diagonal =
        1.0 - SUPERELLIPSE_DIAGONAL_INSET / (1.0 + smoothing);
    return -log(2.0) / log(normalized_diagonal);
}

float4 normalized_superellipse_reaches(
    Corners corner_radii,
    float corner_smoothing
) {
    return max(corner_values(corner_radii), float4(0.0, 0.0, 0.0, 0.0)) *
        (1.0 + clamp(corner_smoothing, 0.0, 1.0));
}

bool can_use_normalized_superellipse(
    float2 size,
    Corners corner_radii,
    float corner_smoothing
) {
    float4 radii = max(
        corner_values(corner_radii),
        float4(0.0, 0.0, 0.0, 0.0)
    );
    float4 reaches = normalized_superellipse_reaches(
        corner_radii,
        corner_smoothing
    );
    float half_short_side = 0.5 * min(size.x, size.y);
    return corner_smoothing > 0.0 &&
        size.x > 0.0 && size.y > 0.0 &&
        any(radii > float4(0.0, 0.0, 0.0, 0.0)) &&
        all(reaches <= float4(
            half_short_side,
            half_short_side,
            half_short_side,
            half_short_side
        ));
}

float normalized_superellipse_sdf_impl(
    float2 corner_to_point,
    float corner_radius,
    float corner_smoothing,
    float power
) {
    float extent = max(corner_radius, 0.0) *
        (1.0 + clamp(corner_smoothing, 0.0, 1.0));
    float2 corner_center_to_point = corner_to_point + extent;
    if (extent <= 0.0 ||
        corner_center_to_point.x <= 0.0 ||
        corner_center_to_point.y <= 0.0) {
        return max(corner_to_point.x, corner_to_point.y);
    }

    float2 normalized = corner_center_to_point / extent;
    float2 powered = pow(normalized, float2(power, power));
    float gradient = power * length(pow(
        normalized,
        float2(power - 1.0, power - 1.0)
    ));
    return extent * (powered.x + powered.y - 1.0) /
        max(gradient, 0.000001);
}

float normalized_superellipse_sdf(
    float2 sample_position,
    Bounds bounds,
    Corners corner_radii,
    float corner_smoothing,
    float power
) {
    float2 half_size = bounds.size / 2.0;
    float2 center_to_point = sample_position - (bounds.origin + half_size);
    float corner_radius = pick_corner_radius(center_to_point, corner_radii);
    float2 corner_to_point = abs(center_to_point) - half_size;
    return normalized_superellipse_sdf_impl(
        corner_to_point,
        corner_radius,
        corner_smoothing,
        power
    );
}

// The Figma-style corner construction below is derived from
// Tien Pham's figma-squircle implementation:
// https://github.com/phamfoo/figma-squircle
// Copyright (c) 2021 Tien Pham
// Licensed under the MIT License. See ../THIRD_PARTY_NOTICES.md.

struct FigmaCornerLayout {
    float4 horizontal_budgets;
    float4 vertical_budgets;
};

struct FigmaCornerExtents {
    float4 horizontal;
    float4 vertical;
};

float2 figma_split_side(float length, float first_radius, float second_radius) {
    float total_radius = first_radius + second_radius;
    if (total_radius == 0.0) return float2(0.0, 0.0);
    float first_budget = length * first_radius / total_radius;
    return float2(first_budget, length - first_budget);
}

void figma_sort_corner_pair(
    inout float first_radius,
    inout int first_corner,
    inout float second_radius,
    inout int second_corner
) {
    if (second_radius > first_radius) {
        float temporary_radius = first_radius;
        first_radius = second_radius;
        second_radius = temporary_radius;

        int temporary_corner = first_corner;
        first_corner = second_corner;
        second_corner = temporary_corner;
    }
}

void figma_apply_corner_budget(
    float2 size,
    int corner,
    inout float4 radii,
    inout float4 budgets
) {
    float radius;
    float horizontal_radius;
    float horizontal_neighbor_budget;
    float vertical_radius;
    float vertical_neighbor_budget;

    if (corner == 0) {
        radius = radii.x;
        horizontal_radius = radii.y;
        horizontal_neighbor_budget = budgets.y;
        vertical_radius = radii.w;
        vertical_neighbor_budget = budgets.w;
    } else if (corner == 1) {
        radius = radii.y;
        horizontal_radius = radii.x;
        horizontal_neighbor_budget = budgets.x;
        vertical_radius = radii.z;
        vertical_neighbor_budget = budgets.z;
    } else if (corner == 2) {
        radius = radii.z;
        horizontal_radius = radii.w;
        horizontal_neighbor_budget = budgets.w;
        vertical_radius = radii.y;
        vertical_neighbor_budget = budgets.y;
    } else {
        radius = radii.w;
        horizontal_radius = radii.z;
        horizontal_neighbor_budget = budgets.z;
        vertical_radius = radii.x;
        vertical_neighbor_budget = budgets.x;
    }

    float horizontal_budget = 0.0;
    if (radius != 0.0 || horizontal_radius != 0.0) {
        horizontal_budget = horizontal_neighbor_budget >= 0.0
            ? size.x - horizontal_neighbor_budget
            : size.x * radius / (radius + horizontal_radius);
    }

    float vertical_budget = 0.0;
    if (radius != 0.0 || vertical_radius != 0.0) {
        vertical_budget = vertical_neighbor_budget >= 0.0
            ? size.y - vertical_neighbor_budget
            : size.y * radius / (radius + vertical_radius);
    }

    float budget = max(0.0, min(horizontal_budget, vertical_budget));
    float clamped_radius = min(radius, budget);
    if (corner == 0) {
        budgets.x = budget;
        radii.x = clamped_radius;
    } else if (corner == 1) {
        budgets.y = budget;
        radii.y = clamped_radius;
    } else if (corner == 2) {
        budgets.z = budget;
        radii.z = clamped_radius;
    } else {
        budgets.w = budget;
        radii.w = clamped_radius;
    }
}

// Normalizes the radii, then assigns each corner its share of each adjacent
// edge. The stable radius order is TL, TR, BL, BR. Vectors use TL, TR, BR, BL.
FigmaCornerLayout figma_corner_layout(float2 size, Corners corner_radii) {
    float4 radii = max(
        corner_values(corner_radii),
        float4(0.0, 0.0, 0.0, 0.0)
    );
    float4 budgets = float4(-1.0, -1.0, -1.0, -1.0);
    // FXC cannot dynamically index a vector used as an l-value. Keep the
    // stable sort slots scalar and write each budget through a fixed component.
    float first_radius = radii.x;
    float second_radius = radii.y;
    float third_radius = radii.w;
    float fourth_radius = radii.z;
    int first_corner = 0;
    int second_corner = 1;
    int third_corner = 3;
    int fourth_corner = 2;

    figma_sort_corner_pair(
        first_radius,
        first_corner,
        second_radius,
        second_corner
    );
    figma_sort_corner_pair(
        second_radius,
        second_corner,
        third_radius,
        third_corner
    );
    figma_sort_corner_pair(
        third_radius,
        third_corner,
        fourth_radius,
        fourth_corner
    );
    figma_sort_corner_pair(
        first_radius,
        first_corner,
        second_radius,
        second_corner
    );
    figma_sort_corner_pair(
        second_radius,
        second_corner,
        third_radius,
        third_corner
    );
    figma_sort_corner_pair(
        first_radius,
        first_corner,
        second_radius,
        second_corner
    );

    figma_apply_corner_budget(size, first_corner, radii, budgets);
    figma_apply_corner_budget(size, second_corner, radii, budgets);
    figma_apply_corner_budget(size, third_corner, radii, budgets);
    figma_apply_corner_budget(size, fourth_corner, radii, budgets);

    float2 top = figma_split_side(size.x, radii[0], radii[1]);
    float2 bottom = figma_split_side(size.x, radii[3], radii[2]);
    float2 left = figma_split_side(size.y, radii[0], radii[3]);
    float2 right = figma_split_side(size.y, radii[1], radii[2]);

    FigmaCornerLayout corner_layout;
    corner_layout.horizontal_budgets = float4(top.x, top.y, bottom.y, bottom.x);
    corner_layout.vertical_budgets = float4(left.x, right.x, right.y, left.y);
    return corner_layout;
}

struct FigmaAxisParams {
    float a;
    float b;
    float p;
};

struct FigmaCornerParams {
    float radius;
    float smoothing;
    float arc_sweep;
    float c;
    float d;
    FigmaAxisParams horizontal;
    FigmaAxisParams vertical;
};

struct CubicClosestPoint {
    float2 position;
    float2 tangent;
    float distance;
    float path_t;
};

struct SdfSample {
    float distance;
    float2 normal;
    float path_t;
    uint segment;
};

struct FigmaRectSample {
    SdfSample sdf;
    uint corner;
};

static const uint FIGMA_SEGMENT_STRAIGHT = 0u;
// Corner progress runs from the horizontal shoulder through the full arc to
// the vertical shoulder. Cubic path_t always runs from its straight endpoint
// to the arc, while arc path_t covers the full arc from 0 to 1.
static const uint FIGMA_SEGMENT_FIRST_CUBIC = 1u;
static const uint FIGMA_SEGMENT_ARC = 2u;
static const uint FIGMA_SEGMENT_SECOND_CUBIC = 3u;
static const uint FIGMA_NO_CORNER = 4u;
static const float FIGMA_EPSILON = 0.000001;

float4 figma_smoothing_factors(float corner_smoothing) {
    float smoothing = clamp(corner_smoothing, 0.0, 1.0);
    float arc_sweep = 0.5 * M_PI_F * (1.0 - smoothing);
    float beta = (M_PI_F / 4.0) * smoothing;
    float join_handle_factor = tan(0.5 * beta);
    return float4(
        smoothing,
        sin(0.5 * arc_sweep) * sqrt(2.0),
        join_handle_factor * cos(beta),
        join_handle_factor * sin(beta)
    );
}

FigmaCornerParams figma_corner_params(
    float corner_radius,
    float horizontal_reach,
    float vertical_reach,
    float4 smoothing_factors
) {
    horizontal_reach = max(horizontal_reach, 0.0);
    vertical_reach = max(vertical_reach, 0.0);
    float radius = min(
        max(corner_radius, 0.0),
        min(horizontal_reach, vertical_reach)
    );
    float smoothing = radius > FIGMA_EPSILON
        ? smoothing_factors.x
        : 0.0;
    float desired_reach = radius * (1.0 + smoothing);
    float arc_sweep = 0.5 * M_PI_F * (1.0 - smoothing);
    float arc_delta = smoothing_factors.y * radius;
    float c = smoothing_factors.z * radius;
    float d = smoothing_factors.w * radius;
    float core_length = arc_delta + c + d;
    float ideal_b = max((desired_reach - core_length) / 3.0, 0.0);
    float horizontal_available = max(horizontal_reach - core_length, 0.0);
    float vertical_available = max(vertical_reach - core_length, 0.0);
    float shared_b = min(
        ideal_b,
        min(
            horizontal_available * (5.0 / 6.0),
            vertical_available * (5.0 / 6.0)
        )
    );

    FigmaCornerParams params;
    params.radius = radius;
    params.smoothing = smoothing;
    params.arc_sweep = arc_sweep;
    params.c = c;
    params.d = d;
    params.horizontal.a = horizontal_available - shared_b;
    params.horizontal.b = shared_b;
    params.horizontal.p = horizontal_reach;
    params.vertical.a = vertical_available - shared_b;
    params.vertical.b = shared_b;
    params.vertical.p = vertical_reach;
    return params;
}

FigmaCornerExtents figma_corner_extents(
    Corners corner_radii,
    float4 horizontal_budgets,
    float4 vertical_budgets,
    float corner_smoothing
) {
    float4 radii = corner_values(corner_radii);
    FigmaCornerExtents extents;
    [unroll]
    for (int corner = 0; corner < 4; corner++) {
        float horizontal_budget = max(horizontal_budgets[corner], 0.0);
        float vertical_budget = max(vertical_budgets[corner], 0.0);
        float radius = min(
            max(radii[corner], 0.0),
            min(horizontal_budget, vertical_budget)
        );
        float desired_reach = radius *
            (1.0 + clamp(corner_smoothing, 0.0, 1.0));
        extents.horizontal[corner] = min(desired_reach, horizontal_budget);
        extents.vertical[corner] = min(desired_reach, vertical_budget);
    }
    return extents;
}

float2 figma_cubic_point(
    FigmaAxisParams axis,
    float c,
    float d,
    float t
) {
    float x1 = 3.0 * axis.a;
    float x2 = 3.0 * (axis.b - axis.a);
    float x3 = axis.a - 2.0 * axis.b + c;
    float t2 = t * t;
    return float2(
        t * (x1 + t * (x2 + t * x3)),
        d * t2 * t
    );
}

float2 figma_cubic_derivative(
    FigmaAxisParams axis,
    float c,
    float d,
    float t
) {
    float x1 = 3.0 * axis.a;
    float x2 = 3.0 * (axis.b - axis.a);
    float x3 = axis.a - 2.0 * axis.b + c;
    float t2 = t * t;
    return float2(
        x1 + 2.0 * x2 * t + 3.0 * x3 * t2,
        3.0 * d * t2
    );
}

float2 figma_cubic_second_derivative(
    FigmaAxisParams axis,
    float c,
    float d,
    float t
) {
    float x2 = 3.0 * (axis.b - axis.a);
    float x3 = axis.a - 2.0 * axis.b + c;
    return float2(
        2.0 * x2 + 6.0 * x3 * t,
        6.0 * d * t
    );
}

CubicClosestPoint closest_figma_cubic(
    float2 sample_position,
    FigmaAxisParams axis,
    float c,
    float d
) {
    float y_seed = pow(
        clamp(sample_position.y / max(d, 0.00001), 0.0, 1.0),
        1.0 / 3.0
    );
    float2 chord = figma_cubic_point(axis, c, d, 1.0);
    float chord_seed = clamp(
        dot(sample_position, chord) /
            max(dot(chord, chord), FIGMA_EPSILON),
        0.0,
        1.0
    );
    float2 y_delta =
        figma_cubic_point(axis, c, d, y_seed) - sample_position;
    float2 chord_delta =
        figma_cubic_point(axis, c, d, chord_seed) - sample_position;
    float t = dot(chord_delta, chord_delta) < dot(y_delta, y_delta)
        ? chord_seed
        : y_seed;

    [unroll]
    for (int iteration = 0; iteration < 4; iteration++) {
        float2 curve_point = figma_cubic_point(axis, c, d, t);
        float2 tangent = figma_cubic_derivative(axis, c, d, t);
        float2 second_derivative = figma_cubic_second_derivative(axis, c, d, t);
        float2 delta = curve_point - sample_position;
        float denominator =
            dot(tangent, tangent) + dot(delta, second_derivative);
        if (abs(denominator) > FIGMA_EPSILON) {
            t = clamp(t - dot(delta, tangent) / denominator, 0.0, 1.0);
        }
    }

    float closest_t = t;
    float2 closest_point = figma_cubic_point(axis, c, d, t);
    float closest_distance = length(sample_position - closest_point);

    float2 start = float2(0.0, 0.0);
    float start_distance = length(sample_position - start);
    if (start_distance < closest_distance) {
        closest_t = 0.0;
        closest_point = start;
        closest_distance = start_distance;
    }

    float2 end = chord;
    float end_distance = length(sample_position - end);
    if (end_distance < closest_distance) {
        closest_t = 1.0;
        closest_point = end;
        closest_distance = end_distance;
    }

    CubicClosestPoint result;
    result.position = closest_point;
    result.tangent = figma_cubic_derivative(axis, c, d, closest_t);
    result.distance = closest_distance;
    result.path_t = closest_t;
    return result;
}

float figma_signed_distance(float2 delta, float2 normal, float distance) {
    return dot(delta, normal) >= 0.0 ? distance : -distance;
}

float figma_cross_2d(float2 a, float2 b) {
    return a.x * b.y - a.y * b.x;
}

float2 figma_unfold_normal(float2 normal, bool mirrored) {
    return mirrored
        ? float2(-normal.y, normal.x)
        : float2(normal.x, -normal.y);
}

SdfSample figma_corner_sdf_impl(
    float2 corner_to_point,
    FigmaCornerParams params
) {
    float2 z = corner_to_point + float2(
        params.horizontal.p,
        params.vertical.p
    );
    SdfSample sample;
    sample.path_t = 0.0;
    sample.segment = FIGMA_SEGMENT_STRAIGHT;

    if (params.radius <= FIGMA_EPSILON || z.x <= 0.0 || z.y <= 0.0) {
        sample.distance = max(corner_to_point.x, corner_to_point.y);
        sample.normal = corner_to_point.x > corner_to_point.y
            ? float2(1.0, 0.0)
            : float2(0.0, 1.0);
        return sample;
    }

    float2 horizontal_point = float2(z.x, params.vertical.p - z.y);
    float2 vertical_point = float2(z.y, params.horizontal.p - z.x);

    float2 circle_center = float2(
        params.horizontal.p - params.radius,
        params.radius
    );
    float2 join = float2(
        params.horizontal.a + params.horizontal.b + params.c,
        params.d
    );
    float2 start_direction = (join - circle_center) / params.radius;
    float2 to_point = horizontal_point - circle_center;
    float to_point_length = length(to_point);
    float2 point_direction = to_point_length > FIGMA_EPSILON
        ? to_point / max(to_point_length, FIGMA_EPSILON)
        : start_direction;
    float arc_angle = clamp(
        atan2(
            figma_cross_2d(start_direction, point_direction),
            dot(start_direction, point_direction)
        ),
        0.0,
        params.arc_sweep
    );
    float arc_sine = sin(arc_angle);
    float arc_cosine = cos(arc_angle);
    float2 arc_normal = float2(
        arc_cosine * start_direction.x - arc_sine * start_direction.y,
        arc_sine * start_direction.x + arc_cosine * start_direction.y
    );
    float2 arc_point = circle_center + params.radius * arc_normal;
    float2 arc_delta = horizontal_point - arc_point;
    float arc_distance = figma_signed_distance(
        arc_delta,
        arc_normal,
        length(arc_delta)
    );
    float arc_t = params.arc_sweep > FIGMA_EPSILON
        ? arc_angle / max(params.arc_sweep, FIGMA_EPSILON)
        : 0.0;

    sample.distance = arc_distance;
    sample.normal = figma_unfold_normal(arc_normal, false);
    sample.path_t = arc_t;
    sample.segment = FIGMA_SEGMENT_ARC;

    if (params.smoothing > FIGMA_EPSILON) {
        // The cubic stays inside its control-point bounds. Skip Newton when
        // that box cannot beat the current arc distance.
        float2 horizontal_bounds_delta = max(
            float2(0.0, 0.0),
            max(-horizontal_point, horizontal_point - join)
        );
        if (dot(horizontal_bounds_delta, horizontal_bounds_delta) <=
                sample.distance * sample.distance * 1.000001 + FIGMA_EPSILON) {
            CubicClosestPoint horizontal_cubic = closest_figma_cubic(
                horizontal_point,
                params.horizontal,
                params.c,
                params.d
            );
            float2 horizontal_normal = normalize(float2(
                horizontal_cubic.tangent.y,
                -horizontal_cubic.tangent.x
            ));
            float horizontal_distance = figma_signed_distance(
                horizontal_point - horizontal_cubic.position,
                horizontal_normal,
                horizontal_cubic.distance
            );
            if (abs(horizontal_distance) <= abs(sample.distance)) {
                sample.distance = horizontal_distance;
                sample.normal = figma_unfold_normal(horizontal_normal, false);
                sample.path_t = horizontal_cubic.path_t;
                sample.segment = FIGMA_SEGMENT_FIRST_CUBIC;
            }
        }

        float2 vertical_join = float2(
            params.vertical.a + params.vertical.b + params.c,
            params.d
        );
        float2 vertical_bounds_delta = max(
            float2(0.0, 0.0),
            max(-vertical_point, vertical_point - vertical_join)
        );
        if (dot(vertical_bounds_delta, vertical_bounds_delta) <=
                sample.distance * sample.distance * 1.000001 + FIGMA_EPSILON) {
            CubicClosestPoint vertical_cubic = closest_figma_cubic(
                vertical_point,
                params.vertical,
                params.c,
                params.d
            );
            float2 vertical_normal = normalize(float2(
                vertical_cubic.tangent.y,
                -vertical_cubic.tangent.x
            ));
            float vertical_distance = figma_signed_distance(
                vertical_point - vertical_cubic.position,
                vertical_normal,
                vertical_cubic.distance
            );
            if (abs(vertical_distance) <= abs(sample.distance)) {
                sample.distance = vertical_distance;
                sample.normal = figma_unfold_normal(vertical_normal, true);
                sample.path_t = vertical_cubic.path_t;
                sample.segment = FIGMA_SEGMENT_SECOND_CUBIC;
            }
        }
    }
    return sample;
}

float figma_cubic_length(
    FigmaCornerParams params,
    FigmaAxisParams axis,
    float end_t
) {
    float half_t = clamp(end_t, 0.0, 1.0) / 2.0;
    if (half_t <= 0.0 || params.smoothing <= 0.0) {
        return 0.0;
    }

    // Five-point Gauss-Legendre quadrature keeps dash phase stable without a
    // data-dependent loop.
    float center = half_t;
    float offset1 = half_t * 0.5384693101;
    float offset2 = half_t * 0.9061798459;
    float speed0 = length(figma_cubic_derivative(
        axis, params.c, params.d, center
    ));
    float speed1 = length(figma_cubic_derivative(
        axis, params.c, params.d, center - offset1
    )) + length(figma_cubic_derivative(
        axis, params.c, params.d, center + offset1
    ));
    float speed2 = length(figma_cubic_derivative(
        axis, params.c, params.d, center - offset2
    )) + length(figma_cubic_derivative(
        axis, params.c, params.d, center + offset2
    ));
    return half_t * (
        0.5688888889 * speed0 +
        0.4786286705 * speed1 +
        0.2369268851 * speed2
    );
}

float figma_corner_length(FigmaCornerParams params) {
    return figma_cubic_length(params, params.horizontal, 1.0) +
        params.radius * params.arc_sweep +
        figma_cubic_length(params, params.vertical, 1.0);
}

float figma_corner_progress(
    FigmaCornerParams params,
    SdfSample sample,
    float total_length
) {
    if (sample.segment == FIGMA_SEGMENT_FIRST_CUBIC) {
        return figma_cubic_length(
            params, params.horizontal, sample.path_t
        );
    }
    if (sample.segment == FIGMA_SEGMENT_ARC) {
        return figma_cubic_length(params, params.horizontal, 1.0) +
            params.radius * params.arc_sweep * sample.path_t;
    }
    if (sample.segment == FIGMA_SEGMENT_SECOND_CUBIC) {
        return total_length - figma_cubic_length(
            params, params.vertical, sample.path_t
        );
    }
    return 0.0;
}

bool figma_is_corner_candidate(
    float2 sample_position,
    float2 size,
    float horizontal_extent,
    float vertical_extent,
    uint corner
) {
    if (horizontal_extent <= FIGMA_EPSILON ||
        vertical_extent <= FIGMA_EPSILON) return false;
    if (corner == 0u) {
        return sample_position.x <= horizontal_extent &&
            sample_position.y <= vertical_extent;
    }
    if (corner == 1u) {
        return size.x - sample_position.x <= horizontal_extent &&
            sample_position.y <= vertical_extent;
    }
    if (corner == 2u) {
        return size.x - sample_position.x <= horizontal_extent &&
            size.y - sample_position.y <= vertical_extent;
    }
    return sample_position.x <= horizontal_extent &&
        size.y - sample_position.y <= vertical_extent;
}

bool figma_has_corner_candidate(
    float2 sample_position,
    float2 size,
    float4 horizontal_extents,
    float4 vertical_extents
) {
    return figma_is_corner_candidate(
        sample_position, size, horizontal_extents.x, vertical_extents.x, 0u
    ) || figma_is_corner_candidate(
        sample_position, size, horizontal_extents.y, vertical_extents.y, 1u
    ) || figma_is_corner_candidate(
        sample_position, size, horizontal_extents.z, vertical_extents.z, 2u
    ) || figma_is_corner_candidate(
        sample_position, size, horizontal_extents.w, vertical_extents.w, 3u
    );
}

float2 figma_corner_to_point(
    float2 sample_position,
    float2 size,
    uint corner
) {
    if (corner == 0u) return -sample_position;
    if (corner == 1u) {
        return float2(sample_position.x - size.x, -sample_position.y);
    }
    if (corner == 2u) return sample_position - size;
    return float2(-sample_position.x, sample_position.y - size.y);
}

float2 figma_orient_corner_normal(float2 normal, uint corner) {
    if (corner == 0u) return -normal;
    if (corner == 1u) return float2(normal.x, -normal.y);
    if (corner == 2u) return normal;
    return float2(-normal.x, normal.y);
}

SdfSample figma_nearest_straight_sample(
    float2 sample_position,
    float2 size,
    float4 horizontal_extents,
    float4 vertical_extents
) {
    float2 nearest_delta = sample_position - float2(
        clamp(
            sample_position.x,
            horizontal_extents.x,
            size.x - horizontal_extents.y
        ),
        0.0
    );
    float2 nearest_normal = float2(0.0, -1.0);
    float nearest_distance_squared = dot(nearest_delta, nearest_delta);

    float2 candidate_delta = sample_position - float2(
        size.x,
        clamp(
            sample_position.y,
            vertical_extents.y,
            size.y - vertical_extents.z
        )
    );
    float candidate_distance_squared = dot(candidate_delta, candidate_delta);
    if (candidate_distance_squared < nearest_distance_squared) {
        nearest_delta = candidate_delta;
        nearest_normal = float2(1.0, 0.0);
        nearest_distance_squared = candidate_distance_squared;
    }

    candidate_delta = sample_position - float2(
        clamp(
            sample_position.x,
            horizontal_extents.w,
            size.x - horizontal_extents.z
        ),
        size.y
    );
    candidate_distance_squared = dot(candidate_delta, candidate_delta);
    if (candidate_distance_squared < nearest_distance_squared) {
        nearest_delta = candidate_delta;
        nearest_normal = float2(0.0, 1.0);
        nearest_distance_squared = candidate_distance_squared;
    }

    candidate_delta = sample_position - float2(
        0.0,
        clamp(
            sample_position.y,
            vertical_extents.x,
            size.y - vertical_extents.w
        )
    );
    candidate_distance_squared = dot(candidate_delta, candidate_delta);
    if (candidate_distance_squared < nearest_distance_squared) {
        nearest_delta = candidate_delta;
        nearest_normal = float2(-1.0, 0.0);
        nearest_distance_squared = candidate_distance_squared;
    }

    SdfSample sample;
    sample.distance = figma_signed_distance(
        nearest_delta,
        nearest_normal,
        sqrt(nearest_distance_squared)
    );
    sample.normal = nearest_normal;
    sample.path_t = 0.0;
    sample.segment = FIGMA_SEGMENT_STRAIGHT;
    return sample;
}

FigmaRectSample figma_smooth_rect_sdf_sample(
    float2 sample_position,
    Bounds bounds,
    Corners corner_radii,
    float4 horizontal_reaches,
    float4 vertical_reaches,
    float4 smoothing_factors
) {
    float2 local_point = sample_position - bounds.origin;
    float4 radii = corner_values(corner_radii);
    FigmaRectSample result;
    result.corner = FIGMA_NO_CORNER;

    if (!figma_has_corner_candidate(
        local_point,
        bounds.size,
        horizontal_reaches,
        vertical_reaches
    )) {
        result.sdf = figma_nearest_straight_sample(
            local_point,
            bounds.size,
            horizontal_reaches,
            vertical_reaches
        );
        return result;
    }

    result.sdf = figma_nearest_straight_sample(
        local_point,
        bounds.size,
        horizontal_reaches,
        vertical_reaches
    );

    [unroll]
    for (uint corner = 0u; corner < 4u; corner++) {
        if (figma_is_corner_candidate(
            local_point,
            bounds.size,
            horizontal_reaches[corner],
            vertical_reaches[corner],
            corner
        )) {
            FigmaCornerParams params = figma_corner_params(
                radii[corner],
                horizontal_reaches[corner],
                vertical_reaches[corner],
                smoothing_factors
            );
            if (params.radius > FIGMA_EPSILON) {
                SdfSample candidate = figma_corner_sdf_impl(
                    figma_corner_to_point(local_point, bounds.size, corner),
                    params
                );
                candidate.normal = figma_orient_corner_normal(
                    candidate.normal,
                    corner
                );
                if (abs(candidate.distance) <= abs(result.sdf.distance)) {
                    result.sdf = candidate;
                    result.corner = corner;
                }
            }
        }
    }
    return result;
}

float figma_smooth_rect_sdf(
    float2 sample_position,
    Bounds bounds,
    Corners corner_radii,
    float4 horizontal_reaches,
    float4 vertical_reaches,
    float4 smoothing_factors
) {
    FigmaRectSample sample = figma_smooth_rect_sdf_sample(
        sample_position,
        bounds,
        corner_radii,
        horizontal_reaches,
        vertical_reaches,
        smoothing_factors
    );
    return sample.sdf.distance;
}

float styled_rect_sdf(
    float2 sample_position,
    Bounds bounds,
    Corners corner_radii,
    float4 horizontal_reaches,
    float4 vertical_reaches,
    float4 smoothing_factors
) {
    if (smoothing_factors.x <= 0.0 ||
        all(corner_values(corner_radii) <= float4(0.0, 0.0, 0.0, 0.0))) {
        return quad_sdf(sample_position, bounds, corner_radii);
    }

    return figma_smooth_rect_sdf(
        sample_position,
        bounds,
        corner_radii,
        horizontal_reaches,
        vertical_reaches,
        smoothing_factors
    );
}

float gaussian_sdf_coverage(float distance, float sigma) {
    float normalized = distance / (sqrt(2.0) * sigma);
    return saturate(0.5 - 0.5 * erf(float2(normalized, normalized)).x);
}

GradientColor prepare_gradient_color(uint tag, uint color_space, Hsla solid, LinearColorStop colors[2]) {
    GradientColor output;
    if (tag == 0 || tag == 2 || tag == 3) {
        output.solid = hsla_to_rgba(solid);
    } else if (tag == 1) {
        output.color0 = hsla_to_rgba(colors[0].color);
        output.color1 = hsla_to_rgba(colors[1].color);

        // Prepare color space in vertex for avoid conversion
        // in fragment shader for performance reasons
        if (color_space == 1) {
            // Oklab
            output.color0 = srgb_to_oklab(output.color0);
            output.color1 = srgb_to_oklab(output.color1);
        }
    }

    return output;
}

float2x2 rotate2d(float angle) {
    float s = sin(angle);
    float c = cos(angle);
    return float2x2(c, -s, s, c);
}

float4 gradient_color(Background background,
                      float2 position,
                      Bounds bounds,
                      float4 solid_color, float4 color0, float4 color1) {
    float4 color;

    switch (background.tag) {
        case 0:
            color = solid_color;
            break;
        case 1: {
            // -90 degrees to match the CSS gradient angle.
            float gradient_angle = background.gradient_angle_or_pattern_height;
            float radians = (fmod(gradient_angle, 360.0) - 90.0) * (M_PI_F / 180.0);
            float2 direction = float2(cos(radians), sin(radians));

            // Expand the short side to be the same as the long side
            if (bounds.size.x > bounds.size.y) {
                direction.y *= bounds.size.y / bounds.size.x;
            } else {
                direction.x *=  bounds.size.x / bounds.size.y;
            }

            // Get the t value for the linear gradient with the color stop percentages.
            float2 half_size = bounds.size * 0.5;
            float2 center = bounds.origin + half_size;
            float2 center_to_point = position - center;
            float t = dot(center_to_point, direction) / length(direction);
            // Check the direct to determine the use x or y
            if (abs(direction.x) > abs(direction.y)) {
                t = (t + half_size.x) / bounds.size.x;
            } else {
                t = (t + half_size.y) / bounds.size.y;
            }

            // Adjust t based on the stop percentages
            t = (t - background.colors[0].percentage)
                / (background.colors[1].percentage
                - background.colors[0].percentage);
            t = clamp(t, 0.0, 1.0);

            switch (background.color_space) {
                case 0:
                    color = lerp(color0, color1, t);
                    break;
                case 1: {
                    float4 oklab_color = lerp(color0, color1, t);
                    color = oklab_to_srgb(oklab_color);
                    break;
                }
            }

            // Dither to reduce banding in gradients (especially dark/alpha).
            // Triangular-distributed noise breaks up 8-bit quantization steps.
            // ±2/255 for RGB (enough for dark-on-dark compositing),
            // ±3/255 for alpha (needs more because alpha × dark color = tiny steps).
            {
                float2 seed = position * 0.6180339887; // golden ratio spread
                float r1 = frac(sin(dot(seed, float2(12.9898, 78.233))) * 43758.5453);
                float r2 = frac(sin(dot(seed, float2(39.3460, 11.135))) * 24634.6345);
                float tri = r1 + r2 - 1.0; // triangular PDF, range [-1, +1]
                color.rgb += tri * 2.0 / 255.0;
                color.a   += tri * 3.0 / 255.0;
            }

            break;
        }
        case 2: {
            float gradient_angle_or_pattern_height = background.gradient_angle_or_pattern_height;
            float pattern_width = (gradient_angle_or_pattern_height / 65535.0f) / 255.0f;
            float pattern_interval = fmod(gradient_angle_or_pattern_height, 65535.0f) / 255.0f;
            float pattern_height = pattern_width + pattern_interval;
            float stripe_angle = M_PI_F / 4.0;
            float pattern_period = pattern_height * sin(stripe_angle);
            float2x2 rotation = rotate2d(stripe_angle);
            float2 relative_position = position - bounds.origin;
            float2 rotated_point = mul(relative_position, rotation);
            float pattern = fmod(rotated_point.x, pattern_period);
            float distance = min(pattern, pattern_period - pattern) - pattern_period * (pattern_width / pattern_height) /  2.0f;
            color = solid_color;
            color.a *= saturate(0.5 - distance);
            break;
        }
        case 3: {
            // checkerboard
            float size = background.gradient_angle_or_pattern_height;
            float2 relative_position = position - bounds.origin;

            float x_index = floor(relative_position.x / size);
            float y_index = floor(relative_position.y / size);
            float should_be_colored = (x_index + y_index) % 2.0;

            color = solid_color;
            color.a *= saturate(should_be_colored);
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
    float corner_smoothing;
    uint pad;
};

struct QuadVertexOutput {
    nointerpolation uint quad_id: TEXCOORD0;
    nointerpolation float4 horizontal_corner_reaches: TEXCOORD1;
    nointerpolation float4 vertical_corner_reaches: TEXCOORD2;
    nointerpolation float4 corner_lengths: TEXCOORD3;
    nointerpolation float4 smoothing_factors: TEXCOORD4;
    nointerpolation float superellipse_power: TEXCOORD5;
    float4 position: SV_Position;
    nointerpolation float4 border_color: COLOR0;
    nointerpolation float4 background_solid: COLOR1;
    nointerpolation float4 background_color0: COLOR2;
    nointerpolation float4 background_color1: COLOR3;
    float4 clip_distance: SV_ClipDistance;
};

struct QuadFragmentInput {
    nointerpolation uint quad_id: TEXCOORD0;
    nointerpolation float4 horizontal_corner_reaches: TEXCOORD1;
    nointerpolation float4 vertical_corner_reaches: TEXCOORD2;
    nointerpolation float4 corner_lengths: TEXCOORD3;
    nointerpolation float4 smoothing_factors: TEXCOORD4;
    nointerpolation float superellipse_power: TEXCOORD5;
    float4 position: SV_Position;
    nointerpolation float4 border_color: COLOR0;
    nointerpolation float4 background_solid: COLOR1;
    nointerpolation float4 background_color0: COLOR2;
    nointerpolation float4 background_color1: COLOR3;
};

StructuredBuffer<Quad> quads: register(t1);

QuadVertexOutput quad_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    uint quad_id = batch_start_index + instance_id;
    Quad quad = quads[quad_id];
    float4 device_position = to_device_position(unit_vertex, quad.bounds);

    GradientColor gradient = prepare_gradient_color(
        quad.background.tag,
        quad.background.color_space,
        quad.background.solid,
        quad.background.colors
    );
    float4 clip_distance = distance_from_clip_rect(unit_vertex, quad.bounds, quad.content_mask);
    float4 border_color = hsla_to_rgba(quad.border_color);

    QuadVertexOutput output;
    output.position = device_position;
    output.border_color = border_color;
    output.quad_id = quad_id;
    output.horizontal_corner_reaches = corner_values(quad.corner_radii);
    output.vertical_corner_reaches = corner_values(quad.corner_radii);
    output.smoothing_factors = float4(0.0, 1.0, 0.0, 0.0);
    output.superellipse_power = 0.0;
    if (quad.corner_smoothing > 0.0) {
        bool has_no_border =
            quad.border_widths.top == 0.0 &&
            quad.border_widths.right == 0.0 &&
            quad.border_widths.bottom == 0.0 &&
            quad.border_widths.left == 0.0;
        if (quad.border_style == 0u && has_no_border &&
            can_use_normalized_superellipse(
                quad.bounds.size,
                quad.corner_radii,
                quad.corner_smoothing
            )) {
            float4 reaches = normalized_superellipse_reaches(
                quad.corner_radii,
                quad.corner_smoothing
            );
            output.horizontal_corner_reaches = reaches;
            output.vertical_corner_reaches = reaches;
            output.superellipse_power =
                normalized_superellipse_power(quad.corner_smoothing);
        } else {
            output.smoothing_factors = figma_smoothing_factors(quad.corner_smoothing);
            FigmaCornerLayout corner_layout = figma_corner_layout(
                quad.bounds.size,
                quad.corner_radii
            );
            FigmaCornerExtents corner_extents = figma_corner_extents(
                quad.corner_radii,
                corner_layout.horizontal_budgets,
                corner_layout.vertical_budgets,
                quad.corner_smoothing
            );
            output.horizontal_corner_reaches = corner_extents.horizontal;
            output.vertical_corner_reaches = corner_extents.vertical;
        }
    }
    output.corner_lengths = corner_values(quad.corner_radii) * (M_PI_F / 2.0);
    if (quad.border_style == 1u && quad.corner_smoothing > 0.0) {
        float4 radii = corner_values(quad.corner_radii);
        [unroll]
        for (uint corner = 0u; corner < 4u; corner++) {
            FigmaCornerParams params = figma_corner_params(
                radii[corner],
                output.horizontal_corner_reaches[corner],
                output.vertical_corner_reaches[corner],
                output.smoothing_factors
            );
            output.corner_lengths[corner] = figma_corner_length(params);
        }
    }
    output.background_solid = gradient.solid;
    output.background_color0 = gradient.color0;
    output.background_color1 = gradient.color1;
    output.clip_distance = clip_distance;
    return output;
}

float4 quad_fragment(QuadFragmentInput input): SV_Target {
    Quad quad = quads[input.quad_id];
    float4 background_color = gradient_color(quad.background, input.position.xy, quad.bounds,
    input.background_solid, input.background_color0, input.background_color1);

    bool unrounded = quad.corner_radii.top_left == 0.0 &&
        quad.corner_radii.top_right == 0.0 &&
        quad.corner_radii.bottom_left == 0.0 &&
        quad.corner_radii.bottom_right == 0.0;

    // Fast path when the quad is not rounded and doesn't have any border
    if (quad.border_widths.top == 0.0 &&
        quad.border_widths.left == 0.0 &&
        quad.border_widths.right == 0.0 &&
        quad.border_widths.bottom == 0.0 &&
        unrounded) {
        return background_color;
    }

    float2 size = quad.bounds.size;
    float2 half_size = size / 2.;
    float2 the_point = input.position.xy - quad.bounds.origin;
    float2 center_to_point = the_point - half_size;

    // Signed distance field threshold for inclusion of pixels. 0.5 is the
    // minimum distance between the center of the pixel and the edge.
    const float antialias_threshold = 0.5;

    // Circular corners keep the existing center-quadrant fast path. Smoothed
    // corners use edge ownership because a Figma shoulder can cross a half-side.
    float corner_radius = pick_corner_radius(center_to_point, quad.corner_radii);
    float4 horizontal_corner_extents = corner_values(quad.corner_radii);
    float4 vertical_corner_extents = corner_values(quad.corner_radii);
    uint smooth_corner = FIGMA_NO_CORNER;
    bool has_smooth_corner_candidate = false;
    if (input.superellipse_power > 0.0) {
        float corner_reach = corner_radius *
            (1.0 + clamp(quad.corner_smoothing, 0.0, 1.0));
        float2 corner_offset = abs(center_to_point) - half_size + corner_reach;
        has_smooth_corner_candidate =
            corner_offset.x >= 0.0 && corner_offset.y >= 0.0;
    } else if (quad.corner_smoothing > 0.0) {
        horizontal_corner_extents = input.horizontal_corner_reaches;
        vertical_corner_extents = input.vertical_corner_reaches;
        has_smooth_corner_candidate = figma_has_corner_candidate(
            the_point,
            size,
            horizontal_corner_extents,
            vertical_corner_extents
        );
    }

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

    // Vector from the point to the center of the circular corner, also mirrored
    // into the bottom right quadrant.
    float2 corner_center_to_point = corner_to_point + corner_radius;

    // Whether the nearest point on the border is rounded
    bool is_near_rounded_corner = quad.corner_smoothing > 0.0
        ? has_smooth_corner_candidate
        : corner_center_to_point.x >= 0.0 && corner_center_to_point.y >= 0.0;

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
        return background_color;
    }

    // Signed distance of the point to the outside edge of the quad's border
    float outer_sdf;
    SdfSample figma_sample;
    if (input.superellipse_power > 0.0) {
        outer_sdf = normalized_superellipse_sdf_impl(
            corner_to_point,
            corner_radius,
            quad.corner_smoothing,
            input.superellipse_power
        );
        float corner_reach = corner_radius *
            (1.0 + clamp(quad.corner_smoothing, 0.0, 1.0));
        is_near_rounded_corner =
            corner_to_point.x + corner_reach > 0.0 &&
            corner_to_point.y + corner_reach > 0.0;
    } else if (quad.corner_smoothing > 0.0) {
        FigmaRectSample rect_sample = figma_smooth_rect_sdf_sample(
            input.position.xy,
            quad.bounds,
            quad.corner_radii,
            input.horizontal_corner_reaches,
            input.vertical_corner_reaches,
            input.smoothing_factors
        );
        figma_sample = rect_sample.sdf;
        smooth_corner = rect_sample.corner;
        outer_sdf = figma_sample.distance;
        is_near_rounded_corner = figma_sample.segment != FIGMA_SEGMENT_STRAIGHT;
        if (smooth_corner != FIGMA_NO_CORNER) {
            if (smooth_corner == 0u) {
                border = float2(quad.border_widths.left, quad.border_widths.top);
            } else if (smooth_corner == 1u) {
                border = float2(quad.border_widths.right, quad.border_widths.top);
            } else if (smooth_corner == 2u) {
                border = float2(quad.border_widths.right, quad.border_widths.bottom);
            } else {
                border = float2(quad.border_widths.left, quad.border_widths.bottom);
            }
            reduced_border = float2(
                border.x == 0.0 ? -antialias_threshold : border.x,
                border.y == 0.0 ? -antialias_threshold : border.y
            );
            straight_border_inner_corner_to_point =
                corner_to_point + reduced_border;
        }
    } else {
        outer_sdf = quad_sdf_impl(corner_center_to_point, corner_radius);
    }

    // Approximate signed distance of the point to the inside edge of the quad's
    // border. It is negative outside this edge (within the border), and
    // positive inside.
    //
    // This is not always an accurate signed distance:
    // * The rounded portions with varying border width use an approximation of
    //   nearest-point-on-ellipse.
    // * When it is quickly known to be outside the edge, -1.0 is used.
    float inner_sdf = 0.0;
    if (quad.corner_smoothing > 0.0) {
        if (!is_near_rounded_corner) {
            // Keep exact rectangular offsets on straight segments, including
            // sharp zero-radius corners with unequal adjacent border widths.
            inner_sdf = -max(
                straight_border_inner_corner_to_point.x,
                straight_border_inner_corner_to_point.y
            );
        } else if (input.superellipse_power > 0.0) {
            inner_sdf = -(outer_sdf + reduced_border.x);
        } else {
            float effective_border_width = reduced_border.x;
            if (border.x != border.y) {
                float2 normal = abs(figma_sample.normal);
                float2 active_sides = float2(
                    border.x > 0.0 ? 1.0 : 0.0,
                    border.y > 0.0 ? 1.0 : 0.0
                );
                effective_border_width = length(border * normal) -
                    antialias_threshold * (1.0 - length(active_sides * normal));
            }
            inner_sdf = -(outer_sdf + effective_border_width);
        }
    } else if (corner_center_to_point.x <= 0.0 || corner_center_to_point.y <= 0.0) {
        // Fast paths for straight borders
        inner_sdf = -max(straight_border_inner_corner_to_point.x,
                        straight_border_inner_corner_to_point.y);
    } else if (is_beyond_inner_straight_border) {
        // Fast path for points that must be outside the inner edge
        inner_sdf = -1.0;
    } else if (reduced_border.x == reduced_border.y) {
        // Fast path for a uniform-width inner edge.
        inner_sdf = -(outer_sdf + reduced_border.x);
    } else {
        float2 ellipse_radii = max(
            float2(0.0, 0.0),
            float2(corner_radius, corner_radius) - reduced_border
        );
        inner_sdf = quarter_ellipse_sdf(corner_center_to_point, ellipse_radii);
    }

    // Negative when inside the border
    float border_sdf = max(inner_sdf, outer_sdf);

    float4 color = background_color;
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
                bool is_horizontal =
                    straight_border_inner_corner_to_point.x <
                    straight_border_inner_corner_to_point.y;
                float border_width = is_horizontal ? border.y : border.x;
                if (border_width <= 0.0) {
                    is_horizontal = !is_horizontal;
                    border_width = is_horizontal ? border.y : border.x;
                }
                if (border_width > 0.0) {
                    dash_velocity = dv_numerator / border_width;
                    t = is_horizontal ? the_point.x : the_point.y;
                    t *= dash_velocity;
                    max_t = is_horizontal ? size.x : size.y;
                    max_t *= dash_velocity;
                }
            } else {
                // When corners are rounded, the dashes are laid out clockwise
                // around the whole perimeter.

                float h_tl = quad.corner_smoothing > 0.0
                    ? horizontal_corner_extents.x
                    : quad.corner_radii.top_left;
                float h_tr = quad.corner_smoothing > 0.0
                    ? horizontal_corner_extents.y
                    : quad.corner_radii.top_right;
                float h_br = quad.corner_smoothing > 0.0
                    ? horizontal_corner_extents.z
                    : quad.corner_radii.bottom_right;
                float h_bl = quad.corner_smoothing > 0.0
                    ? horizontal_corner_extents.w
                    : quad.corner_radii.bottom_left;
                float v_tl = quad.corner_smoothing > 0.0
                    ? vertical_corner_extents.x
                    : quad.corner_radii.top_left;
                float v_tr = quad.corner_smoothing > 0.0
                    ? vertical_corner_extents.y
                    : quad.corner_radii.top_right;
                float v_br = quad.corner_smoothing > 0.0
                    ? vertical_corner_extents.z
                    : quad.corner_radii.bottom_right;
                float v_bl = quad.corner_smoothing > 0.0
                    ? vertical_corner_extents.w
                    : quad.corner_radii.bottom_left;

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
                float s_t = (size.x - h_tl - h_tr) * dv_t;
                float s_r = (size.y - v_tr - v_br) * dv_r;
                float s_b = (size.x - h_br - h_bl) * dv_b;
                float s_l = (size.y - v_bl - v_tl) * dv_l;

                float corner_dash_velocity_tr = corner_dash_velocity(dv_t, dv_r);
                float corner_dash_velocity_br = corner_dash_velocity(dv_b, dv_r);
                float corner_dash_velocity_bl = corner_dash_velocity(dv_b, dv_l);
                float corner_dash_velocity_tl = corner_dash_velocity(dv_t, dv_l);

                float length_tl = input.corner_lengths.x;
                float length_tr = input.corner_lengths.y;
                float length_br = input.corner_lengths.z;
                float length_bl = input.corner_lengths.w;
                float c_tr = length_tr * corner_dash_velocity_tr;
                float c_br = length_br * corner_dash_velocity_br;
                float c_bl = length_bl * corner_dash_velocity_bl;
                float c_tl = length_tl * corner_dash_velocity_tl;

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
                    if (quad.corner_smoothing > 0.0) {
                        float selected_radius = quad.corner_radii.top_left;
                        float selected_horizontal_reach = input.horizontal_corner_reaches.x;
                        float selected_vertical_reach = input.vertical_corner_reaches.x;
                        float selected_length = length_tl;
                        if (smooth_corner == 1u) {
                            selected_radius = quad.corner_radii.top_right;
                            selected_horizontal_reach = input.horizontal_corner_reaches.y;
                            selected_vertical_reach = input.vertical_corner_reaches.y;
                            selected_length = length_tr;
                        } else if (smooth_corner == 2u) {
                            selected_radius = quad.corner_radii.bottom_right;
                            selected_horizontal_reach = input.horizontal_corner_reaches.z;
                            selected_vertical_reach = input.vertical_corner_reaches.z;
                            selected_length = length_br;
                        } else if (smooth_corner == 3u) {
                            selected_radius = quad.corner_radii.bottom_left;
                            selected_horizontal_reach = input.horizontal_corner_reaches.w;
                            selected_vertical_reach = input.vertical_corner_reaches.w;
                            selected_length = length_bl;
                        }

                        FigmaCornerParams selected_params = figma_corner_params(
                            selected_radius,
                            selected_horizontal_reach,
                            selected_vertical_reach,
                            input.smoothing_factors
                        );
                        float corner_progress = figma_corner_progress(
                            selected_params,
                            figma_sample,
                            selected_length
                        );
                        if (smooth_corner == 0u) {
                            dash_velocity = corner_dash_velocity_tl;
                            t = upto_tl +
                                (selected_length - corner_progress) * dash_velocity;
                        } else if (smooth_corner == 1u) {
                            dash_velocity = corner_dash_velocity_tr;
                            t = upto_tr + corner_progress * dash_velocity;
                        } else if (smooth_corner == 2u) {
                            dash_velocity = corner_dash_velocity_br;
                            t = upto_br +
                                (selected_length - corner_progress) * dash_velocity;
                        } else {
                            dash_velocity = corner_dash_velocity_bl;
                            t = upto_bl + corner_progress * dash_velocity;
                        }
                    } else {
                        float radians = atan2(
                            corner_center_to_point.y,
                            corner_center_to_point.x
                        );
                        float corner_t = radians * corner_radius;

                        if (center_to_point.x >= 0.0) {
                            if (center_to_point.y < 0.0) {
                                dash_velocity = corner_dash_velocity_tr;
                                t = upto_r - corner_t * dash_velocity;
                            } else {
                                dash_velocity = corner_dash_velocity_br;
                                t = upto_br + corner_t * dash_velocity;
                            }
                        } else if (center_to_point.y >= 0.0) {
                            dash_velocity = corner_dash_velocity_bl;
                            t = upto_l - corner_t * dash_velocity;
                        } else {
                            dash_velocity = corner_dash_velocity_tl;
                            t = upto_tl + corner_t * dash_velocity;
                        }
                    }
                } else {
                    // Straight borders
                    bool is_horizontal =
                        straight_border_inner_corner_to_point.x <
                        straight_border_inner_corner_to_point.y;
                    float straight_width = is_horizontal ? border.y : border.x;
                    if (straight_width <= 0.0) {
                        is_horizontal = !is_horizontal;
                    }
                    if (is_horizontal) {
                        if (center_to_point.y < 0.0) {
                            dash_velocity = dv_t;
                            t = (the_point.x - h_tl) * dash_velocity;
                        } else {
                            dash_velocity = dv_b;
                            t = upto_bl - (the_point.x - h_bl) * dash_velocity;
                        }
                    } else if (center_to_point.x < 0.0) {
                        dash_velocity = dv_l;
                        t = upto_tl - (the_point.y - v_tl) * dash_velocity;
                    } else {
                        dash_velocity = dv_r;
                        t = upto_r + (the_point.y - v_tr) * dash_velocity;
                    }
                }
            }
            float dash_length = dash_length_per_width / dash_period_per_width;

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
        float4 blended_border = over(background_color, border_color);
        color = lerp(background_color, blended_border,
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
    float corner_smoothing;
};

struct ShadowVertexOutput {
    nointerpolation uint shadow_id: TEXCOORD0;
    nointerpolation float4 horizontal_corner_reaches: TEXCOORD1;
    nointerpolation float4 vertical_corner_reaches: TEXCOORD2;
    nointerpolation float4 element_horizontal_corner_reaches: TEXCOORD3;
    nointerpolation float4 element_vertical_corner_reaches: TEXCOORD4;
    nointerpolation float4 smoothing_factors: TEXCOORD5;
    float4 position: SV_Position;
    nointerpolation float4 color: COLOR;
    float4 clip_distance: SV_ClipDistance;
};

struct ShadowFragmentInput {
  nointerpolation uint shadow_id: TEXCOORD0;
  nointerpolation float4 horizontal_corner_reaches: TEXCOORD1;
  nointerpolation float4 vertical_corner_reaches: TEXCOORD2;
  nointerpolation float4 element_horizontal_corner_reaches: TEXCOORD3;
  nointerpolation float4 element_vertical_corner_reaches: TEXCOORD4;
  nointerpolation float4 smoothing_factors: TEXCOORD5;
  float4 position: SV_Position;
  nointerpolation float4 color: COLOR;
};

StructuredBuffer<Shadow> shadows: register(t1);

ShadowVertexOutput shadow_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
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

    float4 device_position = to_device_position(unit_vertex, bounds);
    float4 clip_distance = distance_from_clip_rect(unit_vertex, bounds, shadow.content_mask);
    float4 color = hsla_to_rgba(shadow.color);

    ShadowVertexOutput output;
    output.position = device_position;
    output.color = color;
    output.shadow_id = shadow_id;
    output.clip_distance = clip_distance;
    output.horizontal_corner_reaches = corner_values(shadow.corner_radii);
    output.vertical_corner_reaches = corner_values(shadow.corner_radii);
    output.element_horizontal_corner_reaches = corner_values(shadow.element_corner_radii);
    output.element_vertical_corner_reaches = corner_values(shadow.element_corner_radii);
    output.smoothing_factors = float4(0.0, 1.0, 0.0, 0.0);
    if (shadow.corner_smoothing > 0.0) {
        output.smoothing_factors = figma_smoothing_factors(shadow.corner_smoothing);
        FigmaCornerLayout layout = figma_corner_layout(
            shadow.bounds.size,
            shadow.corner_radii
        );
        FigmaCornerExtents extents = figma_corner_extents(
            shadow.corner_radii,
            layout.horizontal_budgets,
            layout.vertical_budgets,
            shadow.corner_smoothing
        );
        output.horizontal_corner_reaches = extents.horizontal;
        output.vertical_corner_reaches = extents.vertical;

        FigmaCornerLayout element_layout = figma_corner_layout(
            shadow.element_bounds.size,
            shadow.element_corner_radii
        );
        FigmaCornerExtents element_extents = figma_corner_extents(
            shadow.element_corner_radii,
            element_layout.horizontal_budgets,
            element_layout.vertical_budgets,
            shadow.corner_smoothing
        );
        output.element_horizontal_corner_reaches = element_extents.horizontal;
        output.element_vertical_corner_reaches = element_extents.vertical;
    }

    return output;
}

float4 shadow_fragment(ShadowFragmentInput input): SV_TARGET {
    Shadow shadow = shadows[input.shadow_id];

    float2 half_size = shadow.bounds.size / 2.;
    float2 center = shadow.bounds.origin + half_size;
    float2 point0 = input.position.xy - center;
    float corner_radius = pick_corner_radius(point0, shadow.corner_radii);
    bool has_rounded_corners =
        any(corner_values(shadow.corner_radii) > float4(0.0, 0.0, 0.0, 0.0));

    float alpha;
    if (shadow.blur_radius == 0.) {
        float distance = styled_rect_sdf(
            input.position.xy,
            shadow.bounds,
            shadow.corner_radii,
            input.horizontal_corner_reaches,
            input.vertical_corner_reaches,
            input.smoothing_factors
        );
        alpha = saturate(0.5 - distance);
    } else if (has_rounded_corners) {
        float blur_limit = 3.0 * shadow.blur_radius;
        float2 box_delta = abs(point0) - half_size;
        float2 outside_delta = max(box_delta, float2(0.0, 0.0));
        if (dot(outside_delta, outside_delta) > blur_limit * blur_limit) {
            alpha = 0.0;
        } else {
            float2 local_point = input.position.xy - shadow.bounds.origin;
            float edge_depth = min(
                min(local_point.x, shadow.bounds.size.x - local_point.x),
                min(local_point.y, shadow.bounds.size.y - local_point.y)
            );
            bool is_corner_candidate = figma_has_corner_candidate(
                local_point,
                shadow.bounds.size,
                input.horizontal_corner_reaches,
                input.vertical_corner_reaches
            );
            if (!is_corner_candidate && edge_depth >= blur_limit) {
                alpha = 1.0;
            } else {
                float distance = styled_rect_sdf(
                    input.position.xy,
                    shadow.bounds,
                    shadow.corner_radii,
                    input.horizontal_corner_reaches,
                    input.vertical_corner_reaches,
                    input.smoothing_factors
                );
                alpha = gaussian_sdf_coverage(distance, shadow.blur_radius);
            }
        }
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
        float element_distance = styled_rect_sdf(
            input.position.xy,
            shadow.element_bounds,
            shadow.element_corner_radii,
            input.element_horizontal_corner_reaches,
            input.element_vertical_corner_reaches,
            input.smoothing_factors
        );
        alpha *= saturate(0.5 - element_distance);
    }

    return input.color * float4(1., 1., 1., alpha);
}

/*
**
**              Path Rasterization
**
*/

struct PathRasterizationSprite {
    float2 xy_position;
    float2 st_position;
    Background color;
    Bounds bounds;
};

StructuredBuffer<PathRasterizationSprite> path_rasterization_sprites: register(t1);

struct PathVertexOutput {
    float4 position: SV_Position;
    float2 st_position: TEXCOORD0;
    nointerpolation uint vertex_id: TEXCOORD1;
    float4 clip_distance: SV_ClipDistance;
};

struct PathFragmentInput {
    float4 position: SV_Position;
    float2 st_position: TEXCOORD0;
    nointerpolation uint vertex_id: TEXCOORD1;
};

PathVertexOutput path_rasterization_vertex(uint vertex_id: SV_VertexID) {
    PathRasterizationSprite sprite = path_rasterization_sprites[vertex_id];

    PathVertexOutput output;
    output.position = to_device_position_impl(sprite.xy_position);
    output.st_position = sprite.st_position;
    output.vertex_id = vertex_id;
    output.clip_distance = distance_from_clip_rect_impl(sprite.xy_position, sprite.bounds);

    return output;
}

float4 path_rasterization_fragment(PathFragmentInput input): SV_Target {
    float2 dx = ddx(input.st_position);
    float2 dy = ddy(input.st_position);
    PathRasterizationSprite sprite = path_rasterization_sprites[input.vertex_id];

    Background background = sprite.color;
    Bounds bounds = sprite.bounds;

    float alpha;
    if (length(float2(dx.x, dy.x))) {
        alpha = 1.0;
    } else {
        float2 gradient = 2.0 * input.st_position.xx * float2(dx.x, dy.x) - float2(dx.y, dy.y);
        float f = input.st_position.x * input.st_position.x - input.st_position.y;
        float distance = f / length(gradient);
        alpha = saturate(0.5 - distance);
    }

    GradientColor gradient = prepare_gradient_color(
        background.tag, background.color_space, background.solid, background.colors);

    float4 color = gradient_color(background, input.position.xy, bounds,
        gradient.solid, gradient.color0, gradient.color1);
    return float4(color.rgb * color.a * alpha, alpha * color.a);
}

/*
**
**              Path Sprites
**
*/

struct PathSprite {
    Bounds bounds;
};

struct PathSpriteVertexOutput {
    float4 position: SV_Position;
    float2 texture_coords: TEXCOORD0;
};

StructuredBuffer<PathSprite> path_sprites: register(t1);

PathSpriteVertexOutput path_sprite_vertex(uint vertex_id: SV_VertexID, uint sprite_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    PathSprite sprite = path_sprites[sprite_id];

    // Don't apply content mask because it was already accounted for when rasterizing the path
    float4 device_position = to_device_position(unit_vertex, sprite.bounds);

    float2 screen_position = sprite.bounds.origin + unit_vertex * sprite.bounds.size;
    float2 texture_coords = screen_position / global_viewport_size;

    PathSpriteVertexOutput output;
    output.position = device_position;
    output.texture_coords = texture_coords;
    return output;
}

float4 path_sprite_fragment(PathSpriteVertexOutput input): SV_Target {
    return t_sprite.Sample(s_sprite, input.texture_coords);
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

struct UnderlineVertexOutput {
  nointerpolation uint underline_id: TEXCOORD0;
  float4 position: SV_Position;
  nointerpolation float4 color: COLOR;
  float4 clip_distance: SV_ClipDistance;
};

struct UnderlineFragmentInput {
  nointerpolation uint underline_id: TEXCOORD0;
  float4 position: SV_Position;
  nointerpolation float4 color: COLOR;
};

StructuredBuffer<Underline> underlines: register(t1);

UnderlineVertexOutput underline_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    uint underline_id = batch_start_index + instance_id;
    Underline underline = underlines[underline_id];
    float4 device_position = to_device_position(unit_vertex, underline.bounds);
    float4 clip_distance = distance_from_clip_rect(unit_vertex, underline.bounds,
                                                    underline.content_mask);
    float4 color = hsla_to_rgba(underline.color);

    UnderlineVertexOutput output;
    output.position = device_position;
    output.color = color;
    output.underline_id = underline_id;
    output.clip_distance = clip_distance;
    return output;
}

float4 underline_fragment(UnderlineFragmentInput input): SV_Target {
    const float WAVE_FREQUENCY = 2.0;
    const float WAVE_HEIGHT_RATIO = 0.8;

    Underline underline = underlines[input.underline_id];
    if (underline.wavy) {
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
    } else {
        return input.color;
    }
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

struct MonochromeSpriteVertexOutput {
    float4 position: SV_Position;
    float2 tile_position: POSITION;
    nointerpolation float4 color: COLOR;
    float4 clip_distance: SV_ClipDistance;
};

struct MonochromeSpriteFragmentInput {
    float4 position: SV_Position;
    float2 tile_position: POSITION;
    nointerpolation float4 color: COLOR;
    float4 clip_distance: SV_ClipDistance;
};

StructuredBuffer<MonochromeSprite> mono_sprites: register(t1);

MonochromeSpriteVertexOutput monochrome_sprite_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    uint sprite_id = batch_start_index + instance_id;
    MonochromeSprite sprite = mono_sprites[sprite_id];
    float4 device_position =
        to_device_position_transformed(unit_vertex, sprite.bounds, sprite.transformation);
    float4 clip_distance = distance_from_clip_rect_transformed(unit_vertex, sprite.bounds, sprite.content_mask, sprite.transformation);
    float2 tile_position = to_tile_position(unit_vertex, sprite.tile);
    float4 color = hsla_to_rgba(sprite.color);

    MonochromeSpriteVertexOutput output;
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

MonochromeSpriteVertexOutput subpixel_sprite_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
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
    uint grayscale;
    float opacity;
    float corner_smoothing;
    Bounds bounds;
    Bounds content_mask;
    Corners corner_radii;
    AtlasTile tile;
};

struct PolychromeSpriteVertexOutput {
    nointerpolation uint sprite_id: TEXCOORD0;
    nointerpolation float4 horizontal_corner_reaches: TEXCOORD1;
    nointerpolation float4 vertical_corner_reaches: TEXCOORD2;
    nointerpolation float4 smoothing_factors: TEXCOORD3;
    nointerpolation float superellipse_power: TEXCOORD4;
    float4 position: SV_Position;
    float2 tile_position: POSITION;
    float4 clip_distance: SV_ClipDistance;
};

struct PolychromeSpriteFragmentInput {
    nointerpolation uint sprite_id: TEXCOORD0;
    nointerpolation float4 horizontal_corner_reaches: TEXCOORD1;
    nointerpolation float4 vertical_corner_reaches: TEXCOORD2;
    nointerpolation float4 smoothing_factors: TEXCOORD3;
    nointerpolation float superellipse_power: TEXCOORD4;
    float4 position: SV_Position;
    float2 tile_position: POSITION;
};

StructuredBuffer<PolychromeSprite> poly_sprites: register(t1);

PolychromeSpriteVertexOutput polychrome_sprite_vertex(uint vertex_id: SV_VertexID, uint instance_id: SV_InstanceID) {
    float2 unit_vertex = float2(float(vertex_id & 1u), 0.5 * float(vertex_id & 2u));
    uint sprite_id = batch_start_index + instance_id;
    PolychromeSprite sprite = poly_sprites[sprite_id];
    float4 device_position = to_device_position(unit_vertex, sprite.bounds);
    float4 clip_distance = distance_from_clip_rect(unit_vertex, sprite.bounds,
                                                    sprite.content_mask);
    float2 tile_position = to_tile_position(unit_vertex, sprite.tile);

    PolychromeSpriteVertexOutput output;
    output.position = device_position;
    output.tile_position = tile_position;
    output.sprite_id = sprite_id;
    output.horizontal_corner_reaches = corner_values(sprite.corner_radii);
    output.vertical_corner_reaches = corner_values(sprite.corner_radii);
    output.smoothing_factors = float4(0.0, 1.0, 0.0, 0.0);
    output.superellipse_power = 0.0;
    if (sprite.corner_smoothing > 0.0) {
        if (can_use_normalized_superellipse(
            sprite.bounds.size,
            sprite.corner_radii,
            sprite.corner_smoothing
        )) {
            float4 reaches = normalized_superellipse_reaches(
                sprite.corner_radii,
                sprite.corner_smoothing
            );
            output.horizontal_corner_reaches = reaches;
            output.vertical_corner_reaches = reaches;
            output.superellipse_power =
                normalized_superellipse_power(sprite.corner_smoothing);
        } else {
            output.smoothing_factors = figma_smoothing_factors(sprite.corner_smoothing);
            FigmaCornerLayout corner_layout = figma_corner_layout(
                sprite.bounds.size,
                sprite.corner_radii
            );
            FigmaCornerExtents corner_extents = figma_corner_extents(
                sprite.corner_radii,
                corner_layout.horizontal_budgets,
                corner_layout.vertical_budgets,
                sprite.corner_smoothing
            );
            output.horizontal_corner_reaches = corner_extents.horizontal;
            output.vertical_corner_reaches = corner_extents.vertical;
        }
    }
    output.clip_distance = clip_distance;
    return output;
}

float4 polychrome_sprite_fragment(PolychromeSpriteFragmentInput input): SV_Target {
    PolychromeSprite sprite = poly_sprites[input.sprite_id];
    float4 sample = t_sprite.Sample(s_sprite, input.tile_position);
    float distance;
    if (input.superellipse_power > 0.0) {
        distance = normalized_superellipse_sdf(
            input.position.xy,
            sprite.bounds,
            sprite.corner_radii,
            sprite.corner_smoothing,
            input.superellipse_power
        );
    } else if (sprite.corner_smoothing > 0.0) {
        distance = figma_smooth_rect_sdf(
            input.position.xy,
            sprite.bounds,
            sprite.corner_radii,
            input.horizontal_corner_reaches,
            input.vertical_corner_reaches,
            input.smoothing_factors
        );
    } else {
        distance = quad_sdf(input.position.xy, sprite.bounds, sprite.corner_radii);
    }

    float4 color = sample;
    if (sprite.grayscale != 0u) {
        float3 grayscale = dot(color.rgb, GRAYSCALE_FACTORS);
        color = float4(grayscale, sample.a);
    }
    color.a *= sprite.opacity * saturate(0.5 - distance);
    return color;
}
