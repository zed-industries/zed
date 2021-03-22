#include <metal_stdlib>
#include "shaders.h"

using namespace metal;

float4 coloru_to_colorf(uchar4 coloru) {
    return float4(coloru) / float4(0xff, 0xff, 0xff, 0xff);
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
    constant GPUIQuadUniforms *uniforms [[buffer(GPUIQuadInputIndexUniforms)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUIQuad quad = quads[quad_id];
    float2 position = unit_vertex * quad.size + quad.origin;
    float4 device_position = float4(position / uniforms->viewport_size * float2(2.0, -2.0) + float2(-1.0, 1.0), 0.0, 1.0);

    return QuadFragmentInput {
        device_position,
        quad,
    };
}

fragment float4 quad_fragment(
    QuadFragmentInput input [[stage_in]],
    constant GPUIQuadUniforms *uniforms [[buffer(GPUIQuadInputIndexUniforms)]]
) {
    float2 half_size = input.quad.size / 2.;
    float2 center = input.quad.origin + half_size;
    float2 center_to_point = input.position.xy - center;
    float2 edge_to_point = abs(center_to_point) - half_size;
    float2 rounded_edge_to_point = abs(center_to_point) - half_size + input.quad.corner_radius;
    float distance = length(max(0.0, rounded_edge_to_point)) + min(0.0, max(rounded_edge_to_point.x, rounded_edge_to_point.y)) - input.quad.corner_radius;

    float border_width = 0.0;
    if (edge_to_point.x > edge_to_point.y) {
        border_width = center_to_point.x <= 0.0 ? input.quad.border_left : input.quad.border_right;
    } else {
        border_width = center_to_point.y <= 0.0 ? input.quad.border_top : input.quad.border_bottom;
    }

    float inset_distance = distance + border_width;
    float4 color = mix(
        coloru_to_colorf(input.quad.border_color),
        coloru_to_colorf(input.quad.background_color),
        saturate(0.5 - inset_distance)
    );
    float4 coverage = float4(1.0, 1.0, 1.0, saturate(0.5 - distance));
    return coverage * color;
}
