use crate::{DevicePixels, Scene, Size};
use futures::{future::BoxFuture, FutureExt};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::Backends;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    quad_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    uniforms_buffer: wgpu::Buffer,
}

pub(crate) trait RenderTarget: HasRawWindowHandle + HasRawDisplayHandle {
    fn content_device_size(&self) -> Size<DevicePixels>;
}

impl Renderer {
    pub(crate) fn new<'a, W>(window: &'a W) -> BoxFuture<'static, Self>
    where
        W: RenderTarget,
    {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::METAL,
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(window).unwrap() };
        let width = window.content_device_size().width;
        let height = window.content_device_size().height;

        async move {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions::default())
                .await
                .unwrap();

            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor::default(), None)
                .await
                .unwrap();

            let surface_config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: width.into(),
                height: height.into(),

                // "FIFO" mode renders frames in queue synced with the display's refresh rate.
                // Avoids screen tearing but may not offer the lowest latency. Ideal when image
                // quality takes priority over input latency.
                present_mode: wgpu::PresentMode::Fifo,

                // When blending, assume the RGB have not yet been multiplied by the alpha channel.
                alpha_mode: wgpu::CompositeAlphaMode::PostMultiplied,
PostMultiplied
                // Specify the color formats for the views the surface can have.
                // In this case, the format is BGRA (blue, green, red, alpha) with unsigned
                // normalised integers in the 8-bit range and the color space is sRGB (standard RGB).
                // sRGB is the standard color space for displaying images and video on digital displays,
                // as it optimises color accuracy and consistency.
                view_formats: vec![wgpu::TextureFormat::Bgra8UnormSrgb],
            };

            surface.configure(&device, &surface_config);

            let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Vertex Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.vert.wgsl").into()),
            });

            let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Fragment Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.frag.wgsl").into()),
            });

            let uniforms_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Uniforms Buffer"),
                size: std::mem::size_of::<QuadUniforms>() as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

            let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Vertex Buffer"),
                size: 0,
                usage: wgpu::BufferUsages::VERTEX,
                mapped_at_creation: false,
            });

            let quad_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vs_module,
                    entry_point: "quad",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fs_module,
                    entry_point: "quad",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

            Self {
                device,
                queue,
                surface,
                surface_config,
                quad_pipeline,
                vertex_buffer,
                vertex_count: 0,
                uniforms_buffer,
            }
        }
        .boxed()
    }

    pub fn render(&mut self, scene: &Scene) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(&scene.opaque_primitives().quads),
        );
        self.vertex_count = scene.opaque_primitives().quads.len() as u32;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.quad_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.uniforms_buffer, &[]);
            render_pass.draw(0..self.vertex_count, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
