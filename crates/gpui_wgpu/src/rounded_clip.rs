pub(crate) struct Bindings {
    pub(crate) layout: wgpu::BindGroupLayout,
    pub(crate) unmasked: wgpu::BindGroup,
}

impl Bindings {
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("rounded_clip_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(16),
                    },
                    count: None,
                },
            ],
        });
        let dummy = texture(device, wgpu::TextureFormat::Rgba8Unorm, 1, 1);
        let unmasked = bind(
            device,
            &layout,
            &dummy.create_view(&Default::default()),
            &[0; 4],
        );
        Self { layout, unmasked }
    }

    pub(crate) fn clip_sets(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        sets: &[Vec<[[f32; 4]; 3]>],
    ) -> (wgpu::BindGroup, u32) {
        let alignment = device.limits().min_uniform_buffer_offset_alignment.max(16);
        let mut all = Vec::new();
        let mut flags = vec![0u32; (alignment / 4) as usize];
        for clips in sets {
            let start = all.len() as u32;
            all.extend_from_slice(clips);
            let mut row = vec![0u32; (alignment / 4) as usize];
            row[0] = clips.len() as u32;
            row[1] = start;
            flags.extend(row);
        }
        let texels = all.len() as u32 * 3;
        let width = texels.min(device.limits().max_texture_dimension_2d).max(1);
        let height = texels.div_ceil(width).max(1);
        let data_texture = texture(device, wgpu::TextureFormat::Rgba32Float, width, height);
        let mut data = bytemuck::cast_slice::<[[f32; 4]; 3], u8>(&all).to_vec();
        data.resize((width * height * 16) as usize, 0);
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &data_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 16),
                rows_per_image: None,
            },
            data_texture.size(),
        );
        (
            bind(
                device,
                &self.layout,
                &data_texture.create_view(&Default::default()),
                &flags,
            ),
            alignment,
        )
    }
}

fn bind(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    view: &wgpu::TextureView,
    values: &[u32],
) -> wgpu::BindGroup {
    use wgpu::util::DeviceExt as _;
    let flags = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("rounded_clip_flags"),
        contents: bytemuck::cast_slice(values),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("rounded_clip_bind_group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &flags,
                    offset: 0,
                    size: wgpu::BufferSize::new(16),
                }),
            },
        ],
    })
}

pub(crate) fn texture(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    width: u32,
    height: u32,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("rounded_clip_mask"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    })
}
