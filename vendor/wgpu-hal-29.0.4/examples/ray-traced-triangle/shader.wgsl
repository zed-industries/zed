enable wgpu_ray_query;

struct Uniforms {
    view_inv: mat4x4<f32>,
    proj_inv: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var output: texture_storage_2d<rgba8unorm, write>;

@group(0) @binding(2)
var acc_struct: acceleration_structure;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let target_size = textureDimensions(output);

    let pixel_center = vec2<f32>(global_id.xy) + vec2<f32>(0.5);
    let in_uv = pixel_center / vec2<f32>(target_size.xy);
    let d = in_uv * 2.0 - 1.0;

    let origin = (uniforms.view_inv * vec4<f32>(0.0, 0.0, 0.0, 1.0)).xyz;
    let temp = uniforms.proj_inv * vec4<f32>(d.x, d.y, 1.0, 1.0);
    let direction = (uniforms.view_inv * vec4<f32>(normalize(temp.xyz), 0.0)).xyz;

    var rq: ray_query;
    rayQueryInitialize(&rq, acc_struct, RayDesc(0u, 0xFFu, 0.1, 200.0, origin, direction));
    rayQueryProceed(&rq);

    var color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    let intersection = rayQueryGetCommittedIntersection(&rq);
    if intersection.kind != RAY_QUERY_INTERSECTION_NONE {
        color = vec4<f32>(intersection.barycentrics, 1.0 - intersection.barycentrics.x - intersection.barycentrics.y, 1.0);
    }

    textureStore(output, global_id.xy, color);
}