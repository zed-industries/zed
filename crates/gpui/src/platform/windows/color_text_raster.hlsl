
struct RasterVertexInput {
    float2 position : POSITION;
};

struct RasterVertexOutput {
    float4 position : SV_Position;
};

RasterVertexOutput vertex(RasterVertexInput input) {
    RasterVertexOutput output;
    output.position = float4(input.position, 0.0, 1.0);
    return output;
}

struct PixelInput {
    float4 position: SV_Position;
};

float4 pixel(PixelInput input): SV_Target {
    return float4(input.position.xy, 0.0, 1.0);
}
