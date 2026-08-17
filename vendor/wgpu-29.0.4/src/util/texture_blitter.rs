#![cfg(feature = "wgsl")]

use wgt::BlendState;

use crate::{
    include_wgsl, AddressMode, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, ColorTargetState, ColorWrites,
    CommandEncoder, Device, FilterMode, FragmentState, FrontFace, LoadOp, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler, SamplerBindingType,
    SamplerDescriptor, ShaderStages, StoreOp, TextureFormat, TextureSampleType, TextureView,
    TextureViewDimension, VertexState,
};

/// A builder for the [`TextureBlitter`] utility.
/// If you want the default [`TextureBlitter`] use [`TextureBlitter::new`] instead.
pub struct TextureBlitterBuilder<'a> {
    device: &'a Device,
    format: TextureFormat,
    sample_type: FilterMode,
    blend_state: Option<BlendState>,
}

impl<'a> TextureBlitterBuilder<'a> {
    /// Returns a new [`TextureBlitterBuilder`]
    ///
    /// # Arguments
    /// - `device` - A [`Device`]
    /// - `format` - The [`TextureFormat`] of the texture that will be copied to. This has to have the `RENDER_TARGET` usage.
    pub fn new(device: &'a Device, format: TextureFormat) -> Self {
        Self {
            device,
            format,
            sample_type: FilterMode::Nearest,
            blend_state: None,
        }
    }

    /// Sets the [`Sampler`] Filtering Mode
    pub fn sample_type(mut self, sample_type: FilterMode) -> Self {
        self.sample_type = sample_type;
        self
    }

    /// Sets the [`BlendState`] that is used.
    pub fn blend_state(mut self, blend_state: BlendState) -> Self {
        self.blend_state = Some(blend_state);
        self
    }

    /// Returns a new [`TextureBlitter`] with given settings.
    pub fn build(self) -> TextureBlitter {
        let sampler = self.device.create_sampler(&SamplerDescriptor {
            label: Some("wgpu::util::TextureBlitter::sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: self.sample_type,
            ..Default::default()
        });

        let bind_group_layout = self
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("wgpu::util::TextureBlitter::bind_group_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float {
                                filterable: self.sample_type == FilterMode::Linear,
                            },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(if self.sample_type == FilterMode::Linear {
                            SamplerBindingType::Filtering
                        } else {
                            SamplerBindingType::NonFiltering
                        }),
                        count: None,
                    },
                ],
            });

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("wgpu::util::TextureBlitter::pipeline_layout"),
                bind_group_layouts: &[Some(&bind_group_layout)],
                immediate_size: 0,
            });

        let shader = self.device.create_shader_module(include_wgsl!("blit.wgsl"));
        let pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("wgpu::util::TextureBlitter::pipeline"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: PipelineCompilationOptions::default(),
                    buffers: &[],
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgt::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: PipelineCompilationOptions::default(),
                    targets: &[Some(ColorTargetState {
                        format: self.format,
                        blend: self.blend_state,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            });

        TextureBlitter {
            pipeline,
            bind_group_layout,
            sampler,
        }
    }
}

/// Texture Blitting (Copying) Utility
///
/// Use this if you want to just render/copy texture A to texture B where [`CommandEncoder::copy_texture_to_texture`] would not work because:
/// - Textures are in incompatible formats.
/// - Textures are of different sizes.
/// - Your copy destination is the surface texture and does not have the `COPY_DST` usage.
pub struct TextureBlitter {
    pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    sampler: Sampler,
}

impl TextureBlitter {
    /// Returns a [`TextureBlitter`] with default settings.
    ///
    /// # Arguments
    /// - `device` - A [`Device`]
    /// - `format` - The [`TextureFormat`] of the texture that will be copied to. This has to have the `RENDER_TARGET` usage.
    ///
    /// Properties of the blitting (such as the [`BlendState`]) can be customised by using [`TextureBlitterBuilder`] instead.
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        TextureBlitterBuilder::new(device, format).build()
    }

    /// Copies the data from the source [`TextureView`] to the target [`TextureView`]
    ///
    /// # Arguments
    /// - `device` - A [`Device`]
    /// - `encoder` - A [`CommandEncoder`]
    /// - `source` - A [`TextureView`] that gets copied. The format does not matter.
    /// - `target` - A [`TextureView`] that gets the data copied from the `source`. It has to be the same format as the format specified in [`TextureBlitter::new`]
    pub fn copy(
        &self,
        device: &Device,
        encoder: &mut CommandEncoder,
        source: &TextureView,
        target: &TextureView,
    ) {
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("wgpu::util::TextureBlitter::bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: crate::BindingResource::TextureView(source),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: crate::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("wgpu::util::TextureBlitter::pass"),
            color_attachments: &[Some(crate::RenderPassColorAttachment {
                view: target,
                depth_slice: None,
                resolve_target: None,
                ops: wgt::Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}
