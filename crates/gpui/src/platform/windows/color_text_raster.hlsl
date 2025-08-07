struct RasterVertexOutput {
    float4 position : SV_Position;
    float2 texcoord : TEXCOORD0;
};

RasterVertexOutput emoji_rasterization_vertex(uint vertexID : SV_VERTEXID)
{
    RasterVertexOutput output;
    output.texcoord = float2((vertexID << 1) & 2, vertexID & 2);
    output.position = float4(output.texcoord * 2.0f - 1.0f, 0.0f, 1.0f);
    output.position.y = -output.position.y;

    return output;
}

struct PixelInput {
    float4 position: SV_Position;
    float2 texcoord : TEXCOORD0;
};

struct Bounds {
    int2 origin;
    int2 size;
};

Texture2D<float4> t_layer : register(t0);
SamplerState s_layer : register(s0);

cbuffer GlyphLayerTextureParams : register(b0) {
    Bounds bounds;
    float4 run_color;
};

float4 emoji_rasterization_fragment(PixelInput input): SV_Target {
    float3 sampled = t_layer.Sample(s_layer, input.texcoord.xy).rgb;
    float alpha = (sampled.r + sampled.g + sampled.b) / 3;

    return float4(run_color.rgb, alpha);
}
