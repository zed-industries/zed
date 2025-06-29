use crate::{DevicePixels, PaintMetalView, PrimitiveBatch, ScaledPixels, Scene, Size};
use metal::{
    CommandBufferRef, CommandQueue, Device, MTLLoadAction, MTLStoreAction, RenderCommandEncoderRef,
};

/// Represents a single render command in the rendering pipeline
#[derive(Debug)]
pub enum RenderCommand<'a> {
    /// Begin a new render pass with the specified configuration
    BeginRenderPass { descriptor: RenderPassDescriptor },
    /// Draw a batch of GPUI primitives
    DrawPrimitives {
        batch: PrimitiveBatch<'a>,
        viewport_size: Size<DevicePixels>,
    },
    /// Execute custom Metal rendering
    ExecuteMetalCallback {
        metal_view: &'a PaintMetalView,
        viewport_size: Size<DevicePixels>,
    },
    /// End the current render pass
    EndRenderPass,
}

/// Configuration for a render pass
#[derive(Clone, Debug)]
pub struct RenderPassDescriptor {
    pub texture: metal::Texture,
    pub load_action: MTLLoadAction,
    pub store_action: MTLStoreAction,
    pub clear_color: metal::MTLClearColor,
    pub viewport: metal::MTLViewport,
}

/// State that needs to be preserved across render pass breaks
#[derive(Clone, Debug)]
pub struct RenderState {
    pub viewport: metal::MTLViewport,
    pub blend_mode: Option<BlendMode>,
    // Add other state that needs to be preserved
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    // Add other blend modes as needed
}

/// Context provided to Metal render callbacks
pub struct MetalRenderContext<'a> {
    pub command_buffer: &'a CommandBufferRef,
    pub drawable_texture: &'a metal::TextureRef,
    pub viewport_size: Size<DevicePixels>,
    pub device: &'a Device,
    pub bounds: crate::Bounds<ScaledPixels>,
    pub scale_factor: f32,
}

/// Manages the rendering pipeline with support for render pass breaks
pub struct RenderPassManager {
    device: Device,
    command_queue: CommandQueue,
    current_state: RenderState,
}

impl RenderPassManager {
    pub fn new(device: Device, command_queue: CommandQueue) -> Self {
        Self {
            device,
            command_queue,
            current_state: RenderState {
                viewport: metal::MTLViewport {
                    originX: 0.0,
                    originY: 0.0,
                    width: 0.0,
                    height: 0.0,
                    znear: 0.0,
                    zfar: 1.0,
                },
                blend_mode: None,
            },
        }
    }

    /// Convert a scene into a list of render commands
    pub fn build_render_commands<'a>(
        &self,
        scene: &'a Scene,
        drawable_texture: &metal::TextureRef,
        viewport_size: Size<DevicePixels>,
        is_opaque: bool,
    ) -> Vec<RenderCommand<'a>> {
        let mut commands = Vec::new();

        // Initial render pass configuration
        let alpha = if is_opaque { 1.0 } else { 0.0 };
        let descriptor = RenderPassDescriptor {
            texture: drawable_texture.to_owned(),
            load_action: MTLLoadAction::Clear,
            store_action: MTLStoreAction::Store,
            clear_color: metal::MTLClearColor::new(0.0, 0.0, 0.0, alpha),
            viewport: metal::MTLViewport {
                originX: 0.0,
                originY: 0.0,
                width: i32::from(viewport_size.width) as f64,
                height: i32::from(viewport_size.height) as f64,
                znear: 0.0,
                zfar: 1.0,
            },
        };

        commands.push(RenderCommand::BeginRenderPass { descriptor });

        // Process batches, inserting render pass breaks for MetalViews
        let mut in_render_pass = true;

        for batch in scene.batches() {
            match batch {
                #[cfg(target_os = "macos")]
                PrimitiveBatch::MetalViews(metal_views) => {
                    // End current render pass
                    if in_render_pass {
                        commands.push(RenderCommand::EndRenderPass);
                        in_render_pass = false;
                    }

                    // Add commands for each MetalView
                    for metal_view in metal_views {
                        commands.push(RenderCommand::ExecuteMetalCallback {
                            metal_view,
                            viewport_size,
                        });
                    }
                }
                _ => {
                    // Ensure we're in a render pass
                    if !in_render_pass {
                        let descriptor = RenderPassDescriptor {
                            texture: drawable_texture.to_owned(),
                            load_action: MTLLoadAction::Load, // Load existing content
                            store_action: MTLStoreAction::Store,
                            clear_color: metal::MTLClearColor::new(0.0, 0.0, 0.0, 0.0),
                            viewport: self.current_state.viewport,
                        };
                        commands.push(RenderCommand::BeginRenderPass { descriptor });
                        in_render_pass = true;
                    }

                    // Add primitive drawing command
                    commands.push(RenderCommand::DrawPrimitives {
                        batch,
                        viewport_size,
                    });
                }
            }
        }

        // Ensure we end the final render pass
        if in_render_pass {
            commands.push(RenderCommand::EndRenderPass);
        }

        commands
    }

    /// Execute a list of render commands
    pub fn execute_commands<F>(
        &mut self,
        commands: &[RenderCommand],
        command_buffer: &CommandBufferRef,
        drawable_texture: &metal::TextureRef,
        mut draw_primitives: F,
    ) -> Result<(), anyhow::Error>
    where
        F: FnMut(
            PrimitiveBatch,
            &RenderCommandEncoderRef,
            Size<DevicePixels>,
        ) -> Result<(), anyhow::Error>,
    {
        let mut current_encoder: Option<metal::RenderCommandEncoder> = None;

        for command in commands {
            match command {
                RenderCommand::BeginRenderPass { descriptor } => {
                    // End any existing encoder
                    if let Some(encoder) = current_encoder.take() {
                        encoder.end_encoding();
                    }

                    // Create new render pass
                    let render_pass_descriptor = metal::RenderPassDescriptor::new();
                    let color_attachment = render_pass_descriptor
                        .color_attachments()
                        .object_at(0)
                        .unwrap();

                    color_attachment.set_texture(Some(&descriptor.texture));
                    color_attachment.set_load_action(descriptor.load_action);
                    color_attachment.set_store_action(descriptor.store_action);
                    color_attachment.set_clear_color(descriptor.clear_color);

                    let encoder =
                        command_buffer.new_render_command_encoder(&render_pass_descriptor);
                    encoder.set_viewport(descriptor.viewport);
                    self.current_state.viewport = descriptor.viewport;

                    current_encoder = Some(encoder);
                }

                RenderCommand::DrawPrimitives {
                    batch,
                    viewport_size,
                } => {
                    if let Some(ref encoder) = current_encoder {
                        draw_primitives(*batch, encoder, *viewport_size)?;
                    }
                }

                RenderCommand::ExecuteMetalCallback {
                    metal_view,
                    viewport_size,
                } => {
                    // End current encoder if any
                    if let Some(encoder) = current_encoder.take() {
                        encoder.end_encoding();
                    }

                    // Create context for the callback
                    let context = MetalRenderContext {
                        command_buffer,
                        drawable_texture,
                        viewport_size: *viewport_size,
                        device: &self.device,
                        bounds: metal_view.bounds.clone(),
                        scale_factor: 2.0, // TODO: Get actual scale factor
                    };

                    // Create a new render command encoder for the callback
                    let render_pass_descriptor = metal::RenderPassDescriptor::new();
                    let color_attachment = render_pass_descriptor
                        .color_attachments()
                        .object_at(0)
                        .unwrap();

                    color_attachment.set_texture(Some(drawable_texture));
                    color_attachment.set_load_action(MTLLoadAction::Load);
                    color_attachment.set_store_action(MTLStoreAction::Store);

                    let encoder =
                        command_buffer.new_render_command_encoder(&render_pass_descriptor);

                    // Invoke the callback
                    (metal_view.render_callback)(
                        &encoder,
                        drawable_texture,
                        context.bounds.into(),
                        context.scale_factor,
                    );

                    encoder.end_encoding();
                }

                RenderCommand::EndRenderPass => {
                    if let Some(encoder) = current_encoder.take() {
                        encoder.end_encoding();
                    }
                }
            }
        }

        // Ensure any remaining encoder is ended
        if let Some(encoder) = current_encoder {
            encoder.end_encoding();
        }

        Ok(())
    }

    /// Save the current render state
    pub fn save_state(&self) -> RenderState {
        self.current_state.clone()
    }

    /// Restore a previously saved render state
    pub fn restore_state(&mut self, state: RenderState) {
        self.current_state = state;
    }
}

/// Builder for constructing render command lists
pub struct RenderCommandBuilder<'a> {
    commands: Vec<RenderCommand<'a>>,
}

impl<'a> RenderCommandBuilder<'a> {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn begin_render_pass(mut self, descriptor: RenderPassDescriptor) -> Self {
        self.commands
            .push(RenderCommand::BeginRenderPass { descriptor });
        self
    }

    pub fn draw_primitives(
        mut self,
        batch: PrimitiveBatch<'a>,
        viewport_size: Size<DevicePixels>,
    ) -> Self {
        self.commands.push(RenderCommand::DrawPrimitives {
            batch,
            viewport_size,
        });
        self
    }

    pub fn execute_metal_callback(
        mut self,
        metal_view: &'a PaintMetalView,
        viewport_size: Size<DevicePixels>,
    ) -> Self {
        self.commands.push(RenderCommand::ExecuteMetalCallback {
            metal_view,
            viewport_size,
        });
        self
    }

    pub fn end_render_pass(mut self) -> Self {
        self.commands.push(RenderCommand::EndRenderPass);
        self
    }

    pub fn build(self) -> Vec<RenderCommand<'a>> {
        self.commands
    }
}
