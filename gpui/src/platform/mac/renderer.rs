use super::{sprite_cache::SpriteCache, window::RenderContext};
use crate::{
    color::ColorU,
    geometry::vector::{vec2i, Vector2I},
    platform,
    scene::Layer,
    Scene,
};
use anyhow::{anyhow, Result};
use metal::{MTLResourceOptions, NSRange};
use shaders::{ToFloat2 as _, ToUchar4 as _};
use std::{collections::HashMap, ffi::c_void, mem, sync::Arc};

const SHADERS_METALLIB: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));
const INSTANCE_BUFFER_SIZE: usize = 1024 * 1024; // This is an arbitrary decision. There's probably a more optimal value.

pub struct Renderer {
    sprite_cache: SpriteCache,
    quad_pipeline_state: metal::RenderPipelineState,
    shadow_pipeline_state: metal::RenderPipelineState,
    sprite_pipeline_state: metal::RenderPipelineState,
    unit_vertices: metal::Buffer,
    instances: metal::Buffer,
}

impl Renderer {
    pub fn new(
        device: metal::Device,
        pixel_format: metal::MTLPixelFormat,
        fonts: Arc<dyn platform::FontSystem>,
    ) -> Result<Self> {
        let library = device
            .new_library_with_data(SHADERS_METALLIB)
            .map_err(|message| anyhow!("error building metal library: {}", message))?;

        let unit_vertices = [
            (0., 0.).to_float2(),
            (1., 0.).to_float2(),
            (0., 1.).to_float2(),
            (0., 1.).to_float2(),
            (1., 0.).to_float2(),
            (1., 1.).to_float2(),
        ];
        let unit_vertices = device.new_buffer_with_data(
            unit_vertices.as_ptr() as *const c_void,
            (unit_vertices.len() * mem::size_of::<shaders::vector_float2>()) as u64,
            MTLResourceOptions::StorageModeManaged,
        );
        let instances = device.new_buffer(
            INSTANCE_BUFFER_SIZE as u64,
            MTLResourceOptions::StorageModeManaged,
        );

        let atlas_size: Vector2I = vec2i(1024, 768);
        Ok(Self {
            sprite_cache: SpriteCache::new(device.clone(), atlas_size, fonts),
            quad_pipeline_state: build_pipeline_state(
                &device,
                &library,
                "quad",
                "quad_vertex",
                "quad_fragment",
                pixel_format,
            )?,
            shadow_pipeline_state: build_pipeline_state(
                &device,
                &library,
                "shadow",
                "shadow_vertex",
                "shadow_fragment",
                pixel_format,
            )?,
            sprite_pipeline_state: build_pipeline_state(
                &device,
                &library,
                "sprite",
                "sprite_vertex",
                "sprite_fragment",
                pixel_format,
            )?,
            unit_vertices,
            instances,
        })
    }

    pub fn render(&mut self, scene: &Scene, ctx: &RenderContext) {
        ctx.command_encoder.set_viewport(metal::MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: ctx.drawable_size.x() as f64,
            height: ctx.drawable_size.y() as f64,
            znear: 0.0,
            zfar: 1.0,
        });

        let mut offset = 0;
        for layer in scene.layers() {
            self.render_shadows(scene, layer, &mut offset, ctx);
            self.render_quads(scene, layer, &mut offset, ctx);
            self.render_sprites(scene, layer, &mut offset, ctx);
        }
    }

    fn render_shadows(
        &mut self,
        scene: &Scene,
        layer: &Layer,
        offset: &mut usize,
        ctx: &RenderContext,
    ) {
        if layer.shadows().is_empty() {
            return;
        }

        align_offset(offset);
        let next_offset = *offset + layer.shadows().len() * mem::size_of::<shaders::GPUIShadow>();
        assert!(
            next_offset <= INSTANCE_BUFFER_SIZE,
            "instance buffer exhausted"
        );

        ctx.command_encoder
            .set_render_pipeline_state(&self.shadow_pipeline_state);
        ctx.command_encoder.set_vertex_buffer(
            shaders::GPUIShadowInputIndex_GPUIShadowInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        ctx.command_encoder.set_vertex_buffer(
            shaders::GPUIShadowInputIndex_GPUIShadowInputIndexShadows as u64,
            Some(&self.instances),
            *offset as u64,
        );
        ctx.command_encoder.set_vertex_bytes(
            shaders::GPUIShadowInputIndex_GPUIShadowInputIndexUniforms as u64,
            mem::size_of::<shaders::GPUIUniforms>() as u64,
            [shaders::GPUIUniforms {
                viewport_size: ctx.drawable_size.to_float2(),
            }]
            .as_ptr() as *const c_void,
        );

        let buffer_contents = unsafe {
            (self.instances.contents() as *mut u8).offset(*offset as isize)
                as *mut shaders::GPUIShadow
        };
        for (ix, shadow) in layer.shadows().iter().enumerate() {
            let shape_bounds = shadow.bounds * scene.scale_factor();
            let shader_shadow = shaders::GPUIShadow {
                origin: shape_bounds.origin().to_float2(),
                size: shape_bounds.size().to_float2(),
                corner_radius: shadow.corner_radius * scene.scale_factor(),
                sigma: shadow.sigma,
                color: shadow.color.to_uchar4(),
            };
            unsafe {
                *(buffer_contents.offset(ix as isize)) = shader_shadow;
            }
        }

        self.instances.did_modify_range(NSRange {
            location: *offset as u64,
            length: (next_offset - *offset) as u64,
        });
        *offset = next_offset;

        ctx.command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            layer.shadows().len() as u64,
        );
    }

    fn render_quads(
        &mut self,
        scene: &Scene,
        layer: &Layer,
        offset: &mut usize,
        ctx: &RenderContext,
    ) {
        if layer.quads().is_empty() {
            return;
        }
        align_offset(offset);
        let next_offset = *offset + layer.quads().len() * mem::size_of::<shaders::GPUIQuad>();
        assert!(
            next_offset <= INSTANCE_BUFFER_SIZE,
            "instance buffer exhausted"
        );

        ctx.command_encoder
            .set_render_pipeline_state(&self.quad_pipeline_state);
        ctx.command_encoder.set_vertex_buffer(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        ctx.command_encoder.set_vertex_buffer(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexQuads as u64,
            Some(&self.instances),
            *offset as u64,
        );
        ctx.command_encoder.set_vertex_bytes(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexUniforms as u64,
            mem::size_of::<shaders::GPUIUniforms>() as u64,
            [shaders::GPUIUniforms {
                viewport_size: ctx.drawable_size.to_float2(),
            }]
            .as_ptr() as *const c_void,
        );

        let buffer_contents = unsafe {
            (self.instances.contents() as *mut u8).offset(*offset as isize)
                as *mut shaders::GPUIQuad
        };
        for (ix, quad) in layer.quads().iter().enumerate() {
            let bounds = quad.bounds * scene.scale_factor();
            let border_width = quad.border.width * scene.scale_factor();
            let shader_quad = shaders::GPUIQuad {
                origin: bounds.origin().to_float2(),
                size: bounds.size().to_float2(),
                background_color: quad
                    .background
                    .unwrap_or(ColorU::transparent_black())
                    .to_uchar4(),
                border_top: border_width * (quad.border.top as usize as f32),
                border_right: border_width * (quad.border.right as usize as f32),
                border_bottom: border_width * (quad.border.bottom as usize as f32),
                border_left: border_width * (quad.border.left as usize as f32),
                border_color: quad
                    .border
                    .color
                    .unwrap_or(ColorU::transparent_black())
                    .to_uchar4(),
                corner_radius: quad.corner_radius * scene.scale_factor(),
            };
            unsafe {
                *(buffer_contents.offset(ix as isize)) = shader_quad;
            }
        }

        self.instances.did_modify_range(NSRange {
            location: *offset as u64,
            length: (next_offset - *offset) as u64,
        });
        *offset = next_offset;

        ctx.command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            layer.quads().len() as u64,
        );
    }

    fn render_sprites(
        &mut self,
        scene: &Scene,
        layer: &Layer,
        offset: &mut usize,
        ctx: &RenderContext,
    ) {
        if layer.glyphs().is_empty() {
            return;
        }

        let mut sprites_by_atlas = HashMap::new();
        for glyph in layer.glyphs() {
            if let Some(sprite) = self.sprite_cache.render_glyph(
                glyph.font_id,
                glyph.font_size,
                glyph.id,
                glyph.origin.x(),
                scene.scale_factor(),
            ) {
                sprites_by_atlas
                    .entry(sprite.atlas_id)
                    .or_insert_with(Vec::new)
                    .push(shaders::GPUISprite {
                        origin: (glyph.origin * scene.scale_factor() + sprite.offset).to_float2(),
                        size: sprite.size.to_float2(),
                        atlas_origin: sprite.atlas_origin.to_float2(),
                        color: glyph.color.to_uchar4(),
                    });
            }
        }

        ctx.command_encoder
            .set_render_pipeline_state(&self.sprite_pipeline_state);
        ctx.command_encoder.set_vertex_buffer(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        ctx.command_encoder.set_vertex_bytes(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexViewportSize as u64,
            mem::size_of::<shaders::vector_float2>() as u64,
            [ctx.drawable_size.to_float2()].as_ptr() as *const c_void,
        );
        ctx.command_encoder.set_vertex_bytes(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexAtlasSize as u64,
            mem::size_of::<shaders::vector_float2>() as u64,
            [self.sprite_cache.atlas_size().to_float2()].as_ptr() as *const c_void,
        );

        for (atlas_id, sprites) in sprites_by_atlas {
            align_offset(offset);
            let next_offset = *offset + sprites.len() * mem::size_of::<shaders::GPUISprite>();
            assert!(
                next_offset <= INSTANCE_BUFFER_SIZE,
                "instance buffer exhausted"
            );

            ctx.command_encoder.set_vertex_buffer(
                shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexSprites as u64,
                Some(&self.instances),
                *offset as u64,
            );

            let texture = self.sprite_cache.atlas_texture(atlas_id).unwrap();
            ctx.command_encoder.set_fragment_texture(
                shaders::GPUISpriteFragmentInputIndex_GPUISpriteFragmentInputIndexAtlas as u64,
                Some(texture),
            );

            unsafe {
                let buffer_contents = (self.instances.contents() as *mut u8)
                    .offset(*offset as isize)
                    as *mut shaders::GPUISprite;
                std::ptr::copy_nonoverlapping(sprites.as_ptr(), buffer_contents, sprites.len());
            }
            self.instances.did_modify_range(NSRange {
                location: *offset as u64,
                length: (next_offset - *offset) as u64,
            });
            *offset = next_offset;

            ctx.command_encoder.draw_primitives_instanced(
                metal::MTLPrimitiveType::Triangle,
                0,
                6,
                sprites.len() as u64,
            );
        }
    }
}

fn align_offset(offset: &mut usize) {
    let r = *offset % 256;
    if r > 0 {
        *offset += 256 - r; // Align to a multiple of 256 to make Metal happy
    }
}

fn build_pipeline_state(
    device: &metal::DeviceRef,
    library: &metal::LibraryRef,
    label: &str,
    vertex_fn_name: &str,
    fragment_fn_name: &str,
    pixel_format: metal::MTLPixelFormat,
) -> Result<metal::RenderPipelineState> {
    let vertex_fn = library
        .get_function(vertex_fn_name, None)
        .map_err(|message| anyhow!("error locating vertex function: {}", message))?;
    let fragment_fn = library
        .get_function(fragment_fn_name, None)
        .map_err(|message| anyhow!("error locating fragment function: {}", message))?;

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
    color_attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    color_attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
    color_attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);

    device
        .new_render_pipeline_state(&descriptor)
        .map_err(|message| anyhow!("could not create render pipeline state: {}", message))
}

mod shaders {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    use pathfinder_geometry::vector::Vector2I;

    use crate::{color::ColorU, geometry::vector::Vector2F};
    use std::mem;

    include!(concat!(env!("OUT_DIR"), "/shaders.rs"));

    pub trait ToFloat2 {
        fn to_float2(&self) -> vector_float2;
    }

    pub trait ToUchar4 {
        fn to_uchar4(&self) -> vector_uchar4;
    }

    impl ToFloat2 for (f32, f32) {
        fn to_float2(&self) -> vector_float2 {
            unsafe {
                let mut output = mem::transmute::<_, u32>(self.1.to_bits()) as vector_float2;
                output <<= 32;
                output |= mem::transmute::<_, u32>(self.0.to_bits()) as vector_float2;
                output
            }
        }
    }

    impl ToFloat2 for Vector2F {
        fn to_float2(&self) -> vector_float2 {
            unsafe {
                let mut output = mem::transmute::<_, u32>(self.y().to_bits()) as vector_float2;
                output <<= 32;
                output |= mem::transmute::<_, u32>(self.x().to_bits()) as vector_float2;
                output
            }
        }
    }

    impl ToFloat2 for Vector2I {
        fn to_float2(&self) -> vector_float2 {
            self.to_f32().to_float2()
        }
    }

    impl ToUchar4 for ColorU {
        fn to_uchar4(&self) -> vector_uchar4 {
            let mut vec = self.a as vector_uchar4;
            vec <<= 8;
            vec |= self.b as vector_uchar4;
            vec <<= 8;
            vec |= self.g as vector_uchar4;
            vec <<= 8;
            vec |= self.r as vector_uchar4;
            vec
        }
    }
}
