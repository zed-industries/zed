#include <metal_stdlib>

struct SceneProperties {
    float time;
};

struct VertexInput {
    metal::packed_float3 position;
    metal::packed_float3 color;
};

struct VertexOutput {
    metal::float4 position [[position]];
    metal::float4 color;
};

vertex VertexOutput vertex_main(
    device const SceneProperties& properties [[buffer(0)]],
    device const VertexInput* vertices [[buffer(1)]],
    uint vertex_idx [[vertex_id]]
) {
    VertexOutput out;
    VertexInput in = vertices[vertex_idx];
    out.position =
        metal::float4(
            metal::float2x2(
                metal::cos(properties.time), -metal::sin(properties.time),
                metal::sin(properties.time),  metal::cos(properties.time)
            ) * in.position.xy,
            in.position.z,
            1);
    out.color = metal::float4(in.color, 1);
    return out;
}

fragment metal::float4 fragment_main(VertexOutput in [[stage_in]]) {
    return in.color;
}
