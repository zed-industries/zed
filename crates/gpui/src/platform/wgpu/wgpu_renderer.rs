use super::{WgpuAtlas, WgpuContext};
use crate::{
    Bounds, DevicePixels, GpuSpecs, Path, Point, PrimitiveBatch, ScaledPixels, Scene, Size,
    get_gamma_correction_ratios, Background,
};
use bytemuck::{Pod, Zeroable};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::num::NonZeroU64;
use std::sync::Arc;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GlobalParams {
    viewport_size: [f32; 2],
    premultiplied_alpha: u32,
    pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct PodBounds {
    origin: [f32; 2],
    size: [f32; 2],
}

impl From<Bounds<ScaledPixels>> for PodBounds {
    fn from(bounds: Bounds<ScaledPixels>) -> Self {
        Self {
            origin: [bounds.origin.x.0, bounds.origin.y.0],
            size: [bounds.size.width.0, bounds.size.height.0],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct SurfaceParams {
    bounds: PodBounds,
    content_mask: PodBounds,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GammaParams {
    gamma_ratios: [f32; 4],
    grayscale_enhanced_contrast: f32,
    subpixel_enhanced_contrast: f32,
    _pad: [f32; 2],
}

#[derive(Clone, Debug)]
#[repr(C)]
struct PathSprite {
    bounds: Bounds<ScaledPixels>,
}

#[derive(Clone, Debug)]
#[repr(C)]
struct PathRasterizationVertex {
    xy_position: Point<ScaledPixels>,
    st_position: Point<f32>,
    color: Background,
    bounds: Bounds<ScaledPixels>,
}

pub struct WgpuSurfaceConfig {
    pub size: Size<DevicePixels>,
    pub transparent: bool,
}

struct WgpuPipelines {
    quads: wgpu::RenderPipeline,
    shadows: wgpu::RenderPipeline,
    path_rasterization: wgpu::RenderPipeline,
    paths: wgpu::RenderPipeline,
    underlines: wgpu::RenderPipeline,
    mono_sprites: wgpu::RenderPipeline,
    subpixel_sprites: Option<wgpu::RenderPipeline>,
    poly_sprites: wgpu::RenderPipeline,
    #[allow(dead_code)]
    surfaces: wgpu::RenderPipeline,
}

struct WgpuBindGroupLayouts {
    globals: wgpu::BindGroupLayout,
    globals_with_gamma: wgpu::BindGroupLayout,
    quads: wgpu::BindGroupLayout,
    shadows: wgpu::BindGroupLayout,
    path_rasterization: wgpu::BindGroupLayout,
    paths: wgpu::BindGroupLayout,
    underlines: wgpu::BindGroupLayout,
    sprites: wgpu::BindGroupLayout,
    surfaces: wgpu::BindGroupLayout,
}

pub struct WgpuRenderer {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    pipelines: WgpuPipelines,
    bind_group_layouts: WgpuBindGroupLayouts,
    atlas: Arc<WgpuAtlas>,
    atlas_sampler: wgpu::Sampler,
    globals_buffer: wgpu::Buffer,
    gamma_buffer: wgpu::Buffer,
    path_intermediate_texture: wgpu::Texture,
    path_intermediate_view: wgpu::TextureView,
    path_msaa_texture: Option<wgpu::Texture>,
    path_msaa_view: Option<wgpu::TextureView>,
    rendering_params: RenderingParameters,
    dual_source_blending: bool,
}

impl WgpuRenderer {
    /// Creates a new WgpuRenderer from raw window handles.
    ///
    /// # Safety
    /// The caller must ensure that the window handle remains valid for the lifetime
    /// of the returned renderer.
    pub fn new<W: HasWindowHandle + HasDisplayHandle>(
        context: &WgpuContext,
        window: &W,
        config: WgpuSurfaceConfig,
    ) -> anyhow::Result<Self> {
        let window_handle = window.window_handle()
            .map_err(|e| anyhow::anyhow!("Failed to get window handle: {e}"))?;
        let display_handle = window.display_handle()
            .map_err(|e| anyhow::anyhow!("Failed to get display handle: {e}"))?;

        let target = wgpu::SurfaceTargetUnsafe::RawHandle {
            raw_display_handle: display_handle.as_raw(),
            raw_window_handle: window_handle.as_raw(),
        };

        // Safety: The caller guarantees that the window handle is valid for the
        // lifetime of this renderer. In practice, the RawWindow struct is created
        // from the native window handles and the surface is dropped before the window.
        let surface = unsafe {
            context.instance.create_surface_unsafe(target)
                .map_err(|e| anyhow::anyhow!("Failed to create surface: {e}"))?
        };

        let surface_caps = surface.get_capabilities(&context.adapter);
        // Prefer non-sRGB format to avoid sRGB blending issues
        // The shader outputs linear values which will be displayed directly
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| !f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let alpha_mode = if config.transparent {
            surface_caps
                .alpha_modes
                .iter()
                .find(|m| **m == wgpu::CompositeAlphaMode::PreMultiplied)
                .copied()
                .unwrap_or(surface_caps.alpha_modes[0])
        } else {
            wgpu::CompositeAlphaMode::Opaque
        };

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: config.size.width.0 as u32,
            height: config.size.height.0 as u32,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode,
            view_formats: vec![],
        };
        surface.configure(&context.device, &surface_config);

        let device = Arc::clone(&context.device);
        let queue = Arc::clone(&context.queue);
        let dual_source_blending = context.supports_dual_source_blending();

        let rendering_params = RenderingParameters::from_env(&context.adapter);
        let bind_group_layouts = Self::create_bind_group_layouts(&device);
        let pipelines = Self::create_pipelines(
            &device,
            &bind_group_layouts,
            surface_format,
            alpha_mode,
            rendering_params.path_sample_count,
            dual_source_blending,
        );

        let atlas = Arc::new(WgpuAtlas::new(Arc::clone(&device), Arc::clone(&queue)));
        let atlas_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("atlas_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let globals_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("globals_buffer"),
            size: std::mem::size_of::<GlobalParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let gamma_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("gamma_buffer"),
            size: std::mem::size_of::<GammaParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let (path_intermediate_texture, path_intermediate_view) = Self::create_path_intermediate(
            &device,
            surface_format,
            config.size.width.0 as u32,
            config.size.height.0 as u32,
        );

        let (path_msaa_texture, path_msaa_view) = Self::create_msaa_if_needed(
            &device,
            surface_format,
            config.size.width.0 as u32,
            config.size.height.0 as u32,
            rendering_params.path_sample_count,
        )
        .map(|(t, v)| (Some(t), Some(v)))
        .unwrap_or((None, None));

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            pipelines,
            bind_group_layouts,
            atlas,
            atlas_sampler,
            globals_buffer,
            gamma_buffer,
            path_intermediate_texture,
            path_intermediate_view,
            path_msaa_texture,
            path_msaa_view,
            rendering_params,
            dual_source_blending,
        })
    }

    fn create_bind_group_layouts(device: &wgpu::Device) -> WgpuBindGroupLayouts {
        let globals = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("globals_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(std::mem::size_of::<GlobalParams>() as u64),
                },
                count: None,
            }],
        });

        let globals_with_gamma = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("globals_with_gamma_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(std::mem::size_of::<GlobalParams>() as u64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(std::mem::size_of::<GammaParams>() as u64),
                    },
                    count: None,
                },
            ],
        });

        let storage_buffer_entry = |binding: u32| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        let quads = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("quads_layout"),
            entries: &[storage_buffer_entry(0)],
        });

        let shadows = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shadows_layout"),
            entries: &[storage_buffer_entry(0)],
        });

        let path_rasterization = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("path_rasterization_layout"),
            entries: &[storage_buffer_entry(0)],
        });

        let paths = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("paths_layout"),
            entries: &[
                storage_buffer_entry(0),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let underlines = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("underlines_layout"),
            entries: &[storage_buffer_entry(0)],
        });

        let sprites = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sprites_layout"),
            entries: &[
                storage_buffer_entry(0),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let surfaces = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("surfaces_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(std::mem::size_of::<SurfaceParams>() as u64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        WgpuBindGroupLayouts {
            globals,
            globals_with_gamma,
            quads,
            shadows,
            path_rasterization,
            paths,
            underlines,
            sprites,
            surfaces,
        }
    }

    fn create_pipelines(
        device: &wgpu::Device,
        layouts: &WgpuBindGroupLayouts,
        surface_format: wgpu::TextureFormat,
        alpha_mode: wgpu::CompositeAlphaMode,
        path_sample_count: u32,
        dual_source_blending: bool,
    ) -> WgpuPipelines {
        let shader_source = include_str!("shaders.wgsl");
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gpui_shaders"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let blend_mode = match alpha_mode {
            wgpu::CompositeAlphaMode::PreMultiplied => wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
            _ => wgpu::BlendState::ALPHA_BLENDING,
        };

        let color_target = wgpu::ColorTargetState {
            format: surface_format,
            blend: Some(blend_mode),
            write_mask: wgpu::ColorWrites::ALL,
        };

        let create_pipeline = |name: &str,
                               vs_entry: &str,
                               fs_entry: &str,
                               globals_layout: &wgpu::BindGroupLayout,
                               data_layout: &wgpu::BindGroupLayout,
                               topology: wgpu::PrimitiveTopology,
                               color_targets: &[Option<wgpu::ColorTargetState>],
                               sample_count: u32| {
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&format!("{name}_layout")),
                bind_group_layouts: &[globals_layout, data_layout],
                immediate_size: 0,
            });

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(name),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: Some(vs_entry),
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: Some(fs_entry),
                    targets: color_targets,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: sample_count,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview_mask: None,
                cache: None,
            })
        };

        let quads = create_pipeline(
            "quads",
            "vs_quad",
            "fs_quad",
            &layouts.globals,
            &layouts.quads,
            wgpu::PrimitiveTopology::TriangleStrip,
            &[Some(color_target.clone())],
            1,
        );

        let shadows = create_pipeline(
            "shadows",
            "vs_shadow",
            "fs_shadow",
            &layouts.globals,
            &layouts.shadows,
            wgpu::PrimitiveTopology::TriangleStrip,
            &[Some(color_target.clone())],
            1,
        );

        let path_rasterization = create_pipeline(
            "path_rasterization",
            "vs_path_rasterization",
            "fs_path_rasterization",
            &layouts.globals,
            &layouts.path_rasterization,
            wgpu::PrimitiveTopology::TriangleList,
            &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            path_sample_count,
        );

        let paths_blend = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        };

        let paths = create_pipeline(
            "paths",
            "vs_path",
            "fs_path",
            &layouts.globals,
            &layouts.paths,
            wgpu::PrimitiveTopology::TriangleStrip,
            &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(paths_blend),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            1,
        );

        let underlines = create_pipeline(
            "underlines",
            "vs_underline",
            "fs_underline",
            &layouts.globals,
            &layouts.underlines,
            wgpu::PrimitiveTopology::TriangleStrip,
            &[Some(color_target.clone())],
            1,
        );

        let mono_sprites = create_pipeline(
            "mono_sprites",
            "vs_mono_sprite",
            "fs_mono_sprite",
            &layouts.globals_with_gamma,
            &layouts.sprites,
            wgpu::PrimitiveTopology::TriangleStrip,
            &[Some(color_target.clone())],
            1,
        );

        let subpixel_sprites = if dual_source_blending {
            let subpixel_blend = wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::Src1,
                    dst_factor: wgpu::BlendFactor::OneMinusSrc1,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
            };

            Some(create_pipeline(
                "subpixel_sprites",
                "vs_subpixel_sprite",
                "fs_subpixel_sprite",
                &layouts.globals_with_gamma,
                &layouts.sprites,
                wgpu::PrimitiveTopology::TriangleStrip,
                &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(subpixel_blend),
                    write_mask: wgpu::ColorWrites::COLOR,
                })],
                1,
            ))
        } else {
            None
        };

        let poly_sprites = create_pipeline(
            "poly_sprites",
            "vs_poly_sprite",
            "fs_poly_sprite",
            &layouts.globals,
            &layouts.sprites,
            wgpu::PrimitiveTopology::TriangleStrip,
            &[Some(color_target.clone())],
            1,
        );

        let surfaces = create_pipeline(
            "surfaces",
            "vs_surface",
            "fs_surface",
            &layouts.globals,
            &layouts.surfaces,
            wgpu::PrimitiveTopology::TriangleStrip,
            &[Some(color_target)],
            1,
        );

        WgpuPipelines {
            quads,
            shadows,
            path_rasterization,
            paths,
            underlines,
            mono_sprites,
            subpixel_sprites,
            poly_sprites,
            surfaces,
        }
    }

    fn create_path_intermediate(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("path_intermediate"),
            size: wgpu::Extent3d {
                width: width.max(1),
                height: height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        (texture, view)
    }

    fn create_msaa_if_needed(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
        sample_count: u32,
    ) -> Option<(wgpu::Texture, wgpu::TextureView)> {
        if sample_count <= 1 {
            return None;
        }
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("path_msaa"),
            size: wgpu::Extent3d {
                width: width.max(1),
                height: height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Some((texture, view))
    }

    pub fn update_drawable_size(&mut self, size: Size<DevicePixels>) {
        let width = size.width.0 as u32;
        let height = size.height.0 as u32;

        if width != self.surface_config.width || height != self.surface_config.height {
            self.surface_config.width = width.max(1);
            self.surface_config.height = height.max(1);
            self.surface.configure(&self.device, &self.surface_config);

            let (path_intermediate_texture, path_intermediate_view) = Self::create_path_intermediate(
                &self.device,
                self.surface_config.format,
                self.surface_config.width,
                self.surface_config.height,
            );
            self.path_intermediate_texture = path_intermediate_texture;
            self.path_intermediate_view = path_intermediate_view;

            let (path_msaa_texture, path_msaa_view) = Self::create_msaa_if_needed(
                &self.device,
                self.surface_config.format,
                self.surface_config.width,
                self.surface_config.height,
                self.rendering_params.path_sample_count,
            )
            .map(|(t, v)| (Some(t), Some(v)))
            .unwrap_or((None, None));
            self.path_msaa_texture = path_msaa_texture;
            self.path_msaa_view = path_msaa_view;
        }
    }

    pub fn update_transparency(&mut self, transparent: bool) {
        let new_alpha_mode = if transparent {
            wgpu::CompositeAlphaMode::PreMultiplied
        } else {
            wgpu::CompositeAlphaMode::Opaque
        };

        if new_alpha_mode != self.surface_config.alpha_mode {
            self.surface_config.alpha_mode = new_alpha_mode;
            self.surface.configure(&self.device, &self.surface_config);
            self.pipelines = Self::create_pipelines(
                &self.device,
                &self.bind_group_layouts,
                self.surface_config.format,
                self.surface_config.alpha_mode,
                self.rendering_params.path_sample_count,
                self.dual_source_blending,
            );
        }
    }

    #[allow(dead_code)]
    pub fn viewport_size(&self) -> Size<DevicePixels> {
        Size {
            width: DevicePixels(self.surface_config.width as i32),
            height: DevicePixels(self.surface_config.height as i32),
        }
    }

    pub fn sprite_atlas(&self) -> &Arc<WgpuAtlas> {
        &self.atlas
    }

    pub fn gpu_specs(&self) -> GpuSpecs {
        GpuSpecs {
            is_software_emulated: false,
            device_name: "wgpu".to_string(),
            driver_name: "wgpu".to_string(),
            driver_info: "wgpu renderer".to_string(),
        }
    }

    fn create_storage_buffer(&self, data: &[u8]) -> wgpu::Buffer {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("instance_buffer"),
            size: (data.len() as u64).max(16),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&buffer, 0, data);
        buffer
    }

    pub fn draw(&mut self, scene: &Scene) {
        self.atlas.before_frame();

        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.surface_config);
                return;
            }
            Err(e) => {
                log::error!("Failed to acquire surface texture: {e}");
                return;
            }
        };
        let frame_view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let globals = GlobalParams {
            viewport_size: [
                self.surface_config.width as f32,
                self.surface_config.height as f32,
            ],
            premultiplied_alpha: if self.surface_config.alpha_mode == wgpu::CompositeAlphaMode::PreMultiplied {
                1
            } else {
                0
            },
            pad: 0,
        };
        self.queue.write_buffer(&self.globals_buffer, 0, bytemuck::bytes_of(&globals));

        let gamma_params = GammaParams {
            gamma_ratios: self.rendering_params.gamma_ratios,
            grayscale_enhanced_contrast: self.rendering_params.grayscale_enhanced_contrast,
            subpixel_enhanced_contrast: self.rendering_params.subpixel_enhanced_contrast,
            _pad: [0.0; 2],
        };
        self.queue.write_buffer(&self.gamma_buffer, 0, bytemuck::bytes_of(&gamma_params));

        let globals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("globals_bind_group"),
            layout: &self.bind_group_layouts.globals,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.globals_buffer.as_entire_binding(),
            }],
        });

        let globals_with_gamma_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("globals_with_gamma_bind_group"),
            layout: &self.bind_group_layouts.globals_with_gamma,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.globals_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.gamma_buffer.as_entire_binding(),
                },
            ],
        });

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("main_encoder"),
        });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            for batch in scene.batches() {
                match batch {
                    PrimitiveBatch::Quads(quads) => {
                        let data = unsafe {
                            std::slice::from_raw_parts(
                                quads.as_ptr() as *const u8,
                                std::mem::size_of_val(quads),
                            )
                        };
                        let buffer = self.create_storage_buffer(data);
                        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: Some("quads_bind_group"),
                            layout: &self.bind_group_layouts.quads,
                            entries: &[wgpu::BindGroupEntry {
                                binding: 0,
                                resource: buffer.as_entire_binding(),
                            }],
                        });
                        pass.set_pipeline(&self.pipelines.quads);
                        pass.set_bind_group(0, &globals_bind_group, &[]);
                        pass.set_bind_group(1, &bind_group, &[]);
                        pass.draw(0..4, 0..quads.len() as u32);
                    }
                    PrimitiveBatch::Shadows(shadows) => {
                        let data = unsafe {
                            std::slice::from_raw_parts(
                                shadows.as_ptr() as *const u8,
                                std::mem::size_of_val(shadows),
                            )
                        };
                        let buffer = self.create_storage_buffer(data);
                        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: Some("shadows_bind_group"),
                            layout: &self.bind_group_layouts.shadows,
                            entries: &[wgpu::BindGroupEntry {
                                binding: 0,
                                resource: buffer.as_entire_binding(),
                            }],
                        });
                        pass.set_pipeline(&self.pipelines.shadows);
                        pass.set_bind_group(0, &globals_bind_group, &[]);
                        pass.set_bind_group(1, &bind_group, &[]);
                        pass.draw(0..4, 0..shadows.len() as u32);
                    }
                    PrimitiveBatch::Paths(paths) => {
                        if paths.is_empty() {
                            continue;
                        }

                        drop(pass);

                        self.draw_paths_to_intermediate(&mut encoder, paths);

                        pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("main_pass_continued"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &frame_view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: wgpu::StoreOp::Store,
                                },
                                depth_slice: None,
                            })],
                            depth_stencil_attachment: None,
                            ..Default::default()
                        });

                        let first_path = &paths[0];
                        let sprites: Vec<PathSprite> = if paths.last().map(|p| &p.order) == Some(&first_path.order) {
                            paths.iter().map(|p| PathSprite { bounds: p.clipped_bounds() }).collect()
                        } else {
                            let mut bounds = first_path.clipped_bounds();
                            for path in paths.iter().skip(1) {
                                bounds = bounds.union(&path.clipped_bounds());
                            }
                            vec![PathSprite { bounds }]
                        };

                        let sprite_data = unsafe {
                            std::slice::from_raw_parts(
                                sprites.as_ptr() as *const u8,
                                std::mem::size_of_val(sprites.as_slice()),
                            )
                        };
                        let sprite_buffer = self.create_storage_buffer(sprite_data);

                        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: Some("paths_bind_group"),
                            layout: &self.bind_group_layouts.paths,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: sprite_buffer.as_entire_binding(),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::TextureView(&self.path_intermediate_view),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 2,
                                    resource: wgpu::BindingResource::Sampler(&self.atlas_sampler),
                                },
                            ],
                        });

                        pass.set_pipeline(&self.pipelines.paths);
                        pass.set_bind_group(0, &globals_bind_group, &[]);
                        pass.set_bind_group(1, &bind_group, &[]);
                        pass.draw(0..4, 0..sprites.len() as u32);
                    }
                    PrimitiveBatch::Underlines(underlines) => {
                        let data = unsafe {
                            std::slice::from_raw_parts(
                                underlines.as_ptr() as *const u8,
                                std::mem::size_of_val(underlines),
                            )
                        };
                        let buffer = self.create_storage_buffer(data);
                        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: Some("underlines_bind_group"),
                            layout: &self.bind_group_layouts.underlines,
                            entries: &[wgpu::BindGroupEntry {
                                binding: 0,
                                resource: buffer.as_entire_binding(),
                            }],
                        });
                        pass.set_pipeline(&self.pipelines.underlines);
                        pass.set_bind_group(0, &globals_bind_group, &[]);
                        pass.set_bind_group(1, &bind_group, &[]);
                        pass.draw(0..4, 0..underlines.len() as u32);
                    }
                    PrimitiveBatch::MonochromeSprites { texture_id, sprites } => {
                        let tex_info = self.atlas.get_texture_info(texture_id);
                        let data = unsafe {
                            std::slice::from_raw_parts(
                                sprites.as_ptr() as *const u8,
                                std::mem::size_of_val(sprites),
                            )
                        };
                        let buffer = self.create_storage_buffer(data);
                        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: Some("mono_sprites_bind_group"),
                            layout: &self.bind_group_layouts.sprites,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: buffer.as_entire_binding(),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::TextureView(&tex_info.view),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 2,
                                    resource: wgpu::BindingResource::Sampler(&self.atlas_sampler),
                                },
                            ],
                        });
                        pass.set_pipeline(&self.pipelines.mono_sprites);
                        pass.set_bind_group(0, &globals_with_gamma_bind_group, &[]);
                        pass.set_bind_group(1, &bind_group, &[]);
                        pass.draw(0..4, 0..sprites.len() as u32);
                    }
                    PrimitiveBatch::SubpixelSprites { texture_id, sprites } => {
                        let tex_info = self.atlas.get_texture_info(texture_id);
                        let data = unsafe {
                            std::slice::from_raw_parts(
                                sprites.as_ptr() as *const u8,
                                std::mem::size_of_val(sprites),
                            )
                        };
                        let buffer = self.create_storage_buffer(data);
                        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: Some("subpixel_sprites_bind_group"),
                            layout: &self.bind_group_layouts.sprites,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: buffer.as_entire_binding(),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::TextureView(&tex_info.view),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 2,
                                    resource: wgpu::BindingResource::Sampler(&self.atlas_sampler),
                                },
                            ],
                        });

                        if let Some(ref pipeline) = self.pipelines.subpixel_sprites {
                            let subpixel_gamma = GammaParams {
                                gamma_ratios: self.rendering_params.gamma_ratios,
                                grayscale_enhanced_contrast: self.rendering_params.grayscale_enhanced_contrast,
                                subpixel_enhanced_contrast: self.rendering_params.subpixel_enhanced_contrast,
                                _pad: [0.0; 2],
                            };
                            self.queue.write_buffer(&self.gamma_buffer, 0, bytemuck::bytes_of(&subpixel_gamma));

                            pass.set_pipeline(pipeline);
                            pass.set_bind_group(0, &globals_with_gamma_bind_group, &[]);
                            pass.set_bind_group(1, &bind_group, &[]);
                            pass.draw(0..4, 0..sprites.len() as u32);

                            self.queue.write_buffer(&self.gamma_buffer, 0, bytemuck::bytes_of(&gamma_params));
                        } else {
                            pass.set_pipeline(&self.pipelines.mono_sprites);
                            pass.set_bind_group(0, &globals_with_gamma_bind_group, &[]);
                            pass.set_bind_group(1, &bind_group, &[]);
                            pass.draw(0..4, 0..sprites.len() as u32);
                        }
                    }
                    PrimitiveBatch::PolychromeSprites { texture_id, sprites } => {
                        let tex_info = self.atlas.get_texture_info(texture_id);
                        let data = unsafe {
                            std::slice::from_raw_parts(
                                sprites.as_ptr() as *const u8,
                                std::mem::size_of_val(sprites),
                            )
                        };
                        let buffer = self.create_storage_buffer(data);
                        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: Some("poly_sprites_bind_group"),
                            layout: &self.bind_group_layouts.sprites,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: buffer.as_entire_binding(),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::TextureView(&tex_info.view),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 2,
                                    resource: wgpu::BindingResource::Sampler(&self.atlas_sampler),
                                },
                            ],
                        });
                        pass.set_pipeline(&self.pipelines.poly_sprites);
                        pass.set_bind_group(0, &globals_bind_group, &[]);
                        pass.set_bind_group(1, &bind_group, &[]);
                        pass.draw(0..4, 0..sprites.len() as u32);
                    }
                    PrimitiveBatch::Surfaces(_surfaces) => {
                        // Surfaces are macOS-only for video playback
                        // Not implemented for Linux/wgpu
                    }
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }

    fn draw_paths_to_intermediate(&self, encoder: &mut wgpu::CommandEncoder, paths: &[Path<ScaledPixels>]) {
        let mut vertices = Vec::new();
        for path in paths {
            vertices.extend(path.vertices.iter().map(|v| PathRasterizationVertex {
                xy_position: v.xy_position,
                st_position: v.st_position,
                color: path.color,
                bounds: path.clipped_bounds(),
            }));
        }

        if vertices.is_empty() {
            return;
        }

        let vertex_data = unsafe {
            std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                std::mem::size_of_val(vertices.as_slice()),
            )
        };
        let vertex_buffer = self.create_storage_buffer(vertex_data);

        let globals = GlobalParams {
            viewport_size: [
                self.surface_config.width as f32,
                self.surface_config.height as f32,
            ],
            premultiplied_alpha: 0,
            pad: 0,
        };
        let globals_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("path_globals"),
            size: std::mem::size_of::<GlobalParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&globals_buffer, 0, bytemuck::bytes_of(&globals));

        let globals_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("path_globals_bind_group"),
            layout: &self.bind_group_layouts.globals,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: globals_buffer.as_entire_binding(),
            }],
        });

        let data_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("path_rasterization_bind_group"),
            layout: &self.bind_group_layouts.path_rasterization,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: vertex_buffer.as_entire_binding(),
            }],
        });

        let (target_view, resolve_target) = if let Some(ref msaa_view) = self.path_msaa_view {
            (msaa_view, Some(&self.path_intermediate_view))
        } else {
            (&self.path_intermediate_view, None)
        };

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("path_rasterization_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target_view,
                    resolve_target,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            pass.set_pipeline(&self.pipelines.path_rasterization);
            pass.set_bind_group(0, &globals_bind_group, &[]);
            pass.set_bind_group(1, &data_bind_group, &[]);
            pass.draw(0..vertices.len() as u32, 0..1);
        }
    }

    pub fn destroy(&mut self) {
        // wgpu resources are automatically cleaned up when dropped
    }
}

struct RenderingParameters {
    path_sample_count: u32,
    gamma_ratios: [f32; 4],
    grayscale_enhanced_contrast: f32,
    subpixel_enhanced_contrast: f32,
}

impl RenderingParameters {
    fn from_env(adapter: &wgpu::Adapter) -> Self {
        use std::env;

        let sample_count_mask = adapter.get_texture_format_features(wgpu::TextureFormat::Bgra8Unorm)
            .flags
            .sample_count_supported(4) as u32 * 4
            | adapter.get_texture_format_features(wgpu::TextureFormat::Bgra8Unorm)
                .flags
                .sample_count_supported(2) as u32 * 2
            | 1;

        let path_sample_count = env::var("ZED_PATH_SAMPLE_COUNT")
            .ok()
            .and_then(|v| v.parse().ok())
            .or_else(|| {
                [4, 2, 1]
                    .into_iter()
                    .find(|&n| (sample_count_mask & n) != 0)
            })
            .unwrap_or(1);

        let gamma = env::var("ZED_FONTS_GAMMA")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1.8_f32)
            .clamp(1.0, 2.2);
        let gamma_ratios = get_gamma_correction_ratios(gamma);

        let grayscale_enhanced_contrast = env::var("ZED_FONTS_GRAYSCALE_ENHANCED_CONTRAST")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1.0_f32)
            .max(0.0);

        let subpixel_enhanced_contrast = env::var("ZED_FONTS_SUBPIXEL_ENHANCED_CONTRAST")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.5_f32)
            .max(0.0);

        Self {
            path_sample_count,
            gamma_ratios,
            grayscale_enhanced_contrast,
            subpixel_enhanced_contrast,
        }
    }
}
