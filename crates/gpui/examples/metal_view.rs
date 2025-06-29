use gpui::{prelude::*, *};
use std::sync::Arc;

#[cfg(target_os = "macos")]
use metal::{Device, MTLPrimitiveType, RenderCommandEncoderRef, RenderPipelineState, TextureRef};

struct MetalViewExample {
    #[cfg(target_os = "macos")]
    pipeline_state: Option<RenderPipelineState>,
    #[cfg(target_os = "macos")]
    device: Option<Device>,
    epoch: u64,
}

impl MetalViewExample {
    fn new() -> Self {
        Self {
            #[cfg(target_os = "macos")]
            pipeline_state: None,
            #[cfg(target_os = "macos")]
            device: None,
            epoch: 0,
        }
    }

    fn update_epoch(&mut self, cx: &mut Context<Self>) {
        const MAX_EPOCH: u64 = 1024;
        self.epoch = (self.epoch + 1) % MAX_EPOCH;
        cx.notify();
    }

    #[cfg(target_os = "macos")]
    fn setup_metal(&mut self) {
        let device = Device::system_default().expect("no Metal device");

        // Shader that properly handles viewport transformation
        let shader_source = r#"
            #include <metal_stdlib>
            using namespace metal;

            struct Uniforms {
                float2 viewport_size;
                float epoch;
            };

            struct VertexOut {
                float4 position [[position]];
                float4 color;
            };

            vertex VertexOut vertex_main(
                uint vid [[vertex_id]],
                constant Uniforms& uniforms [[buffer(0)]]
            ) {
                VertexOut out;

                // Define a quad in pixel coordinates (0,0 to viewport_size)
                float2 positions[6] = {
                    float2(0.0, 0.0),                                      // top-left
                    float2(uniforms.viewport_size.x, 0.0),                // top-right
                    float2(0.0, uniforms.viewport_size.y),                // bottom-left
                    float2(uniforms.viewport_size.x, 0.0),                // top-right
                    float2(uniforms.viewport_size.x, uniforms.viewport_size.y), // bottom-right
                    float2(0.0, uniforms.viewport_size.y),                // bottom-left
                };

                // Transform from pixel coordinates to normalized device coordinates
                float2 pos = positions[vid];
                float2 ndc = (pos / uniforms.viewport_size) * 2.0 - 1.0;
                ndc.y = -ndc.y; // Flip Y axis to match screen coordinates

                out.position = float4(ndc, 0.0, 1.0);

                // Create an animated gradient using epoch
                float2 uv = pos / uniforms.viewport_size;
                float time = uniforms.epoch * 0.01;

                // Animate the gradient with some trigonometric functions
                out.color = float4(
                    0.5 + 0.5 * sin(uv.x * 3.14159 + time),          // Red
                    0.5 + 0.5 * sin(uv.y * 3.14159 + time * 1.3),    // Green
                    0.5 + 0.5 * sin((uv.x + uv.y) * 3.14159 - time * 0.7), // Blue
                    1.0                                                // Full opacity
                );

                return out;
            }

            fragment float4 fragment_main(VertexOut in [[stage_in]]) {
                return in.color;
            }
        "#;

        let library = device
            .new_library_with_source(shader_source, &metal::CompileOptions::new())
            .expect("Failed to create shader library");

        let vertex_function = library.get_function("vertex_main", None).unwrap();
        let fragment_function = library.get_function("fragment_main", None).unwrap();

        // Create pipeline state
        let pipeline_descriptor = metal::RenderPipelineDescriptor::new();
        pipeline_descriptor.set_vertex_function(Some(&vertex_function));
        pipeline_descriptor.set_fragment_function(Some(&fragment_function));

        let color_attachment = pipeline_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        color_attachment.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);

        // Enable blending
        color_attachment.set_blending_enabled(true);
        color_attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
        color_attachment
            .set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
        color_attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::One);
        color_attachment
            .set_destination_alpha_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);

        let pipeline_state = device
            .new_render_pipeline_state(&pipeline_descriptor)
            .expect("Failed to create pipeline state");

        self.device = Some(device);
        self.pipeline_state = Some(pipeline_state);
    }

    #[cfg(target_os = "macos")]
    fn create_render_callback(&self, epoch: u64) -> MetalRenderCallback {
        let pipeline_state = self.pipeline_state.clone().unwrap();

        Arc::new(
            move |encoder: &RenderCommandEncoderRef,
                  _target: &TextureRef,
                  bounds: Bounds<Pixels>,
                  scale_factor: f32| {
                // Set the pipeline state
                encoder.set_render_pipeline_state(&pipeline_state);

                // Set viewport to match element bounds
                let viewport = metal::MTLViewport {
                    originX: bounds.origin.x.0 as f64 * scale_factor as f64,
                    originY: bounds.origin.y.0 as f64 * scale_factor as f64,
                    width: bounds.size.width.0 as f64 * scale_factor as f64,
                    height: bounds.size.height.0 as f64 * scale_factor as f64,
                    znear: 0.0,
                    zfar: 1.0,
                };
                encoder.set_viewport(viewport);

                // Set scissor rectangle to clip to bounds
                let scissor_rect = metal::MTLScissorRect {
                    x: (bounds.origin.x.0 * scale_factor) as u64,
                    y: (bounds.origin.y.0 * scale_factor) as u64,
                    width: (bounds.size.width.0 * scale_factor) as u64,
                    height: (bounds.size.height.0 * scale_factor) as u64,
                };
                encoder.set_scissor_rect(scissor_rect);

                // Pass viewport size as uniform
                #[repr(C)]
                struct Uniforms {
                    viewport_size: [f32; 2],
                    epoch: f32,
                }

                let uniforms = Uniforms {
                    viewport_size: [
                        bounds.size.width.0 * scale_factor,
                        bounds.size.height.0 * scale_factor,
                    ],
                    epoch: epoch as f32,
                };

                encoder.set_vertex_bytes(
                    0,
                    std::mem::size_of::<Uniforms>() as u64,
                    &uniforms as *const Uniforms as *const _,
                );

                // Draw the quad
                encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 6);
            },
        )
    }
}

impl Render for MetalViewExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Initialize Metal on first render if on macOS
        #[cfg(target_os = "macos")]
        if self.pipeline_state.is_none() {
            self.setup_metal();
        }

        // Update epoch and request animation frame
        self.update_epoch(cx);
        window.request_animation_frame();

        div()
            .flex()
            .flex_col()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .p_8()
            .gap_6()
            .child(
                div()
                    .child("Metal View Element")
                    .text_2xl()
                    .text_color(rgb(0xffffff)),
            )
            .child(
                div()
                    .child("While GPUI normally handles all Metal rendering for you, the metal_view() element gives you direct access to write custom Metal shaders and GPU drawing commands")
                    .text_color(rgb(0xaaaaaa)),
            )
            .child(
                div()
                    .child("This is useful for special effects, custom visualizations, or when you need GPU performance that GPUI's standard elements can't provide")
                    .text_sm()
                    .text_color(rgb(0x888888)),
            )
            .child(div().overflow_hidden().child(
                #[cfg(target_os = "macos")]
                {
                    let callback = self.create_render_callback(self.epoch);
                    metal_view()
                        .render_with_shared(callback)
                        .w(px(600.0))
                        .h(px(400.0))
                        .bg(rgb(0x000000))
                },
                #[cfg(not(target_os = "macos"))]
                {
                    div()
                        .w(px(600.0))
                        .h(px(400.0))
                        .bg(rgb(0x222222))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(div().child("Metal (macOS only)").text_color(rgb(0x666666)))
                },
            ))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(900.0), px(600.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Metal View Element".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| MetalViewExample::new()),
        );

        cx.activate(false);
    });
}
