use super::{atlas::AtlasAllocator, sprite_cache::SpriteCache};
use crate::{
    color::ColorU,
    geometry::{
        rect::RectF,
        vector::{vec2f, vec2i, Vector2F, Vector2I},
    },
    platform,
    scene::Layer,
    Scene,
};
use anyhow::{anyhow, Result};
use cocoa::foundation::NSUInteger;
use metal::{MTLPixelFormat, MTLResourceOptions, NSRange};
use shaders::{ToFloat2 as _, ToUchar4 as _};
use std::{collections::HashMap, ffi::c_void, mem, sync::Arc};

const SHADERS_METALLIB: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));
const INSTANCE_BUFFER_SIZE: usize = 1024 * 1024; // This is an arbitrary decision. There's probably a more optimal value.

pub struct Renderer {
    device: metal::Device,
    sprite_cache: SpriteCache,
    path_atlasses: AtlasAllocator,
    quad_pipeline_state: metal::RenderPipelineState,
    shadow_pipeline_state: metal::RenderPipelineState,
    sprite_pipeline_state: metal::RenderPipelineState,
    path_stencil_pipeline_state: metal::RenderPipelineState,
    unit_vertices: metal::Buffer,
    instances: metal::Buffer,
}

struct PathSprite {
    layer_id: usize,
    atlas_id: usize,
    sprite: shaders::GPUISprite,
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

        let sprite_cache = SpriteCache::new(device.clone(), vec2i(1024, 768), fonts);
        let path_atlasses = build_path_atlas_allocator(pixel_format, &device);
        let quad_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "quad",
            "quad_vertex",
            "quad_fragment",
            pixel_format,
        )?;
        let shadow_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "shadow",
            "shadow_vertex",
            "shadow_fragment",
            pixel_format,
        )?;
        let sprite_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "sprite",
            "sprite_vertex",
            "sprite_fragment",
            pixel_format,
        )?;
        let path_stencil_pipeline_state = build_stencil_pipeline_state(
            &device,
            &library,
            "path_winding",
            "path_winding_vertex",
            "path_winding_fragment",
            pixel_format,
        )?;
        Ok(Self {
            device,
            sprite_cache,
            path_atlasses,
            quad_pipeline_state,
            shadow_pipeline_state,
            sprite_pipeline_state,
            path_stencil_pipeline_state,
            unit_vertices,
            instances,
        })
    }

    pub fn render(
        &mut self,
        scene: &Scene,
        drawable_size: Vector2F,
        command_buffer: &metal::CommandBufferRef,
        output: &metal::TextureRef,
    ) {
        let mut offset = 0;
        let stencils = self.render_path_stencils(scene, &mut offset, command_buffer);
        self.render_layers(
            scene,
            stencils,
            &mut offset,
            drawable_size,
            command_buffer,
            output,
        );
    }

    fn render_path_stencils(
        &mut self,
        scene: &Scene,
        offset: &mut usize,
        command_buffer: &metal::CommandBufferRef,
    ) -> Vec<PathSprite> {
        let mut stencils = Vec::new();
        let mut vertices = Vec::<shaders::GPUIPathVertex>::new();
        let mut current_atlas_id = None;
        for (layer_id, layer) in scene.layers().iter().enumerate() {
            for path in layer.paths() {
                // Push a PathStencil struct for use later when sampling from the atlas as we draw the content of the layers
                let origin = path.bounds.origin() * scene.scale_factor();
                let size = (path.bounds.size() * scene.scale_factor()).ceil();
                let (atlas_id, atlas_origin) =
                    self.path_atlasses.allocate(size.ceil().to_i32()).unwrap();
                let atlas_origin = atlas_origin.to_f32();
                stencils.push(PathSprite {
                    layer_id,
                    atlas_id,
                    sprite: shaders::GPUISprite {
                        origin: origin.floor().to_float2(),
                        size: size.to_float2(),
                        atlas_origin: atlas_origin.to_float2(),
                        color: path.color.to_uchar4(),
                        compute_winding: 1,
                    },
                });

                if current_atlas_id.map_or(false, |current_atlas_id| atlas_id != current_atlas_id) {
                    self.render_path_stencils_for_atlas(
                        offset,
                        &vertices,
                        atlas_id,
                        command_buffer,
                    );
                    vertices.clear();
                }

                current_atlas_id = Some(atlas_id);

                // Populate the vertices by translating them to their appropriate location in the atlas.
                for vertex in &path.vertices {
                    let xy_position =
                        (vertex.xy_position - path.bounds.origin()) * scene.scale_factor();
                    vertices.push(shaders::GPUIPathVertex {
                        xy_position: (atlas_origin + xy_position).to_float2(),
                        st_position: vertex.st_position.to_float2(),
                    });
                }
            }
        }

        if let Some(atlas_id) = current_atlas_id {
            self.render_path_stencils_for_atlas(offset, &vertices, atlas_id, command_buffer);
        }

        stencils
    }

    fn render_path_stencils_for_atlas(
        &mut self,
        offset: &mut usize,
        vertices: &[shaders::GPUIPathVertex],
        atlas_id: usize,
        command_buffer: &metal::CommandBufferRef,
    ) {
        align_offset(offset);
        let next_offset = *offset + vertices.len() * mem::size_of::<shaders::GPUIPathVertex>();
        assert!(
            next_offset <= INSTANCE_BUFFER_SIZE,
            "instance buffer exhausted"
        );

        let render_pass_descriptor = metal::RenderPassDescriptor::new();
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        let texture = self.path_atlasses.texture(atlas_id).unwrap();
        color_attachment.set_texture(Some(texture));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_store_action(metal::MTLStoreAction::Store);
        color_attachment.set_clear_color(metal::MTLClearColor::new(0., 0., 0., 1.));

        let winding_command_encoder =
            command_buffer.new_render_command_encoder(render_pass_descriptor);
        winding_command_encoder.set_render_pipeline_state(&self.path_stencil_pipeline_state);
        winding_command_encoder.set_vertex_buffer(
            shaders::GPUIPathWindingVertexInputIndex_GPUIPathWindingVertexInputIndexVertices as u64,
            Some(&self.instances),
            *offset as u64,
        );
        winding_command_encoder.set_vertex_bytes(
            shaders::GPUIPathWindingVertexInputIndex_GPUIPathWindingVertexInputIndexAtlasSize
                as u64,
            mem::size_of::<shaders::vector_float2>() as u64,
            [vec2i(texture.width() as i32, texture.height() as i32).to_float2()].as_ptr()
                as *const c_void,
        );

        let buffer_contents = unsafe {
            (self.instances.contents() as *mut u8).add(*offset) as *mut shaders::GPUIPathVertex
        };

        for (ix, vertex) in vertices.iter().enumerate() {
            unsafe {
                *buffer_contents.add(ix) = *vertex;
            }
        }

        self.instances.did_modify_range(NSRange {
            location: *offset as u64,
            length: (next_offset - *offset) as u64,
        });
        *offset = next_offset;

        winding_command_encoder.draw_primitives(
            metal::MTLPrimitiveType::Triangle,
            0,
            vertices.len() as u64,
        );
        winding_command_encoder.end_encoding();
    }

    fn render_layers(
        &mut self,
        scene: &Scene,
        path_sprites: Vec<PathSprite>,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_buffer: &metal::CommandBufferRef,
        output: &metal::TextureRef,
    ) {
        let render_pass_descriptor = metal::RenderPassDescriptor::new();
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        color_attachment.set_texture(Some(output));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_store_action(metal::MTLStoreAction::Store);
        color_attachment.set_clear_color(metal::MTLClearColor::new(0., 0., 0., 1.));
        let command_encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);

        command_encoder.set_viewport(metal::MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: drawable_size.x() as f64,
            height: drawable_size.y() as f64,
            znear: 0.0,
            zfar: 1.0,
        });

        for (layer_id, layer) in scene.layers().iter().enumerate() {
            self.clip(scene, layer, drawable_size, command_encoder);
            self.render_shadows(scene, layer, offset, drawable_size, command_encoder);
            self.render_quads(scene, layer, offset, drawable_size, command_encoder);
            // TODO: Pass sprites relevant to this layer in a more efficient manner.
            self.render_path_sprites(
                scene,
                layer,
                path_sprites.iter().filter(|s| s.layer_id == layer_id),
                offset,
                drawable_size,
                command_encoder,
            );
            self.render_glyph_sprites(scene, layer, offset, drawable_size, command_encoder);
        }

        command_encoder.end_encoding();
    }

    fn clip(
        &mut self,
        scene: &Scene,
        layer: &Layer,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        let clip_bounds = layer.clip_bounds().unwrap_or(RectF::new(
            vec2f(0., 0.),
            drawable_size / scene.scale_factor(),
        )) * scene.scale_factor();
        command_encoder.set_scissor_rect(metal::MTLScissorRect {
            x: clip_bounds.origin_x() as NSUInteger,
            y: clip_bounds.origin_y() as NSUInteger,
            width: clip_bounds.width() as NSUInteger,
            height: clip_bounds.height() as NSUInteger,
        });
    }

    fn render_shadows(
        &mut self,
        scene: &Scene,
        layer: &Layer,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
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

        command_encoder.set_render_pipeline_state(&self.shadow_pipeline_state);
        command_encoder.set_vertex_buffer(
            shaders::GPUIShadowInputIndex_GPUIShadowInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_buffer(
            shaders::GPUIShadowInputIndex_GPUIShadowInputIndexShadows as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_vertex_bytes(
            shaders::GPUIShadowInputIndex_GPUIShadowInputIndexUniforms as u64,
            mem::size_of::<shaders::GPUIUniforms>() as u64,
            [shaders::GPUIUniforms {
                viewport_size: drawable_size.to_float2(),
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

        command_encoder.draw_primitives_instanced(
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
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
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

        command_encoder.set_render_pipeline_state(&self.quad_pipeline_state);
        command_encoder.set_vertex_buffer(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_buffer(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexQuads as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_vertex_bytes(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexUniforms as u64,
            mem::size_of::<shaders::GPUIUniforms>() as u64,
            [shaders::GPUIUniforms {
                viewport_size: drawable_size.to_float2(),
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

        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            layer.quads().len() as u64,
        );
    }

    fn render_glyph_sprites(
        &mut self,
        scene: &Scene,
        layer: &Layer,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
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
                glyph.origin,
                scene.scale_factor(),
            ) {
                // Snap sprite to pixel grid.
                let origin = (glyph.origin * scene.scale_factor()).floor() + sprite.offset.to_f32();
                sprites_by_atlas
                    .entry(sprite.atlas_id)
                    .or_insert_with(Vec::new)
                    .push(shaders::GPUISprite {
                        origin: origin.to_float2(),
                        size: sprite.size.to_float2(),
                        atlas_origin: sprite.atlas_origin.to_float2(),
                        color: glyph.color.to_uchar4(),
                        compute_winding: 0,
                    });
            }
        }

        command_encoder.set_render_pipeline_state(&self.sprite_pipeline_state);
        command_encoder.set_vertex_buffer(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_bytes(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexViewportSize as u64,
            mem::size_of::<shaders::vector_float2>() as u64,
            [drawable_size.to_float2()].as_ptr() as *const c_void,
        );
        command_encoder.set_vertex_bytes(
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

            command_encoder.set_vertex_buffer(
                shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexSprites as u64,
                Some(&self.instances),
                *offset as u64,
            );

            let texture = self.sprite_cache.atlas_texture(atlas_id).unwrap();
            command_encoder.set_fragment_texture(
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

            command_encoder.draw_primitives_instanced(
                metal::MTLPrimitiveType::Triangle,
                0,
                6,
                sprites.len() as u64,
            );
        }
    }

    fn render_path_sprites<'a>(
        &mut self,
        scene: &Scene,
        layer: &Layer,
        sprites: impl Iterator<Item = &'a PathSprite>,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        let mut sprites = sprites.peekable();
        if sprites.peek().is_none() {
            return;
        }

        let mut sprites_by_atlas = HashMap::new();
        for sprite in sprites {
            sprites_by_atlas
                .entry(sprite.atlas_id)
                .or_insert_with(Vec::new)
                .push(sprite.sprite);
        }

        command_encoder.set_render_pipeline_state(&self.sprite_pipeline_state);
        command_encoder.set_vertex_buffer(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_bytes(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexViewportSize as u64,
            mem::size_of::<shaders::vector_float2>() as u64,
            [drawable_size.to_float2()].as_ptr() as *const c_void,
        );

        for (atlas_id, sprites) in sprites_by_atlas {
            align_offset(offset);
            let next_offset = *offset + sprites.len() * mem::size_of::<shaders::GPUISprite>();
            assert!(
                next_offset <= INSTANCE_BUFFER_SIZE,
                "instance buffer exhausted"
            );

            command_encoder.set_vertex_buffer(
                shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexSprites as u64,
                Some(&self.instances),
                *offset as u64,
            );

            let texture = self.path_atlasses.texture(atlas_id).unwrap();
            command_encoder.set_vertex_bytes(
                shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexAtlasSize as u64,
                mem::size_of::<shaders::vector_float2>() as u64,
                [vec2i(texture.width() as i32, texture.height() as i32).to_float2()].as_ptr()
                    as *const c_void,
            );
            command_encoder.set_fragment_texture(
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

            command_encoder.draw_primitives_instanced(
                metal::MTLPrimitiveType::Triangle,
                0,
                6,
                sprites.len() as u64,
            );
        }
    }
}

fn build_path_atlas_allocator(
    pixel_format: MTLPixelFormat,
    device: &metal::Device,
) -> AtlasAllocator {
    let path_stencil_descriptor = metal::TextureDescriptor::new();
    path_stencil_descriptor.set_width(2048);
    path_stencil_descriptor.set_height(2048);
    path_stencil_descriptor.set_pixel_format(pixel_format);
    path_stencil_descriptor
        .set_usage(metal::MTLTextureUsage::RenderTarget | metal::MTLTextureUsage::ShaderRead);
    path_stencil_descriptor.set_storage_mode(metal::MTLStorageMode::Private);
    let path_atlasses = AtlasAllocator::new(device.clone(), path_stencil_descriptor);
    path_atlasses
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

fn build_stencil_pipeline_state(
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
    color_attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::One);

    device
        .new_render_pipeline_state(&descriptor)
        .map_err(|message| anyhow!("could not create render pipeline state: {}", message))
}

// fn build_stencil_pipeline_state(
//     device: &metal::DeviceRef,
//     library: &metal::LibraryRef,
//     label: &str,
//     vertex_fn_name: &str,
//     fragment_fn_name: &str,
//     pixel_format: metal::MTLPixelFormat,
// ) -> Result<metal::RenderPipelineState> {
//     let vertex_fn = library
//         .get_function(vertex_fn_name, None)
//         .map_err(|message| anyhow!("error locating vertex function: {}", message))?;
//     let fragment_fn = library
//         .get_function(fragment_fn_name, None)
//         .map_err(|message| anyhow!("error locating fragment function: {}", message))?;

//     let descriptor = metal::RenderPipelineDescriptor::new();
//     descriptor.set_label(label);
//     descriptor.set_vertex_function(Some(vertex_fn.as_ref()));
//     descriptor.set_fragment_function(Some(fragment_fn.as_ref()));
//     descriptor.set_stencil_attachment_pixel_format(pixel_format);

//     device
//         .new_render_pipeline_state(&descriptor)
//         .map_err(|message| anyhow!("could not create render pipeline state: {}", message))
// }

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
