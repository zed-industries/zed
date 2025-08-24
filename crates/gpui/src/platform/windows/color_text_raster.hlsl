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

Texture2D<float> t_layer : register(t0);
SamplerState s_layer : register(s0);

cbuffer GlyphLayerTextureParams : register(b0) {
    Bounds bounds;
    float4 run_color;
};

float4 emoji_rasterization_fragment(PixelInput input): SV_Target {
    float sampled = t_layer.Sample(s_layer, input.texcoord.xy);
    float alpha = sampled * run_color.a;

    float3 color_linear = lerp(run_color.rgb / 12.92, pow((run_color.rgb + 0.055) / 1.055, 2.4), step(0.04045, run_color.rgb));
    return float4(color_linear * alpha, alpha);
}
