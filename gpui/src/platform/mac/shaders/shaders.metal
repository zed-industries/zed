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
    float2 position = (unit_vertex * quad.size + quad.origin) / (uniforms->viewport_size / 2.0);
    float4 device_position = float4(position * float2(2.0, -2.0) + float2(-1.0, 1.0), 0.0, 1.0);

    return QuadFragmentInput {
        device_position,
        quad,
    };
}

fragment float4 quad_fragment(QuadFragmentInput input [[stage_in]]) {
    return coloru_to_colorf(input.quad.background_color);
}
