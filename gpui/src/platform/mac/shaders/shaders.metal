#include <metal_stdlib>
#include "shaders.h"

using namespace metal;

float4 coloru_to_colorf(uchar4 coloru) {
    return float4(coloru) / float4(0xff, 0xff, 0xff, 0xff);
}

float4 to_device_position(float2 pixel_position, float2 viewport_size) {
    return float4(pixel_position / viewport_size * float2(2., -2.) + float2(-1., 1.), 0., 1.);
}

// A standard gaussian function, used for weighting samples
float gaussian(float x, float sigma) {
    return exp(-(x * x) / (2. * sigma * sigma)) / (sqrt(2. * M_PI_F) * sigma);
}

// This approximates the error function, needed for the gaussian integral
float2 erf(float2 x) {
    float2 s = sign(x);
    float2 a = abs(x);
    x = 1. + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
    x *= x;
    return s - s / (x * x);
}

float blur_along_x(float x, float y, float sigma, float corner, float2 halfSize) {
    float delta = min(halfSize.y - corner - abs(y), 0.);
    float curved = halfSize.x - corner + sqrt(max(0., corner * corner - delta * delta));
    float2 integral = 0.5 + 0.5 * erf((x + float2(-curved, curved)) * (sqrt(0.5) / sigma));
    return integral.y - integral.x;
}

struct QuadFragmentInput {
    float4 position [[position]];
    GPUIQuad quad;
};

vertex QuadFragmentInput quad_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint quad_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUIQuadInputIndexVertices)]],
    constant GPUIQuad *quads [[buffer(GPUIQuadInputIndexQuads)]],
    constant GPUIUniforms *uniforms [[buffer(GPUIQuadInputIndexUniforms)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUIQuad quad = quads[quad_id];
    float2 position = unit_vertex * quad.size + quad.origin;
    float4 device_position = to_device_position(position, uniforms->viewport_size);

    return QuadFragmentInput {
        device_position,
        quad,
    };
}

fragment float4 quad_fragment(
    QuadFragmentInput input [[stage_in]]
) {
    float2 half_size = input.quad.size / 2.;
    float2 center = input.quad.origin + half_size;
    float2 center_to_point = input.position.xy - center;
    float2 edge_to_point = abs(center_to_point) - half_size;
    float2 rounded_edge_to_point = abs(center_to_point) - half_size + input.quad.corner_radius;
    float distance = length(max(0., rounded_edge_to_point)) + min(0., max(rounded_edge_to_point.x, rounded_edge_to_point.y)) - input.quad.corner_radius;

    float border_width = 0.;
    if (edge_to_point.x > edge_to_point.y) {
        border_width = center_to_point.x <= 0. ? input.quad.border_left : input.quad.border_right;
    } else {
        border_width = center_to_point.y <= 0. ? input.quad.border_top : input.quad.border_bottom;
    }

    float4 color;
    if (border_width == 0.) {
        color = coloru_to_colorf(input.quad.background_color);
    } else {
        float inset_distance = distance + border_width;
        color = mix(
            coloru_to_colorf(input.quad.border_color),
            coloru_to_colorf(input.quad.background_color),
            saturate(0.5 - inset_distance)
        );
    }

    float4 coverage = float4(1., 1., 1., saturate(0.5 - distance));
    return coverage * color;
}

struct ShadowFragmentInput {
    float4 position [[position]];
    GPUIShadow shadow;
};

vertex ShadowFragmentInput shadow_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint shadow_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUIShadowInputIndexVertices)]],
    constant GPUIShadow *shadows [[buffer(GPUIShadowInputIndexShadows)]],
    constant GPUIUniforms *uniforms [[buffer(GPUIShadowInputIndexUniforms)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUIShadow shadow = shadows[shadow_id];

    float margin = 3. * shadow.sigma;
    float2 position = unit_vertex * (shadow.size + 2. * margin) + shadow.origin - margin;
    float4 device_position = to_device_position(position, uniforms->viewport_size);

    return ShadowFragmentInput {
        device_position,
        shadow,
    };
}

fragment float4 shadow_fragment(
    ShadowFragmentInput input [[stage_in]]
) {
    float sigma = input.shadow.sigma;
    float corner_radius = input.shadow.corner_radius;
    float2 half_size = input.shadow.size / 2.;
    float2 center = input.shadow.origin + half_size;
    float2 point = input.position.xy - center;

    // The signal is only non-zero in a limited range, so don't waste samples
    float low = point.y - half_size.y;
    float high = point.y + half_size.y;
    float start = clamp(-3. * sigma, low, high);
    float end = clamp(3. * sigma, low, high);

    // Accumulate samples (we can get away with surprisingly few samples)
    float step = (end - start) / 4.;
    float y = start + step * 0.5;
    float alpha = 0.;
    for (int i = 0; i < 4; i++) {
        alpha += blur_along_x(point.x, point.y - y, sigma, corner_radius, half_size) * gaussian(y, sigma) * step;
        y += step;
    }

    return float4(1., 1., 1., alpha) * coloru_to_colorf(input.shadow.color);
}

struct SpriteFragmentInput {
    float4 position [[position]];
    float2 atlas_position;
    float4 color [[flat]];
};

vertex SpriteFragmentInput sprite_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint sprite_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUISpriteVertexInputIndexVertices)]],
    constant GPUISprite *sprites [[buffer(GPUISpriteVertexInputIndexSprites)]],
    constant float2 *viewport_size [[buffer(GPUISpriteVertexInputIndexViewportSize)]],
    constant float2 *atlas_size [[buffer(GPUISpriteVertexInputIndexAtlasSize)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUISprite sprite = sprites[sprite_id];
    float2 position = unit_vertex * sprite.size + sprite.origin;
    float4 device_position = to_device_position(position, *viewport_size);
    float2 atlas_position = (unit_vertex * sprite.size + sprite.atlas_origin) / *atlas_size;

    return SpriteFragmentInput {
        device_position,
        atlas_position,
        coloru_to_colorf(sprite.color),
    };
}

fragment float4 sprite_fragment(
    SpriteFragmentInput input [[stage_in]],
    texture2d<float> atlas [[ texture(GPUISpriteFragmentInputIndexAtlas) ]]
) {
    constexpr sampler atlas_sampler(mag_filter::linear, min_filter::linear);
    float4 color = input.color;
    float4 mask = atlas.sample(atlas_sampler, input.atlas_position);
    color.a *= mask.a;
    return color;
}
