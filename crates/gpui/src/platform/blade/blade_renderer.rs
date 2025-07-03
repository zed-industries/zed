// Doing `if let` gives you nice scoping with passes/encoders
#![allow(irrefutable_let_patterns)]

use super::{BladeAtlas, BladeContext};
use crate::{
    Background, Bounds, ContentMask, DevicePixels, GpuSpecs, MonochromeSprite, PathVertex,
    PolychromeSprite, PrimitiveBatch, Quad, ScaledPixels, Scene, Shadow, Size, Underline,
};
use blade_graphics::{self as gpu};
use blade_util::{BufferBelt, BufferBeltDescriptor};
use bytemuck::{Pod, Zeroable};
#[cfg(target_os = "macos")]
use media::core_video::CVMetalTextureCache;
use std::{mem, sync::Arc};

const MAX_FRAME_TIME_MS: u32 = 10000;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GlobalParams {
    viewport_size: [f32; 2],
    premultiplied_alpha: u32,
    pad: u32,
}

//Note: we can't use `Bounds` directly here because
// it doesn't implement Pod + Zeroable
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

#[derive(blade_macros::ShaderData)]
struct ShaderQuadsData {
    globals: GlobalParams,
    b_quads: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderShadowsData {
    globals: GlobalParams,
    b_shadows: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderPathsData {
    globals: GlobalParams,
    b_path_vertices: gpu::BufferPiece,
    b_path_sprites: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderUnderlinesData {
    globals: GlobalParams,
    b_underlines: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderMonoSpritesData {
    globals: GlobalParams,
    t_sprite: gpu::TextureView,
    s_sprite: gpu::Sampler,
    b_mono_sprites: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderPolySpritesData {
    globals: GlobalParams,
    t_sprite: gpu::TextureView,
    s_sprite: gpu::Sampler,
    b_poly_sprites: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderSurfacesData {
    globals: GlobalParams,
    surface_locals: SurfaceParams,
    t_y: gpu::TextureView,
    t_cb_cr: gpu::TextureView,
    s_surface: gpu::Sampler,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
struct PathSprite {
    bounds: Bounds<ScaledPixels>,
    color: Background,
}

/// Argument buffer layout for `draw_indirect` commands.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Pod, Zeroable)]
pub struct DrawIndirectArgs {
    /// The number of vertices to draw.
    pub vertex_count: u32,
    /// The number of instances to draw.
    pub instance_count: u32,
    /// The Index of the first vertex to draw.
    pub first_vertex: u32,
    /// The instance ID of the first instance to draw.
    ///
    /// Has to be 0, unless [`Features::INDIRECT_FIRST_INSTANCE`](crate::Features::INDIRECT_FIRST_INSTANCE) is enabled.
    pub first_instance: u32,
}

struct BladePipelines {
    quads: gpu::RenderPipeline,
    shadows: gpu::RenderPipeline,
    paths: gpu::RenderPipeline,
    underlines: gpu::RenderPipeline,
    mono_sprites: gpu::RenderPipeline,
    poly_sprites: gpu::RenderPipeline,
    surfaces: gpu::RenderPipeline,
}

impl BladePipelines {
    fn new(gpu: &gpu::Context, surface_info: gpu::SurfaceInfo, sample_count: u32) -> Self {
        use gpu::ShaderData as _;

        log::info!(
            "Initializing Blade pipelines for surface {:?}",
            surface_info
        );
        let shader = gpu.create_shader(gpu::ShaderDesc {
            source: include_str!("shaders.wgsl"),
        });
        shader.check_struct_size::<GlobalParams>();
        shader.check_struct_size::<SurfaceParams>();
        shader.check_struct_size::<Quad>();
        shader.check_struct_size::<Shadow>();
        assert_eq!(
            mem::size_of::<PathVertex<ScaledPixels>>(),
            shader.get_struct_size("PathVertex") as usize,
        );
        shader.check_struct_size::<PathSprite>();
        shader.check_struct_size::<Underline>();
        shader.check_struct_size::<MonochromeSprite>();
        shader.check_struct_size::<PolychromeSprite>();

        // See https://apoorvaj.io/alpha-compositing-opengl-blending-and-premultiplied-alpha/
        let blend_mode = match surface_info.alpha {
            gpu::AlphaMode::Ignored => gpu::BlendState::ALPHA_BLENDING,
            gpu::AlphaMode::PreMultiplied => gpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
            gpu::AlphaMode::PostMultiplied => gpu::BlendState::ALPHA_BLENDING,
        };
        let color_targets = &[gpu::ColorTargetState {
            format: surface_info.format,
            blend: Some(blend_mode),
            write_mask: gpu::ColorWrites::default(),
        }];

        Self {
            quads: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "quads",
                data_layouts: &[&ShaderQuadsData::layout()],
                vertex: shader.at("vs_quad"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_quad")),
                color_targets,
                multisample_state: gpu::MultisampleState {
                    sample_count,
                    ..Default::default()
                },
            }),
            shadows: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "shadows",
                data_layouts: &[&ShaderShadowsData::layout()],
                vertex: shader.at("vs_shadow"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_shadow")),
                color_targets,
                multisample_state: gpu::MultisampleState {
                    sample_count,
                    ..Default::default()
                },
            }),
            paths: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "paths",
                data_layouts: &[&ShaderPathsData::layout()],
                vertex: shader.at("vs_path"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_path")),
                color_targets,
                multisample_state: gpu::MultisampleState {
                    sample_count,
                    ..Default::default()
                },
            }),
            underlines: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "underlines",
                data_layouts: &[&ShaderUnderlinesData::layout()],
                vertex: shader.at("vs_underline"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_underline")),
                color_targets,
                multisample_state: gpu::MultisampleState {
                    sample_count,
                    ..Default::default()
                },
            }),
            mono_sprites: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "mono-sprites",
                data_layouts: &[&ShaderMonoSpritesData::layout()],
                vertex: shader.at("vs_mono_sprite"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_mono_sprite")),
                color_targets,
                multisample_state: gpu::MultisampleState {
                    sample_count,
                    ..Default::default()
                },
            }),
            poly_sprites: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "poly-sprites",
                data_layouts: &[&ShaderPolySpritesData::layout()],
                vertex: shader.at("vs_poly_sprite"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_poly_sprite")),
                color_targets,
                multisample_state: gpu::MultisampleState {
                    sample_count,
                    ..Default::default()
                },
            }),
            surfaces: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "surfaces",
                data_layouts: &[&ShaderSurfacesData::layout()],
                vertex: shader.at("vs_surface"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_surface")),
                color_targets,
                multisample_state: gpu::MultisampleState {
                    sample_count,
                    ..Default::default()
                },
            }),
        }
    }

    fn destroy(&mut self, gpu: &gpu::Context) {
        gpu.destroy_render_pipeline(&mut self.quads);
        gpu.destroy_render_pipeline(&mut self.shadows);
        gpu.destroy_render_pipeline(&mut self.paths);
        gpu.destroy_render_pipeline(&mut self.underlines);
        gpu.destroy_render_pipeline(&mut self.mono_sprites);
        gpu.destroy_render_pipeline(&mut self.poly_sprites);
        gpu.destroy_render_pipeline(&mut self.surfaces);
    }
}

pub struct BladeSurfaceConfig {
    pub size: gpu::Extent,
    pub transparent: bool,
}

//Note: we could see some of these fields moved into `BladeContext`
// so that they are shared between windows. E.g. `pipelines`.
// But that is complicated by the fact that pipelines depend on
// the format and alpha mode.
pub struct BladeRenderer {
    gpu: Arc<gpu::Context>,
    surface: gpu::Surface,
    surface_config: gpu::SurfaceConfig,
    command_encoder: gpu::CommandEncoder,
    last_sync_point: Option<gpu::SyncPoint>,
    pipelines: BladePipelines,
    instance_belt: BufferBelt,
    atlas: Arc<BladeAtlas>,
    atlas_sampler: gpu::Sampler,
    #[cfg(target_os = "macos")]
    core_video_texture_cache: CVMetalTextureCache,
    sample_count: u32,
    texture_msaa: Option<gpu::Texture>,
    texture_view_msaa: Option<gpu::TextureView>,
}

impl BladeRenderer {
    pub fn new<I: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
        context: &BladeContext,
        window: &I,
        config: BladeSurfaceConfig,
    ) -> anyhow::Result<Self> {
        // workaround for https://github.com/zed-industries/zed/issues/26143
        let sample_count = std::env::var("ZED_SAMPLE_COUNT")
            .ok()
            .or_else(|| std::env::var("ZED_PATH_SAMPLE_COUNT").ok())
            .and_then(|v| v.parse().ok())
            .or_else(|| {
                [4, 2, 1]
                    .into_iter()
                    .find(|count| context.gpu.supports_texture_sample_count(*count))
            })
            .unwrap_or(1);

        let surface_config = gpu::SurfaceConfig {
            size: config.size,
            usage: gpu::TextureUsage::TARGET,
            display_sync: gpu::DisplaySync::Recent,
            color_space: gpu::ColorSpace::Linear,
            allow_exclusive_full_screen: false,
            transparent: config.transparent,
        };
        let surface = context
            .gpu
            .create_surface_configured(window, surface_config)
            .map_err(|err| anyhow::anyhow!("Failed to create surface: {err:?}"))?;

        let (texture_msaa, texture_view_msaa) = create_msaa_texture_if_needed(
            &context.gpu,
            surface.info().format,
            config.size.width,
            config.size.height,
            sample_count,
        )
        .unzip();

        let command_encoder = context.gpu.create_command_encoder(gpu::CommandEncoderDesc {
            name: "main",
            buffer_count: 2,
        });

        let pipelines = BladePipelines::new(&context.gpu, surface.info(), sample_count);
        let instance_belt = BufferBelt::new(BufferBeltDescriptor {
            memory: gpu::Memory::Shared,
            min_chunk_size: 0x1000,
            alignment: 0x40, // Vulkan `minStorageBufferOffsetAlignment` on Intel Xe
        });
        let atlas = Arc::new(BladeAtlas::new(&context.gpu));
        let atlas_sampler = context.gpu.create_sampler(gpu::SamplerDesc {
            name: "atlas",
            mag_filter: gpu::FilterMode::Linear,
            min_filter: gpu::FilterMode::Linear,
            ..Default::default()
        });

        #[cfg(target_os = "macos")]
        let core_video_texture_cache = unsafe {
            CVMetalTextureCache::new(
                objc2::rc::Retained::as_ptr(&context.gpu.metal_device()) as *mut _
            )
            .unwrap()
        };

        Ok(Self {
            gpu: Arc::clone(&context.gpu),
            surface,
            surface_config,
            command_encoder,
            last_sync_point: None,
            pipelines,
            instance_belt,
            atlas,
            atlas_sampler,
            #[cfg(target_os = "macos")]
            core_video_texture_cache,
            sample_count,
            texture_msaa,
            texture_view_msaa,
        })
    }

    fn wait_for_gpu(&mut self) {
        if let Some(last_sp) = self.last_sync_point.take() {
            if !self.gpu.wait_for(&last_sp, MAX_FRAME_TIME_MS) {
                log::error!("GPU hung");
                #[cfg(target_os = "linux")]
                if self.gpu.device_information().driver_name == "radv" {
                    log::error!(
                        "there's a known bug with amdgpu/radv, try setting ZED_PATH_SAMPLE_COUNT=0 as a workaround"
                    );
                    log::error!(
                        "if that helps you're running into https://github.com/zed-industries/zed/issues/26143"
                    );
                }
                log::error!(
                    "your device information is: {:?}",
                    self.gpu.device_information()
                );
                while !self.gpu.wait_for(&last_sp, MAX_FRAME_TIME_MS) {}
            }
        }
    }

    pub fn update_drawable_size(&mut self, size: Size<DevicePixels>) {
        self.update_drawable_size_impl(size, false);
    }

    /// Like `update_drawable_size` but skips the check that the size has changed. This is useful in
    /// cases like restoring a window from minimization where the size is the same but the
    /// renderer's swap chain needs to be recreated.
    #[cfg_attr(
        any(target_os = "macos", target_os = "linux", target_os = "freebsd"),
        allow(dead_code)
    )]
    pub fn update_drawable_size_even_if_unchanged(&mut self, size: Size<DevicePixels>) {
        self.update_drawable_size_impl(size, true);
    }

    fn update_drawable_size_impl(&mut self, size: Size<DevicePixels>, always_resize: bool) {
        let gpu_size = gpu::Extent {
            width: size.width.0 as u32,
            height: size.height.0 as u32,
            depth: 1,
        };

        if always_resize || gpu_size != self.surface_config.size {
            self.wait_for_gpu();
            self.surface_config.size = gpu_size;
            self.gpu
                .reconfigure_surface(&mut self.surface, self.surface_config);

            if let Some(texture_msaa) = self.texture_msaa {
                self.gpu.destroy_texture(texture_msaa);
            }
            if let Some(texture_view_msaa) = self.texture_view_msaa {
                self.gpu.destroy_texture_view(texture_view_msaa);
            }

            let (texture_msaa, texture_view_msaa) = create_msaa_texture_if_needed(
                &self.gpu,
                self.surface.info().format,
                gpu_size.width,
                gpu_size.height,
                self.sample_count,
            )
            .unzip();
            self.texture_msaa = texture_msaa;
            self.texture_view_msaa = texture_view_msaa;
        }
    }

    pub fn update_transparency(&mut self, transparent: bool) {
        if transparent != self.surface_config.transparent {
            self.wait_for_gpu();
            self.surface_config.transparent = transparent;
            self.gpu
                .reconfigure_surface(&mut self.surface, self.surface_config);
            self.pipelines.destroy(&self.gpu);
            self.pipelines = BladePipelines::new(&self.gpu, self.surface.info(), self.sample_count);
        }
    }

    #[cfg_attr(
        any(target_os = "macos", feature = "wayland", target_os = "windows"),
        allow(dead_code)
    )]
    pub fn viewport_size(&self) -> gpu::Extent {
        self.surface_config.size
    }

    pub fn sprite_atlas(&self) -> &Arc<BladeAtlas> {
        &self.atlas
    }

    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub fn gpu_specs(&self) -> GpuSpecs {
        let info = self.gpu.device_information();

        GpuSpecs {
            is_software_emulated: info.is_software_emulated,
            device_name: info.device_name.clone(),
            driver_name: info.driver_name.clone(),
            driver_info: info.driver_info.clone(),
        }
    }

    #[cfg(target_os = "macos")]
    pub fn layer(&self) -> metal::MetalLayer {
        unsafe { foreign_types::ForeignType::from_ptr(self.layer_ptr()) }
    }

    #[cfg(target_os = "macos")]
    pub fn layer_ptr(&self) -> *mut metal::CAMetalLayer {
        objc2::rc::Retained::as_ptr(&self.surface.metal_layer()) as *mut _
    }

    pub fn destroy(&mut self) {
        self.wait_for_gpu();
        self.atlas.destroy();
        self.gpu.destroy_sampler(self.atlas_sampler);
        self.instance_belt.destroy(&self.gpu);
        self.gpu.destroy_command_encoder(&mut self.command_encoder);
        self.pipelines.destroy(&self.gpu);
        self.gpu.destroy_surface(&mut self.surface);
        if let Some(texture_msaa) = self.texture_msaa {
            self.gpu.destroy_texture(texture_msaa);
        }
        if let Some(texture_view_msaa) = self.texture_view_msaa {
            self.gpu.destroy_texture_view(texture_view_msaa);
        }
    }

    pub fn draw(&mut self, scene: &Scene) {
        self.command_encoder.start();
        self.atlas.before_frame(&mut self.command_encoder);

        let frame = {
            profiling::scope!("acquire frame");
            self.surface.acquire_frame()
        };
        let frame_view = frame.texture_view();
        if let Some(texture_msaa) = self.texture_msaa {
            self.command_encoder.init_texture(texture_msaa);
        }
        self.command_encoder.init_texture(frame.texture());

        let globals = GlobalParams {
            viewport_size: [
                self.surface_config.size.width as f32,
                self.surface_config.size.height as f32,
            ],
            premultiplied_alpha: match self.surface.info().alpha {
                gpu::AlphaMode::Ignored | gpu::AlphaMode::PostMultiplied => 0,
                gpu::AlphaMode::PreMultiplied => 1,
            },
            pad: 0,
        };

        let target = if let Some(texture_view_msaa) = self.texture_view_msaa {
            gpu::RenderTarget {
                view: texture_view_msaa,
                init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                finish_op: gpu::FinishOp::ResolveTo(frame_view),
            }
        } else {
            gpu::RenderTarget {
                view: frame_view,
                init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                finish_op: gpu::FinishOp::Store,
            }
        };

        // draw to the target texture
        if let mut pass = self.command_encoder.render(
            "main",
            gpu::RenderTargetSet {
                colors: &[target],
                depth_stencil: None,
            },
        ) {
            profiling::scope!("render pass");
            for batch in scene.batches() {
                match batch {
                    PrimitiveBatch::Quads(quads) => {
                        let instance_buf =
                            unsafe { self.instance_belt.alloc_typed(quads, &self.gpu) };
                        let mut encoder = pass.with(&self.pipelines.quads);
                        encoder.bind(
                            0,
                            &ShaderQuadsData {
                                globals,
                                b_quads: instance_buf,
                            },
                        );
                        encoder.draw(0, 4, 0, quads.len() as u32);
                    }
                    PrimitiveBatch::Shadows(shadows) => {
                        let instance_buf =
                            unsafe { self.instance_belt.alloc_typed(shadows, &self.gpu) };
                        let mut encoder = pass.with(&self.pipelines.shadows);
                        encoder.bind(
                            0,
                            &ShaderShadowsData {
                                globals,
                                b_shadows: instance_buf,
                            },
                        );
                        encoder.draw(0, 4, 0, shadows.len() as u32);
                    }
                    PrimitiveBatch::Paths(paths) => {
                        let mut encoder = pass.with(&self.pipelines.paths);

                        let mut vertices = Vec::new();
                        let mut sprites = Vec::with_capacity(paths.len());
                        let mut draw_indirect_commands = Vec::with_capacity(paths.len());
                        let mut first_vertex = 0;

                        for (i, path) in paths.iter().enumerate() {
                            draw_indirect_commands.push(DrawIndirectArgs {
                                vertex_count: path.vertices.len() as u32,
                                instance_count: 1,
                                first_vertex,
                                first_instance: i as u32,
                            });
                            first_vertex += path.vertices.len() as u32;

                            vertices.extend(path.vertices.iter().map(|v| PathVertex {
                                xy_position: v.xy_position,
                                content_mask: ContentMask {
                                    bounds: path.content_mask.bounds,
                                },
                            }));

                            sprites.push(PathSprite {
                                bounds: path.bounds,
                                color: path.color,
                            });
                        }

                        let b_path_vertices =
                            unsafe { self.instance_belt.alloc_typed(&vertices, &self.gpu) };
                        let instance_buf =
                            unsafe { self.instance_belt.alloc_typed(&sprites, &self.gpu) };
                        let indirect_buf = unsafe {
                            self.instance_belt
                                .alloc_typed(&draw_indirect_commands, &self.gpu)
                        };

                        encoder.bind(
                            0,
                            &ShaderPathsData {
                                globals,
                                b_path_vertices,
                                b_path_sprites: instance_buf,
                            },
                        );

                        for i in 0..paths.len() {
                            encoder.draw_indirect(indirect_buf.buffer.at(indirect_buf.offset
                                + (i * mem::size_of::<DrawIndirectArgs>()) as u64));
                        }
                    }
                    PrimitiveBatch::Underlines(underlines) => {
                        let instance_buf =
                            unsafe { self.instance_belt.alloc_typed(underlines, &self.gpu) };
                        let mut encoder = pass.with(&self.pipelines.underlines);
                        encoder.bind(
                            0,
                            &ShaderUnderlinesData {
                                globals,
                                b_underlines: instance_buf,
                            },
                        );
                        encoder.draw(0, 4, 0, underlines.len() as u32);
                    }
                    PrimitiveBatch::MonochromeSprites {
                        texture_id,
                        sprites,
                    } => {
                        let tex_info = self.atlas.get_texture_info(texture_id);
                        let instance_buf =
                            unsafe { self.instance_belt.alloc_typed(sprites, &self.gpu) };
                        let mut encoder = pass.with(&self.pipelines.mono_sprites);
                        encoder.bind(
                            0,
                            &ShaderMonoSpritesData {
                                globals,
                                t_sprite: tex_info.raw_view,
                                s_sprite: self.atlas_sampler,
                                b_mono_sprites: instance_buf,
                            },
                        );
                        encoder.draw(0, 4, 0, sprites.len() as u32);
                    }
                    PrimitiveBatch::PolychromeSprites {
                        texture_id,
                        sprites,
                    } => {
                        let tex_info = self.atlas.get_texture_info(texture_id);
                        let instance_buf =
                            unsafe { self.instance_belt.alloc_typed(sprites, &self.gpu) };
                        let mut encoder = pass.with(&self.pipelines.poly_sprites);
                        encoder.bind(
                            0,
                            &ShaderPolySpritesData {
                                globals,
                                t_sprite: tex_info.raw_view,
                                s_sprite: self.atlas_sampler,
                                b_poly_sprites: instance_buf,
                            },
                        );
                        encoder.draw(0, 4, 0, sprites.len() as u32);
                    }
                    PrimitiveBatch::Surfaces(surfaces) => {
                        let mut _encoder = pass.with(&self.pipelines.surfaces);

                        for surface in surfaces {
                            #[cfg(not(target_os = "macos"))]
                            {
                                let _ = surface;
                                continue;
                            };

                            #[cfg(target_os = "macos")]
                            {
                                let (t_y, t_cb_cr) = unsafe {
                                    use core_foundation::base::TCFType as _;
                                    use std::ptr;

                                    assert_eq!(
                                        surface.image_buffer.get_pixel_format(),
                                        core_video::pixel_buffer::kCVPixelFormatType_420YpCbCr8BiPlanarFullRange
                                    );

                                    let y_texture = self
                                        .core_video_texture_cache
                                        .create_texture_from_image(
                                            surface.image_buffer.as_concrete_TypeRef(),
                                            ptr::null(),
                                            metal::MTLPixelFormat::R8Unorm,
                                            surface.image_buffer.get_width_of_plane(0),
                                            surface.image_buffer.get_height_of_plane(0),
                                            0,
                                        )
                                        .unwrap();
                                    let cb_cr_texture = self
                                        .core_video_texture_cache
                                        .create_texture_from_image(
                                            surface.image_buffer.as_concrete_TypeRef(),
                                            ptr::null(),
                                            metal::MTLPixelFormat::RG8Unorm,
                                            surface.image_buffer.get_width_of_plane(1),
                                            surface.image_buffer.get_height_of_plane(1),
                                            1,
                                        )
                                        .unwrap();
                                    (
                                        gpu::TextureView::from_metal_texture(
                                            &objc2::rc::Retained::retain(
                                                foreign_types::ForeignTypeRef::as_ptr(
                                                    y_texture.as_texture_ref(),
                                                )
                                                    as *mut objc2::runtime::ProtocolObject<
                                                        dyn objc2_metal::MTLTexture,
                                                    >,
                                            )
                                            .unwrap(),
                                            gpu::TexelAspects::COLOR,
                                        ),
                                        gpu::TextureView::from_metal_texture(
                                            &objc2::rc::Retained::retain(
                                                foreign_types::ForeignTypeRef::as_ptr(
                                                    cb_cr_texture.as_texture_ref(),
                                                )
                                                    as *mut objc2::runtime::ProtocolObject<
                                                        dyn objc2_metal::MTLTexture,
                                                    >,
                                            )
                                            .unwrap(),
                                            gpu::TexelAspects::COLOR,
                                        ),
                                    )
                                };

                                _encoder.bind(
                                    0,
                                    &ShaderSurfacesData {
                                        globals,
                                        surface_locals: SurfaceParams {
                                            bounds: surface.bounds.into(),
                                            content_mask: surface.content_mask.bounds.into(),
                                        },
                                        t_y,
                                        t_cb_cr,
                                        s_surface: self.atlas_sampler,
                                    },
                                );

                                _encoder.draw(0, 4, 0, 1);
                            }
                        }
                    }
                }
            }
        }

        self.command_encoder.present(frame);
        let sync_point = self.gpu.submit(&mut self.command_encoder);

        profiling::scope!("finish");
        self.instance_belt.flush(&sync_point);
        self.atlas.after_frame(&sync_point);

        self.wait_for_gpu();
        self.last_sync_point = Some(sync_point);
    }
}

fn create_msaa_texture_if_needed(
    gpu: &gpu::Context,
    format: gpu::TextureFormat,
    width: u32,
    height: u32,
    sample_count: u32,
) -> Option<(gpu::Texture, gpu::TextureView)> {
    if sample_count <= 1 {
        return None;
    }

    let texture_msaa = gpu.create_texture(gpu::TextureDesc {
        name: "msaa",
        format,
        size: gpu::Extent {
            width,
            height,
            depth: 1,
        },
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count,
        dimension: gpu::TextureDimension::D2,
        usage: gpu::TextureUsage::TARGET,
        external: None,
    });
    let texture_view_msaa = gpu.create_texture_view(
        texture_msaa,
        gpu::TextureViewDesc {
            name: "msaa view",
            format,
            dimension: gpu::ViewDimension::D2,
            subresources: &Default::default(),
        },
    );

    Some((texture_msaa, texture_view_msaa))
}
