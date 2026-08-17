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

struct App {
    pub _device: Device,
    pub command_queue: CommandQueue,
    pub layer: MetalLayer,
    pub image_fill_cps: ComputePipelineState,
    pub width: u32,
    pub height: u32,
}

fn select_device() -> Option<Device> {
    let devices = Device::all();
    devices.into_iter().find(|d| d.supports_dynamic_libraries())
}

impl App {
    fn new(window: &winit::window::Window) -> Self {
        let device = select_device().expect("no device found that supports dynamic libraries");
        let command_queue = device.new_command_queue();

        let mut layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);
        layer.set_framebuffer_only(false);
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

        // compile dynamic lib shader
        let dylib_src_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/shader-dylib/test_dylib.metal");
        let install_path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test_dylib.metallib");

        let dylib_src = std::fs::read_to_string(dylib_src_path).expect("bad shit");
        let opts = metal::CompileOptions::new();
        opts.set_library_type(MTLLibraryType::Dynamic);
        opts.set_install_name(install_path.to_str().unwrap());

        let lib = device
            .new_library_with_source(dylib_src.as_str(), &opts)
            .unwrap();

        // create dylib
        let dylib = device.new_dynamic_library(&lib).unwrap();
        dylib.set_label("test_dylib");

        // optional: serialize binary blob that can be loaded later
        let blob_url = String::from("file://") + install_path.to_str().unwrap();
        let url = URL::new_with_string(&blob_url);
        dylib.serialize_to_url(&url).unwrap();

        // create shader that links with dylib
        let shader_src_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/shader-dylib/test_shader.metal");

        let shader_src = std::fs::read_to_string(shader_src_path).expect("bad shit");
        let opts = metal::CompileOptions::new();
        // add dynamic library to link with
        let libraries = [dylib.as_ref()];
        opts.set_libraries(&libraries);

        // compile
        let shader_lib = device
            .new_library_with_source(shader_src.as_str(), &opts)
            .unwrap();

        let func = shader_lib.get_function("test_kernel", None).unwrap();

        // create pipeline state
        // linking occurs here
        let image_fill_cps = device
            .new_compute_pipeline_state_with_function(&func)
            .unwrap();

        Self {
            _device: device,
            command_queue,
            layer,
            image_fill_cps,
            width: draw_size.width,
            height: draw_size.height,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.layer
            .set_drawable_size(CGSize::new(width as f64, height as f64));
        self.width = width;
        self.height = height;
    }

    fn draw(&self) {
        let drawable = match self.layer.next_drawable() {
            Some(drawable) => drawable,
            None => return,
        };

        let w = self.image_fill_cps.thread_execution_width();
        let h = self.image_fill_cps.max_total_threads_per_threadgroup() / w;
        let threads_per_threadgroup = MTLSize::new(w, h, 1);
        let threads_per_grid = MTLSize::new(self.width as _, self.height as _, 1);

        let command_buffer = self.command_queue.new_command_buffer();

        {
            let encoder = command_buffer.new_compute_command_encoder();
            encoder.set_compute_pipeline_state(&self.image_fill_cps);
            encoder.set_texture(0, Some(drawable.texture()));
            encoder.dispatch_threads(threads_per_grid, threads_per_threadgroup);
            encoder.end_encoding();
        }

        command_buffer.present_drawable(drawable);
        command_buffer.commit();
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let size = winit::dpi::LogicalSize::new(800, 600);

    let window = winit::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_title("Metal Shader Dylib Example".to_string())
        .build(&event_loop)
        .unwrap();

    let mut app = App::new(&window);

    event_loop
        .run(move |event, event_loop| {
            autoreleasepool(|| {
                event_loop.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::AboutToWait => window.request_redraw(),
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => event_loop.exit(),
                        WindowEvent::Resized(size) => {
                            app.resize(size.width, size.height);
                        }
                        WindowEvent::RedrawRequested => {
                            app.draw();
                        }
                        _ => (),
                    },
                    _ => {}
                }
            });
        })
        .unwrap();
}
