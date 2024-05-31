// Doing `if let` gives you nice scoping with passes/encoders
#![allow(irrefutable_let_patterns)]

use super::{BladeAtlas, PATH_TEXTURE_FORMAT};
use crate::{
    AtlasTextureKind, AtlasTile, Bounds, ContentMask, Hsla, MonochromeSprite, Path, PathId,
    PathVertex, PolychromeSprite, PrimitiveBatch, Quad, ScaledPixels, Scene, Shadow, Size,
    Underline,
};
use bytemuck::{Pod, Zeroable};
use collections::HashMap;
#[cfg(target_os = "macos")]
use media::core_video::CVMetalTextureCache;
#[cfg(target_os = "macos")]
use std::{ffi::c_void, ptr::NonNull};

use blade_graphics as gpu;
use blade_util::{BufferBelt, BufferBeltDescriptor};
use std::{mem, sync::Arc};

const MAX_FRAME_TIME_MS: u32 = 1000;

pub type Context = ();
pub type Renderer = BladeRenderer;

#[cfg(target_os = "macos")]
pub unsafe fn new_renderer(
    _context: self::Context,
    _native_window: *mut c_void,
    native_view: *mut c_void,
    bounds: crate::Size<f32>,
    transparent: bool,
) -> Renderer {
    use raw_window_handle as rwh;
    struct RawWindow {
        view: *mut c_void,
    }

    impl rwh::HasWindowHandle for RawWindow {
        fn window_handle(&self) -> Result<rwh::WindowHandle, rwh::HandleError> {
            let view = NonNull::new(self.view).unwrap();
            let handle = rwh::AppKitWindowHandle::new(view);
            Ok(unsafe { rwh::WindowHandle::borrow_raw(handle.into()) })
        }
    }
    impl rwh::HasDisplayHandle for RawWindow {
        fn display_handle(&self) -> Result<rwh::DisplayHandle, rwh::HandleError> {
            let handle = rwh::AppKitDisplayHandle::new();
            Ok(unsafe { rwh::DisplayHandle::borrow_raw(handle.into()) })
        }
    }

    let gpu = Arc::new(
        gpu::Context::init_windowed(
            &RawWindow {
                view: native_view as *mut _,
            },
            gpu::ContextDesc {
                validation: cfg!(debug_assertions),
                capture: false,
                overlay: false,
            },
        )
        .unwrap(),
    );

    BladeRenderer::new(
        gpu,
        BladeSurfaceConfig {
            size: gpu::Extent {
                width: bounds.width as u32,
                height: bounds.height as u32,
                depth: 1,
            },
            transparent,
        },
    )
}

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
    color: Hsla,
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
    fn new(gpu: &gpu::Context, surface_info: gpu::SurfaceInfo) -> Self {
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
                fragment: shader.at("fs_quad"),
                color_targets,
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
                fragment: shader.at("fs_shadow"),
                color_targets,
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
                fragment: shader.at("fs_path_rasterization"),
                color_targets: &[gpu::ColorTargetState {
                    format: PATH_TEXTURE_FORMAT,
                    blend: Some(gpu::BlendState::ADDITIVE),
                    write_mask: gpu::ColorWrites::default(),
                }],
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
                fragment: shader.at("fs_path"),
                color_targets,
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
                fragment: shader.at("fs_underline"),
                color_targets,
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
                fragment: shader.at("fs_mono_sprite"),
                color_targets,
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
                fragment: shader.at("fs_poly_sprite"),
                color_targets,
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
                fragment: shader.at("fs_surface"),
                color_targets,
            }),
        }
    }
}

pub struct BladeSurfaceConfig {
    pub size: gpu::Extent,
    pub transparent: bool,
}

pub struct BladeRenderer {
    gpu: Arc<gpu::Context>,
    surface_config: gpu::SurfaceConfig,
    alpha_mode: gpu::AlphaMode,
    command_encoder: gpu::CommandEncoder,
    last_sync_point: Option<gpu::SyncPoint>,
    pipelines: BladePipelines,
    instance_belt: BufferBelt,
    path_tiles: HashMap<PathId, AtlasTile>,
    atlas: Arc<BladeAtlas>,
    atlas_sampler: gpu::Sampler,
    #[cfg(target_os = "macos")]
    core_video_texture_cache: CVMetalTextureCache,
}

impl BladeRenderer {
    pub fn new(gpu: Arc<gpu::Context>, config: BladeSurfaceConfig) -> Self {
        let surface_config = gpu::SurfaceConfig {
            size: config.size,
            usage: gpu::TextureUsage::TARGET,
            display_sync: gpu::DisplaySync::Recent,
            color_space: gpu::ColorSpace::Linear,
            allow_exclusive_full_screen: false,
            transparent: config.transparent,
        };
        let surface_info = gpu.resize(surface_config);

        let command_encoder = gpu.create_command_encoder(gpu::CommandEncoderDesc {
            name: "main",
            buffer_count: 2,
        });
        let pipelines = BladePipelines::new(&gpu, surface_info);
        let instance_belt = BufferBelt::new(BufferBeltDescriptor {
            memory: gpu::Memory::Shared,
            min_chunk_size: 0x1000,
            alignment: 0x40, // Vulkan `minStorageBufferOffsetAlignment` on Intel Xe
        });
        let atlas = Arc::new(BladeAtlas::new(&gpu));
        let atlas_sampler = gpu.create_sampler(gpu::SamplerDesc {
            name: "atlas",
            mag_filter: gpu::FilterMode::Linear,
            min_filter: gpu::FilterMode::Linear,
            ..Default::default()
        });

        #[cfg(target_os = "macos")]
        let core_video_texture_cache = unsafe {
            use foreign_types::ForeignType as _;
            CVMetalTextureCache::new(gpu.metal_device().as_ptr()).unwrap()
        };

        Self {
            gpu,
            surface_config,
            alpha_mode: surface_info.alpha,
            command_encoder,
            last_sync_point: None,
            pipelines,
            instance_belt,
            path_tiles: HashMap::default(),
            atlas,
            atlas_sampler,
            #[cfg(target_os = "macos")]
            core_video_texture_cache,
        }
    }

    fn wait_for_gpu(&mut self) {
        if let Some(last_sp) = self.last_sync_point.take() {
            if !self.gpu.wait_for(&last_sp, MAX_FRAME_TIME_MS) {
                panic!("GPU hung");
            }
        }
    }

    pub fn update_drawable_size(&mut self, size: Size<f64>) {
        let gpu_size = gpu::Extent {
            width: size.width as u32,
            height: size.height as u32,
            depth: 1,
        };

        if gpu_size != self.surface_config.size {
            self.wait_for_gpu();
            self.surface_config.size = gpu_size;
            self.gpu.resize(self.surface_config);
        }
    }

    pub fn update_transparency(&mut self, transparent: bool) {
        if transparent != self.surface_config.transparent {
            self.wait_for_gpu();
            self.surface_config.transparent = transparent;
            let surface_info = self.gpu.resize(self.surface_config);
            self.pipelines = BladePipelines::new(&self.gpu, surface_info);
            self.alpha_mode = surface_info.alpha;
        }
    }

    #[cfg_attr(target_os = "macos", allow(dead_code))]
    pub fn viewport_size(&self) -> gpu::Extent {
        self.surface_config.size
    }

    pub fn sprite_atlas(&self) -> &Arc<BladeAtlas> {
        &self.atlas
    }

    #[cfg(target_os = "macos")]
    pub fn layer(&self) -> metal::MetalLayer {
        self.gpu.metal_layer().unwrap()
    }

    #[cfg(target_os = "macos")]
    pub fn layer_ptr(&self) -> *mut metal::CAMetalLayer {
        use metal::foreign_types::ForeignType as _;
        self.gpu.metal_layer().unwrap().as_ptr()
    }

    #[profiling::function]
    fn rasterize_paths(&mut self, paths: &[Path<ScaledPixels>]) {
        self.path_tiles.clear();
        let mut vertices_by_texture_id = HashMap::default();

        for path in paths {
            let clipped_bounds = path.bounds.intersect(&path.content_mask.bounds);
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
            let mut pass = self.command_encoder.render(gpu::RenderTargetSet {
                colors: &[gpu::RenderTarget {
                    view: tex_info.raw_view,
                    init_op: gpu::InitOp::Clear(gpu::TextureColor::OpaqueBlack),
                    finish_op: gpu::FinishOp::Store,
                }],
                depth_stencil: None,
            });

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

    pub fn destroy(&mut self) {
        self.wait_for_gpu();
        self.atlas.destroy();
        self.instance_belt.destroy(&self.gpu);
        self.gpu.destroy_command_encoder(&mut self.command_encoder);
    }

    pub fn draw(&mut self, scene: &Scene) {
        self.command_encoder.start();
        self.atlas.before_frame(&mut self.command_encoder);
        self.rasterize_paths(scene.paths());

        let frame = {
            profiling::scope!("acquire frame");
            self.gpu.acquire_frame()
        };
        self.command_encoder.init_texture(frame.texture());

        let globals = GlobalParams {
            viewport_size: [
                self.surface_config.size.width as f32,
                self.surface_config.size.height as f32,
            ],
            premultiplied_alpha: match self.alpha_mode {
                gpu::AlphaMode::Ignored | gpu::AlphaMode::PostMultiplied => 0,
                gpu::AlphaMode::PreMultiplied => 1,
            },
            pad: 0,
        };

        if let mut pass = self.command_encoder.render(gpu::RenderTargetSet {
            colors: &[gpu::RenderTarget {
                view: frame.texture_view(),
                init_op: gpu::InitOp::Clear(gpu::TextureColor::TransparentBlack),
                finish_op: gpu::FinishOp::Store,
            }],
            depth_stencil: None,
        }) {
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
                                let (t_y, t_cb_cr) = {
                                    use core_foundation::base::TCFType as _;
                                    use std::ptr;

                                    assert_eq!(
                                    surface.image_buffer.pixel_format_type(),
                                    media::core_video::kCVPixelFormatType_420YpCbCr8BiPlanarFullRange
                                );

                                    let y_texture = unsafe {
                                        self.core_video_texture_cache
                                            .create_texture_from_image(
                                                surface.image_buffer.as_concrete_TypeRef(),
                                                ptr::null(),
                                                metal::MTLPixelFormat::R8Unorm,
                                                surface.image_buffer.plane_width(0),
                                                surface.image_buffer.plane_height(0),
                                                0,
                                            )
                                            .unwrap()
                                    };
                                    let cb_cr_texture = unsafe {
                                        self.core_video_texture_cache
                                            .create_texture_from_image(
                                                surface.image_buffer.as_concrete_TypeRef(),
                                                ptr::null(),
                                                metal::MTLPixelFormat::RG8Unorm,
                                                surface.image_buffer.plane_width(1),
                                                surface.image_buffer.plane_height(1),
                                                1,
                                            )
                                            .unwrap()
                                    };
                                    (
                                        gpu::TextureView::from_metal_texture(
                                            y_texture.as_texture_ref(),
                                        ),
                                        gpu::TextureView::from_metal_texture(
                                            cb_cr_texture.as_texture_ref(),
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
