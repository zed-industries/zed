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
    float2 atlas_position; // only used in the image shader
    float2 origin;
    float2 size;
    float4 background_color;
    float border_top;
    float border_right;
    float border_bottom;
    float border_left;
    float4 border_color;
    float corner_radius;
};

float4 quad_sdf(QuadFragmentInput input) {
    float2 half_size = input.size / 2.;
    float2 center = input.origin + half_size;
    float2 center_to_point = input.position.xy - center;
    float2 rounded_edge_to_point = abs(center_to_point) - half_size + input.corner_radius;
    float distance = length(max(0., rounded_edge_to_point)) + min(0., max(rounded_edge_to_point.x, rounded_edge_to_point.y)) - input.corner_radius;

    float vertical_border = center_to_point.x <= 0. ? input.border_left : input.border_right;
    float horizontal_border = center_to_point.y <= 0. ? input.border_top : input.border_bottom;
    float2 inset_size = half_size - input.corner_radius - float2(vertical_border, horizontal_border);
    float2 point_to_inset_corner = abs(center_to_point) - inset_size;
    float border_width;
    if (point_to_inset_corner.x < 0. && point_to_inset_corner.y < 0.) {
        border_width = 0.;
    } else if (point_to_inset_corner.y > point_to_inset_corner.x) {
        border_width = horizontal_border;
    } else {
        border_width = vertical_border;
    }

    float4 color = input.background_color * float4(1., 1., 1., saturate(0.5 - distance));
    if (border_width != 0.) {
        float inset_distance = distance + border_width;
        color = mix(input.border_color, color, saturate(0.5 - inset_distance));
    }

    return color;
}

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
        float2(0., 0.),
        quad.origin,
        quad.size,
        coloru_to_colorf(quad.background_color),
        quad.border_top,
        quad.border_right,
        quad.border_bottom,
        quad.border_left,
        coloru_to_colorf(quad.border_color),
        quad.corner_radius,
    };
}

fragment float4 quad_fragment(
    QuadFragmentInput input [[stage_in]]
) {
    return quad_sdf(input);
}

struct ShadowFragmentInput {
    float4 position [[position]];
    vector_float2 origin;
    vector_float2 size;
    float corner_radius;
    float sigma;
    vector_uchar4 color;
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
        shadow.origin,
        shadow.size,
        shadow.corner_radius,
        shadow.sigma,
        shadow.color,
    };
}

fragment float4 shadow_fragment(
    ShadowFragmentInput input [[stage_in]]
) {
    float sigma = input.sigma;
    float corner_radius = input.corner_radius;
    float2 half_size = input.size / 2.;
    float2 center = input.origin + half_size;
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

    return float4(1., 1., 1., alpha) * coloru_to_colorf(input.color);
}

struct SpriteFragmentInput {
    float4 position [[position]];
    float2 atlas_position;
    float4 color [[flat]];
    uchar compute_winding [[flat]];
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
    float2 position = unit_vertex * sprite.target_size + sprite.origin;
    float4 device_position = to_device_position(position, *viewport_size);
    float2 atlas_position = (unit_vertex * sprite.source_size + sprite.atlas_origin) / *atlas_size;

    return SpriteFragmentInput {
        device_position,
        atlas_position,
        coloru_to_colorf(sprite.color),
        sprite.compute_winding
    };
}

fragment float4 sprite_fragment(
    SpriteFragmentInput input [[stage_in]],
    texture2d<float> atlas [[ texture(GPUISpriteFragmentInputIndexAtlas) ]]
) {
    constexpr sampler atlas_sampler(mag_filter::linear, min_filter::linear);
    float4 color = input.color;
    float4 sample = atlas.sample(atlas_sampler, input.atlas_position);
    float mask;
    if (input.compute_winding) {
        mask = 1. - abs(1. - fmod(sample.r, 2.));
    } else {
        mask = sample.a;
    }
    color.a *= mask;
    return color;
}

vertex QuadFragmentInput image_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint image_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUIImageVertexInputIndexVertices)]],
    constant GPUIImage *images [[buffer(GPUIImageVertexInputIndexImages)]],
    constant float2 *viewport_size [[buffer(GPUIImageVertexInputIndexViewportSize)]],
    constant float2 *atlas_size [[buffer(GPUIImageVertexInputIndexAtlasSize)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUIImage image = images[image_id];
    float2 position = unit_vertex * image.target_size + image.origin;
    float4 device_position = to_device_position(position, *viewport_size);
    float2 atlas_position = (unit_vertex * image.source_size + image.atlas_origin) / *atlas_size;

    return QuadFragmentInput {
        device_position,
        atlas_position,
        image.origin,
        image.target_size,
        float4(0.),
        image.border_top,
        image.border_right,
        image.border_bottom,
        image.border_left,
        coloru_to_colorf(image.border_color),
        image.corner_radius,
    };
}

fragment float4 image_fragment(
    QuadFragmentInput input [[stage_in]],
    texture2d<float> atlas [[ texture(GPUIImageFragmentInputIndexAtlas) ]]
) {
    constexpr sampler atlas_sampler(mag_filter::linear, min_filter::linear);
    input.background_color = atlas.sample(atlas_sampler, input.atlas_position);
    return quad_sdf(input);
}

struct PathAtlasVertexOutput {
    float4 position [[position]];
    float2 st_position;
    float clip_rect_distance [[clip_distance]] [4];
};

struct PathAtlasFragmentInput {
    float4 position [[position]];
    float2 st_position;
};

vertex PathAtlasVertexOutput path_atlas_vertex(
    uint vertex_id [[vertex_id]],
    constant GPUIPathVertex *vertices [[buffer(GPUIPathAtlasVertexInputIndexVertices)]],
    constant float2 *atlas_size [[buffer(GPUIPathAtlasVertexInputIndexAtlasSize)]]
) {
    GPUIPathVertex v = vertices[vertex_id];
    float4 device_position = to_device_position(v.xy_position, *atlas_size);
    return PathAtlasVertexOutput {
        device_position,
        v.st_position,
        {
            v.xy_position.x - v.clip_rect_origin.x,
            v.clip_rect_origin.x + v.clip_rect_size.x - v.xy_position.x,
            v.xy_position.y - v.clip_rect_origin.y,
            v.clip_rect_origin.y + v.clip_rect_size.y - v.xy_position.y
        }
    };
}

fragment float4 path_atlas_fragment(
    PathAtlasFragmentInput input [[stage_in]]
) {
    float2 dx = dfdx(input.st_position);
    float2 dy = dfdy(input.st_position);
    float2 gradient = float2(
        (2. * input.st_position.x) * dx.x - dx.y,
        (2. * input.st_position.x) * dy.x - dy.y
    );
    float f = (input.st_position.x * input.st_position.x) - input.st_position.y;
    float distance = f / length(gradient);
    float alpha = saturate(0.5 - distance);
    return float4(alpha, 0., 0., 1.);
}

struct UnderlineFragmentInput {
    float4 position [[position]];
    float2 origin;
    float2 size;
    float thickness;
    float4 color;
    bool squiggly;
};

vertex UnderlineFragmentInput underline_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint underline_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUIUnderlineInputIndexVertices)]],
    constant GPUIUnderline *underlines [[buffer(GPUIUnderlineInputIndexUnderlines)]],
    constant GPUIUniforms *uniforms [[buffer(GPUIUnderlineInputIndexUniforms)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUIUnderline underline = underlines[underline_id];
    float2 position = unit_vertex * underline.size + underline.origin;
    float4 device_position = to_device_position(position, uniforms->viewport_size);

    return UnderlineFragmentInput {
        device_position,
        underline.origin,
        underline.size,
        underline.thickness,
        coloru_to_colorf(underline.color),
        underline.squiggly != 0,
    };
}

fragment float4 underline_fragment(
    UnderlineFragmentInput input [[stage_in]]
) {
    if (input.squiggly) {
        float half_thickness = input.thickness * 0.5;
        float2 st = ((input.position.xy - input.origin) / input.size.y) - float2(0., 0.5);
        float frequency = (M_PI_F * (3. * input.thickness)) / 8.;
        float amplitude = 1. / (2. * input.thickness);
        float sine = sin(st.x * frequency) * amplitude;
        float dSine = cos(st.x * frequency) * amplitude * frequency;
        float distance = (st.y - sine) / sqrt(1. + dSine * dSine);
        float distance_in_pixels = distance * input.size.y;
        float distance_from_top_border = distance_in_pixels - half_thickness;
        float distance_from_bottom_border = distance_in_pixels + half_thickness;
        float alpha = saturate(0.5 - max(-distance_from_bottom_border, distance_from_top_border));
        return input.color * float4(1., 1., 1., alpha);
    } else {
        return input.color;
    }
}
