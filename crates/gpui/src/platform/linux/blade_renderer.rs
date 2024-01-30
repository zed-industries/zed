use super::{BladeBelt, BladeBeltDescriptor};
use crate::{PrimitiveBatch, Quad, Scene};
use bytemuck::{Pod, Zeroable};

use blade_graphics as gpu;
use std::sync::Arc;

const SURFACE_FRAME_COUNT: u32 = 3;
const MAX_FRAME_TIME_MS: u32 = 1000;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GlobalParams {
    viewport_size: [f32; 2],
    pad: [u32; 2],
}

#[derive(blade_macros::ShaderData)]
struct ShaderQuadsData {
    globals: GlobalParams,
    quads: gpu::BufferPiece,
}

struct BladePipelines {
    quads: gpu::RenderPipeline,
}

impl BladePipelines {
    fn new(gpu: &gpu::Context, surface_format: gpu::TextureFormat) -> Self {
        let shader = gpu.create_shader(gpu::ShaderDesc {
            source: include_str!("shaders.wgsl"),
        });
        shader.check_struct_size::<Quad>();
        let layout = <ShaderQuadsData as gpu::ShaderData>::layout();
        Self {
            quads: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "quads",
                data_layouts: &[&layout],
                vertex: shader.at("vs_quads"),
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: shader.at("fs_quads"),
                color_targets: &[gpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(gpu::BlendState::ALPHA_BLENDING),
                    write_mask: gpu::ColorWrites::default(),
                }],
            }),
        }
    }
}

pub struct BladeRenderer {
    gpu: Arc<gpu::Context>,
    command_encoder: gpu::CommandEncoder,
    last_sync_point: Option<gpu::SyncPoint>,
    pipelines: BladePipelines,
    instance_belt: BladeBelt,
    viewport_size: gpu::Extent,
}

impl BladeRenderer {
    pub fn new(gpu: Arc<gpu::Context>, size: gpu::Extent) -> Self {
        let surface_format = gpu.resize(gpu::SurfaceConfig {
            size,
            usage: gpu::TextureUsage::TARGET,
            frame_count: SURFACE_FRAME_COUNT,
        });
        let command_encoder = gpu.create_command_encoder(gpu::CommandEncoderDesc {
            name: "main",
            buffer_count: 2,
        });
        let pipelines = BladePipelines::new(&gpu, surface_format);
        let instance_belt = BladeBelt::new(BladeBeltDescriptor {
            memory: gpu::Memory::Shared,
            min_chunk_size: 0x1000,
        });
        Self {
            gpu,
            command_encoder,
            last_sync_point: None,
            pipelines,
            instance_belt,
            viewport_size: size,
        }
    }

    fn wait_for_gpu(&mut self) {
        if let Some(last_sp) = self.last_sync_point.take() {
            if !self.gpu.wait_for(&last_sp, MAX_FRAME_TIME_MS) {
                panic!("GPU hung");
            }
        }
    }

    pub fn destroy(&mut self) {
        self.wait_for_gpu();
        self.instance_belt.destroy(&self.gpu);
        self.gpu.destroy_command_encoder(&mut self.command_encoder);
    }

    pub fn resize(&mut self, size: gpu::Extent) {
        self.wait_for_gpu();
        self.gpu.resize(gpu::SurfaceConfig {
            size,
            usage: gpu::TextureUsage::TARGET,
            frame_count: SURFACE_FRAME_COUNT,
        });
        self.viewport_size = size;
    }

    pub fn draw(&mut self, scene: &Scene) {
        let frame = self.gpu.acquire_frame();
        self.command_encoder.start();
        self.command_encoder.init_texture(frame.texture());

        if let mut pass = self.command_encoder.render(gpu::RenderTargetSet {
            colors: &[gpu::RenderTarget {
                view: frame.texture_view(),
                init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                finish_op: gpu::FinishOp::Store,
            }],
            depth_stencil: None,
        }) {
            for batch in scene.batches() {
                match batch {
                    PrimitiveBatch::Quads(quads) => {
                        let instances = self.instance_belt.alloc_data(quads, &self.gpu);
                        let mut encoder = pass.with(&self.pipelines.quads);
                        encoder.bind(
                            0,
                            &ShaderQuadsData {
                                globals: GlobalParams {
                                    viewport_size: [
                                        self.viewport_size.width as f32,
                                        self.viewport_size.height as f32,
                                    ],
                                    pad: [0; 2],
                                },
                                quads: instances,
                            },
                        );
                        encoder.draw(0, 4, 0, quads.len() as u32);
                    }
                    _ => continue,
                }
            }
        }

        self.command_encoder.present(frame);
        let sync_point = self.gpu.submit(&mut self.command_encoder);
        self.instance_belt.flush(&sync_point);
        self.wait_for_gpu();
        self.last_sync_point = Some(sync_point);
    }
}
