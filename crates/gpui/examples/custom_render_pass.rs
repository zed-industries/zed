//! Proof-of-concept example for custom render passes.
//!
//! This example demonstrates how to register a custom render pass that executes
//! before the UI rendering, drawing a colored background that appears behind
//! the GPUI UI elements.
//!
//! Run with:
//! ```
//! cargo run -p gpui --example custom_render_pass --features custom_render_pass,macos-blade
//! ```

use std::sync::Arc;

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

#[cfg(feature = "custom_render_pass")]
use gpui::{gpu, BladeRenderPassContext, CustomRenderPass, DevicePixels, RenderStage, Size};

/// A simple custom render pass that clears the background to a gradient color.
#[cfg(feature = "custom_render_pass")]
struct GradientBackgroundPass {
    pipeline: std::sync::OnceLock<gpu::RenderPipeline>,
}

#[cfg(feature = "custom_render_pass")]
impl GradientBackgroundPass {
    fn new() -> Self {
        Self {
            pipeline: std::sync::OnceLock::new(),
        }
    }

    fn get_or_create_pipeline(&self, gpu_context: &gpu::Context) -> &gpu::RenderPipeline {
        self.pipeline.get_or_init(|| {
            let shader_source = r#"
                struct VertexOutput {
                    @builtin(position) position: vec4<f32>,
                    @location(0) uv: vec2<f32>,
                }

                @vertex
                fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
                    // Full-screen triangle
                    var positions = array<vec2<f32>, 3>(
                        vec2<f32>(-1.0, -1.0),
                        vec2<f32>(3.0, -1.0),
                        vec2<f32>(-1.0, 3.0)
                    );
                    var uvs = array<vec2<f32>, 3>(
                        vec2<f32>(0.0, 1.0),
                        vec2<f32>(2.0, 1.0),
                        vec2<f32>(0.0, -1.0)
                    );

                    var output: VertexOutput;
                    output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
                    output.uv = uvs[vertex_index];
                    return output;
                }

                @fragment
                fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
                    // Create a gradient from purple (top-left) to teal (bottom-right)
                    let purple = vec3<f32>(0.4, 0.2, 0.6);
                    let teal = vec3<f32>(0.2, 0.5, 0.5);
                    let t = (input.uv.x + input.uv.y) * 0.5;
                    let color = mix(purple, teal, t);
                    return vec4<f32>(color, 1.0);
                }
            "#;

            let shader = gpu_context.create_shader(gpu::ShaderDesc {
                source: shader_source,
            });

            gpu_context.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "gradient_background",
                data_layouts: &[],
                vertex: shader.at("vs_main"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_main")),
                color_targets: &[gpu::ColorTargetState {
                    format: gpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(gpu::BlendState::REPLACE),
                    write_mask: gpu::ColorWrites::ALL,
                }],
                multisample_state: gpu::MultisampleState::default(),
            })
        })
    }
}

#[cfg(feature = "custom_render_pass")]
impl CustomRenderPass for GradientBackgroundPass {
    fn render(&self, ctx: &mut BladeRenderPassContext) {
        let pipeline = self.get_or_create_pipeline(ctx.gpu);

        let mut pass = ctx.encoder.render(
            "gradient_background",
            gpu::RenderTargetSet {
                colors: &[gpu::RenderTarget {
                    view: ctx.frame,
                    init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                    finish_op: gpu::FinishOp::Store,
                }],
                depth_stencil: None,
            },
        );

        let viewport_width = ctx.frame_info.viewport_size.width.0 as f32;
        let viewport_height = ctx.frame_info.viewport_size.height.0 as f32;

        pass.set_scissor_rect(&gpu::ScissorRect {
            x: 0,
            y: 0,
            w: viewport_width as u32,
            h: viewport_height as u32,
        });
        pass.set_viewport(&gpu::Viewport {
            x: 0.0,
            y: 0.0,
            w: viewport_width,
            h: viewport_height,
            depth: 0.0..1.0,
        });

        pass.with(&pipeline).draw(0, 3, 0, 1);
    }

    fn resize(&self, _new_size: Size<DevicePixels>) {
        // Pipeline doesn't need to be recreated on resize
    }

    fn name(&self) -> &str {
        "gradient_background"
    }
}

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            // Semi-transparent background so we can see the gradient behind
            .bg(gpui::rgba(0x50505080))
            .size(px(400.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0xffffff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child("Custom render pass is active!")
            .child(
                div()
                    .text_sm()
                    .text_color(gpui::rgba(0xffffffaa))
                    .child("The gradient background is rendered by a custom GPU pass"),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                // Register the custom render pass
                #[cfg(feature = "custom_render_pass")]
                {
                    let pass = Arc::new(GradientBackgroundPass::new());
                    window.register_render_pass(RenderStage::BeforeUi, pass);
                }

                #[cfg(not(feature = "custom_render_pass"))]
                {
                    eprintln!(
                        "Warning: custom_render_pass feature is not enabled. \
                         Run with --features custom_render_pass,macos-blade"
                    );
                }

                cx.new(|_| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
