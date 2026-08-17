use metal::*;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
};

#[allow(deprecated)]
use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;

use objc::{rc::autoreleasepool, runtime::YES};

// Declare the data structures needed to carry vertex layout to
// metal shading language(MSL) program. Use #[repr(C)], to make
// the data structure compatible with C++ type data structure
// for vertex defined in MSL program as MSL program is broadly
// based on C++
#[repr(C)]
#[derive(Debug)]
pub struct Position(std::ffi::c_float, std::ffi::c_float);
#[repr(C)]
#[derive(Debug)]
pub struct Color(std::ffi::c_float, std::ffi::c_float, std::ffi::c_float);
#[repr(C)]
#[derive(Debug)]
pub struct AAPLVertex {
    p: Position,
    c: Color,
}

fn main() {
    // Create a window for viewing the content
    let event_loop = EventLoop::new().unwrap();
    let size = winit::dpi::LogicalSize::new(800, 600);

    let window = winit::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_title("Metal".to_string())
        .build(&event_loop)
        .unwrap();

    // Set up the GPU device found in the system
    let device = Device::system_default().expect("no device found");
    println!("Your device is: {}", device.name(),);

    // Scaffold required to sample the GPU and CPU timestamps
    let mut cpu_start = 0;
    let mut gpu_start = 0;
    device.sample_timestamps(&mut cpu_start, &mut gpu_start);
    let counter_sample_buffer = create_counter_sample_buffer(&device);
    let destination_buffer = device.new_buffer(
        (size_of::<u64>() * 4usize) as u64,
        MTLResourceOptions::StorageModeShared,
    );
    let counter_sampling_point = MTLCounterSamplingPoint::AtStageBoundary;
    assert!(device.supports_counter_sampling(counter_sampling_point));

    let binary_archive_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/circle/binary_archive.metallib");

    let binary_archive_url =
        URL::new_with_string(&format!("file://{}", binary_archive_path.display()));

    let binary_archive_descriptor = BinaryArchiveDescriptor::new();
    if binary_archive_path.exists() {
        binary_archive_descriptor.set_url(&binary_archive_url);
    }

    // Set up a binary archive to cache compiled shaders.
    let binary_archive = device
        .new_binary_archive_with_descriptor(&binary_archive_descriptor)
        .unwrap();

    let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/circle/shaders.metallib");

    // Use the metallib file generated out of .metal shader file
    let library = device.new_library_with_file(library_path).unwrap();

    // The render pipeline generated from the vertex and fragment shaders in the .metal shader file.
    let pipeline_state = prepare_pipeline_state(&device, &library, &binary_archive);

    // Serialize the binary archive to disk.
    binary_archive
        .serialize_to_url(&binary_archive_url)
        .unwrap();

    // Set the command queue used to pass commands to the device.
    let command_queue = device.new_command_queue();

    // Currently, MetalLayer is the only interface that provide
    // layers to carry drawable texture from GPU rendaring through metal
    // library to viewable windows.
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

    let vbuf = {
        let vertex_data = create_vertex_points_for_circle();

        device.new_buffer_with_data(
            vertex_data.as_ptr().cast(),
            size_of_val(vertex_data.as_slice()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        )
    };

    event_loop
        .run(move |event, event_loop| {
            autoreleasepool(|| {
                // ControlFlow::Wait pauses the event loop if no events are available to process.
                // This is ideal for non-game applications that only update in response to user
                // input, and uses significantly less power/CPU time than ControlFlow::Poll.
                event_loop.set_control_flow(ControlFlow::Wait);

                match event {
                    Event::AboutToWait => window.request_redraw(),
                    Event::WindowEvent { event, .. } => {
                        match event {
                            WindowEvent::CloseRequested => {
                                println!("The close button was pressed; stopping");
                                event_loop.exit();
                            }
                            WindowEvent::RedrawRequested => {
                                // It's preferrable to render in this event rather than in MainEventsCleared, since
                                // rendering in here allows the program to gracefully handle redraws requested
                                // by the OS.
                                let drawable = match layer.next_drawable() {
                                    Some(drawable) => drawable,
                                    None => return,
                                };

                                // Create a new command buffer for each render pass to the current drawable
                                let command_buffer = command_queue.new_command_buffer();

                                // Obtain a renderPassDescriptor generated from the view's drawable textures.
                                let render_pass_descriptor = RenderPassDescriptor::new();
                                handle_render_pass_color_attachment(
                                    render_pass_descriptor,
                                    drawable.texture(),
                                );
                                handle_render_pass_sample_buffer_attachment(
                                    render_pass_descriptor,
                                    &counter_sample_buffer,
                                );

                                // Create a render command encoder.
                                let encoder = command_buffer
                                    .new_render_command_encoder(render_pass_descriptor);
                                encoder.set_render_pipeline_state(&pipeline_state);
                                // Pass in the parameter data.
                                encoder.set_vertex_buffer(0, Some(&vbuf), 0);
                                // Draw the triangles which will eventually form the circle.
                                encoder.draw_primitives(MTLPrimitiveType::TriangleStrip, 0, 1080);
                                encoder.end_encoding();

                                resolve_samples_into_buffer(
                                    command_buffer,
                                    &counter_sample_buffer,
                                    &destination_buffer,
                                );

                                // Schedule a present once the framebuffer is complete using the current drawable.
                                command_buffer.present_drawable(drawable);

                                // Finalize rendering here & push the command buffer to the GPU.
                                command_buffer.commit();
                                command_buffer.wait_until_completed();

                                let mut cpu_end = 0;
                                let mut gpu_end = 0;
                                device.sample_timestamps(&mut cpu_end, &mut gpu_end);
                                handle_timestamps(
                                    &destination_buffer,
                                    cpu_start,
                                    cpu_end,
                                    gpu_start,
                                    gpu_end,
                                );
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            });
        })
        .unwrap();
}

// If we want to draw a circle, we need to draw it out of the three primitive
// types available with metal framework. Triangle is used in this case to form
// the circle. If we consider a circle to be total of 360 degree at center, we
// can form small triangle with one point at origin and two points at the
// perimeter of the circle for each degree. Eventually, if we can take enough
// triangle virtices for total of 360 degree, the triangles together will
// form a circle. This function captures the triangle vertices for each degree
// and push the co-ordinates of the vertices to a rust vector
fn create_vertex_points_for_circle() -> Vec<AAPLVertex> {
    let mut v: Vec<AAPLVertex> = Vec::new();
    let origin_x: f32 = 0.0;
    let origin_y: f32 = 0.0;

    // Size of the circle
    let circle_size = 0.8f32;

    for i in 0..720 {
        let y = i as f32;
        // Get the X co-ordinate of each point on the perimeter of circle
        let position_x: f32 = y.to_radians().cos() * 100.0;
        let position_x: f32 = position_x.trunc() / 100.0;
        // Set the size of the circle
        let position_x: f32 = position_x * circle_size;
        // Get the Y co-ordinate of each point on the perimeter of circle
        let position_y: f32 = y.to_radians().sin() * 100.0;
        let position_y: f32 = position_y.trunc() / 100.0;
        // Set the size of the circle
        let position_y: f32 = position_y * circle_size;

        v.push(AAPLVertex {
            p: Position(position_x, position_y),
            c: Color(0.7, 0.3, 0.5),
        });

        if (i + 1) % 2 == 0 {
            // For each two points on perimeter, push one point of origin
            v.push(AAPLVertex {
                p: Position(origin_x, origin_y),
                c: Color(0.2, 0.7, 0.4),
            });
        }
    }

    v
}

fn handle_render_pass_sample_buffer_attachment(
    descriptor: &RenderPassDescriptorRef,
    counter_sample_buffer: &CounterSampleBufferRef,
) {
    let sample_buffer_attachment_descriptor =
        descriptor.sample_buffer_attachments().object_at(0).unwrap();
    sample_buffer_attachment_descriptor.set_sample_buffer(counter_sample_buffer);
    sample_buffer_attachment_descriptor.set_start_of_vertex_sample_index(0 as NSUInteger);
    sample_buffer_attachment_descriptor.set_end_of_vertex_sample_index(1 as NSUInteger);
    sample_buffer_attachment_descriptor.set_start_of_fragment_sample_index(2 as NSUInteger);
    sample_buffer_attachment_descriptor.set_end_of_fragment_sample_index(3 as NSUInteger);
}

fn handle_render_pass_color_attachment(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    // Setting a background color
    color_attachment.set_clear_color(MTLClearColor::new(0.5, 0.5, 0.8, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

fn prepare_pipeline_state(
    device: &Device,
    library: &Library,
    binary_archive: &BinaryArchive,
) -> RenderPipelineState {
    let vert = library.get_function("vs", None).unwrap();
    let frag = library.get_function("ps", None).unwrap();

    let pipeline_state_descriptor = RenderPipelineDescriptor::new();
    pipeline_state_descriptor.set_vertex_function(Some(&vert));
    pipeline_state_descriptor.set_fragment_function(Some(&frag));
    pipeline_state_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap()
        .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    // Set the binary archives to search for a cached pipeline in.
    pipeline_state_descriptor.set_binary_archives(&[binary_archive]);

    // Add the pipeline descriptor to the binary archive cache.
    binary_archive
        .add_render_pipeline_functions_with_descriptor(&pipeline_state_descriptor)
        .unwrap();

    device
        .new_render_pipeline_state(&pipeline_state_descriptor)
        .unwrap()
}

fn resolve_samples_into_buffer(
    command_buffer: &CommandBufferRef,
    counter_sample_buffer: &CounterSampleBufferRef,
    destination_buffer: &BufferRef,
) {
    let blit_encoder = command_buffer.new_blit_command_encoder();
    blit_encoder.resolve_counters(
        counter_sample_buffer,
        crate::NSRange::new(0u64, 4),
        destination_buffer,
        0u64,
    );
    blit_encoder.end_encoding();
}

fn handle_timestamps(
    resolved_sample_buffer: &BufferRef,
    cpu_start: u64,
    cpu_end: u64,
    gpu_start: u64,
    gpu_end: u64,
) {
    let samples = unsafe {
        std::slice::from_raw_parts(resolved_sample_buffer.contents().cast::<u64>(), 4usize)
    };
    let vertex_pass_start = samples[0];
    let vertex_pass_end = samples[1];
    let fragment_pass_start = samples[2];
    let fragment_pass_end = samples[3];

    let cpu_time_span = cpu_end - cpu_start;
    let gpu_time_span = gpu_end - gpu_start;

    let vertex_micros = microseconds_between_begin(
        vertex_pass_start,
        vertex_pass_end,
        gpu_time_span,
        cpu_time_span,
    );
    let fragment_micros = microseconds_between_begin(
        fragment_pass_start,
        fragment_pass_end,
        gpu_time_span,
        cpu_time_span,
    );

    println!("Vertex pass duration:   {:.2} µs", vertex_micros);
    println!("Fragment pass duration: {:.2} µs\n", fragment_micros);
}

fn create_counter_sample_buffer(device: &Device) -> CounterSampleBuffer {
    let counter_sample_buffer_desc = metal::CounterSampleBufferDescriptor::new();
    counter_sample_buffer_desc.set_storage_mode(metal::MTLStorageMode::Shared);
    counter_sample_buffer_desc.set_sample_count(4u64);
    counter_sample_buffer_desc.set_counter_set(&fetch_timestamp_counter_set(device));

    device
        .new_counter_sample_buffer_with_descriptor(&counter_sample_buffer_desc)
        .unwrap()
}

fn fetch_timestamp_counter_set(device: &Device) -> metal::CounterSet {
    let counter_sets = device.counter_sets();
    let mut timestamp_counter = None;
    for cs in counter_sets.iter() {
        if cs.name() == "timestamp" {
            timestamp_counter = Some(cs);
            break;
        }
    }
    timestamp_counter
        .expect("No timestamp counter found")
        .clone()
}

/// <https://developer.apple.com/documentation/metal/gpu_counters_and_counter_sample_buffers/converting_gpu_timestamps_into_cpu_time>
fn microseconds_between_begin(begin: u64, end: u64, gpu_time_span: u64, cpu_time_span: u64) -> f64 {
    let time_span = (end as f64) - (begin as f64);
    let nanoseconds = time_span / (gpu_time_span as f64) * (cpu_time_span as f64);
    nanoseconds / 1000.0
}
