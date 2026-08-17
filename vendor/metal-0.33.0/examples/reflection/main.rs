// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use metal::*;
use objc::rc::autoreleasepool;

const PROGRAM: &str = r"
    #include <metal_stdlib>

    using namespace metal;

    typedef struct {
        float2 position;
        float3 color;
    } vertex_t;

    struct ColorInOut {
        float4 position [[position]];
        float4 color;
    };

    vertex ColorInOut vs(device vertex_t* vertex_array [[ buffer(0) ]],
                                      unsigned int vid [[ vertex_id ]])
    {
        ColorInOut out;

        out.position = float4(float2(vertex_array[vid].position), 0.0, 1.0);
        out.color = float4(float3(vertex_array[vid].color), 1.0);

        return out;
    }

    fragment float4 ps(ColorInOut in [[stage_in]])
    {
        return in.color;
    };
";

fn main() {
    autoreleasepool(|| {
        let device = Device::system_default().expect("no device found");

        let options = CompileOptions::new();
        let library = device.new_library_with_source(PROGRAM, &options).unwrap();
        let (vs, ps) = (
            library.get_function("vs", None).unwrap(),
            library.get_function("ps", None).unwrap(),
        );

        let vertex_desc = VertexDescriptor::new();

        let desc = RenderPipelineDescriptor::new();
        desc.set_vertex_function(Some(&vs));
        desc.set_fragment_function(Some(&ps));
        desc.set_vertex_descriptor(Some(vertex_desc));

        println!("{:?}", desc);

        let reflect_options = MTLPipelineOption::ArgumentInfo | MTLPipelineOption::BufferTypeInfo;
        let (_, reflection) = device
            .new_render_pipeline_state_with_reflection(&desc, reflect_options)
            .unwrap();

        println!("Vertex arguments: ");
        let vertex_arguments = reflection.vertex_arguments();
        for index in 0..vertex_arguments.count() {
            let argument = vertex_arguments.object_at(index).unwrap();
            println!("{:?}", argument);
        }
    });
}
