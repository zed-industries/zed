use crate::Scene;

use std::sync::Arc;

const SURFACE_FRAME_COUNT: u32 = 3;
const MAX_FRAME_TIME_MS: u32 = 1000;

pub struct BladeRenderer {
    gpu: Arc<blade::Context>,
    command_encoder: blade::CommandEncoder,
    last_sync_point: Option<blade::SyncPoint>,
}

impl BladeRenderer {
    pub fn new(gpu: Arc<blade::Context>, size: blade::Extent) -> Self {
        let _surface_format = gpu.resize(blade::SurfaceConfig {
            size,
            usage: blade::TextureUsage::TARGET,
            frame_count: SURFACE_FRAME_COUNT,
        });
        let command_encoder = gpu.create_command_encoder(blade::CommandEncoderDesc {
            name: "main",
            buffer_count: 2,
        });
        Self {
            gpu,
            command_encoder,
            last_sync_point: None,
        }
    }

    pub fn destroy(&mut self) {
        self.gpu.destroy_command_encoder(&mut self.command_encoder);
    }

    pub fn resize(&mut self, size: blade::Extent) {
        self.gpu.resize(blade::SurfaceConfig {
            size,
            usage: blade::TextureUsage::TARGET,
            frame_count: SURFACE_FRAME_COUNT,
        });
    }

    pub fn draw(&mut self, scene: &Scene) {
        let frame = self.gpu.acquire_frame();
        self.command_encoder.start();

        self.command_encoder.present(frame);

        let sync_point = self.gpu.submit(&mut self.command_encoder);
        if let Some(ref last_sp) = self.last_sync_point {
            if !self.gpu.wait_for(last_sp, MAX_FRAME_TIME_MS) {
                panic!("GPU hung");
            }
        }
        self.last_sync_point = Some(sync_point);
    }
}
