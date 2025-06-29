use gpui::{Application, *};
use std::sync::Arc;

#[cfg(target_os = "macos")]
use metal::{Device, MTLPrimitiveType, RenderCommandEncoderRef, RenderPipelineState, TextureRef};

struct MetalViewExample {
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
        }
    }

    #[cfg(target_os = "macos")]
    fn setup_metal(&mut self) {
        // Create Metal device
        let device = Device::system_default().expect("no Metal device");

        // Create shader library from source
        let shader_source = r#"
            #include <metal_stdlib>
            using namespace metal;

            struct VertexOut {
                float4 position [[position]];
                float4 color;
            };

            vertex VertexOut vertex_main(uint vid [[vertex_id]]) {
                VertexOut out;

                // Create a rectangle using two triangles
                // Triangle 1: top-left, top-right, bottom-left
                // Triangle 2: top-right, bottom-right, bottom-left
                float2 positions[6] = {
                    float2(-1.0,  1.0), // top-left
                    float2( 1.0,  1.0), // top-right
                    float2(-1.0, -1.0), // bottom-left
                    float2( 1.0,  1.0), // top-right
                    float2( 1.0, -1.0), // bottom-right
                    float2(-1.0, -1.0), // bottom-left
                };

                out.position = float4(positions[vid], 0.0, 1.0);
                // Create a gradient color based on position
                out.color = float4(
                    (positions[vid].x + 1.0) * 0.5,  // Red based on X
                    (positions[vid].y + 1.0) * 0.5,  // Green based on Y
                    0.7,                              // Blue constant
                    1.0                               // Alpha
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

        // Configure color attachment
        let color_attachment = pipeline_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        color_attachment.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);

        // Enable blending to work with GPUI's existing content
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
    fn create_render_callback(&self) -> MetalRenderCallback {
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

                // Draw the rectangle (6 vertices for 2 triangles)
                encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 6);
            },
        )
    }
}

impl Render for MetalViewExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Initialize Metal on first render if on macOS
        #[cfg(target_os = "macos")]
        if self.pipeline_state.is_none() {
            self.setup_metal();
        }

        div()
            .flex()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .justify_center()
            .items_center()
            .child(
                div()
                    .flex_col()
                    .gap_4()
                    .child(
                        div().flex().justify_center().child(
                            div()
                                .child("Metal View Example")
                                .text_xl()
                                .text_color(rgb(0xffffff)),
                        ),
                    )
                    .child(
                        div()
                            .border_1()
                            .border_color(rgb(0x444444))
                            .rounded_md()
                            .overflow_hidden()
                            .child(
                                // The Metal view
                                #[cfg(target_os = "macos")]
                                {
                                    let callback = self.create_render_callback();
                                    metal_view()
                                        .render_with_shared(callback)
                                        .w(px(400.0))
                                        .h(px(300.0))
                                        .bg(rgb(0x000000))
                                },
                                #[cfg(not(target_os = "macos"))]
                                {
                                    // Fallback for non-macOS platforms
                                    div()
                                        .w(px(400.0))
                                        .h(px(300.0))
                                        .bg(rgb(0x222222))
                                        .flex()
                                        .justify_center()
                                        .items_center()
                                        .child(
                                            div()
                                                .child("Metal rendering is only available on macOS")
                                                .text_color(rgb(0x888888)),
                                        )
                                },
                            ),
                    )
                    .child(
                        div().flex().justify_center().child(
                            div()
                                .child("A gradient rectangle rendered with custom Metal shaders")
                                .text_sm()
                                .text_color(rgb(0xaaaaaa)),
                        ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point::new(px(100.0), px(100.0)),
                    size: Size {
                        width: px(600.0),
                        height: px(500.0),
                    },
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("Metal View Example".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| MetalViewExample::new()),
        );
    });
}

// Additional example: Using MetalView for more complex rendering
#[cfg(target_os = "macos")]
#[allow(dead_code)]
mod advanced_example {
    use super::*;
    use std::sync::Mutex;

    /// Example of a MetalView that renders an animated scene
    pub struct AnimatedMetalView {
        device: Device,
        pipeline_state: RenderPipelineState,
        frame_count: Arc<Mutex<f32>>,
    }

    impl AnimatedMetalView {
        pub fn create_animated_renderer(&self) -> MetalRenderCallback {
            let pipeline_state = self.pipeline_state.clone();
            let frame_count = self.frame_count.clone();

            Arc::new(
                move |encoder: &RenderCommandEncoderRef,
                      _target: &TextureRef,
                      bounds: Bounds<Pixels>,
                      scale_factor: f32| {
                    // Update animation state
                    let mut count = frame_count.lock().unwrap();
                    *count += 0.01;
                    let time = *count;

                    // Set pipeline and viewport
                    encoder.set_render_pipeline_state(&pipeline_state);

                    let viewport = metal::MTLViewport {
                        originX: bounds.origin.x.0 as f64 * scale_factor as f64,
                        originY: bounds.origin.y.0 as f64 * scale_factor as f64,
                        width: bounds.size.width.0 as f64 * scale_factor as f64,
                        height: bounds.size.height.0 as f64 * scale_factor as f64,
                        znear: 0.0,
                        zfar: 1.0,
                    };
                    encoder.set_viewport(viewport);

                    // Pass time as a uniform
                    encoder.set_vertex_bytes(
                        0,
                        std::mem::size_of::<f32>() as u64,
                        &time as *const f32 as *const _,
                    );

                    // Draw animated geometry
                    encoder.draw_primitives(MTLPrimitiveType::TriangleStrip, 0, 4);
                },
            )
        }
    }
}

// Example usage in a component:
// ```rust
// struct MyApp {
//     metal_renderer: Option<MetalRenderCallback>,
// }
//
// impl Render for MyApp {
//     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//         div()
//             .child(
//                 metal_view()
//                     .render_with(|encoder, target, bounds, scale_factor| {
//                         // Your custom Metal rendering code here
//                     })
//                     .size_full()
//             )
//     }
// }
// ```
