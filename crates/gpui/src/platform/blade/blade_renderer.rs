// Doing `if let` gives you nice scoping with passes/encoders
#![allow(irrefutable_let_patterns)]

use super::{BladeAtlas, BladeContext, PATH_TEXTURE_FORMAT};
use crate::{
    AtlasTextureKind, AtlasTile, Background, Bounds, ContentMask, DevicePixels, GpuSpecs,
    MonochromeSprite, Path, PathId, PathVertex, PolychromeSprite, PrimitiveBatch, Quad,
    ScaledPixels, Scene, Shadow, Size, Underline,
};
use blade_graphics as gpu;
use blade_util::{BufferBelt, BufferBeltDescriptor};
use bytemuck::{Pod, Zeroable};
use collections::HashMap;
#[cfg(target_os = "macos")]
use media::core_video::CVMetalTextureCache;
use std::{mem, sync::Arc};

const MAX_FRAME_TIME_MS: u32 = 10000;
// Use 4x MSAA, all devices support it.
// https://developer.apple.com/documentation/metal/mtldevice/1433355-supportstexturesamplecount
const DEFAULT_PATH_SAMPLE_COUNT: u32 = 4;

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
struct ShaderPathRasterizationData {
    globals: GlobalParams,
    b_path_vertices: gpu::BufferPiece,
}

#[derive(blade_macros::ShaderData)]
struct ShaderPathsData {
    globals: GlobalParams,
    t_sprite: gpu::TextureView,
    s_sprite: gpu::Sampler,
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
    tile: AtlasTile,
}

struct BladePipelines {
    quads: gpu::RenderPipeline,
    shadows: gpu::RenderPipeline,
    path_rasterization: gpu::RenderPipeline,
    paths: gpu::RenderPipeline,
    underlines: gpu::RenderPipeline,
    mono_sprites: gpu::RenderPipeline,
    poly_sprites: gpu::RenderPipeline,
    surfaces: gpu::RenderPipeline,
}

impl BladePipelines {
    fn new(gpu: &gpu::Context, surface_info: gpu::SurfaceInfo, path_sample_count: u32) -> Self {
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
                multisample_state: gpu::MultisampleState::default(),
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
                multisample_state: gpu::MultisampleState::default(),
            }),
            path_rasterization: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "path_rasterization",
                data_layouts: &[&ShaderPathRasterizationData::layout()],
                vertex: shader.at("vs_path_rasterization"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_path_rasterization")),
                color_targets: &[gpu::ColorTargetState {
                    format: PATH_TEXTURE_FORMAT,
                    blend: Some(gpu::BlendState::ADDITIVE),
                    write_mask: gpu::ColorWrites::default(),
                }],
                multisample_state: gpu::MultisampleState {
                    sample_count: path_sample_count,
                    ..Default::default()
                },
            }),
            paths: gpu.create_render_pipeline(gpu::RenderPipelineDesc {
                name: "paths",
                data_layouts: &[&ShaderPathsData::layout()],
                vertex: shader.at("vs_path"),
                vertex_fetches: &[],
                primitive: gpu::PrimitiveState {
                    topology: gpu::PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                },
                depth_stencil: None,
                fragment: Some(shader.at("fs_path")),
                color_targets,
                multisample_state: gpu::MultisampleState::default(),
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
                multisample_state: gpu::MultisampleState::default(),
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
                multisample_state: gpu::MultisampleState::default(),
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
                multisample_state: gpu::MultisampleState::default(),
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
                multisample_state: gpu::MultisampleState::default(),
            }),
        }
    }

    fn destroy(&mut self, gpu: &gpu::Context) {
        gpu.destroy_render_pipeline(&mut self.quads);
        gpu.destroy_render_pipeline(&mut self.shadows);
        gpu.destroy_render_pipeline(&mut self.path_rasterization);
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
    path_tiles: HashMap<PathId, AtlasTile>,
    atlas: Arc<BladeAtlas>,
    atlas_sampler: gpu::Sampler,
    #[cfg(target_os = "macos")]
    core_video_texture_cache: CVMetalTextureCache,
    path_sample_count: u32,
}

impl BladeRenderer {
    pub fn new<I: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
        context: &BladeContext,
        window: &I,
        config: BladeSurfaceConfig,
    ) -> anyhow::Result<Self> {
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
            .unwrap();

        let command_encoder = context.gpu.create_command_encoder(gpu::CommandEncoderDesc {
            name: "main",
            buffer_count: 2,
        });
        // workaround for https://github.com/zed-industries/zed/issues/26143
        let path_sample_count = std::env::var("ZED_PATH_SAMPLE_COUNT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_PATH_SAMPLE_COUNT);
        let pipelines = BladePipelines::new(&context.gpu, surface.info(), path_sample_count);
        let instance_belt = BufferBelt::new(BufferBeltDescriptor {
            memory: gpu::Memory::Shared,
            min_chunk_size: 0x1000,
            alignment: 0x40, // Vulkan `minStorageBufferOffsetAlignment` on Intel Xe
        });
        let atlas = Arc::new(BladeAtlas::new(&context.gpu, path_sample_count));
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
            path_tiles: HashMap::default(),
            atlas,
            atlas_sampler,
            #[cfg(target_os = "macos")]
            core_video_texture_cache,
            path_sample_count,
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
    #[cfg_attr(any(target_os = "macos", target_os = "linux"), allow(dead_code))]
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
        }
    }

    pub fn update_transparency(&mut self, transparent: bool) {
        if transparent != self.surface_config.transparent {
            self.wait_for_gpu();
            self.surface_config.transparent = transparent;
            self.gpu
                .reconfigure_surface(&mut self.surface, self.surface_config);
            self.pipelines.destroy(&self.gpu);
            self.pipelines =
                BladePipelines::new(&self.gpu, self.surface.info(), self.path_sample_count);
        }
    }

    #[cfg_attr(
        any(target_os = "macos", target_os = "windows", feature = "wayland"),
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

    #[profiling::function]
    fn rasterize_paths(&mut self, paths: &[Path<ScaledPixels>]) {
        self.path_tiles.clear();
        let mut vertices_by_texture_id = HashMap::default();

        for path in paths {
            let clipped_bounds = path
                .bounds
                .intersect(&path.content_mask.bounds)
                .map_origin(|origin| origin.floor())
                .map_size(|size| size.ceil());
            let tile = self.atlas.allocate_for_rendering(
                clipped_bounds.size.map(Into::into),
                AtlasTextureKind::Path,
                &mut self.command_encoder,
            );
            vertices_by_texture_id
                .entry(tile.texture_id)
                .or_insert(Vec::new())
                .extend(path.vertices.iter().map(|vertex| PathVertex {
                    xy_position: vertex.xy_position - clipped_bounds.origin
                        + tile.bounds.origin.map(Into::into),
                    st_position: vertex.st_position,
                    content_mask: ContentMask {
                        bounds: tile.bounds.map(Into::into),
                    },
                }));
            self.path_tiles.insert(path.id, tile);
        }

        for (texture_id, vertices) in vertices_by_texture_id {
            let tex_info = self.atlas.get_texture_info(texture_id);
            let globals = GlobalParams {
                viewport_size: [tex_info.size.width as f32, tex_info.size.height as f32],
                premultiplied_alpha: 0,
                pad: 0,
            };

            let vertex_buf = unsafe { self.instance_belt.alloc_typed(&vertices, &self.gpu) };
            let frame_view = tex_info.raw_view;
            let color_target = if let Some(msaa_view) = tex_info.msaa_view {
                gpu::RenderTarget {
                    view: msaa_view,
                    init_op: gpu::InitOp::Clear(gpu::TextureColor::OpaqueBlack),
                    finish_op: gpu::FinishOp::ResolveTo(frame_view),
                }
            } else {
                gpu::RenderTarget {
                    view: frame_view,
                    init_op: gpu::InitOp::Clear(gpu::TextureColor::OpaqueBlack),
                    finish_op: gpu::FinishOp::Store,
                }
            };

            if let mut pass = self.command_encoder.render(
                "paths",
                gpu::RenderTargetSet {
                    colors: &[color_target],
                    depth_stencil: None,
                },
            ) {
                let mut encoder = pass.with(&self.pipelines.path_rasterization);
                encoder.bind(
                    0,
                    &ShaderPathRasterizationData {
                        globals,
                        b_path_vertices: vertex_buf,
                    },
                );
                encoder.draw(0, vertices.len() as u32, 0, 1);
            }
        }
    }

    pub fn destroy(&mut self) {
        self.wait_for_gpu();
        self.atlas.destroy();
        self.gpu.destroy_sampler(self.atlas_sampler);
        self.instance_belt.destroy(&self.gpu);
        self.gpu.destroy_command_encoder(&mut self.command_encoder);
        self.pipelines.destroy(&self.gpu);
        self.gpu.destroy_surface(&mut self.surface);
    }

    pub fn draw(&mut self, scene: &Scene) {
        self.command_encoder.start();
        self.atlas.before_frame(&mut self.command_encoder);
        self.rasterize_paths(scene.paths());

        let frame = {
            profiling::scope!("acquire frame");
            self.surface.acquire_frame()
        };
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

        if let mut pass = self.command_encoder.render(
            "main",
            gpu::RenderTargetSet {
                colors: &[gpu::RenderTarget {
                    view: frame.texture_view(),
                    init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                    finish_op: gpu::FinishOp::Store,
                }],
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
                        // todo(linux): group by texture ID
                        for path in paths {
                            let tile = &self.path_tiles[&path.id];
                            let tex_info = self.atlas.get_texture_info(tile.texture_id);
                            let origin = path.bounds.intersect(&path.content_mask.bounds).origin;
                            let sprites = [PathSprite {
                                bounds: Bounds {
                                    origin: origin.map(|p| p.floor()),
                                    size: tile.bounds.size.map(Into::into),
                                },
                                color: path.color,
                                tile: (*tile).clone(),
                            }];

                            let instance_buf =
                                unsafe { self.instance_belt.alloc_typed(&sprites, &self.gpu) };
                            encoder.bind(
                                0,
                                &ShaderPathsData {
                                    globals,
                                    t_sprite: tex_info.raw_view,
                                    s_sprite: self.atlas_sampler,
                                    b_path_sprites: instance_buf,
                                },
                            );
                            encoder.draw(0, 4, 0, sprites.len() as u32);
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
        self.atlas.clear_textures(AtlasTextureKind::Path);

        self.wait_for_gpu();
        self.last_sync_point = Some(sync_point);
    }
}
