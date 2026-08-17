#include <metal_stdlib>

using namespace metal;

kernel void sum(device uint *data [[ buffer(0) ]],
                volatile device atomic_uint *sum [[ buffer(1) ]],
                uint gid [[ thread_position_in_grid ]])
{
    atomic_fetch_add_explicit(sum, data[gid], memory_order_relaxed);
}
