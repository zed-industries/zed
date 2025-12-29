//! Spinning cube example for custom render passes.
//!
//! This example demonstrates:
//! - Vertex buffer creation for cube geometry
//! - Index buffer for cube faces
//! - Uniform buffer for MVP matrix
//! - Rotating camera animation
//! - Buffer lifetime management across frames
//!
//! Run with:
//! ```
//! cargo run -p gpui --example spinning_cube --features custom_render_pass,macos-blade
//! ```

use std::sync::Arc;
use std::time::Instant;

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};

#[cfg(feature = "custom_render_pass")]
use gpui::{gpu, BladeRenderPassContext, CustomRenderPass, DevicePixels, RenderStage, Size};

#[cfg(feature = "custom_render_pass")]
use std::sync::Mutex;

/// Vertex data for the cube (position + color)
#[cfg(feature = "custom_render_pass")]
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CubeVertex {
    position: [f32; 3],
    color: [f32; 3],
}

/// Uniform data for the MVP matrix
#[cfg(feature = "custom_render_pass")]
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [[f32; 4]; 4],
}


/// GPU resources for the cube renderer
#[cfg(feature = "custom_render_pass")]
struct CubeResources {
    pipeline: gpu::RenderPipeline,
    vertex_buffer: gpu::Buffer,
    index_buffer: gpu::Buffer,
    uniform_buffer: gpu::Buffer,
}

/// A custom render pass that renders a spinning 3D cube
#[cfg(feature = "custom_render_pass")]
struct SpinningCubePass {
    start_time: Instant,
    resources: Mutex<Option<CubeResources>>,
}

#[cfg(feature = "custom_render_pass")]
impl SpinningCubePass {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            resources: Mutex::new(None),
        }
    }

    fn create_resources(&self, gpu_context: &gpu::Context) -> CubeResources {
        // Define cube vertices with colors
        // Each face has a different color
        let vertices: [CubeVertex; 24] = [
            // Front face (red)
            CubeVertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.2, 0.2] },
            CubeVertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 0.2, 0.2] },
            CubeVertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.2, 0.2] },
            CubeVertex { position: [-0.5,  0.5,  0.5], color: [1.0, 0.2, 0.2] },
            // Back face (green)
            CubeVertex { position: [-0.5, -0.5, -0.5], color: [0.2, 1.0, 0.2] },
            CubeVertex { position: [-0.5,  0.5, -0.5], color: [0.2, 1.0, 0.2] },
            CubeVertex { position: [ 0.5,  0.5, -0.5], color: [0.2, 1.0, 0.2] },
            CubeVertex { position: [ 0.5, -0.5, -0.5], color: [0.2, 1.0, 0.2] },
            // Top face (blue)
            CubeVertex { position: [-0.5,  0.5, -0.5], color: [0.2, 0.2, 1.0] },
            CubeVertex { position: [-0.5,  0.5,  0.5], color: [0.2, 0.2, 1.0] },
            CubeVertex { position: [ 0.5,  0.5,  0.5], color: [0.2, 0.2, 1.0] },
            CubeVertex { position: [ 0.5,  0.5, -0.5], color: [0.2, 0.2, 1.0] },
            // Bottom face (yellow)
            CubeVertex { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.2] },
            CubeVertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 0.2] },
            CubeVertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 0.2] },
            CubeVertex { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 0.2] },
            // Right face (magenta)
            CubeVertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 0.2, 1.0] },
            CubeVertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 0.2, 1.0] },
            CubeVertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.2, 1.0] },
            CubeVertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 0.2, 1.0] },
            // Left face (cyan)
            CubeVertex { position: [-0.5, -0.5, -0.5], color: [0.2, 1.0, 1.0] },
            CubeVertex { position: [-0.5, -0.5,  0.5], color: [0.2, 1.0, 1.0] },
            CubeVertex { position: [-0.5,  0.5,  0.5], color: [0.2, 1.0, 1.0] },
            CubeVertex { position: [-0.5,  0.5, -0.5], color: [0.2, 1.0, 1.0] },
        ];

        // Index buffer for 12 triangles (6 faces * 2 triangles each)
        let indices: [u16; 36] = [
            0,  1,  2,  0,  2,  3,   // front
            4,  5,  6,  4,  6,  7,   // back
            8,  9,  10, 8,  10, 11,  // top
            12, 13, 14, 12, 14, 15,  // bottom
            16, 17, 18, 16, 18, 19,  // right
            20, 21, 22, 20, 22, 23,  // left
        ];

        // Create vertex buffer
        let vertex_data = bytemuck::cast_slice(&vertices);
        let vertex_buffer = gpu_context.create_buffer(gpu::BufferDesc {
            name: "cube_vertices",
            size: vertex_data.len() as u64,
            memory: gpu::Memory::Shared,
        });
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertex_data.as_ptr(),
                vertex_buffer.data(),
                vertex_data.len(),
            );
        }

        // Create index buffer
        let index_data = bytemuck::cast_slice(&indices);
        let index_buffer = gpu_context.create_buffer(gpu::BufferDesc {
            name: "cube_indices",
            size: index_data.len() as u64,
            memory: gpu::Memory::Shared,
        });
        unsafe {
            std::ptr::copy_nonoverlapping(
                index_data.as_ptr(),
                index_buffer.data(),
                index_data.len(),
            );
        }

        // Create uniform buffer
        let uniform_buffer = gpu_context.create_buffer(gpu::BufferDesc {
            name: "cube_uniforms",
            size: std::mem::size_of::<Uniforms>() as u64,
            memory: gpu::Memory::Shared,
        });

        // Create shader and pipeline
        // Note: blade uses implicit binding, not explicit @group/@binding annotations
        // The uniforms variable name must match the field name in CubeShaderData
        // Vertex input struct members must match VertexLayout attribute names
        let shader_source = r#"
struct Uniforms {
    mvp: mat4x4<f32>,
}

var<uniform> uniforms: Uniforms;

struct VertexInput {
    position: vec3<f32>,
    color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = uniforms.mvp * vec4<f32>(input.position, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
        "#;

        let shader = gpu_context.create_shader(gpu::ShaderDesc {
            source: shader_source,
        });
        shader.check_struct_size::<Uniforms>();

        let pipeline = gpu_context.create_render_pipeline(gpu::RenderPipelineDesc {
            name: "spinning_cube",
            data_layouts: &[&<CubeShaderData as gpu::ShaderData>::layout()],
            vertex: shader.at("vs_main"),
            vertex_fetches: &[
                gpu::VertexFetchState {
                    layout: &gpu::VertexLayout {
                        attributes: vec![
                            ("position", gpu::VertexAttribute {
                                offset: 0,
                                format: gpu::VertexFormat::F32Vec3,
                            }),
                            ("color", gpu::VertexAttribute {
                                offset: 12,
                                format: gpu::VertexFormat::F32Vec3,
                            }),
                        ],
                        stride: std::mem::size_of::<CubeVertex>() as u32,
                    },
                    instanced: false,
                },
            ],
            primitive: gpu::PrimitiveState {
                topology: gpu::PrimitiveTopology::TriangleList,
                front_face: gpu::FrontFace::Ccw,
                cull_mode: None, // Disable culling so all faces show
                ..Default::default()
            },
            depth_stencil: None, // Disable depth testing for now
            fragment: Some(shader.at("fs_main")),
            color_targets: &[gpu::ColorTargetState {
                format: gpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(gpu::BlendState::REPLACE),
                write_mask: gpu::ColorWrites::ALL,
            }],
            multisample_state: gpu::MultisampleState::default(),
        });

        CubeResources {
            pipeline,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
        }
    }

    fn compute_mvp(&self, aspect_ratio: f32) -> [[f32; 4]; 4] {
        let time = self.start_time.elapsed().as_secs_f32();

        // Simple rotation around Y axis
        let angle = time * 1.0;
        let c = angle.cos();
        let s = angle.sin();

        // Scale and rotate around Y axis
        // Using column-major order (each inner array is a column)
        let scale = 0.5; // Make it bigger

        // Simple orthographic-ish projection with rotation
        // Maps cube vertices directly to NDC coordinates
        [
            [scale * c / aspect_ratio, 0.0, scale * s / aspect_ratio, 0.0],
            [0.0, scale, 0.0, 0.0],
            [-scale * s, 0.0, scale * c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}

#[cfg(feature = "custom_render_pass")]
impl CustomRenderPass for SpinningCubePass {
    fn render(&self, ctx: &mut BladeRenderPassContext) {
        let mut resources_guard = self.resources.lock().unwrap();

        // Initialize resources on first render
        if resources_guard.is_none() {
            *resources_guard = Some(self.create_resources(ctx.gpu));
        }

        let resources = resources_guard.as_ref().unwrap();

        // Update uniform buffer with current MVP matrix
        let viewport_width = ctx.frame_info.viewport_size.width.0 as f32;
        let viewport_height = ctx.frame_info.viewport_size.height.0 as f32;
        let aspect_ratio = viewport_width / viewport_height;

        let uniforms = Uniforms {
            mvp: self.compute_mvp(aspect_ratio),
        };

        unsafe {
            let uniform_data = bytemuck::bytes_of(&uniforms);
            std::ptr::copy_nonoverlapping(
                uniform_data.as_ptr(),
                resources.uniform_buffer.data(),
                uniform_data.len(),
            );
        }

        // Begin render pass - no depth buffer for now
        let mut pass = ctx.encoder.render(
            "spinning_cube",
            gpu::RenderTargetSet {
                colors: &[gpu::RenderTarget {
                    view: ctx.frame,
                    init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                    finish_op: gpu::FinishOp::Store,
                }],
                depth_stencil: None,
            },
        );

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

        // Bind pipeline and draw
        let mut encoder = pass.with(&resources.pipeline);
        encoder.bind(
            0,
            &CubeShaderData {
                uniforms: resources.uniform_buffer.into(),
            },
        );
        encoder.bind_vertex(0, resources.vertex_buffer.into());
        encoder.draw_indexed(
            resources.index_buffer.into(),
            gpu::IndexType::U16,
            36,  // index_count: 36 indices
            0,   // base_vertex
            0,   // start_instance
            1,   // instance_count
        );
    }

    fn resize(&self, _new_size: Size<DevicePixels>) {
        // No special handling needed - MVP is recomputed each frame
    }

    fn name(&self) -> &str {
        "spinning_cube"
    }
}

/// Shader data binding for the cube uniforms
#[cfg(feature = "custom_render_pass")]
#[derive(blade_macros::ShaderData)]
struct CubeShaderData {
    uniforms: gpu::BufferPiece,
}

struct HelloWorld {
    text: SharedString,
    _animation_task: gpui::Task<()>,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(gpui::rgba(0x20202080))
            .size(px(300.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0xffffff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child("Spinning Cube Demo")
            .child(
                div()
                    .text_sm()
                    .text_color(gpui::rgba(0xffffffaa))
                    .child("3D cube rendered via custom GPU pass"),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                // Register the spinning cube render pass
                #[cfg(feature = "custom_render_pass")]
                {
                    let pass = Arc::new(SpinningCubePass::new());
                    window.register_render_pass(RenderStage::BeforeUi, pass);
                }

                #[cfg(not(feature = "custom_render_pass"))]
                {
                    eprintln!(
                        "Warning: custom_render_pass feature is not enabled. \
                         Run with --features custom_render_pass,macos-blade"
                    );
                }

                cx.new(|cx| {
                    // Spawn a task that continuously requests redraws for animation
                    let animation_task = cx.spawn(async move |this, cx| {
                        loop {
                            cx.background_executor()
                                .timer(std::time::Duration::from_millis(16)) // ~60fps
                                .await;
                            let result = this.update(cx, |_, cx| {
                                cx.notify(); // Request redraw
                            });
                            if result.is_err() {
                                break; // Entity was dropped
                            }
                        }
                    });
                    HelloWorld {
                        text: "World".into(),
                        _animation_task: animation_task,
                    }
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
