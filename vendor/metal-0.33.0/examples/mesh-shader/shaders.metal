#include <metal_stdlib>

using namespace metal;

struct VertexOut {
    float4 position [[position]];
};

using mesh_t = mesh<VertexOut, void, 3, 1, topology::triangle>;

[[mesh]] void mesh_function(mesh_t m) {
    VertexOut v;
    v.position = float4(-1.0, -1.0, 0.0, 1.0);

    m.set_primitive_count(1);

    m.set_vertex(0, v);
    v.position = float4(0.0, 1.0, 0.0, 1.0);
    m.set_vertex(1, v);
    v.position = float4(1.0, -1.0, 0.0, 1.0);
    m.set_vertex(2, v);

    m.set_index(0, 0);
    m.set_index(1, 1);
    m.set_index(2, 2);
}

fragment half4 fragment_function() {
    return half4(0.1, 1.0, 0.1, 1.0);
}