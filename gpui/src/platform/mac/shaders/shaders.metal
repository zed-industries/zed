#include <metal_stdlib>
#include "shaders.h"

using namespace metal;

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
    float4 position = float4((unit_vertex * quad.size + quad.origin) / (uniforms->viewport_size / 2.0), 0.0, 1.0);

    return QuadFragmentInput {
        position,
        quad,
    };
}

fragment float4 quad_fragment(QuadFragmentInput input [[stage_in]]) {
    return input.quad.background_color;
}