// How can I fix this?
// -[MTLDebugRenderCommandEncoder setRenderPipelineState:]:1580: failed assertion `Set Render Pipeline State Validation
// For depth attachment, the render pipeline's pixelFormat (MTLPixelFormatInvalid) does not match the framebuffer's pixelFormat (MTLPixelFormatDepth32Float).
// '
// -[MTLDebugRenderCommandEncoder setRenderPipelineState:]:1580: failed assertion `Set Render Pipeline State Validation
// For depth attachment, the render pipeline's pixelFormat (MTLPixelFormatInvalid) does not match the framebuffer's pixelFormat (MTLPixelFormatDepth32Float).
// // It seems like the error you're facing has to do with the difference between the
// pixel format of the render pipeline and the framebuffer. If the pixel format of
// those two doesn't match, Metal throws an error. To resolve this issue, you need
// to set the pixel format of your depth attachment and your render pipeline state
// to the same value.

// In this code:
// ---
/*
descriptor.set_depth_attachment_pixel_format(MTLPixelFormat::Depth32Float);
*/
// ---
// you've commented out the line where you set the depth attachment pixel format
// to MTLPixelFormat::Depth32Float. If you uncomment this line, it should resolve
// the error as your depth attachment's pixel format will then match your framebuffer's.

// If you still encounter the same problem, you might be configuring another render
// pipeline state elsewhere in your code with a different depth pixel format. Make
// sure all configurations have matching pixel formats.

// Additionally, be aware of the limitations of certain pixel formats. For example,
// not all pixel formats support depth stencil attachments, and some are only
// compatible with certain types of GPU hardware. Implementation of pixel formats
// can vary between different versions of iOS, so ensure that your choice of pixel
// format is compatible with your minimum target version.
//
// I want it to be UANorm

use crate::{
    point, size, AtlasTextureId, DevicePixels, GlyphRasterParams, MetalAtlas, MonochromeSprite,
    Quad, Scene, Size,
};
use cocoa::{
    base::{NO, YES},
    foundation::NSUInteger,
    quartzcore::AutoresizingMask,
};
use metal::{CommandQueue, MTLPixelFormat, MTLResourceOptions, NSRange};
use objc::{self, msg_send, sel, sel_impl};
use std::{ffi::c_void, mem, ptr, sync::Arc};

const SHADERS_METALLIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));
const INSTANCE_BUFFER_SIZE: usize = 8192 * 1024; // This is an arbitrary decision. There's probably a more optimal value.

pub struct MetalRenderer {
    layer: metal::MetalLayer,
    command_queue: CommandQueue,
    quads_pipeline_state: metal::RenderPipelineState,
    sprites_pipeline_state: metal::RenderPipelineState,
    unit_vertices: metal::Buffer,
    instances: metal::Buffer,
    glyph_atlas: Arc<MetalAtlas<GlyphRasterParams>>,
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

        let quads_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "quads",
            "quad_vertex",
            "quad_fragment",
            PIXEL_FORMAT,
        );

        let sprites_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "sprites",
            "monochrome_sprite_vertex",
            "monochrome_sprite_fragment",
            PIXEL_FORMAT,
        );

        let command_queue = device.new_command_queue();
        let glyph_atlas = Arc::new(MetalAtlas::new(
            Size {
                width: DevicePixels(1024),
                height: DevicePixels(768),
            },
            MTLPixelFormat::A8Unorm,
            device.clone(),
        ));

        Self {
            layer,
            command_queue,
            quads_pipeline_state,
            sprites_pipeline_state,
            unit_vertices,
            instances,
            glyph_atlas,
        }
    }

    pub fn layer(&self) -> &metal::MetalLayerRef {
        &*self.layer
    }

    pub fn glyph_atlas(&self) -> &Arc<MetalAtlas<GlyphRasterParams>> {
        &self.glyph_atlas
    }

    pub fn draw(&mut self, scene: &mut Scene) {
        let layer = self.layer.clone();
        let viewport_size = layer.drawable_size();
        let viewport_size: Size<DevicePixels> = size(
            (viewport_size.width.ceil() as i32).into(),
            (viewport_size.height.ceil() as i32).into(),
        );
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

        // let depth_texture_desc = metal::TextureDescriptor::new();
        // depth_texture_desc.set_pixel_format(metal::MTLPixelFormat::Depth32Float);
        // depth_texture_desc.set_storage_mode(metal::MTLStorageMode::Private);
        // depth_texture_desc.set_usage(metal::MTLTextureUsage::RenderTarget);
        // depth_texture_desc.set_width(i32::from(viewport_size.width) as u64);
        // depth_texture_desc.set_height(i32::from(viewport_size.height) as u64);
        // let depth_texture = self.device.new_texture(&depth_texture_desc);
        // let depth_attachment = render_pass_descriptor.depth_attachment().unwrap();
        // depth_attachment.set_texture(Some(&depth_texture));
        // depth_attachment.set_clear_depth(1.);
        // depth_attachment.set_store_action(metal::MTLStoreAction::Store);

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
            width: i32::from(viewport_size.width) as f64,
            height: i32::from(viewport_size.height) as f64,
            znear: 0.0,
            zfar: 1.0,
        });

        let mut instance_offset = 0;
        for layer in scene.layers() {
            for batch in layer.batches() {
                match batch {
                    crate::PrimitiveBatch::Quads(quads) => {
                        self.draw_quads(
                            quads,
                            &mut instance_offset,
                            viewport_size,
                            command_encoder,
                        );
                    }
                    crate::PrimitiveBatch::Sprites {
                        texture_id,
                        sprites,
                    } => {
                        self.draw_monochrome_sprites(
                            texture_id,
                            sprites,
                            &mut instance_offset,
                            viewport_size,
                            command_encoder,
                        );
                    }
                }
            }
        }

        command_encoder.end_encoding();

        self.instances.did_modify_range(NSRange {
            location: 0,
            length: instance_offset as NSUInteger,
        });

        command_buffer.commit();
        command_buffer.wait_until_completed();
        drawable.present();
    }

    fn draw_quads(
        &mut self,
        quads: &[Quad],
        offset: &mut usize,
        viewport_size: Size<DevicePixels>,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if quads.is_empty() {
            return;
        }
        align_offset(offset);

        command_encoder.set_render_pipeline_state(&self.quads_pipeline_state);
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
        command_encoder.set_fragment_buffer(
            QuadInputIndex::Quads as u64,
            Some(&self.instances),
            *offset as u64,
        );

        command_encoder.set_vertex_bytes(
            QuadInputIndex::ViewportSize as u64,
            mem::size_of_val(&viewport_size) as u64,
            &viewport_size as *const Size<DevicePixels> as *const _,
        );

        let quad_bytes_len = mem::size_of::<Quad>() * quads.len();
        let buffer_contents = unsafe { (self.instances.contents() as *mut u8).add(*offset) };
        unsafe {
            ptr::copy_nonoverlapping(quads.as_ptr() as *const u8, buffer_contents, quad_bytes_len);
        }

        let next_offset = *offset + quad_bytes_len;
        assert!(
            next_offset <= INSTANCE_BUFFER_SIZE,
            "instance buffer exhausted"
        );

        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            quads.len() as u64,
        );
        *offset = next_offset;
    }

    fn draw_monochrome_sprites(
        &mut self,
        texture_id: AtlasTextureId,
        sprites: &[MonochromeSprite],
        offset: &mut usize,
        viewport_size: Size<DevicePixels>,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if sprites.is_empty() {
            return;
        }
        align_offset(offset);

        let texture = self.glyph_atlas.texture(texture_id);
        let texture_size = size(
            DevicePixels(texture.width() as i32),
            DevicePixels(texture.height() as i32),
        );
        command_encoder.set_render_pipeline_state(&self.sprites_pipeline_state);
        command_encoder.set_vertex_buffer(
            MonochromeSpriteInputIndex::Vertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_buffer(
            MonochromeSpriteInputIndex::Sprites as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_vertex_bytes(
            MonochromeSpriteInputIndex::ViewportSize as u64,
            mem::size_of_val(&viewport_size) as u64,
            &viewport_size as *const Size<DevicePixels> as *const _,
        );
        command_encoder.set_vertex_bytes(
            MonochromeSpriteInputIndex::AtlasTextureSize as u64,
            mem::size_of_val(&texture_size) as u64,
            &texture_size as *const Size<DevicePixels> as *const _,
        );
        command_encoder.set_fragment_buffer(
            MonochromeSpriteInputIndex::Sprites as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_fragment_texture(
            MonochromeSpriteInputIndex::AtlasTexture as u64,
            Some(&texture),
        );

        let sprite_bytes_len = mem::size_of::<MonochromeSprite>() * sprites.len();
        let buffer_contents = unsafe { (self.instances.contents() as *mut u8).add(*offset) };
        unsafe {
            ptr::copy_nonoverlapping(
                sprites.as_ptr() as *const u8,
                buffer_contents,
                sprite_bytes_len,
            );
        }

        let next_offset = *offset + sprite_bytes_len;
        assert!(
            next_offset <= INSTANCE_BUFFER_SIZE,
            "instance buffer exhausted"
        );

        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            sprites.len() as u64,
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
    descriptor.set_depth_attachment_pixel_format(MTLPixelFormat::Invalid);

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
    ViewportSize = 2,
}

#[repr(C)]
enum MonochromeSpriteInputIndex {
    Vertices = 0,
    Sprites = 1,
    ViewportSize = 2,
    AtlasTextureSize = 3,
    AtlasTexture = 4,
}
