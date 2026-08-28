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

use crate::metal_renderer::new_command_encoder_for_texture;
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
}

/// An effect layer whose content is drawing now.
struct OpenLayer {
    region: LayerRegion,
    /// None when the region is empty, so the content draws into the parent
    /// with no effect.
    texture: Option<metal::Texture>,
}

pub(crate) struct Effects {
    device: metal::Device,
    unit_vertices: metal::Buffer,
    blur_pipeline_state: metal::RenderPipelineState,
    /// The blur whose width follows the mask of the layer, pixel by pixel.
    variable_blur_pipeline_state: metal::RenderPipelineState,
    composite_pipeline_state: metal::RenderPipelineState,
    /// Textures free for the next layer, any size.
    pool: Vec<metal::Texture>,
    open: Vec<OpenLayer>,
}

impl Effects {
    pub fn new(
        device: &metal::DeviceRef,
        library: &metal::LibraryRef,
        unit_vertices: metal::Buffer,
    ) -> Self {
        Self {
            device: device.to_owned(),
            unit_vertices,
            blur_pipeline_state: build_copy_pipeline_state(
                device,
                library,
                "blur",
                "blur_vertex",
                "blur_fragment",
                MTLPixelFormat::BGRA8Unorm,
            ),
            variable_blur_pipeline_state: build_copy_pipeline_state(
                device,
                library,
                "variable_blur",
                "variable_blur_vertex",
                "variable_blur_fragment",
                MTLPixelFormat::BGRA8Unorm,
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

    /// Forgets the layers of a frame that never closed them. A frame that
    /// stops on an error leaves them open.
    pub fn begin_frame(&mut self) {
        self.open.clear();
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
        new_command_encoder_for_texture(
            frame.command_buffer,
            self.target(frame).texture,
            frame.viewport_size,
            None,
        )
    }

    /// Opens `layer`. The returned encoder draws the content of the layer.
    pub fn begin_layer<'a>(
        &mut self,
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
            LayerRegion::of_viewport(frame.viewport_size),
            MTLPixelFormat::BGRA8Unorm,
        );
        let encoder = new_command_encoder_for_texture(
            frame.command_buffer,
            &texture,
            frame.viewport_size,
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
        let content = self.take_texture(region, MTLPixelFormat::BGRA8Unorm);
        copy_region(frame.command_buffer, &content_frame, &content, region);
        self.give_back(content_frame);
        let under = self.take_texture(region, MTLPixelFormat::BGRA8Unorm);
        copy_region(frame.command_buffer, &parent_texture, &under, region);

        let composite = LayerComposite {
            layer: *layer,
            region: region.bounds(),
        };

        let blurred_content = (layer.blur > 0.0).then(|| {
            self.blur(
                frame,
                &content,
                region,
                layer.blur,
                MTLPixelFormat::BGRA8Unorm,
            )
        });
        // A mask over a blurred backdrop asks for a blur whose width
        // follows the mask, pixel by pixel. That blur reads the mask,
        // so it runs at full size, not on the shrunk texture the fixed
        // blur uses.
        let blurred_under = (layer.backdrop_blur > 0.0).then(|| {
            if layer.has_mask != 0 {
                self.variable_blur(frame, &under, region, &composite)
            } else {
                self.blur(
                    frame,
                    &under,
                    region,
                    layer.backdrop_blur,
                    MTLPixelFormat::BGRA8Unorm,
                )
            }
        });

        let encoder = new_command_encoder_for_texture(
            frame.command_buffer,
            &parent_texture,
            frame.viewport_size,
            None,
        );
        encoder.set_render_pipeline_state(&self.composite_pipeline_state);
        encoder.set_vertex_buffer(
            LayerInputIndex::Vertices as u64,
            Some(&self.unit_vertices),
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
        encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, 0, 6);

        self.give_back(content);
        self.give_back(under);
        if let Some(texture) = blurred_content {
            self.give_back(texture);
        }
        if let Some(texture) = blurred_under {
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
            let next = self.take_texture(next_size, pixel_format);
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
            );
            if let Some(texture) = shrunk {
                self.give_back(texture);
            }
            shrunk = Some(next);
            size = next_size;
        }

        let first = self.take_texture(size, pixel_format);
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
        );
        if let Some(texture) = shrunk {
            self.give_back(texture);
        }
        let second = self.take_texture(size, pixel_format);
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
        );
        self.give_back(first);
        second
    }

    /// Blurs `source` with the Gaussian the mask of the layer asks for at
    /// each pixel: the mask value there times the `backdrop_blur` sigma.
    /// Two passes, one per axis, at full size, because every pixel has its
    /// own width and a shrunk texture would lose the sharp end.
    fn variable_blur(
        &mut self,
        frame: &Frame,
        source: &metal::TextureRef,
        region: LayerRegion,
        composite: &LayerComposite,
    ) -> metal::Texture {
        let size = LayerRegion::new(0, 0, region.width, region.height);
        let first = self.take_texture(size, MTLPixelFormat::BGRA8Unorm);
        self.variable_blur_pass(
            frame,
            source,
            &first,
            size,
            [1.0 / size.width as f32, 0.0],
            composite,
        );
        let second = self.take_texture(size, MTLPixelFormat::BGRA8Unorm);
        self.variable_blur_pass(
            frame,
            &first,
            &second,
            size,
            [0.0, 1.0 / size.height as f32],
            composite,
        );
        self.give_back(first);
        second
    }

    fn variable_blur_pass(
        &self,
        frame: &Frame,
        source: &metal::TextureRef,
        target: &metal::TextureRef,
        target_region: LayerRegion,
        step: [f32; 2],
        composite: &LayerComposite,
    ) {
        let params = BlurParams {
            step,
            sigma: composite.layer.backdrop_blur,
            radius: VARIABLE_BLUR_RADIUS_CAP,
        };
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
        encoder.set_render_pipeline_state(&self.variable_blur_pipeline_state);
        encoder.set_vertex_buffer(
            BlurInputIndex::Vertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        encoder.set_vertex_bytes(
            BlurInputIndex::Layer as u64,
            mem::size_of::<LayerComposite>() as u64,
            composite as *const LayerComposite as *const _,
        );
        encoder.set_fragment_bytes(
            BlurInputIndex::Params as u64,
            mem::size_of::<BlurParams>() as u64,
            &params as *const BlurParams as *const _,
        );
        encoder.set_fragment_bytes(
            BlurInputIndex::Layer as u64,
            mem::size_of::<LayerComposite>() as u64,
            composite as *const LayerComposite as *const _,
        );
        encoder.set_fragment_texture(BlurInputIndex::Source as u64, Some(source));
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
        encoder.set_render_pipeline_state(&self.blur_pipeline_state);
        encoder.set_vertex_buffer(
            BlurInputIndex::Vertices as u64,
            Some(&self.unit_vertices),
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
        self.device.new_texture(&descriptor)
    }

    fn give_back(&mut self, texture: metal::Texture) {
        if self.pool.len() >= POOL_LIMIT {
            self.pool.remove(0);
        }
        self.pool.push(texture);
    }
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
    /// The `LayerComposite`, for the variable blur, which reads the mask.
    Layer = 3,
}

/// The most taps a variable blur pass takes on each side of a pixel. The
/// cap trims the tails of a sigma past 32 device pixels, and it bounds
/// the cost of one huge blur.
pub(crate) const VARIABLE_BLUR_RADIUS_CAP: i32 = 96;

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
}

/// Everything the composite shader needs to paint one layer.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct LayerComposite {
    pub layer: EffectLayer,
    /// The pixels of the frame the layer textures cover.
    pub region: Bounds<ScaledPixels>,
}
