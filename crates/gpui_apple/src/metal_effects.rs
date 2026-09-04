//! Effect layers for the Metal renderer.
//!
//! An effect layer draws part of the scene into a texture of its own, then
//! paints that texture over the frame with a blur, a colour matrix, a mask
//! and a blend mode. `Effects` owns the pipelines, a pool of textures and
//! the stack of layers that are open while a frame draws.
//!
//! The content of a layer draws into a texture the size of the frame. The
//! fragment shaders read their pixel position from the render target, so a
//! smaller texture with a shifted viewport would put every rounded box and
//! gradient in the wrong place. The blur and composite steps then work on a
//! copy of just the region the layer covers.

use gpui::{BlurPlan, Bounds, DevicePixels, EffectLayer, LayerRegion, ScaledPixels, Size};
use metal::MTLPixelFormat;
use std::mem;

/// The biggest texture pool. Beyond this, a returned texture is dropped.
const POOL_LIMIT: usize = 24;

/// What the renderer draws into right now, a texture the size of the frame.
pub(crate) struct Target<'a> {
    pub texture: &'a metal::TextureRef,
    /// The pixels of the frame that matter in it.
    pub region: LayerRegion,
}

/// The parts of a frame every layer step needs.
pub(crate) struct Frame<'a> {
    pub command_buffer: &'a metal::CommandBufferRef,
    /// The texture of the frame itself.
    pub texture: &'a metal::TextureRef,
    pub viewport_size: Size<DevicePixels>,
    pub unit_vertices: &'a metal::BufferRef,
}

/// An effect layer whose content is drawing now.
struct OpenLayer {
    region: LayerRegion,
    /// None when the region is empty, so the content draws into the parent
    /// with no effect.
    texture: Option<metal::Texture>,
}

pub(crate) struct Effects {
    blur_pipeline_state: metal::RenderPipelineState,
    /// The blur again, drawing into a float texture. The mask-weighted
    /// backdrop needs the precision: its values divide back out in the
    /// composite, and eight bits leave steps the divide makes visible.
    blur_float_pipeline_state: metal::RenderPipelineState,
    premask_pipeline_state: metal::RenderPipelineState,
    composite_pipeline_state: metal::RenderPipelineState,
    /// Textures free for the next layer, any size.
    pool: Vec<metal::Texture>,
    open: Vec<OpenLayer>,
}

impl Effects {
    pub fn new(device: &metal::DeviceRef, library: &metal::LibraryRef) -> Self {
        Self {
            blur_pipeline_state: build_copy_pipeline_state(
                device,
                library,
                "blur",
                "blur_vertex",
                "blur_fragment",
                MTLPixelFormat::BGRA8Unorm,
            ),
            blur_float_pipeline_state: build_copy_pipeline_state(
                device,
                library,
                "blur_float",
                "blur_vertex",
                "blur_fragment",
                MTLPixelFormat::RGBA16Float,
            ),
            premask_pipeline_state: build_copy_pipeline_state(
                device,
                library,
                "premask",
                "premask_vertex",
                "premask_fragment",
                MTLPixelFormat::RGBA16Float,
            ),
            composite_pipeline_state: build_copy_pipeline_state(
                device,
                library,
                "layer_composite",
                "layer_composite_vertex",
                "layer_composite_fragment",
                MTLPixelFormat::BGRA8Unorm,
            ),
            pool: Vec::new(),
            open: Vec::new(),
        }
    }

    /// Drops the pooled textures, for when the window changes size.
    pub fn forget_textures(&mut self) {
        self.pool.clear();
    }

    /// The texture and offset the scene draws into now.
    pub fn target<'a>(&'a self, frame: &Frame<'a>) -> Target<'a> {
        for layer in self.open.iter().rev() {
            if let Some(texture) = &layer.texture {
                return Target {
                    texture,
                    region: layer.region,
                };
            }
        }
        Target {
            texture: frame.texture,
            region: LayerRegion::of_viewport(frame.viewport_size),
        }
    }

    /// A new encoder on the current target that keeps what is there.
    pub fn resume<'a>(&self, frame: &Frame<'a>) -> &'a metal::RenderCommandEncoderRef {
        encoder_on(frame, self.target(frame).texture, None)
    }

    /// Opens `layer`. The returned encoder draws the content of the layer.
    pub fn begin_layer<'a>(
        &mut self,
        device: &metal::DeviceRef,
        frame: &Frame<'a>,
        layer: &EffectLayer,
    ) -> &'a metal::RenderCommandEncoderRef {
        let region = layer.region(self.target(frame).region);
        if region.is_empty() {
            self.open.push(OpenLayer {
                region,
                texture: None,
            });
            return self.resume(frame);
        }
        let texture = self.take_texture(
            device,
            LayerRegion::of_viewport(frame.viewport_size),
            MTLPixelFormat::BGRA8Unorm,
        );
        let encoder = encoder_on(
            frame,
            &texture,
            Some(metal::MTLClearColor::new(0., 0., 0., 0.)),
        );
        self.open.push(OpenLayer {
            region,
            texture: Some(texture),
        });
        encoder
    }

    /// Closes the innermost layer and paints it over its parent. The
    /// returned encoder continues the parent.
    pub fn end_layer<'a>(
        &mut self,
        device: &metal::DeviceRef,
        frame: &Frame<'a>,
        layer: &EffectLayer,
    ) -> &'a metal::RenderCommandEncoderRef {
        let Some(open) = self.open.pop() else {
            return self.resume(frame);
        };
        let Some(content_frame) = open.texture else {
            return self.resume(frame);
        };
        let region = open.region;
        let parent_texture = self.target(frame).texture.to_owned();

        // The region of the content, and what is under the layer there, so
        // the shader can read it while it writes the same pixels.
        let content = self.take_texture(device, region, MTLPixelFormat::BGRA8Unorm);
        copy_region(frame.command_buffer, &content_frame, &content, region);
        self.give_back(content_frame);
        let under = self.take_texture(device, region, MTLPixelFormat::BGRA8Unorm);
        copy_region(frame.command_buffer, &parent_texture, &under, region);

        let composite = LayerComposite {
            layer: *layer,
            region: region.bounds(),
        };

        let blurred_content = (layer.blur > 0.0).then(|| {
            self.blur(
                device,
                frame,
                &content,
                region,
                layer.blur,
                MTLPixelFormat::BGRA8Unorm,
            )
        });
        // A mask over a blurred backdrop asks for a blur that grows with
        // the mask. Two weaker blurs give the shader levels to mix, and
        // the mask weighs every level's source pixels, so the blur is an
        // average of what the mask keeps and nothing else.
        let progressive = layer.has_mask != 0 && layer.backdrop_blur > 0.0;
        let masked = progressive.then(|| {
            let texture = self.take_texture(device, region, MTLPixelFormat::RGBA16Float);
            self.premask_pass(frame, &under, &texture, region, &composite);
            texture
        });
        let backdrop_format = if progressive {
            MTLPixelFormat::RGBA16Float
        } else {
            MTLPixelFormat::BGRA8Unorm
        };
        let backdrop_source = masked.as_deref().unwrap_or(&under);
        let blurred_under = (layer.backdrop_blur > 0.0).then(|| {
            self.blur(
                device,
                frame,
                backdrop_source,
                region,
                layer.backdrop_blur,
                backdrop_format,
            )
        });
        let blurred_mid = progressive.then(|| {
            self.blur(
                device,
                frame,
                backdrop_source,
                region,
                layer.backdrop_blur * 0.25,
                backdrop_format,
            )
        });
        let blurred_low = progressive.then(|| {
            self.blur(
                device,
                frame,
                backdrop_source,
                region,
                layer.backdrop_blur * 0.0625,
                backdrop_format,
            )
        });
        if let Some(texture) = masked {
            self.give_back(texture);
        }

        let encoder = encoder_on(frame, &parent_texture, None);
        encoder.set_render_pipeline_state(&self.composite_pipeline_state);
        encoder.set_vertex_buffer(
            LayerInputIndex::Vertices as u64,
            Some(frame.unit_vertices),
            0,
        );
        encoder.set_vertex_bytes(
            LayerInputIndex::Layer as u64,
            mem::size_of::<LayerComposite>() as u64,
            &composite as *const LayerComposite as *const _,
        );
        encoder.set_fragment_bytes(
            LayerInputIndex::Layer as u64,
            mem::size_of::<LayerComposite>() as u64,
            &composite as *const LayerComposite as *const _,
        );
        let viewport_size = frame.viewport_size;
        encoder.set_vertex_bytes(
            LayerInputIndex::ViewportSize as u64,
            mem::size_of_val(&viewport_size) as u64,
            &viewport_size as *const Size<DevicePixels> as *const _,
        );
        encoder.set_fragment_texture(
            LayerInputIndex::ContentTexture as u64,
            Some(blurred_content.as_deref().unwrap_or(&content)),
        );
        encoder.set_fragment_texture(LayerInputIndex::UnderTexture as u64, Some(&under));
        let backdrop = blurred_under.as_deref().unwrap_or(&under);
        encoder.set_fragment_texture(LayerInputIndex::BackdropTexture as u64, Some(backdrop));
        encoder.set_fragment_texture(
            LayerInputIndex::BackdropMidTexture as u64,
            Some(blurred_mid.as_deref().unwrap_or(backdrop)),
        );
        encoder.set_fragment_texture(
            LayerInputIndex::BackdropLowTexture as u64,
            Some(blurred_low.as_deref().unwrap_or(backdrop)),
        );
        encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, 0, 6);

        self.give_back(content);
        self.give_back(under);
        if let Some(texture) = blurred_content {
            self.give_back(texture);
        }
        for texture in [blurred_under, blurred_mid, blurred_low]
            .into_iter()
            .flatten()
        {
            self.give_back(texture);
        }
        encoder
    }

    /// Whether a layer is still open. True at the end of a frame means the
    /// scene pushed more layers than it popped.
    pub fn has_open_layers(&self) -> bool {
        !self.open.is_empty()
    }

    /// Blurs `source` with a Gaussian of `sigma` device pixels into a
    /// smaller texture. Two passes, one per axis, on a texture shrunk by
    /// `scale`, so a wide blur costs the same as a narrow one.
    fn blur(
        &mut self,
        device: &metal::DeviceRef,
        frame: &Frame,
        source: &metal::TextureRef,
        region: LayerRegion,
        sigma: f32,
        pixel_format: MTLPixelFormat,
    ) -> metal::Texture {
        let plan = BlurPlan::new(sigma);
        let BlurPlan { sigma, radius, .. } = plan;

        // Halve the source until it is `plan.scale` times smaller. One tap
        // at the centre of a 2 by 2 block is an exact box average.
        let mut size = LayerRegion::new(0, 0, region.width, region.height);
        let mut shrunk: Option<metal::Texture> = None;
        for _ in 0..plan.shrink_steps() {
            let next_size = size.halved();
            let next = self.take_texture(device, next_size, pixel_format);
            self.blur_pass(
                frame,
                shrunk.as_deref().unwrap_or(source),
                &next,
                next_size,
                BlurParams {
                    step: [0.0, 0.0],
                    sigma: 1.0,
                    radius: 0,
                },
                pixel_format,
            );
            if let Some(texture) = shrunk {
                self.give_back(texture);
            }
            shrunk = Some(next);
            size = next_size;
        }

        let first = self.take_texture(device, size, pixel_format);
        self.blur_pass(
            frame,
            shrunk.as_deref().unwrap_or(source),
            &first,
            size,
            BlurParams {
                step: [1.0 / size.width as f32, 0.0],
                sigma,
                radius,
            },
            pixel_format,
        );
        if let Some(texture) = shrunk {
            self.give_back(texture);
        }
        let second = self.take_texture(device, size, pixel_format);
        self.blur_pass(
            frame,
            &first,
            &second,
            size,
            BlurParams {
                step: [0.0, 1.0 / size.height as f32],
                sigma,
                radius,
            },
            pixel_format,
        );
        self.give_back(first);
        second
    }

    /// Writes `under` times the mask of the layer into `target`, with the
    /// mask value in alpha. The blur of this texture is a weighted average
    /// of only the pixels the mask keeps.
    fn premask_pass(
        &self,
        frame: &Frame,
        under: &metal::TextureRef,
        target: &metal::TextureRef,
        region: LayerRegion,
        composite: &LayerComposite,
    ) {
        let descriptor = metal::RenderPassDescriptor::new();
        let attachment = descriptor.color_attachments().object_at(0).unwrap();
        attachment.set_texture(Some(target));
        attachment.set_load_action(metal::MTLLoadAction::DontCare);
        attachment.set_store_action(metal::MTLStoreAction::Store);
        let encoder = frame.command_buffer.new_render_command_encoder(descriptor);
        encoder.set_viewport(metal::MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: region.width as f64,
            height: region.height as f64,
            znear: 0.0,
            zfar: 1.0,
        });
        encoder.set_render_pipeline_state(&self.premask_pipeline_state);
        encoder.set_vertex_buffer(
            LayerInputIndex::Vertices as u64,
            Some(frame.unit_vertices),
            0,
        );
        encoder.set_vertex_bytes(
            LayerInputIndex::Layer as u64,
            mem::size_of::<LayerComposite>() as u64,
            composite as *const LayerComposite as *const _,
        );
        encoder.set_fragment_bytes(
            LayerInputIndex::Layer as u64,
            mem::size_of::<LayerComposite>() as u64,
            composite as *const LayerComposite as *const _,
        );
        encoder.set_fragment_texture(LayerInputIndex::UnderTexture as u64, Some(under));
        encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, 0, 6);
        encoder.end_encoding();
    }

    fn blur_pass(
        &self,
        frame: &Frame,
        source: &metal::TextureRef,
        target: &metal::TextureRef,
        target_region: LayerRegion,
        params: BlurParams,
        pixel_format: MTLPixelFormat,
    ) {
        let descriptor = metal::RenderPassDescriptor::new();
        let attachment = descriptor.color_attachments().object_at(0).unwrap();
        attachment.set_texture(Some(target));
        attachment.set_load_action(metal::MTLLoadAction::DontCare);
        attachment.set_store_action(metal::MTLStoreAction::Store);
        let encoder = frame.command_buffer.new_render_command_encoder(descriptor);
        encoder.set_viewport(metal::MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: target_region.width as f64,
            height: target_region.height as f64,
            znear: 0.0,
            zfar: 1.0,
        });
        encoder.set_render_pipeline_state(if pixel_format == MTLPixelFormat::RGBA16Float {
            &self.blur_float_pipeline_state
        } else {
            &self.blur_pipeline_state
        });
        encoder.set_vertex_buffer(
            BlurInputIndex::Vertices as u64,
            Some(frame.unit_vertices),
            0,
        );
        encoder.set_fragment_bytes(
            BlurInputIndex::Params as u64,
            mem::size_of::<BlurParams>() as u64,
            &params as *const BlurParams as *const _,
        );
        encoder.set_fragment_texture(BlurInputIndex::Source as u64, Some(source));
        encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, 0, 6);
        encoder.end_encoding();
    }

    /// A texture of exactly the size of `region`, from the pool when one fits.
    fn take_texture(
        &mut self,
        device: &metal::DeviceRef,
        region: LayerRegion,
        pixel_format: MTLPixelFormat,
    ) -> metal::Texture {
        let width = region.width.max(1) as u64;
        let height = region.height.max(1) as u64;
        if let Some(index) = self.pool.iter().position(|texture| {
            texture.width() == width
                && texture.height() == height
                && texture.pixel_format() == pixel_format
        }) {
            return self.pool.swap_remove(index);
        }
        let descriptor = metal::TextureDescriptor::new();
        descriptor.set_width(width);
        descriptor.set_height(height);
        descriptor.set_pixel_format(pixel_format);
        descriptor.set_storage_mode(metal::MTLStorageMode::Private);
        descriptor
            .set_usage(metal::MTLTextureUsage::RenderTarget | metal::MTLTextureUsage::ShaderRead);
        device.new_texture(&descriptor)
    }

    fn give_back(&mut self, texture: metal::Texture) {
        if self.pool.len() >= POOL_LIMIT {
            self.pool.remove(0);
        }
        self.pool.push(texture);
    }
}

/// A render encoder on `texture`, a texture the size of the frame.
pub(crate) fn encoder_on<'a>(
    frame: &Frame<'a>,
    texture: &metal::TextureRef,
    clear_color: Option<metal::MTLClearColor>,
) -> &'a metal::RenderCommandEncoderRef {
    let descriptor = metal::RenderPassDescriptor::new();
    let attachment = descriptor.color_attachments().object_at(0).unwrap();
    attachment.set_texture(Some(texture));
    attachment.set_store_action(metal::MTLStoreAction::Store);
    if let Some(clear_color) = clear_color {
        attachment.set_load_action(metal::MTLLoadAction::Clear);
        attachment.set_clear_color(clear_color);
    } else {
        attachment.set_load_action(metal::MTLLoadAction::Load);
    }
    let encoder = frame.command_buffer.new_render_command_encoder(descriptor);
    encoder.set_viewport(metal::MTLViewport {
        originX: 0.0,
        originY: 0.0,
        width: frame.viewport_size.width.0 as f64,
        height: frame.viewport_size.height.0 as f64,
        znear: 0.0,
        zfar: 1.0,
    });
    encoder
}

/// Copies the pixels of `region` from `source`, a texture the size of the
/// frame, into the top left of `target`.
fn copy_region(
    command_buffer: &metal::CommandBufferRef,
    source: &metal::TextureRef,
    target: &metal::TextureRef,
    region: LayerRegion,
) {
    let blit = command_buffer.new_blit_command_encoder();
    blit.copy_from_texture(
        source,
        0,
        0,
        metal::MTLOrigin {
            x: region.x.max(0) as u64,
            y: region.y.max(0) as u64,
            z: 0,
        },
        metal::MTLSize {
            width: region.width as u64,
            height: region.height as u64,
            depth: 1,
        },
        target,
        0,
        0,
        metal::MTLOrigin { x: 0, y: 0, z: 0 },
    );
    blit.end_encoding();
}

/// A pipeline that writes its output as is. The blur and composite shaders
/// do their own mixing.
fn build_copy_pipeline_state(
    device: &metal::DeviceRef,
    library: &metal::LibraryRef,
    label: &str,
    vertex_fn_name: &str,
    fragment_fn_name: &str,
    pixel_format: MTLPixelFormat,
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
    color_attachment.set_blending_enabled(false);
    device
        .new_render_pipeline_state(&descriptor)
        .expect("could not create render pipeline state")
}

#[repr(C)]
pub(crate) enum BlurInputIndex {
    Vertices = 0,
    Params = 1,
    Source = 2,
}

/// One pass of a separable Gaussian blur.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct BlurParams {
    /// One tap in texture coordinates of the source.
    pub step: [f32; 2],
    /// Standard deviation in taps.
    pub sigma: f32,
    /// Taps on each side of the centre.
    pub radius: i32,
}

#[repr(C)]
pub(crate) enum LayerInputIndex {
    Vertices = 0,
    Layer = 1,
    ViewportSize = 2,
    ContentTexture = 3,
    UnderTexture = 4,
    BackdropTexture = 5,
    BackdropMidTexture = 6,
    BackdropLowTexture = 7,
}

/// Everything the composite shader needs to paint one layer.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct LayerComposite {
    pub layer: EffectLayer,
    /// The pixels of the frame the layer textures cover.
    pub region: Bounds<ScaledPixels>,
}
