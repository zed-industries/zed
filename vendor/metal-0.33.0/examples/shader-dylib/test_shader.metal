#include <metal_stdlib>

using namespace metal;

extern float4 get_color_test(float4 inColor);

kernel void test_kernel(
    texture2d<float, access::write> image [[texture(0)]],
    uint2 coordinates [[thread_position_in_grid]],
    uint2 size [[threads_per_grid]])
{
    float2 uv = float2(coordinates) / float2(size - 1);
    image.write(get_color_test(float4(uv, 0.0, 1.0)), coordinates);
}
