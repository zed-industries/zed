use super::{atlas::AtlasAllocator, image_cache::ImageCache, sprite_cache::SpriteCache};
use crate::{
    color::Color,
    geometry::{
        rect::RectF,
        vector::{vec2f, vec2i, Vector2F},
    },
    platform,
    scene::{Glyph, Icon, Image, ImageGlyph, Layer, Quad, Scene, Shadow, Underline},
};
use cocoa::foundation::NSUInteger;
use log::warn;
use metal::{MTLPixelFormat, MTLResourceOptions, NSRange};
use shaders::ToFloat2 as _;
use std::{collections::HashMap, ffi::c_void, iter::Peekable, mem, sync::Arc, vec};

const SHADERS_METALLIB: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));
const INSTANCE_BUFFER_SIZE: usize = 8192 * 1024; // This is an arbitrary decision. There's probably a more optimal value.

pub struct Renderer {
    sprite_cache: SpriteCache,
    image_cache: ImageCache,
    path_atlases: AtlasAllocator,
    quad_pipeline_state: metal::RenderPipelineState,
    shadow_pipeline_state: metal::RenderPipelineState,
    sprite_pipeline_state: metal::RenderPipelineState,
    image_pipeline_state: metal::RenderPipelineState,
    path_atlas_pipeline_state: metal::RenderPipelineState,
    underline_pipeline_state: metal::RenderPipelineState,
    unit_vertices: metal::Buffer,
    instances: metal::Buffer,
}

struct PathSprite {
    layer_id: usize,
    atlas_id: usize,
    shader_data: shaders::GPUISprite,
}

impl Renderer {
    pub fn new(
        device: metal::Device,
        pixel_format: metal::MTLPixelFormat,
        scale_factor: f32,
        fonts: Arc<dyn platform::FontSystem>,
    ) -> Self {
        let library = device
            .new_library_with_data(SHADERS_METALLIB)
            .expect("error building metal library");

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

        let sprite_cache = SpriteCache::new(
            device.clone(),
            vec2i(1024, 768),
            scale_factor,
            fonts.clone(),
        );
        let image_cache = ImageCache::new(device.clone(), vec2i(1024, 768), scale_factor, fonts);
        let path_atlases =
            AtlasAllocator::new(device.clone(), build_path_atlas_texture_descriptor());
        let quad_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "quad",
            "quad_vertex",
            "quad_fragment",
            pixel_format,
        );
        let shadow_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "shadow",
            "shadow_vertex",
            "shadow_fragment",
            pixel_format,
        );
        let sprite_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "sprite",
            "sprite_vertex",
            "sprite_fragment",
            pixel_format,
        );
        let image_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "image",
            "image_vertex",
            "image_fragment",
            pixel_format,
        );
        let path_atlas_pipeline_state = build_path_atlas_pipeline_state(
            &device,
            &library,
            "path_atlas",
            "path_atlas_vertex",
            "path_atlas_fragment",
            MTLPixelFormat::R16Float,
        );
        let underline_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "underline",
            "underline_vertex",
            "underline_fragment",
            pixel_format,
        );
        Self {
            sprite_cache,
            image_cache,
            path_atlases,
            quad_pipeline_state,
            shadow_pipeline_state,
            sprite_pipeline_state,
            image_pipeline_state,
            path_atlas_pipeline_state,
            underline_pipeline_state,
            unit_vertices,
            instances,
        }
    }

    pub fn render(
        &mut self,
        scene: &Scene,
        drawable_size: Vector2F,
        command_buffer: &metal::CommandBufferRef,
        output: &metal::TextureRef,
    ) {
        self.sprite_cache.set_scale_factor(scene.scale_factor());
        self.image_cache.set_scale_factor(scene.scale_factor());

        let mut offset = 0;

        let path_sprites = self.render_path_atlases(scene, &mut offset, command_buffer);
        self.render_layers(
            scene,
            path_sprites,
            &mut offset,
            drawable_size,
            command_buffer,
            output,
        );
        self.instances.did_modify_range(NSRange {
            location: 0,
            length: offset as NSUInteger,
        });
        self.image_cache.finish_frame();
    }

    fn render_path_atlases(
        &mut self,
        scene: &Scene,
        offset: &mut usize,
        command_buffer: &metal::CommandBufferRef,
    ) -> Vec<PathSprite> {
        self.path_atlases.clear();
        let mut sprites = Vec::new();
        let mut vertices = Vec::<shaders::GPUIPathVertex>::new();
        let mut current_atlas_id = None;
        for (layer_id, layer) in scene.layers().enumerate() {
            for path in layer.paths() {
                let origin = path.bounds.origin() * scene.scale_factor();
                let size = (path.bounds.size() * scene.scale_factor()).ceil();

                let path_allocation = self.path_atlases.allocate(size.to_i32());
                if path_allocation.is_none() {
                    // Path size was likely zero.
                    warn!("could not allocate path texture of size {:?}", size);
                    continue;
                }
                let (alloc_id, atlas_origin) = path_allocation.unwrap();
                let atlas_origin = atlas_origin.to_f32();
                sprites.push(PathSprite {
                    layer_id,
                    atlas_id: alloc_id.atlas_id,
                    shader_data: shaders::GPUISprite {
                        origin: origin.floor().to_float2(),
                        target_size: size.to_float2(),
                        source_size: size.to_float2(),
                        atlas_origin: atlas_origin.to_float2(),
                        color: path.color.to_uchar4(),
                        compute_winding: 1,
                    },
                });

                if let Some(current_atlas_id) = current_atlas_id {
                    if alloc_id.atlas_id != current_atlas_id {
                        self.render_paths_to_atlas(
                            offset,
                            &vertices,
                            current_atlas_id,
                            command_buffer,
                        );
                        vertices.clear();
                    }
                }

                current_atlas_id = Some(alloc_id.atlas_id);

                for vertex in &path.vertices {
                    let xy_position =
                        (vertex.xy_position - path.bounds.origin()) * scene.scale_factor();
                    vertices.push(shaders::GPUIPathVertex {
                        xy_position: (atlas_origin + xy_position).to_float2(),
                        st_position: vertex.st_position.to_float2(),
                        clip_rect_origin: atlas_origin.to_float2(),
                        clip_rect_size: size.to_float2(),
                    });
                }
            }
        }

        if let Some(atlas_id) = current_atlas_id {
            self.render_paths_to_atlas(offset, &vertices, atlas_id, command_buffer);
        }

        sprites
    }

    fn render_paths_to_atlas(
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
        let texture = self.path_atlases.texture(atlas_id).unwrap();
        color_attachment.set_texture(Some(texture));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_store_action(metal::MTLStoreAction::Store);
        color_attachment.set_clear_color(metal::MTLClearColor::new(0., 0., 0., 1.));

        let path_atlas_command_encoder =
            command_buffer.new_render_command_encoder(render_pass_descriptor);
        path_atlas_command_encoder.set_render_pipeline_state(&self.path_atlas_pipeline_state);
        path_atlas_command_encoder.set_vertex_buffer(
            shaders::GPUIPathAtlasVertexInputIndex_GPUIPathAtlasVertexInputIndexVertices as u64,
            Some(&self.instances),
            *offset as u64,
        );
        path_atlas_command_encoder.set_vertex_bytes(
            shaders::GPUIPathAtlasVertexInputIndex_GPUIPathAtlasVertexInputIndexAtlasSize as u64,
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

        path_atlas_command_encoder.draw_primitives(
            metal::MTLPrimitiveType::Triangle,
            0,
            vertices.len() as u64,
        );
        path_atlas_command_encoder.end_encoding();
        *offset = next_offset;
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

        let scale_factor = scene.scale_factor();
        let mut path_sprites = path_sprites.into_iter().peekable();
        for (layer_id, layer) in scene.layers().enumerate() {
            self.clip(scene, layer, drawable_size, command_encoder);
            self.render_shadows(
                layer.shadows(),
                scale_factor,
                offset,
                drawable_size,
                command_encoder,
            );
            self.render_quads(
                layer.quads(),
                scale_factor,
                offset,
                drawable_size,
                command_encoder,
            );
            self.render_path_sprites(
                layer_id,
                &mut path_sprites,
                offset,
                drawable_size,
                command_encoder,
            );
            self.render_underlines(
                layer.underlines(),
                scale_factor,
                offset,
                drawable_size,
                command_encoder,
            );
            self.render_sprites(
                layer.glyphs(),
                layer.icons(),
                scale_factor,
                offset,
                drawable_size,
                command_encoder,
            );
            self.render_images(
                layer.images(),
                layer.image_glyphs(),
                scale_factor,
                offset,
                drawable_size,
                command_encoder,
            );
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
        let clip_bounds = (layer.clip_bounds().unwrap_or(RectF::new(
            vec2f(0., 0.),
            drawable_size / scene.scale_factor(),
        )) * scene.scale_factor())
        .round();
        command_encoder.set_scissor_rect(metal::MTLScissorRect {
            x: clip_bounds.origin_x() as NSUInteger,
            y: clip_bounds.origin_y() as NSUInteger,
            width: clip_bounds.width() as NSUInteger,
            height: clip_bounds.height() as NSUInteger,
        });
    }

    fn render_shadows(
        &mut self,
        shadows: &[Shadow],
        scale_factor: f32,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if shadows.is_empty() {
            return;
        }

        align_offset(offset);
        let next_offset = *offset + shadows.len() * mem::size_of::<shaders::GPUIShadow>();
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
        for (ix, shadow) in shadows.iter().enumerate() {
            let shape_bounds = shadow.bounds * scale_factor;
            let shader_shadow = shaders::GPUIShadow {
                origin: shape_bounds.origin().to_float2(),
                size: shape_bounds.size().to_float2(),
                corner_radius: shadow.corner_radius * scale_factor,
                sigma: shadow.sigma,
                color: shadow.color.to_uchar4(),
            };
            unsafe {
                *(buffer_contents.offset(ix as isize)) = shader_shadow;
            }
        }

        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            shadows.len() as u64,
        );
        *offset = next_offset;
    }

    fn render_quads(
        &mut self,
        quads: &[Quad],
        scale_factor: f32,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if quads.is_empty() {
            return;
        }
        align_offset(offset);
        let next_offset = *offset + quads.len() * mem::size_of::<shaders::GPUIQuad>();
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
        for (ix, quad) in quads.iter().enumerate() {
            let bounds = quad.bounds * scale_factor;
            let border_width = quad.border.width * scale_factor;
            let shader_quad = shaders::GPUIQuad {
                origin: bounds.origin().round().to_float2(),
                size: bounds.size().round().to_float2(),
                background_color: quad
                    .background
                    .unwrap_or(Color::transparent_black())
                    .to_uchar4(),
                border_top: border_width * (quad.border.top as usize as f32),
                border_right: border_width * (quad.border.right as usize as f32),
                border_bottom: border_width * (quad.border.bottom as usize as f32),
                border_left: border_width * (quad.border.left as usize as f32),
                border_color: quad.border.color.to_uchar4(),
                corner_radius: quad.corner_radius * scale_factor,
            };
            unsafe {
                *(buffer_contents.offset(ix as isize)) = shader_quad;
            }
        }

        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            quads.len() as u64,
        );
        *offset = next_offset;
    }

    fn render_sprites(
        &mut self,
        glyphs: &[Glyph],
        icons: &[Icon],
        scale_factor: f32,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if glyphs.is_empty() && icons.is_empty() {
            return;
        }

        let mut sprites_by_atlas = HashMap::new();

        for glyph in glyphs {
            if let Some(sprite) = self.sprite_cache.render_glyph(
                glyph.font_id,
                glyph.font_size,
                glyph.id,
                glyph.origin,
            ) {
                // Snap sprite to pixel grid.
                let origin = (glyph.origin * scale_factor).floor() + sprite.offset.to_f32();
                sprites_by_atlas
                    .entry(sprite.atlas_id)
                    .or_insert_with(Vec::new)
                    .push(shaders::GPUISprite {
                        origin: origin.to_float2(),
                        target_size: sprite.size.to_float2(),
                        source_size: sprite.size.to_float2(),
                        atlas_origin: sprite.atlas_origin.to_float2(),
                        color: glyph.color.to_uchar4(),
                        compute_winding: 0,
                    });
            }
        }

        for icon in icons {
            // Snap sprite to pixel grid.
            let origin = (icon.bounds.origin() * scale_factor).floor();
            let target_size = (icon.bounds.size() * scale_factor).ceil();
            let source_size = (target_size * 2.).to_i32();

            let sprite =
                self.sprite_cache
                    .render_icon(source_size, icon.path.clone(), icon.svg.clone());
            if sprite.is_none() {
                continue;
            }
            let sprite = sprite.unwrap();

            sprites_by_atlas
                .entry(sprite.atlas_id)
                .or_insert_with(Vec::new)
                .push(shaders::GPUISprite {
                    origin: origin.to_float2(),
                    target_size: target_size.to_float2(),
                    source_size: sprite.size.to_float2(),
                    atlas_origin: sprite.atlas_origin.to_float2(),
                    color: icon.color.to_uchar4(),
                    compute_winding: 0,
                });
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

            let texture = self.sprite_cache.atlas_texture(atlas_id).unwrap();
            command_encoder.set_vertex_buffer(
                shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexSprites as u64,
                Some(&self.instances),
                *offset as u64,
            );
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

            command_encoder.draw_primitives_instanced(
                metal::MTLPrimitiveType::Triangle,
                0,
                6,
                sprites.len() as u64,
            );
            *offset = next_offset;
        }
    }

    fn render_images(
        &mut self,
        images: &[Image],
        image_glyphs: &[ImageGlyph],
        scale_factor: f32,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if images.is_empty() && image_glyphs.is_empty() {
            return;
        }

        let mut images_by_atlas = HashMap::new();
        for image in images {
            let origin = image.bounds.origin() * scale_factor;
            let target_size = image.bounds.size() * scale_factor;
            let corner_radius = image.corner_radius * scale_factor;
            let border_width = image.border.width * scale_factor;
            let (alloc_id, atlas_bounds) = self.image_cache.render(&image.data);
            images_by_atlas
                .entry(alloc_id.atlas_id)
                .or_insert_with(Vec::new)
                .push(shaders::GPUIImage {
                    origin: origin.to_float2(),
                    target_size: target_size.to_float2(),
                    source_size: atlas_bounds.size().to_float2(),
                    atlas_origin: atlas_bounds.origin().to_float2(),
                    border_top: border_width * (image.border.top as usize as f32),
                    border_right: border_width * (image.border.right as usize as f32),
                    border_bottom: border_width * (image.border.bottom as usize as f32),
                    border_left: border_width * (image.border.left as usize as f32),
                    border_color: image.border.color.to_uchar4(),
                    corner_radius,
                });
        }

        for image_glyph in image_glyphs {
            let origin = (image_glyph.origin * scale_factor).floor();
            if let Some((alloc_id, atlas_bounds, glyph_origin)) =
                self.image_cache.render_glyph(image_glyph)
            {
                images_by_atlas
                    .entry(alloc_id.atlas_id)
                    .or_insert_with(Vec::new)
                    .push(shaders::GPUIImage {
                        origin: (origin + glyph_origin.to_f32()).to_float2(),
                        target_size: atlas_bounds.size().to_float2(),
                        source_size: atlas_bounds.size().to_float2(),
                        atlas_origin: atlas_bounds.origin().to_float2(),
                        border_top: 0.,
                        border_right: 0.,
                        border_bottom: 0.,
                        border_left: 0.,
                        border_color: Default::default(),
                        corner_radius: 0.,
                    });
            } else {
                log::warn!("could not render glyph with id {}", image_glyph.id);
            }
        }

        command_encoder.set_render_pipeline_state(&self.image_pipeline_state);
        command_encoder.set_vertex_buffer(
            shaders::GPUIImageVertexInputIndex_GPUIImageVertexInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_bytes(
            shaders::GPUIImageVertexInputIndex_GPUIImageVertexInputIndexViewportSize as u64,
            mem::size_of::<shaders::vector_float2>() as u64,
            [drawable_size.to_float2()].as_ptr() as *const c_void,
        );

        for (atlas_id, images) in images_by_atlas {
            align_offset(offset);
            let next_offset = *offset + images.len() * mem::size_of::<shaders::GPUIImage>();
            assert!(
                next_offset <= INSTANCE_BUFFER_SIZE,
                "instance buffer exhausted"
            );

            let texture = self.image_cache.atlas_texture(atlas_id).unwrap();
            command_encoder.set_vertex_buffer(
                shaders::GPUIImageVertexInputIndex_GPUIImageVertexInputIndexImages as u64,
                Some(&self.instances),
                *offset as u64,
            );
            command_encoder.set_vertex_bytes(
                shaders::GPUIImageVertexInputIndex_GPUIImageVertexInputIndexAtlasSize as u64,
                mem::size_of::<shaders::vector_float2>() as u64,
                [vec2i(texture.width() as i32, texture.height() as i32).to_float2()].as_ptr()
                    as *const c_void,
            );
            command_encoder.set_fragment_texture(
                shaders::GPUIImageFragmentInputIndex_GPUIImageFragmentInputIndexAtlas as u64,
                Some(texture),
            );

            unsafe {
                let buffer_contents = (self.instances.contents() as *mut u8)
                    .offset(*offset as isize)
                    as *mut shaders::GPUIImage;
                std::ptr::copy_nonoverlapping(images.as_ptr(), buffer_contents, images.len());
            }

            command_encoder.draw_primitives_instanced(
                metal::MTLPrimitiveType::Triangle,
                0,
                6,
                images.len() as u64,
            );
            *offset = next_offset;
        }
    }

    fn render_path_sprites(
        &mut self,
        layer_id: usize,
        sprites: &mut Peekable<vec::IntoIter<PathSprite>>,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
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

        let mut atlas_id = None;
        let mut atlas_sprite_count = 0;
        align_offset(offset);

        while let Some(sprite) = sprites.peek() {
            if sprite.layer_id != layer_id {
                break;
            }

            let sprite = sprites.next().unwrap();
            if let Some(atlas_id) = atlas_id.as_mut() {
                if sprite.atlas_id != *atlas_id {
                    self.render_path_sprites_for_atlas(
                        offset,
                        *atlas_id,
                        atlas_sprite_count,
                        command_encoder,
                    );

                    *atlas_id = sprite.atlas_id;
                    atlas_sprite_count = 0;
                    align_offset(offset);
                }
            } else {
                atlas_id = Some(sprite.atlas_id);
            }

            unsafe {
                let buffer_contents = (self.instances.contents() as *mut u8)
                    .offset(*offset as isize)
                    as *mut shaders::GPUISprite;
                *buffer_contents.offset(atlas_sprite_count as isize) = sprite.shader_data;
            }

            atlas_sprite_count += 1;
        }

        if let Some(atlas_id) = atlas_id {
            self.render_path_sprites_for_atlas(
                offset,
                atlas_id,
                atlas_sprite_count,
                command_encoder,
            );
        }
    }

    fn render_path_sprites_for_atlas(
        &mut self,
        offset: &mut usize,
        atlas_id: usize,
        sprite_count: usize,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        let next_offset = *offset + sprite_count * mem::size_of::<shaders::GPUISprite>();
        assert!(
            next_offset <= INSTANCE_BUFFER_SIZE,
            "instance buffer exhausted"
        );
        command_encoder.set_vertex_buffer(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexSprites as u64,
            Some(&self.instances),
            *offset as u64,
        );
        let texture = self.path_atlases.texture(atlas_id).unwrap();
        command_encoder.set_fragment_texture(
            shaders::GPUISpriteFragmentInputIndex_GPUISpriteFragmentInputIndexAtlas as u64,
            Some(texture),
        );
        command_encoder.set_vertex_bytes(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexAtlasSize as u64,
            mem::size_of::<shaders::vector_float2>() as u64,
            [vec2i(texture.width() as i32, texture.height() as i32).to_float2()].as_ptr()
                as *const c_void,
        );

        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            sprite_count as u64,
        );
        *offset = next_offset;
    }

    fn render_underlines(
        &mut self,
        underlines: &[Underline],
        scale_factor: f32,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if underlines.is_empty() {
            return;
        }
        align_offset(offset);
        let next_offset = *offset + underlines.len() * mem::size_of::<shaders::GPUIUnderline>();
        assert!(
            next_offset <= INSTANCE_BUFFER_SIZE,
            "instance buffer exhausted"
        );

        command_encoder.set_render_pipeline_state(&self.underline_pipeline_state);
        command_encoder.set_vertex_buffer(
            shaders::GPUIUnderlineInputIndex_GPUIUnderlineInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_buffer(
            shaders::GPUIUnderlineInputIndex_GPUIUnderlineInputIndexUnderlines as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_vertex_bytes(
            shaders::GPUIUnderlineInputIndex_GPUIUnderlineInputIndexUniforms as u64,
            mem::size_of::<shaders::GPUIUniforms>() as u64,
            [shaders::GPUIUniforms {
                viewport_size: drawable_size.to_float2(),
            }]
            .as_ptr() as *const c_void,
        );

        let buffer_contents = unsafe {
            (self.instances.contents() as *mut u8).offset(*offset as isize)
                as *mut shaders::GPUIUnderline
        };
        for (ix, underline) in underlines.iter().enumerate() {
            let origin = underline.origin * scale_factor;
            let mut height = underline.thickness;
            if underline.squiggly {
                height *= 3.;
            }
            let size = vec2f(underline.width, height) * scale_factor;
            let shader_underline = shaders::GPUIUnderline {
                origin: origin.round().to_float2(),
                size: size.round().to_float2(),
                thickness: underline.thickness * scale_factor,
                color: underline.color.to_uchar4(),
                squiggly: underline.squiggly as u8,
            };
            unsafe {
                *(buffer_contents.offset(ix as isize)) = shader_underline;
            }
        }

        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            underlines.len() as u64,
        );
        *offset = next_offset;
    }
}

fn build_path_atlas_texture_descriptor() -> metal::TextureDescriptor {
    let texture_descriptor = metal::TextureDescriptor::new();
    texture_descriptor.set_width(2048);
    texture_descriptor.set_height(2048);
    texture_descriptor.set_pixel_format(MTLPixelFormat::R16Float);
    texture_descriptor
        .set_usage(metal::MTLTextureUsage::RenderTarget | metal::MTLTextureUsage::ShaderRead);
    texture_descriptor.set_storage_mode(metal::MTLStorageMode::Private);
    texture_descriptor
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

fn build_path_atlas_pipeline_state(
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
    color_attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::One);

    device
        .new_render_pipeline_state(&descriptor)
        .expect("could not create render pipeline state")
}

mod shaders {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    use crate::{
        color::Color,
        geometry::vector::{Vector2F, Vector2I},
    };
    use std::mem;

    include!(concat!(env!("OUT_DIR"), "/shaders.rs"));

    pub trait ToFloat2 {
        fn to_float2(&self) -> vector_float2;
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

    impl Color {
        pub fn to_uchar4(&self) -> vector_uchar4 {
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
