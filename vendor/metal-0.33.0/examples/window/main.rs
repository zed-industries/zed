// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate objc;

#[allow(deprecated)]
use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;
use metal::*;
use objc::{rc::autoreleasepool, runtime::YES};

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
};

#[repr(C)]
struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[repr(C)]
struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
struct ClearRect {
    pub rect: Rect,
    pub color: Color,
}

fn prepare_pipeline_state(
    device: &DeviceRef,
    library: &LibraryRef,
    vertex_shader: &str,
    fragment_shader: &str,
) -> RenderPipelineState {
    let vert = library.get_function(vertex_shader, None).unwrap();
    let frag = library.get_function(fragment_shader, None).unwrap();

    let pipeline_state_descriptor = RenderPipelineDescriptor::new();
    pipeline_state_descriptor.set_vertex_function(Some(&vert));
    pipeline_state_descriptor.set_fragment_function(Some(&frag));
    let attachment = pipeline_state_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap();
    attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

    attachment.set_blending_enabled(true);
    attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
    attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
    attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
    attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);

    device
        .new_render_pipeline_state(&pipeline_state_descriptor)
        .unwrap()
}

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    //descriptor.color_attachments().set_object_at(0, MTLRenderPassColorAttachmentDescriptor::alloc());
    //let color_attachment: MTLRenderPassColorAttachmentDescriptor = unsafe { msg_send![descriptor.color_attachments().0, _descriptorAtIndex:0] };//descriptor.color_attachments().object_at(0);
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.2, 0.2, 0.25, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let size = winit::dpi::LogicalSize::new(800, 600);

    let window = winit::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_title("Metal Window Example".to_string())
        .build(&event_loop)
        .unwrap();

    let device = Device::system_default().expect("no device found");

    let mut layer = MetalLayer::new();
    layer.set_device(&device);
    layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    layer.set_presents_with_transaction(false);

    #[allow(deprecated)]
    unsafe {
        if let Ok(RawWindowHandle::AppKit(rw)) = window.window_handle().map(|wh| wh.as_raw()) {
            let view = rw.ns_view.as_ptr() as cocoa_id;
            view.setWantsLayer(YES);
            view.setLayer(<*mut _>::cast(layer.as_mut()));
        }
    }

    let draw_size = window.inner_size();
    layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));

    let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/window/shaders.metallib");

    let library = device.new_library_with_file(library_path).unwrap();
    let triangle_pipeline_state =
        prepare_pipeline_state(&device, &library, "triangle_vertex", "triangle_fragment");
    let clear_rect_pipeline_state = prepare_pipeline_state(
        &device,
        &library,
        "clear_rect_vertex",
        "clear_rect_fragment",
    );

    let command_queue = device.new_command_queue();
    //let nc: () = msg_send![command_queue.0, setExecutionEnabled:true];

    let vbuf = {
        let vertex_data = [
            0.0f32, 0.5, 1.0, 0.0, 0.0, -0.5, -0.5, 0.0, 1.0, 0.0, 0.5, 0.5, 0.0, 0.0, 1.0,
        ];

        device.new_buffer_with_data(
            vertex_data.as_ptr().cast(),
            size_of_val(&vertex_data) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        )
    };

    let mut r = 0.0f32;

    let clear_rect = ClearRect {
        rect: Rect {
            x: -1.0,
            y: -1.0,
            w: 2.0,
            h: 2.0,
        },
        color: Color {
            r: 0.5,
            g: 0.8,
            b: 0.5,
            a: 1.0,
        },
    };

    let clear_rect_buffer = device.new_buffer_with_data(
        (&raw const clear_rect).cast(),
        size_of::<ClearRect>() as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
    );

    event_loop
        .run(move |event, event_loop| {
            autoreleasepool(|| {
                event_loop.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::AboutToWait => window.request_redraw(),
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => event_loop.exit(),
                        WindowEvent::Resized(size) => {
                            layer.set_drawable_size(CGSize::new(
                                size.width as f64,
                                size.height as f64,
                            ));
                        }
                        WindowEvent::RedrawRequested => {
                            let p = vbuf.contents();
                            let vertex_data = [
                                0.0f32,
                                0.5,
                                1.0,
                                0.0,
                                0.0,
                                -0.5 + (r.cos() / 2. + 0.5),
                                -0.5,
                                0.0,
                                1.0,
                                0.0,
                                0.5 - (r.cos() / 2. + 0.5),
                                -0.5,
                                0.0,
                                0.0,
                                1.0,
                            ];

                            unsafe {
                                std::ptr::copy(
                                    vertex_data.as_ptr(),
                                    p.cast(),
                                    size_of_val(&vertex_data),
                                );
                            }

                            vbuf.did_modify_range(crate::NSRange::new(
                                0u64,
                                size_of_val(&vertex_data) as u64,
                            ));

                            let drawable = match layer.next_drawable() {
                                Some(drawable) => drawable,
                                None => return,
                            };

                            let render_pass_descriptor = RenderPassDescriptor::new();

                            prepare_render_pass_descriptor(
                                render_pass_descriptor,
                                drawable.texture(),
                            );

                            let command_buffer = command_queue.new_command_buffer();
                            let encoder =
                                command_buffer.new_render_command_encoder(render_pass_descriptor);

                            encoder.set_scissor_rect(MTLScissorRect {
                                x: 20,
                                y: 20,
                                width: 100,
                                height: 100,
                            });
                            encoder.set_render_pipeline_state(&clear_rect_pipeline_state);
                            encoder.set_vertex_buffer(0, Some(&clear_rect_buffer), 0);
                            encoder.draw_primitives_instanced(
                                metal::MTLPrimitiveType::TriangleStrip,
                                0,
                                4,
                                1,
                            );
                            let physical_size = window.inner_size();
                            encoder.set_scissor_rect(MTLScissorRect {
                                x: 0,
                                y: 0,
                                width: physical_size.width as _,
                                height: physical_size.height as _,
                            });

                            encoder.set_render_pipeline_state(&triangle_pipeline_state);
                            encoder.set_vertex_buffer(0, Some(&vbuf), 0);
                            encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
                            encoder.end_encoding();

                            command_buffer.present_drawable(drawable);
                            command_buffer.commit();

                            r += 0.01f32;
                        }
                        _ => (),
                    },
                    _ => {}
                }
            });
        })
        .unwrap();
}
