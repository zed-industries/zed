use crate::{point, size, Pixels, PointF, Quad, Scene, Size};
use bytemuck::{Pod, Zeroable};
use cocoa::{
    base::{NO, YES},
    foundation::NSUInteger,
    quartzcore::AutoresizingMask,
};
use metal::{CommandQueue, MTLPixelFormat, MTLResourceOptions, NSRange};
use objc::{self, msg_send, sel, sel_impl};
use std::{ffi::c_void, mem, ptr};

const SHADERS_METALLIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));
const INSTANCE_BUFFER_SIZE: usize = 8192 * 1024; // This is an arbitrary decision. There's probably a more optimal value.

pub struct MetalRenderer {
    layer: metal::MetalLayer,
    command_queue: CommandQueue,
    quad_pipeline_state: metal::RenderPipelineState,
    unit_vertices: metal::Buffer,
    instances: metal::Buffer,
}

impl MetalRenderer {
    pub fn new(is_opaque: bool) -> Self {
        const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::BGRA8Unorm;

        let device: metal::Device = if let Some(device) = metal::Device::system_default() {
            device
        } else {
            log::error!("unable to access a compatible graphics device");
            std::process::exit(1);
        };

        let layer = metal::MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(PIXEL_FORMAT);
        layer.set_presents_with_transaction(true);
        layer.set_opaque(is_opaque);
        unsafe {
            let _: () = msg_send![&*layer, setAllowsNextDrawableTimeout: NO];
            let _: () = msg_send![&*layer, setNeedsDisplayOnBoundsChange: YES];
            let _: () = msg_send![
                &*layer,
                setAutoresizingMask: AutoresizingMask::WIDTH_SIZABLE
                    | AutoresizingMask::HEIGHT_SIZABLE
            ];
        }

        let library = device
            .new_library_with_data(SHADERS_METALLIB)
            .expect("error building metal library");

        fn to_float2_bits(point: crate::PointF) -> u64 {
            unsafe {
                let mut output = mem::transmute::<_, u32>(point.y.to_bits()) as u64;
                output <<= 32;
                output |= mem::transmute::<_, u32>(point.x.to_bits()) as u64;
                output
            }
        }

        let unit_vertices = [
            to_float2_bits(point(0., 0.)),
            to_float2_bits(point(1., 0.)),
            to_float2_bits(point(0., 1.)),
            to_float2_bits(point(0., 1.)),
            to_float2_bits(point(1., 0.)),
            to_float2_bits(point(1., 1.)),
        ];
        let unit_vertices = device.new_buffer_with_data(
            unit_vertices.as_ptr() as *const c_void,
            (unit_vertices.len() * mem::size_of::<u64>()) as u64,
            MTLResourceOptions::StorageModeManaged,
        );
        let instances = device.new_buffer(
            INSTANCE_BUFFER_SIZE as u64,
            MTLResourceOptions::StorageModeManaged,
        );

        let quad_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "quad",
            "quad_vertex",
            "quad_fragment",
            PIXEL_FORMAT,
        );

        Self {
            layer,
            command_queue: device.new_command_queue(),
            quad_pipeline_state,
            unit_vertices,
            instances,
        }
    }

    pub fn layer(&self) -> &metal::MetalLayerRef {
        &*self.layer
    }

    pub fn draw(&mut self, scene: &Scene, scale_factor: f32) {
        let layer = self.layer.clone();
        let viewport_size = layer.drawable_size();
        let viewport_size: Size<Pixels> =
            size(viewport_size.width.into(), viewport_size.height.into());
        let drawable = if let Some(drawable) = layer.next_drawable() {
            drawable
        } else {
            log::error!(
                "failed to retrieve next drawable, drawable size: {:?}",
                viewport_size
            );
            return;
        };
        let command_queue = self.command_queue.clone();
        let command_buffer = command_queue.new_command_buffer();
        let render_pass_descriptor = metal::RenderPassDescriptor::new();
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        color_attachment.set_texture(Some(drawable.texture()));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_store_action(metal::MTLStoreAction::Store);
        let alpha = if self.layer.is_opaque() { 1. } else { 0. };
        color_attachment.set_clear_color(metal::MTLClearColor::new(0., 0., 0., alpha));
        let command_encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);

        command_encoder.set_viewport(metal::MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: viewport_size.width.into(),
            height: viewport_size.height.into(),
            znear: 0.0,
            zfar: 1.0,
        });

        let mut buffer_offset = 0;
        self.draw_quads(
            &scene.opaque_primitives().quads,
            &mut buffer_offset,
            scale_factor,
            viewport_size,
            scene.max_order(),
            command_encoder,
        );
        command_encoder.end_encoding();

        self.instances.did_modify_range(NSRange {
            location: 0,
            length: buffer_offset as NSUInteger,
        });

        command_buffer.commit();
        command_buffer.wait_until_completed();
        drawable.present();
    }

    fn draw_quads(
        &mut self,
        quads: &[Quad],
        offset: &mut usize,
        scale_factor: f32,
        viewport_size: Size<Pixels>,
        max_order: u32,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if quads.is_empty() {
            return;
        }
        align_offset(offset);

        command_encoder.set_render_pipeline_state(&self.quad_pipeline_state);
        command_encoder.set_vertex_buffer(
            QuadInputIndex::Vertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_buffer(
            QuadInputIndex::Quads as u64,
            Some(&self.instances),
            *offset as u64,
        );
        let quad_uniforms = QuadUniforms {
            viewport_size,
            scale_factor,
            max_order,
        };

        let quad_uniform_bytes = bytemuck::bytes_of(&quad_uniforms);
        command_encoder.set_vertex_bytes(
            QuadInputIndex::Uniforms as u64,
            quad_uniform_bytes.len() as u64,
            quad_uniform_bytes.as_ptr() as *const c_void,
        );

        let quad_bytes = bytemuck::cast_slice(quads);
        let buffer_contents = unsafe { (self.instances.contents() as *mut u8).add(*offset) };
        unsafe {
            ptr::copy_nonoverlapping(quad_bytes.as_ptr(), buffer_contents, quad_bytes.len());
        }

        let next_offset = *offset + quad_bytes.len();
        assert!(
            next_offset <= INSTANCE_BUFFER_SIZE,
            "instance buffer exhausted"
        );

        dbg!(quads.len());
        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            quads.len() as u64,
        );
        *offset = next_offset;
    }
}

fn build_pipeline_state(
    device: &metal::DeviceRef,
    library: &metal::LibraryRef,
    label: &str,
    vertex_fn_name: &str,
    fragment_fn_name: &str,
    pixel_format: metal::MTLPixelFormat,
) -> metal::RenderPipelineState {
    let vertex_fn = library
        .get_function(vertex_fn_name, None)
        .expect("error locating vertex function");
    let fragment_fn = library
        .get_function(fragment_fn_name, None)
        .expect("error locating fragment function");

    let descriptor = metal::RenderPipelineDescriptor::new();
    descriptor.set_label(label);
    descriptor.set_vertex_function(Some(vertex_fn.as_ref()));
    descriptor.set_fragment_function(Some(fragment_fn.as_ref()));
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();
    color_attachment.set_pixel_format(pixel_format);
    color_attachment.set_blending_enabled(true);
    color_attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
    color_attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
    color_attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    color_attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
    color_attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::One);

    device
        .new_render_pipeline_state(&descriptor)
        .expect("could not create render pipeline state")
}

// Align to multiples of 256 make Metal happy.
fn align_offset(offset: &mut usize) {
    *offset = ((*offset + 255) / 256) * 256;
}

#[repr(C)]
enum QuadInputIndex {
    Vertices = 0,
    Quads = 1,
    Uniforms = 2,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub(crate) struct QuadUniforms {
    viewport_size: Size<Pixels>,
    scale_factor: f32,
    max_order: u32,
}
