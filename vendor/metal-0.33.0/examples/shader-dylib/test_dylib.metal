#include <metal_stdlib>

using namespace metal;

float4 get_color_test(float4 inColor)
{
    return float4(inColor.r, inColor.g, inColor.b, 0);
}
