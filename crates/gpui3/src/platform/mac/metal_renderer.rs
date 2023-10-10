use crate::{
    point, size, AtlasTextureId, AtlasTextureKind, AtlasTile, Bounds, ContentMask, DevicePixels,
    Hsla, MetalAtlas, MonochromeSprite, Path, PathId, PathVertex, PolychromeSprite, PrimitiveBatch,
    Quad, ScaledPixels, Scene, Shadow, Size, Underline,
};
use cocoa::{
    base::{NO, YES},
    foundation::NSUInteger,
    quartzcore::AutoresizingMask,
};
use collections::HashMap;
use metal::{CommandQueue, MTLPixelFormat, MTLResourceOptions, NSRange};
use objc::{self, msg_send, sel, sel_impl};
use smallvec::SmallVec;
use std::{ffi::c_void, mem, ptr, sync::Arc};

const SHADERS_METALLIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));
const INSTANCE_BUFFER_SIZE: usize = 8192 * 1024; // This is an arbitrary decision. There's probably a more optimal value.

pub(crate) struct MetalRenderer {
    layer: metal::MetalLayer,
    command_queue: CommandQueue,
    paths_rasterization_pipeline_state: metal::RenderPipelineState,
    path_sprites_pipeline_state: metal::RenderPipelineState,
    shadows_pipeline_state: metal::RenderPipelineState,
    quads_pipeline_state: metal::RenderPipelineState,
    underlines_pipeline_state: metal::RenderPipelineState,
    monochrome_sprites_pipeline_state: metal::RenderPipelineState,
    polychrome_sprites_pipeline_state: metal::RenderPipelineState,
    unit_vertices: metal::Buffer,
    instances: metal::Buffer,
    sprite_atlas: Arc<MetalAtlas>,
}

impl MetalRenderer {
    pub fn new(is_opaque: bool) -> Self {
        let device: metal::Device = if let Some(device) = metal::Device::system_default() {
            device
        } else {
            log::error!("unable to access a compatible graphics device");
            std::process::exit(1);
        };

        let layer = metal::MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
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

        let paths_rasterization_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "paths_rasterization",
            "path_rasterization_vertex",
            "path_rasterization_fragment",
            MTLPixelFormat::R16Float,
        );
        let path_sprites_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "path_sprites",
            "path_sprite_vertex",
            "path_sprite_fragment",
            MTLPixelFormat::BGRA8Unorm,
        );
        let shadows_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "shadows",
            "shadow_vertex",
            "shadow_fragment",
            MTLPixelFormat::BGRA8Unorm,
        );
        let quads_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "quads",
            "quad_vertex",
            "quad_fragment",
            MTLPixelFormat::BGRA8Unorm,
        );
        let underlines_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "underlines",
            "underline_vertex",
            "underline_fragment",
            MTLPixelFormat::BGRA8Unorm,
        );
        let monochrome_sprites_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "monochrome_sprites",
            "monochrome_sprite_vertex",
            "monochrome_sprite_fragment",
            MTLPixelFormat::BGRA8Unorm,
        );
        let polychrome_sprites_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "polychrome_sprites",
            "polychrome_sprite_vertex",
            "polychrome_sprite_fragment",
            MTLPixelFormat::BGRA8Unorm,
        );

        let command_queue = device.new_command_queue();
        let sprite_atlas = Arc::new(MetalAtlas::new(device.clone()));

        Self {
            layer,
            command_queue,
            paths_rasterization_pipeline_state,
            path_sprites_pipeline_state,
            shadows_pipeline_state,
            quads_pipeline_state,
            underlines_pipeline_state,
            monochrome_sprites_pipeline_state,
            polychrome_sprites_pipeline_state,
            unit_vertices,
            instances,
            sprite_atlas,
        }
    }

    pub fn layer(&self) -> &metal::MetalLayerRef {
        &*self.layer
    }

    pub fn sprite_atlas(&self) -> &Arc<MetalAtlas> {
        &self.sprite_atlas
    }

    pub fn draw(&mut self, scene: &Scene) {
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
        let mut instance_offset = 0;

        let path_tiles = self.rasterize_paths(scene.paths(), &mut instance_offset, &command_buffer);

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
            width: i32::from(viewport_size.width) as f64,
            height: i32::from(viewport_size.height) as f64,
            znear: 0.0,
            zfar: 1.0,
        });
        for batch in scene.batches() {
            match batch {
                PrimitiveBatch::Shadows(shadows) => {
                    self.draw_shadows(
                        shadows,
                        &mut instance_offset,
                        viewport_size,
                        command_encoder,
                    );
                }
                PrimitiveBatch::Quads(quads) => {
                    self.draw_quads(quads, &mut instance_offset, viewport_size, command_encoder);
                }
                PrimitiveBatch::Paths(paths) => {
                    self.draw_paths(
                        paths,
                        &path_tiles,
                        &mut instance_offset,
                        viewport_size,
                        command_encoder,
                    );
                }
                PrimitiveBatch::Underlines(underlines) => {
                    self.draw_underlines(
                        underlines,
                        &mut instance_offset,
                        viewport_size,
                        command_encoder,
                    );
                }
                PrimitiveBatch::MonochromeSprites {
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
                PrimitiveBatch::PolychromeSprites {
                    texture_id,
                    sprites,
                } => {
                    self.draw_polychrome_sprites(
                        texture_id,
                        sprites,
                        &mut instance_offset,
                        viewport_size,
                        command_encoder,
                    );
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

    fn rasterize_paths(
        &mut self,
        paths: &[Path<ScaledPixels>],
        offset: &mut usize,
        command_buffer: &metal::CommandBufferRef,
    ) -> HashMap<PathId, AtlasTile> {
        let mut tiles = HashMap::default();
        let mut vertices_by_texture_id = HashMap::default();
        for path in paths {
            let clipped_bounds = path.bounds.intersect(&path.content_mask.bounds);

            let tile = self
                .sprite_atlas
                .allocate(clipped_bounds.size.map(Into::into), AtlasTextureKind::Path);
            vertices_by_texture_id
                .entry(tile.texture_id)
                .or_insert(Vec::new())
                .extend(path.vertices.iter().map(|vertex| PathVertex {
                    xy_position: vertex.xy_position - path.bounds.origin
                        + tile.bounds.origin.map(Into::into),
                    st_position: vertex.st_position,
                    content_mask: ContentMask {
                        bounds: tile.bounds.map(Into::into),
                    },
                }));
            tiles.insert(path.id, tile);
        }

        for (texture_id, vertices) in vertices_by_texture_id {
            align_offset(offset);
            let next_offset = *offset + vertices.len() * mem::size_of::<PathVertex<ScaledPixels>>();
            assert!(
                next_offset <= INSTANCE_BUFFER_SIZE,
                "instance buffer exhausted"
            );

            let render_pass_descriptor = metal::RenderPassDescriptor::new();
            let color_attachment = render_pass_descriptor
                .color_attachments()
                .object_at(0)
                .unwrap();

            let texture = self.sprite_atlas.metal_texture(texture_id);
            color_attachment.set_texture(Some(&texture));
            color_attachment.set_load_action(metal::MTLLoadAction::Clear);
            color_attachment.set_store_action(metal::MTLStoreAction::Store);
            color_attachment.set_clear_color(metal::MTLClearColor::new(0., 0., 0., 1.));
            let command_encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
            command_encoder.set_render_pipeline_state(&self.paths_rasterization_pipeline_state);
            command_encoder.set_vertex_buffer(
                PathRasterizationInputIndex::Vertices as u64,
                Some(&self.instances),
                *offset as u64,
            );
            let texture_size = Size {
                width: DevicePixels::from(texture.width()),
                height: DevicePixels::from(texture.height()),
            };
            command_encoder.set_vertex_bytes(
                PathRasterizationInputIndex::AtlasTextureSize as u64,
                mem::size_of_val(&texture_size) as u64,
                &texture_size as *const Size<DevicePixels> as *const _,
            );

            let vertices_bytes_len = mem::size_of::<PathVertex<ScaledPixels>>() * vertices.len();
            let buffer_contents = unsafe { (self.instances.contents() as *mut u8).add(*offset) };
            unsafe {
                ptr::copy_nonoverlapping(
                    vertices.as_ptr() as *const u8,
                    buffer_contents,
                    vertices_bytes_len,
                );
            }

            command_encoder.draw_primitives(
                metal::MTLPrimitiveType::Triangle,
                0,
                vertices.len() as u64,
            );
            command_encoder.end_encoding();
            *offset = next_offset;
        }

        tiles
    }

    fn draw_shadows(
        &mut self,
        shadows: &[Shadow],
        offset: &mut usize,
        viewport_size: Size<DevicePixels>,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if shadows.is_empty() {
            return;
        }
        align_offset(offset);

        command_encoder.set_render_pipeline_state(&self.shadows_pipeline_state);
        command_encoder.set_vertex_buffer(
            ShadowInputIndex::Vertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_buffer(
            ShadowInputIndex::Shadows as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_fragment_buffer(
            ShadowInputIndex::Shadows as u64,
            Some(&self.instances),
            *offset as u64,
        );

        command_encoder.set_vertex_bytes(
            ShadowInputIndex::ViewportSize as u64,
            mem::size_of_val(&viewport_size) as u64,
            &viewport_size as *const Size<DevicePixels> as *const _,
        );

        let shadow_bytes_len = mem::size_of::<Shadow>() * shadows.len();
        let buffer_contents = unsafe { (self.instances.contents() as *mut u8).add(*offset) };
        unsafe {
            ptr::copy_nonoverlapping(
                shadows.as_ptr() as *const u8,
                buffer_contents,
                shadow_bytes_len,
            );
        }

        let next_offset = *offset + shadow_bytes_len;
        assert!(
            next_offset <= INSTANCE_BUFFER_SIZE,
            "instance buffer exhausted"
        );

        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            shadows.len() as u64,
        );
        *offset = next_offset;
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

    fn draw_paths(
        &mut self,
        paths: &[Path<ScaledPixels>],
        tiles_by_path_id: &HashMap<PathId, AtlasTile>,
        offset: &mut usize,
        viewport_size: Size<DevicePixels>,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if paths.is_empty() {
            return;
        }

        command_encoder.set_render_pipeline_state(&self.path_sprites_pipeline_state);
        command_encoder.set_vertex_buffer(
            SpriteInputIndex::Vertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_bytes(
            SpriteInputIndex::ViewportSize as u64,
            mem::size_of_val(&viewport_size) as u64,
            &viewport_size as *const Size<DevicePixels> as *const _,
        );

        let mut prev_texture_id = None;
        let mut sprites = SmallVec::<[_; 1]>::new();
        let mut paths_and_tiles = paths
            .into_iter()
            .map(|path| (path, tiles_by_path_id.get(&path.id).unwrap()))
            .peekable();

        loop {
            if let Some((path, tile)) = paths_and_tiles.peek() {
                if prev_texture_id.map_or(true, |texture_id| texture_id == tile.texture_id) {
                    prev_texture_id = Some(tile.texture_id);
                    sprites.push(PathSprite {
                        bounds: Bounds {
                            origin: path.bounds.origin.map(|p| p.floor()),
                            size: tile.bounds.size.map(Into::into),
                        },
                        color: path.color,
                        tile: (*tile).clone(),
                    });
                    paths_and_tiles.next();
                    continue;
                }
            }

            if sprites.is_empty() {
                break;
            } else {
                align_offset(offset);
                let texture_id = prev_texture_id.take().unwrap();
                let texture: metal::Texture = self.sprite_atlas.metal_texture(texture_id);
                let texture_size = size(
                    DevicePixels(texture.width() as i32),
                    DevicePixels(texture.height() as i32),
                );

                command_encoder.set_vertex_buffer(
                    SpriteInputIndex::Sprites as u64,
                    Some(&self.instances),
                    *offset as u64,
                );
                command_encoder.set_vertex_bytes(
                    SpriteInputIndex::AtlasTextureSize as u64,
                    mem::size_of_val(&texture_size) as u64,
                    &texture_size as *const Size<DevicePixels> as *const _,
                );
                command_encoder.set_fragment_buffer(
                    SpriteInputIndex::Sprites as u64,
                    Some(&self.instances),
                    *offset as u64,
                );
                command_encoder
                    .set_fragment_texture(SpriteInputIndex::AtlasTexture as u64, Some(&texture));

                let sprite_bytes_len = mem::size_of::<MonochromeSprite>() * sprites.len();
                let buffer_contents =
                    unsafe { (self.instances.contents() as *mut u8).add(*offset) };
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
                sprites.clear();
            }
        }
    }

    fn draw_underlines(
        &mut self,
        underlines: &[Underline],
        offset: &mut usize,
        viewport_size: Size<DevicePixels>,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if underlines.is_empty() {
            return;
        }
        align_offset(offset);

        command_encoder.set_render_pipeline_state(&self.underlines_pipeline_state);
        command_encoder.set_vertex_buffer(
            UnderlineInputIndex::Vertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_buffer(
            UnderlineInputIndex::Underlines as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_fragment_buffer(
            UnderlineInputIndex::Underlines as u64,
            Some(&self.instances),
            *offset as u64,
        );

        command_encoder.set_vertex_bytes(
            UnderlineInputIndex::ViewportSize as u64,
            mem::size_of_val(&viewport_size) as u64,
            &viewport_size as *const Size<DevicePixels> as *const _,
        );

        let quad_bytes_len = mem::size_of::<Underline>() * underlines.len();
        let buffer_contents = unsafe { (self.instances.contents() as *mut u8).add(*offset) };
        unsafe {
            ptr::copy_nonoverlapping(
                underlines.as_ptr() as *const u8,
                buffer_contents,
                quad_bytes_len,
            );
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
            underlines.len() as u64,
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

        let texture = self.sprite_atlas.metal_texture(texture_id);
        let texture_size = size(
            DevicePixels(texture.width() as i32),
            DevicePixels(texture.height() as i32),
        );
        command_encoder.set_render_pipeline_state(&self.monochrome_sprites_pipeline_state);
        command_encoder.set_vertex_buffer(
            SpriteInputIndex::Vertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_buffer(
            SpriteInputIndex::Sprites as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_vertex_bytes(
            SpriteInputIndex::ViewportSize as u64,
            mem::size_of_val(&viewport_size) as u64,
            &viewport_size as *const Size<DevicePixels> as *const _,
        );
        command_encoder.set_vertex_bytes(
            SpriteInputIndex::AtlasTextureSize as u64,
            mem::size_of_val(&texture_size) as u64,
            &texture_size as *const Size<DevicePixels> as *const _,
        );
        command_encoder.set_fragment_buffer(
            SpriteInputIndex::Sprites as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_fragment_texture(SpriteInputIndex::AtlasTexture as u64, Some(&texture));

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

    fn draw_polychrome_sprites(
        &mut self,
        texture_id: AtlasTextureId,
        sprites: &[PolychromeSprite],
        offset: &mut usize,
        viewport_size: Size<DevicePixels>,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if sprites.is_empty() {
            return;
        }
        align_offset(offset);

        let texture = self.sprite_atlas.metal_texture(texture_id);
        let texture_size = size(
            DevicePixels(texture.width() as i32),
            DevicePixels(texture.height() as i32),
        );
        command_encoder.set_render_pipeline_state(&self.polychrome_sprites_pipeline_state);
        command_encoder.set_vertex_buffer(
            SpriteInputIndex::Vertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_buffer(
            SpriteInputIndex::Sprites as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_vertex_bytes(
            SpriteInputIndex::ViewportSize as u64,
            mem::size_of_val(&viewport_size) as u64,
            &viewport_size as *const Size<DevicePixels> as *const _,
        );
        command_encoder.set_vertex_bytes(
            SpriteInputIndex::AtlasTextureSize as u64,
            mem::size_of_val(&texture_size) as u64,
            &texture_size as *const Size<DevicePixels> as *const _,
        );
        command_encoder.set_fragment_buffer(
            SpriteInputIndex::Sprites as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_fragment_texture(SpriteInputIndex::AtlasTexture as u64, Some(&texture));

        let sprite_bytes_len = mem::size_of::<PolychromeSprite>() * sprites.len();
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
enum ShadowInputIndex {
    Vertices = 0,
    Shadows = 1,
    ViewportSize = 2,
}

#[repr(C)]
enum QuadInputIndex {
    Vertices = 0,
    Quads = 1,
    ViewportSize = 2,
}

#[repr(C)]
enum UnderlineInputIndex {
    Vertices = 0,
    Underlines = 1,
    ViewportSize = 2,
}

#[repr(C)]
enum SpriteInputIndex {
    Vertices = 0,
    Sprites = 1,
    ViewportSize = 2,
    AtlasTextureSize = 3,
    AtlasTexture = 4,
}

#[repr(C)]
enum PathRasterizationInputIndex {
    Vertices = 0,
    AtlasTextureSize = 1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PathSprite {
    pub bounds: Bounds<ScaledPixels>,
    pub color: Hsla,
    pub tile: AtlasTile,
}
