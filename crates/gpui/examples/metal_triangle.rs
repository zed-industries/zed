use core_foundation::base::TCFType;
use core_video::{
    metal_texture::CVMetalTextureGetTexture,
    metal_texture_cache::CVMetalTextureCache,
    pixel_buffer::{kCVPixelFormatType_32BGRA, CVPixelBuffer},
};
use foreign_types::ForeignTypeRef;
use gpui::{
    actions, div, prelude::*, App, AppContext, Bounds, Context, IntoElement, Render, Size,
    Window, WindowBounds, WindowOptions, px,
};
use metal::{
    CommandQueue, Device, MTLPixelFormat, MTLPrimitiveType,
    RenderPassDescriptor, RenderPipelineDescriptor, RenderPipelineState,
};
use std::time::Instant;

actions!(metal_triangle, [Quit]);

struct TriangleView {
    renderer: MetalTriangleRenderer,
    last_frame_time: Instant,
    frame_count: usize,
    fps: f64,
}

impl TriangleView {
    fn new() -> Self {
        Self {
            renderer: MetalTriangleRenderer::new(),
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps: 0.0,
        }
    }
}

impl Render for TriangleView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let buffer = self.renderer.render();
        
        // Calculate FPS
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame_time);
        if elapsed.as_secs_f64() >= 1.0 {
            self.fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_frame_time = now;
        }

        // Schedule the next frame to keep the animation loop running
        let handle = cx.entity().clone();
        window.on_next_frame(move |window, cx| {
            handle.update(cx, |this, cx| {
                this.renderer.update();
                cx.notify();
            });
        });

        // We wrap the CVPixelBuffer in a Surface element.
        // This is the key integration point.
        div().w(px(800.0)).h(px(600.0)).child(
            div().flex().flex_col().size_full()
                .child(
                    gpui::surface(buffer).size_full()
                )
                .child(
                    div()
                        .absolute()
                        .top(px(10.0))
                        .left(px(10.0))
                        .text_color(gpui::white())
                        .child(format!("FPS: {:.2}", self.fps))
                )
        )
    }
}

struct MetalTriangleRenderer {
    device: Device,
    command_queue: CommandQueue,
    pipeline_state: RenderPipelineState,
    width: usize,
    height: usize,
    rotation: f32,
}

#[repr(C)]
struct Uniforms {
    transform: [f32; 16],
}

impl MetalTriangleRenderer {
    fn new() -> Self {
        let device = Device::system_default().expect("no metal device found");
        let command_queue = device.new_command_queue();

        let library_source = r#"
            #include <metal_stdlib>
            using namespace metal;

            struct VertexOut {
                float4 position [[position]];
                float4 color;
            };

            struct Uniforms {
                float4x4 transform;
            };

            vertex VertexOut triangle_vertex(uint vertexID [[vertex_id]], constant Uniforms &uniforms [[buffer(0)]]) {
                const float2 vertices[] = {
                    float2(0.0, 0.5),
                    float2(-0.5, -0.5),
                    float2(0.5, -0.5),
                };
                const float4 colors[] = {
                    float4(1.0, 0.0, 0.0, 1.0),
                    float4(0.0, 1.0, 0.0, 1.0),
                    float4(0.0, 0.0, 1.0, 1.0),
                };

                VertexOut out;
                out.position = uniforms.transform * float4(vertices[vertexID], 0.0, 1.0);
                out.color = colors[vertexID];
                return out;
            }

            fragment float4 triangle_fragment(VertexOut in [[stage_in]]) {
                return in.color;
            }
        "#;

        let library = device
            .new_library_with_source(library_source, &metal::CompileOptions::new())
            .expect("failed to compile metal library");
        
        let vertex_function = library.get_function("triangle_vertex", None).unwrap();
        let fragment_function = library.get_function("triangle_fragment", None).unwrap();

        let pipeline_descriptor = RenderPipelineDescriptor::new();
        pipeline_descriptor.set_vertex_function(Some(&vertex_function));
        pipeline_descriptor.set_fragment_function(Some(&fragment_function));
        pipeline_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap()
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);

        let pipeline_state = device
            .new_render_pipeline_state(&pipeline_descriptor)
            .expect("failed to create pipeline state");

        Self {
            device,
            command_queue,
            pipeline_state,
            width: 800,
            height: 600,
            rotation: 0.0,
        }
    }

    fn update(&mut self) {
        self.rotation += 0.02;
    }

    fn render(&self) -> CVPixelBuffer {
        // 1. Create CVPixelBuffer
        let io_surface_properties = unsafe {
            let key = core_video::pixel_buffer::kCVPixelBufferIOSurfacePropertiesKey;
            let value: core_foundation::dictionary::CFDictionary<core_foundation::string::CFString, core_foundation::boolean::CFBoolean> = core_foundation::dictionary::CFDictionary::from_CFType_pairs(&[]);
            core_foundation::dictionary::CFDictionary::from_CFType_pairs(
                &[(
                    core_foundation::base::TCFType::wrap_under_get_rule(key),
                    value.as_CFType(),
                )],
            )
        };

        let buffer = CVPixelBuffer::new(
            kCVPixelFormatType_32BGRA,
            self.width,
            self.height,
            Some(&io_surface_properties),
        )
        .expect("failed to create CVPixelBuffer");

        // 2. Create Metal texture from CVPixelBuffer
        let core_video_texture_cache = CVMetalTextureCache::new(None, self.device.clone(), None).unwrap();
        let texture_ref = core_video_texture_cache.create_texture_from_image(
            buffer.as_concrete_TypeRef(),
            None,
            MTLPixelFormat::BGRA8Unorm,
            self.width,
            self.height,
            0
        ).unwrap();
        
        let texture = unsafe {
            let ptr = CVMetalTextureGetTexture(texture_ref.as_concrete_TypeRef());
            metal::TextureRef::from_ptr(ptr as *mut _)
        };

        // 3. Render to the texture
        let command_buffer = self.command_queue.new_command_buffer();
        let render_pass_descriptor = RenderPassDescriptor::new();
        let color_attachment = render_pass_descriptor.color_attachments().object_at(0).unwrap();
        
        color_attachment.set_texture(Some(&texture));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_clear_color(metal::MTLClearColor::new(0.0, 0.0, 0.0, 1.0));
        color_attachment.set_store_action(metal::MTLStoreAction::Store);

        let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
        encoder.set_render_pipeline_state(&self.pipeline_state);

        // Create rotation matrix
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        let transform = [
            cos, 0.0, -sin, 0.0,
            0.0, 1.0, 0.0, 0.0,
            sin, 0.0, cos, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        let uniforms = Uniforms { transform };
        
        encoder.set_vertex_bytes(
            0,
            std::mem::size_of::<Uniforms>() as u64,
            &uniforms as *const Uniforms as *const _,
        );

        encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
        encoder.end_encoding();

        command_buffer.commit();
        command_buffer.wait_until_completed();

        buffer
    }
}

fn main() {
    gpui::Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    Size {
                        width: px(800.0),
                        height: px(600.0),
                    },
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_cx| TriangleView::new())
            },
        )
        .unwrap();
    });
}
