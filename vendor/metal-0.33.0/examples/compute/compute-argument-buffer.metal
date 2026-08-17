#include <metal_stdlib>

using namespace metal;

struct SumInput {
    device uint *data;
    volatile device atomic_uint *sum;
};

kernel void sum(device SumInput& input [[ buffer(0) ]],
                uint gid [[ thread_position_in_grid ]])
{
    atomic_fetch_add_explicit(input.sum, input.data[gid], memory_order_relaxed);
}
