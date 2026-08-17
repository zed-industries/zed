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

pub mod camera;
pub mod geometry;
pub mod renderer;
pub mod scene;

fn find_raytracing_supporting_device() -> Device {
    for device in Device::all() {
        if !device.supports_raytracing() {
            continue;
        }
        if device.is_low_power() {
            continue;
        }
        return device;
    }

    panic!("No device in this machine supports raytracing!")
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let size = winit::dpi::LogicalSize::new(800, 600);

    let window = winit::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_title("Metal Raytracing Example".to_string())
        .build(&event_loop)
        .unwrap();

    let device = find_raytracing_supporting_device();

    let mut layer = MetalLayer::new();
    layer.set_device(&device);
    layer.set_pixel_format(MTLPixelFormat::RGBA16Float);
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
    let cg_size = CGSize::new(draw_size.width as f64, draw_size.height as f64);
    layer.set_drawable_size(cg_size);

    let mut renderer = renderer::Renderer::new(device);
    renderer.window_resized(cg_size);

    event_loop
        .run(move |event, event_loop| {
            autoreleasepool(|| {
                event_loop.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::AboutToWait => window.request_redraw(),
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => event_loop.exit(),
                        WindowEvent::Resized(size) => {
                            let size = CGSize::new(size.width as f64, size.height as f64);
                            layer.set_drawable_size(size);
                            renderer.window_resized(size);
                        }
                        WindowEvent::RedrawRequested => {
                            renderer.draw(&layer);
                        }
                        _ => (),
                    },
                    _ => {}
                }
            });
        })
        .unwrap();
}
