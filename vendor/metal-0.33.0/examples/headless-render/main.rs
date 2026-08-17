use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use metal::{
    Buffer, Device, DeviceRef, LibraryRef, MTLClearColor, MTLLoadAction, MTLOrigin, MTLPixelFormat,
    MTLPrimitiveType, MTLRegion, MTLResourceOptions, MTLSize, MTLStoreAction, RenderPassDescriptor,
    RenderPassDescriptorRef, RenderPipelineDescriptor, RenderPipelineState, Texture,
    TextureDescriptor, TextureRef,
};
use png::ColorType;

const VIEW_WIDTH: u64 = 512;
const VIEW_HEIGHT: u64 = 512;
const TOTAL_BYTES: usize = (VIEW_WIDTH * VIEW_HEIGHT * 4) as usize;

const VERTEX_SHADER: &str = "triangle_vertex";
const FRAGMENT_SHADER: &str = "triangle_fragment";

// [2 bytes position, 3 bytes color] * 3
#[rustfmt::skip]
const VERTEX_ATTRIBS: [f32; 15] = [
    0.0, 0.5, 1.0, 0.0, 0.0,
    -0.5, -0.5, 0.0, 1.0, 0.0,
    0.5, -0.5, 0.0, 0.0, 1.0,
];

/// This example shows how to render headlessly by:
///
/// 1. Rendering a triangle to an MtlDrawable
///
/// 2. Waiting for the render to complete and the color texture to be synchronized with the CPU
///    by using a blit command encoder
///
/// 3. Reading the texture bytes from the MtlTexture
///
/// 4. Saving the texture to a PNG file
fn main() {
    let device = Device::system_default().expect("No device found");

    let texture = create_texture(&device);

    let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples/window/shaders.metallib");

    let library = device.new_library_with_file(library_path).unwrap();

    let pipeline_state = prepare_pipeline_state(&device, &library);

    let command_queue = device.new_command_queue();

    let vertex_buffer = create_vertex_buffer(&device);

    let render_pass_descriptor = RenderPassDescriptor::new();
    initialize_color_attachment(render_pass_descriptor, &texture);

    let command_buffer = command_queue.new_command_buffer();
    let rc_encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
    rc_encoder.set_render_pipeline_state(&pipeline_state);
    rc_encoder.set_vertex_buffer(0, Some(&vertex_buffer), 0);
    rc_encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
    rc_encoder.end_encoding();

    render_pass_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap()
        .set_load_action(MTLLoadAction::DontCare);

    let blit_encoder = command_buffer.new_blit_command_encoder();
    blit_encoder.synchronize_resource(&texture);
    blit_encoder.end_encoding();

    command_buffer.commit();

    command_buffer.wait_until_completed();

    save_image(&texture);
}

fn save_image(texture: &TextureRef) {
    let mut image = vec![0; TOTAL_BYTES];

    texture.get_bytes(
        image.as_mut_ptr().cast(),
        VIEW_WIDTH * 4,
        MTLRegion {
            origin: MTLOrigin { x: 0, y: 0, z: 0 },
            size: MTLSize {
                width: VIEW_WIDTH,
                height: VIEW_HEIGHT,
                depth: 1,
            },
        },
        0,
    );

    let out_file =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/headless-render/out.png");
    let file = File::create(&out_file).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, VIEW_WIDTH as u32, VIEW_HEIGHT as u32);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&image).unwrap();

    println!("Image saved to {:?}", out_file);
}

fn create_texture(device: &Device) -> Texture {
    let texture = TextureDescriptor::new();
    texture.set_width(VIEW_WIDTH);
    texture.set_height(VIEW_HEIGHT);
    texture.set_pixel_format(MTLPixelFormat::RGBA8Unorm);

    device.new_texture(&texture)
}

fn prepare_pipeline_state(device: &DeviceRef, library: &LibraryRef) -> RenderPipelineState {
    let vert = library.get_function(VERTEX_SHADER, None).unwrap();
    let frag = library.get_function(FRAGMENT_SHADER, None).unwrap();

    let pipeline_state_descriptor = RenderPipelineDescriptor::new();

    pipeline_state_descriptor.set_vertex_function(Some(&vert));
    pipeline_state_descriptor.set_fragment_function(Some(&frag));

    pipeline_state_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap()
        .set_pixel_format(MTLPixelFormat::RGBA8Unorm);

    device
        .new_render_pipeline_state(&pipeline_state_descriptor)
        .unwrap()
}

fn create_vertex_buffer(device: &DeviceRef) -> Buffer {
    device.new_buffer_with_data(
        VERTEX_ATTRIBS.as_ptr().cast(),
        size_of_val(&VERTEX_ATTRIBS) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
    )
}

fn initialize_color_attachment(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.5, 0.2, 0.2, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}
