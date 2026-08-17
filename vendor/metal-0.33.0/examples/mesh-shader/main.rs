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

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
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
        .with_title("Metal Mesh Shader Example".to_string())
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
        .join("examples/mesh-shader/shaders.metallib");
    let library = device.new_library_with_file(library_path).unwrap();

    let mesh = library.get_function("mesh_function", None).unwrap();
    let frag = library.get_function("fragment_function", None).unwrap();

    let pipeline_state_desc = MeshRenderPipelineDescriptor::new();
    pipeline_state_desc
        .color_attachments()
        .object_at(0)
        .unwrap()
        .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    pipeline_state_desc.set_mesh_function(Some(&mesh));
    pipeline_state_desc.set_fragment_function(Some(&frag));

    let pipeline_state = device
        .new_mesh_render_pipeline_state(&pipeline_state_desc)
        .unwrap();

    let command_queue = device.new_command_queue();

    event_loop
        .run(move |event, event_loop| {
            autoreleasepool(|| {
                event_loop.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::AboutToWait => {
                        window.request_redraw();
                    }
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => event_loop.exit(),
                        WindowEvent::Resized(size) => {
                            layer.set_drawable_size(CGSize::new(
                                size.width as f64,
                                size.height as f64,
                            ));
                        }
                        WindowEvent::RedrawRequested => {
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

                            encoder.set_render_pipeline_state(&pipeline_state);
                            encoder.draw_mesh_threads(
                                MTLSize::new(1, 1, 1),
                                MTLSize::new(1, 1, 1),
                                MTLSize::new(1, 1, 1),
                            );

                            encoder.end_encoding();

                            command_buffer.present_drawable(drawable);
                            command_buffer.commit();
                        }
                        _ => (),
                    },
                    _ => {}
                }
            });
        })
        .unwrap();
}
