use gpui::{prelude::*, *};
use std::sync::Arc;
use std::time::Instant;

#[cfg(target_os = "macos")]
use metal::{Device, MTLPrimitiveType, RenderCommandEncoderRef, RenderPipelineState, TextureRef};

struct MetalViewExample {
    start_time: Instant,
    #[cfg(target_os = "macos")]
    pipeline_state: Option<RenderPipelineState>,
    #[cfg(target_os = "macos")]
    device: Option<Device>,
}

impl MetalViewExample {
    fn new() -> Self {
        Self {
            #[cfg(target_os = "macos")]
            pipeline_state: None,
            #[cfg(target_os = "macos")]
            device: None,
            start_time: Instant::now(),
        }
    }

    #[cfg(target_os = "macos")]
    fn setup_metal(&mut self) {
        let device = Device::system_default().expect("no Metal device");

        // Simplified shader for debugging
        let shader_source = r#"
            #include <metal_stdlib>
            using namespace metal;

            struct Uniforms {
                float time;
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

                // Define triangle vertices in normalized device coordinates
                float2 positions[3] = {
                    float2( 0.0,  0.5),  // Top
                    float2(-0.5, -0.5),  // Bottom left
                    float2( 0.5, -0.5)   // Bottom right
                };

                float3 colors[3] = {
                    float3(1.0, 0.0, 0.0),  // Red
                    float3(0.0, 1.0, 0.0),  // Green
                    float3(0.0, 0.0, 1.0)   // Blue
                };

                // Apply rotation
                float2 pos = positions[vid];
                float c = cos(uniforms.time);
                float s = sin(uniforms.time);
                float2 rotated = float2(
                    pos.x * c - pos.y * s,
                    pos.x * s + pos.y * c
                );

                out.position = float4(rotated, 0.0, 1.0);
                out.color = float4(colors[vid], 1.0);
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

        // Create pipeline state - no vertex descriptor needed for vertex_id based rendering
        let pipeline_descriptor = metal::RenderPipelineDescriptor::new();
        pipeline_descriptor.set_vertex_function(Some(&vertex_function));
        pipeline_descriptor.set_fragment_function(Some(&fragment_function));

        let color_attachment = pipeline_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        color_attachment.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);

        // Note: Depth testing is not enabled for now as it requires proper depth buffer setup
        // in the GPUI rendering pipeline

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
    fn create_render_callback(&self, time_delta: f32) -> MetalRenderCallback {
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

                // Pass time as uniform
                let time = time_delta * 2.0; // Scale for reasonable rotation speed
                #[repr(C)]
                struct Uniforms {
                    time: f32,
                }
                let uniforms = Uniforms { time };
                encoder.set_vertex_bytes(
                    0,
                    std::mem::size_of::<Uniforms>() as u64,
                    &uniforms as *const Uniforms as *const _,
                );

                // Draw triangle using vertex_id - no vertex buffer needed
                encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
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

        // Request animation frame
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
                    .child("This example shows a rotating 3D cube - the 'Hello World' of 3D graphics programming")
                    .text_sm()
                    .text_color(rgb(0x888888)),
            )
            .child(div().overflow_hidden().child(
                #[cfg(target_os = "macos")]
                {
                    let elapsed = self.start_time.elapsed().as_secs_f32();
                    let callback = self.create_render_callback(elapsed);
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
