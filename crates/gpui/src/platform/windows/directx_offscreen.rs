//! DirectX 11 Off-Screen Rendering implementation for Windows.
//!
//! This module provides an off-screen render target using DirectX 11 textures,
//! allowing GPUI content to be rendered without a window surface.

use std::slice;
use std::sync::Arc;

use anyhow::{Context, Result};
use windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY;
use windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP;
use windows::Win32::Graphics::Dxgi::IDXGIKeyedMutex;
use windows::Win32::Graphics::{Direct3D11::*, Dxgi::Common::*, Dxgi::*};
use windows::core::Interface;

use crate::platform::offscreen::{
    D3D11SharedTexture, DrawableOffScreenTarget, OffScreenImage, OffScreenRenderTarget,
    OffScreenTargetConfig, PixelFormat, SharedTextureHandle,
};
use crate::platform::windows::directx_atlas::DirectXAtlas;
use crate::platform::windows::directx_renderer::{DirectXRendererDevices, FontInfo};
use crate::scene::{
    MonochromeSprite, Path, PolychromeSprite, PrimitiveBatch, Quad, Shadow, Underline,
};
use crate::{
    AtlasTextureId, Background, Bounds, DevicePixels, PlatformAtlas, Point, ScaledPixels, Scene,
    Size,
};

use super::directx_renderer::shader_resources::{RawShaderBytes, ShaderModule, ShaderTarget};

/// The render target format used for off-screen rendering.
const RENDER_TARGET_FORMAT: DXGI_FORMAT = DXGI_FORMAT_B8G8R8A8_UNORM;

/// MSAA sample count for path rendering.
const PATH_MULTISAMPLE_COUNT: u32 = 4;

/// Global parameters passed to shaders.
#[repr(C)]
struct GlobalParams {
    gamma_ratios: [f32; 4],
    viewport_size: [f32; 2],
    grayscale_enhanced_contrast: f32,
    _pad: u32,
}

/// Sprite data for path rasterization intermediate pass.
#[derive(Clone, Copy)]
#[repr(C)]
struct PathRasterizationSprite {
    xy_position: Point<ScaledPixels>,
    st_position: Point<f32>,
    color: Background,
    bounds: Bounds<ScaledPixels>,
}

/// Sprite data for path final pass.
#[derive(Clone, Copy)]
#[repr(C)]
struct PathSprite {
    bounds: Bounds<ScaledPixels>,
}

/// DirectX 11 off-screen render target.
///
/// This renders GPUI scenes to an off-screen texture instead of a window surface.
/// It supports:
/// - Reading pixels back to CPU memory via `read_pixels()`
/// - Zero-copy texture sharing via DXGI shared handles
pub(crate) struct DirectXOffScreenTarget {
    /// D3D11 devices (shared with window renderers if available)
    devices: DirectXRendererDevices,
    /// Texture atlas for sprites
    atlas: Arc<DirectXAtlas>,
    /// Render pipelines
    pipelines: DirectXOffScreenPipelines,
    /// Global shader parameters buffer
    global_params_buffer: Option<ID3D11Buffer>,
    /// Sampler state for texture sampling
    sampler: Option<ID3D11SamplerState>,
    /// Font rendering information
    font_info: &'static FontInfo,

    // Render target resources
    /// The main render texture (what we render to)
    render_texture: ID3D11Texture2D,
    /// Render target view for the main texture
    render_target_view: Option<ID3D11RenderTargetView>,
    /// Staging textures for CPU readback (double-buffered if enabled)
    staging_textures: Vec<ID3D11Texture2D>,
    /// Index of the current staging texture for writing
    staging_write_index: usize,
    /// Whether double-buffering is enabled
    double_buffer_enabled: bool,

    // Path intermediate textures (with MSAA)
    path_intermediate_texture: ID3D11Texture2D,
    path_intermediate_srv: Option<ID3D11ShaderResourceView>,
    path_intermediate_msaa_texture: ID3D11Texture2D,
    path_intermediate_msaa_view: Option<ID3D11RenderTargetView>,

    /// Cached viewport
    viewport: D3D11_VIEWPORT,

    /// Current width in pixels
    width: u32,
    /// Current height in pixels
    height: u32,

    /// Whether texture sharing is enabled
    sharing_enabled: bool,
    /// DXGI shared handle (if sharing is enabled)
    shared_handle: Option<windows::Win32::Foundation::HANDLE>,
    /// Keyed mutex for synchronizing access to the shared texture
    keyed_mutex: Option<IDXGIKeyedMutex>,
}

/// Render pipeline states for off-screen rendering.
struct DirectXOffScreenPipelines {
    shadow_pipeline: PipelineState,
    quad_pipeline: PipelineState,
    path_rasterization_pipeline: PipelineState,
    path_sprite_pipeline: PipelineState,
    underline_pipeline: PipelineState,
    mono_sprites: PipelineState,
    poly_sprites: PipelineState,
}

/// A pipeline state for off-screen rendering with buffer management.
struct PipelineState {
    label: &'static str,
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
    buffer: Option<ID3D11Buffer>,
    buffer_view: Option<ID3D11ShaderResourceView>,
    blend_state: Option<ID3D11BlendState>,
    buffer_capacity: usize,
    element_size: usize,
}

impl DirectXOffScreenTarget {
    /// Creates a new off-screen render target with the given configuration.
    pub fn new(devices: DirectXRendererDevices, config: OffScreenTargetConfig) -> Result<Self> {
        let width = config.size.width.0.max(1) as u32;
        let height = config.size.height.0.max(1) as u32;

        let atlas = Arc::new(DirectXAtlas::new(&devices.device, &devices.device_context));

        // Create render texture
        let (render_texture, render_target_view, shared_handle, keyed_mutex) =
            create_render_texture(&devices.device, width, height, config.enable_sharing)?;

        // Create staging texture(s) for CPU readback
        let staging_textures = if config.double_buffer {
            vec![
                create_staging_texture(&devices.device, width, height)?,
                create_staging_texture(&devices.device, width, height)?,
            ]
        } else {
            vec![create_staging_texture(&devices.device, width, height)?]
        };

        // Create path intermediate textures
        let (path_intermediate_texture, path_intermediate_srv) =
            create_path_intermediate_texture(&devices.device, width, height)?;
        let (path_intermediate_msaa_texture, path_intermediate_msaa_view) =
            create_path_intermediate_msaa_texture(&devices.device, width, height)?;

        // Create global elements
        let global_params_buffer = create_constant_buffer(&devices.device)?;
        let sampler = create_sampler(&devices.device)?;

        // Create pipelines
        let pipelines = DirectXOffScreenPipelines::new(&devices.device)?;

        // Set up viewport
        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: width as f32,
            Height: height as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };

        // Set rasterizer state
        set_rasterizer_state(&devices.device, &devices.device_context)?;

        // Get font info
        let font_info = get_font_info();

        Ok(Self {
            devices,
            atlas,
            pipelines,
            global_params_buffer,
            sampler,
            font_info,
            render_texture,
            render_target_view,
            staging_textures,
            staging_write_index: 0,
            double_buffer_enabled: config.double_buffer,
            path_intermediate_texture,
            path_intermediate_srv,
            path_intermediate_msaa_texture,
            path_intermediate_msaa_view,
            viewport,
            width,
            height,
            sharing_enabled: config.enable_sharing,
            shared_handle,
            keyed_mutex,
        })
    }

    /// Acquires the keyed mutex for exclusive access to the shared texture.
    ///
    /// This should be called before rendering to the texture when sharing is enabled.
    /// The mutex ensures proper synchronization between the producer (this renderer)
    /// and any consumers (other processes/APIs using the shared handle).
    ///
    /// # Arguments
    ///
    /// * `key` - The key value to acquire (typically 0)
    /// * `timeout_ms` - Timeout in milliseconds (use u32::MAX for infinite)
    ///
    /// # Returns
    ///
    /// `Ok(true)` if the mutex was acquired, `Ok(false)` if it timed out,
    /// or an error if the acquisition failed.
    pub fn acquire_keyed_mutex(&self, key: u64, timeout_ms: u32) -> Result<bool> {
        if let Some(ref mutex) = self.keyed_mutex {
            let result = unsafe { mutex.AcquireSync(key, timeout_ms) };
            match result {
                Ok(()) => Ok(true),
                Err(e) if e.code() == windows::core::HRESULT(0x80070102u32 as i32) => {
                    // WAIT_TIMEOUT = 0x00000102, as HRESULT = 0x80070102
                    Ok(false)
                }
                Err(e) => Err(e).context("Failed to acquire keyed mutex"),
            }
        } else {
            Ok(true)
        }
    }

    /// Releases the keyed mutex after rendering is complete.
    ///
    /// This should be called after rendering to signal that the texture is ready
    /// for consumers to access.
    ///
    /// # Arguments
    ///
    /// * `key` - The key value to release to (typically 0 or 1 for double-buffering)
    pub fn release_keyed_mutex(&self, key: u64) -> Result<()> {
        if let Some(ref mutex) = self.keyed_mutex {
            unsafe { mutex.ReleaseSync(key) }.context("Failed to release keyed mutex")?;
        }
        Ok(())
    }

    /// Returns the sprite atlas for this renderer.
    pub fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.atlas.clone()
    }

    /// Prepares for drawing by updating global parameters and clearing the render target.
    fn pre_draw(&self) -> Result<()> {
        // Update global params
        update_buffer(
            &self.devices.device_context,
            self.global_params_buffer
                .as_ref()
                .context("missing global params buffer")?,
            &[GlobalParams {
                gamma_ratios: self.font_info.gamma_ratios,
                viewport_size: [self.viewport.Width, self.viewport.Height],
                grayscale_enhanced_contrast: self.font_info.grayscale_enhanced_contrast,
                _pad: 0,
            }],
        )?;

        unsafe {
            // Clear render target
            self.devices.device_context.ClearRenderTargetView(
                self.render_target_view
                    .as_ref()
                    .context("missing render target view")?,
                &[0.0, 0.0, 0.0, 0.0], // Transparent black
            );

            // Set render target
            self.devices
                .device_context
                .OMSetRenderTargets(Some(slice::from_ref(&self.render_target_view)), None);

            // Set viewport
            self.devices
                .device_context
                .RSSetViewports(Some(slice::from_ref(&self.viewport)));
        }

        Ok(())
    }

    /// Copies the render texture to the staging texture for CPU readback.
    fn copy_to_staging(&self) {
        let staging = &self.staging_textures[self.staging_write_index];
        unsafe {
            self.devices
                .device_context
                .CopyResource(staging, &self.render_texture);
        }
    }

    /// Advances to the next staging buffer (for double-buffering).
    ///
    /// This should be called after `copy_to_staging()` to prepare for the
    /// next frame. When double-buffering is enabled, this allows the previous
    /// frame to be read while the next frame is being rendered.
    fn advance_staging_buffer(&mut self) {
        if self.double_buffer_enabled {
            self.staging_write_index = (self.staging_write_index + 1) % self.staging_textures.len();
        }
    }

    /// Returns the staging texture that should be read from.
    ///
    /// For double-buffering, this returns the buffer that was written to
    /// in the previous frame (not the current write buffer).
    fn read_staging_index(&self) -> usize {
        if self.double_buffer_enabled && self.staging_textures.len() > 1 {
            // Read from the buffer that was written to previously
            (self.staging_write_index + self.staging_textures.len() - 1)
                % self.staging_textures.len()
        } else {
            0
        }
    }
}

impl OffScreenRenderTarget for DirectXOffScreenTarget {
    fn size(&self) -> Size<DevicePixels> {
        Size {
            width: DevicePixels(self.width as i32),
            height: DevicePixels(self.height as i32),
        }
    }

    fn resize(&mut self, size: Size<DevicePixels>) {
        let width = size.width.0.max(1) as u32;
        let height = size.height.0.max(1) as u32;

        if width == self.width && height == self.height {
            return;
        }

        // Recreate render texture
        if let Ok((render_texture, render_target_view, shared_handle, keyed_mutex)) =
            create_render_texture(&self.devices.device, width, height, self.sharing_enabled)
        {
            self.render_texture = render_texture;
            self.render_target_view = render_target_view;
            self.shared_handle = shared_handle;
            self.keyed_mutex = keyed_mutex;
        }

        // Recreate staging texture(s)
        if self.double_buffer_enabled {
            if let Ok(staging1) = create_staging_texture(&self.devices.device, width, height) {
                if let Ok(staging2) = create_staging_texture(&self.devices.device, width, height) {
                    self.staging_textures = vec![staging1, staging2];
                    self.staging_write_index = 0;
                }
            }
        } else {
            if let Ok(staging) = create_staging_texture(&self.devices.device, width, height) {
                self.staging_textures = vec![staging];
                self.staging_write_index = 0;
            }
        }

        // Recreate path intermediate textures
        if let Ok((path_intermediate_texture, path_intermediate_srv)) =
            create_path_intermediate_texture(&self.devices.device, width, height)
        {
            self.path_intermediate_texture = path_intermediate_texture;
            self.path_intermediate_srv = path_intermediate_srv;
        }

        if let Ok((path_intermediate_msaa_texture, path_intermediate_msaa_view)) =
            create_path_intermediate_msaa_texture(&self.devices.device, width, height)
        {
            self.path_intermediate_msaa_texture = path_intermediate_msaa_texture;
            self.path_intermediate_msaa_view = path_intermediate_msaa_view;
        }

        // Update viewport
        self.viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: width as f32,
            Height: height as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };

        self.width = width;
        self.height = height;
    }

    fn pixel_format(&self) -> PixelFormat {
        PixelFormat::Bgra8Unorm
    }

    fn read_pixels(&self) -> anyhow::Result<OffScreenImage> {
        // Copy render texture to staging texture
        self.copy_to_staging();

        // Get the staging texture to read from
        let read_index = self.read_staging_index();
        let staging = &self.staging_textures[read_index];

        // Map the staging texture
        let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
        unsafe {
            self.devices
                .device_context
                .Map(staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped))
                .context("Failed to map staging texture")?;
        }

        // Copy data
        let row_pitch = mapped.RowPitch;
        let data_size = (row_pitch * self.height) as usize;
        let mut data = vec![0u8; (self.width * self.height * 4) as usize];

        unsafe {
            let src = slice::from_raw_parts(mapped.pData as *const u8, data_size);

            // Copy row by row to handle potential row pitch differences
            for y in 0..self.height {
                let src_offset = (y * row_pitch) as usize;
                let dst_offset = (y * self.width * 4) as usize;
                let row_size = (self.width * 4) as usize;
                data[dst_offset..dst_offset + row_size]
                    .copy_from_slice(&src[src_offset..src_offset + row_size]);
            }

            self.devices.device_context.Unmap(staging, 0);
        }

        Ok(OffScreenImage::new(
            data,
            self.width,
            self.height,
            PixelFormat::Bgra8Unorm,
        ))
    }

    fn shared_texture_handle(&self) -> Option<SharedTextureHandle> {
        self.shared_handle.map(|handle| {
            SharedTextureHandle::DirectX(D3D11SharedTexture {
                shared_handle: handle,
                width: self.width,
                height: self.height,
                format: RENDER_TARGET_FORMAT.0 as u32,
            })
        })
    }

    fn acquire_sync(&self, key: u64, timeout_ms: u32) -> anyhow::Result<bool> {
        self.acquire_keyed_mutex(key, timeout_ms)
    }

    fn release_sync(&self, key: u64) -> anyhow::Result<()> {
        self.release_keyed_mutex(key)
    }

    fn supports_sync(&self) -> bool {
        self.keyed_mutex.is_some()
    }

    fn is_double_buffered(&self) -> bool {
        self.double_buffer_enabled
    }
}

impl DrawableOffScreenTarget for DirectXOffScreenTarget {
    fn draw(&mut self, scene: &Scene) {
        if let Err(e) = self.pre_draw() {
            log::error!("Failed to prepare for drawing: {:?}", e);
            return;
        }

        for batch in scene.batches() {
            let result = match batch {
                PrimitiveBatch::Shadows(shadows) => self.draw_shadows(shadows),
                PrimitiveBatch::Quads(quads) => self.draw_quads(quads),
                PrimitiveBatch::Paths(paths) => self.draw_paths(paths),
                PrimitiveBatch::Underlines(underlines) => self.draw_underlines(underlines),
                PrimitiveBatch::MonochromeSprites {
                    texture_id,
                    sprites,
                } => self.draw_monochrome_sprites(texture_id, sprites),
                PrimitiveBatch::PolychromeSprites {
                    texture_id,
                    sprites,
                } => self.draw_polychrome_sprites(texture_id, sprites),
                PrimitiveBatch::Surfaces(_surfaces) => {
                    // Surface rendering not supported in off-screen mode yet
                    Ok(())
                }
            };

            if let Err(e) = result {
                log::error!("Failed to draw batch: {:?}", e);
            }
        }
    }

    fn finish_frame(&mut self) {
        // Flush any pending GPU commands
        unsafe {
            self.devices.device_context.Flush();
        }
    }
}

// Drawing implementations
impl DirectXOffScreenTarget {
    fn draw_shadows(&mut self, shadows: &[Shadow]) -> Result<()> {
        if shadows.is_empty() {
            return Ok(());
        }

        self.pipelines.shadow_pipeline.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            shadows,
        )?;

        self.pipelines.shadow_pipeline.draw(
            &self.devices.device_context,
            slice::from_ref(&self.viewport),
            slice::from_ref(&self.global_params_buffer),
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            4,
            shadows.len() as u32,
        )
    }

    fn draw_quads(&mut self, quads: &[Quad]) -> Result<()> {
        if quads.is_empty() {
            return Ok(());
        }

        self.pipelines.quad_pipeline.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            quads,
        )?;

        self.pipelines.quad_pipeline.draw(
            &self.devices.device_context,
            slice::from_ref(&self.viewport),
            slice::from_ref(&self.global_params_buffer),
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            4,
            quads.len() as u32,
        )
    }

    fn draw_paths(&mut self, paths: &[Path<ScaledPixels>]) -> Result<()> {
        if paths.is_empty() {
            return Ok(());
        }

        // First pass: render paths to intermediate MSAA texture
        self.draw_paths_to_intermediate(paths)?;

        // Second pass: composite from intermediate texture to main render target
        self.draw_paths_from_intermediate(paths)?;

        Ok(())
    }

    fn draw_paths_to_intermediate(&mut self, paths: &[Path<ScaledPixels>]) -> Result<()> {
        // Clear intermediate MSAA texture
        unsafe {
            self.devices.device_context.ClearRenderTargetView(
                self.path_intermediate_msaa_view
                    .as_ref()
                    .context("missing path intermediate MSAA view")?,
                &[0.0; 4],
            );

            // Set intermediate MSAA texture as render target
            self.devices.device_context.OMSetRenderTargets(
                Some(slice::from_ref(&self.path_intermediate_msaa_view)),
                None,
            );
        }

        // Collect all vertices for a single draw call
        let mut vertices = Vec::new();
        for path in paths {
            vertices.extend(path.vertices.iter().map(|v| PathRasterizationSprite {
                xy_position: v.xy_position,
                st_position: v.st_position,
                color: path.color,
                bounds: path.clipped_bounds(),
            }));
        }

        self.pipelines.path_rasterization_pipeline.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            &vertices,
        )?;

        self.pipelines.path_rasterization_pipeline.draw(
            &self.devices.device_context,
            slice::from_ref(&self.viewport),
            slice::from_ref(&self.global_params_buffer),
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
            vertices.len() as u32,
            1,
        )?;

        // Resolve MSAA to non-MSAA intermediate texture
        unsafe {
            self.devices.device_context.ResolveSubresource(
                &self.path_intermediate_texture,
                0,
                &self.path_intermediate_msaa_texture,
                0,
                RENDER_TARGET_FORMAT,
            );

            // Restore main render target
            self.devices
                .device_context
                .OMSetRenderTargets(Some(slice::from_ref(&self.render_target_view)), None);
        }

        Ok(())
    }

    fn draw_paths_from_intermediate(&mut self, paths: &[Path<ScaledPixels>]) -> Result<()> {
        let Some(first_path) = paths.first() else {
            return Ok(());
        };

        // When copying paths from the intermediate texture to the drawable,
        // each pixel must only be copied once, in case of transparent paths.
        //
        // If all paths have the same draw order, then their bounds are all
        // disjoint, so we can copy each path's bounds individually. If this
        // batch combines different draw orders, we perform a single copy
        // for a minimal spanning rect.
        let sprites = if paths.last().map(|p| p.order) == Some(first_path.order) {
            paths
                .iter()
                .map(|path| PathSprite {
                    bounds: path.clipped_bounds(),
                })
                .collect::<Vec<_>>()
        } else {
            let mut bounds = first_path.clipped_bounds();
            for path in paths.iter().skip(1) {
                bounds = bounds.union(&path.clipped_bounds());
            }
            vec![PathSprite { bounds }]
        };

        self.pipelines.path_sprite_pipeline.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            &sprites,
        )?;

        self.pipelines.path_sprite_pipeline.draw_with_texture(
            &self.devices.device_context,
            slice::from_ref(&self.path_intermediate_srv),
            slice::from_ref(&self.viewport),
            slice::from_ref(&self.global_params_buffer),
            slice::from_ref(&self.sampler),
            sprites.len() as u32,
        )
    }

    fn draw_underlines(&mut self, underlines: &[Underline]) -> Result<()> {
        if underlines.is_empty() {
            return Ok(());
        }

        self.pipelines.underline_pipeline.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            underlines,
        )?;

        self.pipelines.underline_pipeline.draw(
            &self.devices.device_context,
            slice::from_ref(&self.viewport),
            slice::from_ref(&self.global_params_buffer),
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            4,
            underlines.len() as u32,
        )
    }

    fn draw_monochrome_sprites(
        &mut self,
        texture_id: AtlasTextureId,
        sprites: &[MonochromeSprite],
    ) -> Result<()> {
        if sprites.is_empty() {
            return Ok(());
        }

        self.pipelines.mono_sprites.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            sprites,
        )?;

        let texture_view = self.atlas.get_texture_view(texture_id);
        self.pipelines.mono_sprites.draw_with_texture(
            &self.devices.device_context,
            &texture_view,
            slice::from_ref(&self.viewport),
            slice::from_ref(&self.global_params_buffer),
            slice::from_ref(&self.sampler),
            sprites.len() as u32,
        )
    }

    fn draw_polychrome_sprites(
        &mut self,
        texture_id: AtlasTextureId,
        sprites: &[PolychromeSprite],
    ) -> Result<()> {
        if sprites.is_empty() {
            return Ok(());
        }

        self.pipelines.poly_sprites.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            sprites,
        )?;

        let texture_view = self.atlas.get_texture_view(texture_id);
        self.pipelines.poly_sprites.draw_with_texture(
            &self.devices.device_context,
            &texture_view,
            slice::from_ref(&self.viewport),
            slice::from_ref(&self.global_params_buffer),
            slice::from_ref(&self.sampler),
            sprites.len() as u32,
        )
    }
}

impl DirectXOffScreenPipelines {
    fn new(device: &ID3D11Device) -> Result<Self> {
        let blend_state = create_blend_state(device)?;
        let path_raster_blend = create_path_rasterization_blend_state(device)?;
        let path_sprite_blend = create_path_sprite_blend_state(device)?;

        Ok(Self {
            shadow_pipeline: PipelineState::new::<Shadow>(
                device,
                "shadow_pipeline",
                ShaderModule::Shadow,
                4,
                blend_state.clone(),
            )?,
            quad_pipeline: PipelineState::new::<Quad>(
                device,
                "quad_pipeline",
                ShaderModule::Quad,
                64,
                blend_state.clone(),
            )?,
            path_rasterization_pipeline: PipelineState::new::<PathRasterizationSprite>(
                device,
                "path_rasterization_pipeline",
                ShaderModule::PathRasterization,
                32,
                path_raster_blend,
            )?,
            path_sprite_pipeline: PipelineState::new::<PathSprite>(
                device,
                "path_sprite_pipeline",
                ShaderModule::PathSprite,
                4,
                path_sprite_blend,
            )?,
            underline_pipeline: PipelineState::new::<Underline>(
                device,
                "underline_pipeline",
                ShaderModule::Underline,
                4,
                blend_state.clone(),
            )?,
            mono_sprites: PipelineState::new::<MonochromeSprite>(
                device,
                "monochrome_sprite_pipeline",
                ShaderModule::MonochromeSprite,
                512,
                blend_state.clone(),
            )?,
            poly_sprites: PipelineState::new::<PolychromeSprite>(
                device,
                "polychrome_sprite_pipeline",
                ShaderModule::PolychromeSprite,
                16,
                blend_state,
            )?,
        })
    }
}

impl PipelineState {
    fn new<T>(
        device: &ID3D11Device,
        label: &'static str,
        module: ShaderModule,
        initial_capacity: usize,
        blend_state: Option<ID3D11BlendState>,
    ) -> Result<Self> {
        let vs_bytes = RawShaderBytes::new(module, ShaderTarget::Vertex)?;
        let ps_bytes = RawShaderBytes::new(module, ShaderTarget::Fragment)?;

        let vertex_shader = unsafe {
            let mut shader = None;
            device.CreateVertexShader(vs_bytes.as_bytes(), None, Some(&mut shader))?;
            shader.context("Failed to create vertex shader")?
        };

        let pixel_shader = unsafe {
            let mut shader = None;
            device.CreatePixelShader(ps_bytes.as_bytes(), None, Some(&mut shader))?;
            shader.context("Failed to create pixel shader")?
        };

        let element_size = std::mem::size_of::<T>();
        let buffer = create_buffer(device, element_size, initial_capacity)?;
        let buffer_view = create_buffer_view(device, &buffer)?;

        Ok(Self {
            label,
            vertex_shader,
            pixel_shader,
            buffer: Some(buffer),
            buffer_view,
            blend_state,
            buffer_capacity: initial_capacity,
            element_size,
        })
    }

    fn update_buffer<T>(
        &mut self,
        device: &ID3D11Device,
        device_context: &ID3D11DeviceContext,
        data: &[T],
    ) -> Result<()> {
        if self.buffer_capacity < data.len() {
            let new_capacity = data.len().next_power_of_two();
            log::info!(
                "Updating {} buffer capacity from {} to {}",
                self.label,
                self.buffer_capacity,
                new_capacity
            );
            let buffer = create_buffer(device, self.element_size, new_capacity)?;
            let view = create_buffer_view(device, &buffer)?;
            self.buffer = Some(buffer);
            self.buffer_view = view;
            self.buffer_capacity = new_capacity;
        }

        if let Some(ref buffer) = self.buffer {
            update_buffer(device_context, buffer, data)?;
        }

        Ok(())
    }

    fn draw(
        &self,
        device_context: &ID3D11DeviceContext,
        viewport: &[D3D11_VIEWPORT],
        global_params: &[Option<ID3D11Buffer>],
        topology: D3D_PRIMITIVE_TOPOLOGY,
        vertex_count: u32,
        instance_count: u32,
    ) -> Result<()> {
        self.set_pipeline_state(device_context, viewport, global_params, topology);

        unsafe {
            device_context.DrawInstanced(vertex_count, instance_count, 0, 0);
        }

        Ok(())
    }

    fn draw_with_texture(
        &self,
        device_context: &ID3D11DeviceContext,
        texture: &[Option<ID3D11ShaderResourceView>],
        viewport: &[D3D11_VIEWPORT],
        global_params: &[Option<ID3D11Buffer>],
        sampler: &[Option<ID3D11SamplerState>],
        instance_count: u32,
    ) -> Result<()> {
        self.set_pipeline_state(
            device_context,
            viewport,
            global_params,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
        );

        unsafe {
            device_context.PSSetSamplers(0, Some(sampler));
            device_context.VSSetShaderResources(0, Some(texture));
            device_context.PSSetShaderResources(0, Some(texture));
            device_context.DrawInstanced(4, instance_count, 0, 0);
        }

        Ok(())
    }

    fn set_pipeline_state(
        &self,
        device_context: &ID3D11DeviceContext,
        viewport: &[D3D11_VIEWPORT],
        global_params: &[Option<ID3D11Buffer>],
        topology: D3D_PRIMITIVE_TOPOLOGY,
    ) {
        unsafe {
            device_context.VSSetShaderResources(1, Some(slice::from_ref(&self.buffer_view)));
            device_context.PSSetShaderResources(1, Some(slice::from_ref(&self.buffer_view)));
            device_context.IASetPrimitiveTopology(topology);
            device_context.RSSetViewports(Some(viewport));
            device_context.VSSetShader(&self.vertex_shader, None);
            device_context.PSSetShader(&self.pixel_shader, None);
            device_context.VSSetConstantBuffers(0, Some(global_params));
            device_context.PSSetConstantBuffers(0, Some(global_params));
            if let Some(ref blend_state) = self.blend_state {
                device_context.OMSetBlendState(blend_state, None, 0xFFFFFFFF);
            }
        }
    }
}

// Helper functions

fn create_render_texture(
    device: &ID3D11Device,
    width: u32,
    height: u32,
    enable_sharing: bool,
) -> Result<(
    ID3D11Texture2D,
    Option<ID3D11RenderTargetView>,
    Option<windows::Win32::Foundation::HANDLE>,
    Option<IDXGIKeyedMutex>,
)> {
    let mut misc_flags = 0u32;
    if enable_sharing {
        // Use SHARED_KEYEDMUTEX for proper synchronization between processes
        misc_flags |= D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX.0 as u32;
    }

    let desc = D3D11_TEXTURE2D_DESC {
        Width: width,
        Height: height,
        MipLevels: 1,
        ArraySize: 1,
        Format: RENDER_TARGET_FORMAT,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_RENDER_TARGET.0 as u32 | D3D11_BIND_SHADER_RESOURCE.0 as u32,
        CPUAccessFlags: 0,
        MiscFlags: misc_flags,
    };

    let mut texture = None;
    unsafe { device.CreateTexture2D(&desc, None, Some(&mut texture)) }
        .context("Failed to create render texture")?;
    let texture = texture.context("Render texture is None")?;

    // Create render target view
    let mut rtv = None;
    unsafe { device.CreateRenderTargetView(&texture, None, Some(&mut rtv)) }
        .context("Failed to create render target view")?;

    // Get shared handle and keyed mutex if sharing is enabled
    let (shared_handle, keyed_mutex) = if enable_sharing {
        let resource: IDXGIResource = texture.cast().context("Failed to cast to IDXGIResource")?;
        let handle =
            unsafe { resource.GetSharedHandle() }.context("Failed to get shared handle")?;

        // Get the keyed mutex interface for synchronization
        let mutex: IDXGIKeyedMutex = texture
            .cast()
            .context("Failed to cast texture to IDXGIKeyedMutex")?;

        (Some(handle), Some(mutex))
    } else {
        (None, None)
    };

    Ok((texture, rtv, shared_handle, keyed_mutex))
}

fn create_staging_texture(
    device: &ID3D11Device,
    width: u32,
    height: u32,
) -> Result<ID3D11Texture2D> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: width,
        Height: height,
        MipLevels: 1,
        ArraySize: 1,
        Format: RENDER_TARGET_FORMAT,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: D3D11_USAGE_STAGING,
        BindFlags: 0,
        CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
        MiscFlags: 0,
    };

    let mut texture = None;
    unsafe { device.CreateTexture2D(&desc, None, Some(&mut texture)) }
        .context("Failed to create staging texture")?;
    texture.context("Staging texture is None")
}

fn create_path_intermediate_texture(
    device: &ID3D11Device,
    width: u32,
    height: u32,
) -> Result<(ID3D11Texture2D, Option<ID3D11ShaderResourceView>)> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: width,
        Height: height,
        MipLevels: 1,
        ArraySize: 1,
        Format: RENDER_TARGET_FORMAT,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
        CPUAccessFlags: 0,
        MiscFlags: 0,
    };

    let mut texture = None;
    unsafe { device.CreateTexture2D(&desc, None, Some(&mut texture)) }
        .context("Failed to create path intermediate texture")?;
    let texture = texture.context("Path intermediate texture is None")?;

    let mut srv = None;
    unsafe { device.CreateShaderResourceView(&texture, None, Some(&mut srv)) }
        .context("Failed to create path intermediate SRV")?;

    Ok((texture, srv))
}

fn create_path_intermediate_msaa_texture(
    device: &ID3D11Device,
    width: u32,
    height: u32,
) -> Result<(ID3D11Texture2D, Option<ID3D11RenderTargetView>)> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: width,
        Height: height,
        MipLevels: 1,
        ArraySize: 1,
        Format: RENDER_TARGET_FORMAT,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: PATH_MULTISAMPLE_COUNT,
            Quality: 0,
        },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_RENDER_TARGET.0 as u32,
        CPUAccessFlags: 0,
        MiscFlags: 0,
    };

    let mut texture = None;
    unsafe { device.CreateTexture2D(&desc, None, Some(&mut texture)) }
        .context("Failed to create path intermediate MSAA texture")?;
    let texture = texture.context("Path intermediate MSAA texture is None")?;

    let mut rtv = None;
    unsafe { device.CreateRenderTargetView(&texture, None, Some(&mut rtv)) }
        .context("Failed to create path intermediate MSAA RTV")?;

    Ok((texture, rtv))
}

fn create_constant_buffer(device: &ID3D11Device) -> Result<Option<ID3D11Buffer>> {
    let desc = D3D11_BUFFER_DESC {
        ByteWidth: std::mem::size_of::<GlobalParams>() as u32,
        Usage: D3D11_USAGE_DYNAMIC,
        BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
        CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
        MiscFlags: 0,
        StructureByteStride: 0,
    };

    let mut buffer = None;
    unsafe { device.CreateBuffer(&desc, None, Some(&mut buffer)) }
        .context("Failed to create constant buffer")?;
    Ok(buffer)
}

fn create_sampler(device: &ID3D11Device) -> Result<Option<ID3D11SamplerState>> {
    let desc = D3D11_SAMPLER_DESC {
        Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
        AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
        AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
        AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
        MipLODBias: 0.0,
        MaxAnisotropy: 1,
        ComparisonFunc: D3D11_COMPARISON_NEVER,
        BorderColor: [0.0; 4],
        MinLOD: 0.0,
        MaxLOD: D3D11_FLOAT32_MAX,
    };

    let mut sampler = None;
    unsafe { device.CreateSamplerState(&desc, Some(&mut sampler)) }
        .context("Failed to create sampler state")?;
    Ok(sampler)
}

fn create_blend_state(device: &ID3D11Device) -> Result<Option<ID3D11BlendState>> {
    let mut desc = D3D11_BLEND_DESC::default();
    desc.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
        BlendEnable: true.into(),
        SrcBlend: D3D11_BLEND_ONE,
        DestBlend: D3D11_BLEND_INV_SRC_ALPHA,
        BlendOp: D3D11_BLEND_OP_ADD,
        SrcBlendAlpha: D3D11_BLEND_ONE,
        DestBlendAlpha: D3D11_BLEND_INV_SRC_ALPHA,
        BlendOpAlpha: D3D11_BLEND_OP_ADD,
        RenderTargetWriteMask: D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8,
    };

    let mut state = None;
    unsafe { device.CreateBlendState(&desc, Some(&mut state)) }
        .context("Failed to create blend state")?;
    Ok(state)
}

fn create_path_rasterization_blend_state(
    device: &ID3D11Device,
) -> Result<Option<ID3D11BlendState>> {
    let mut desc = D3D11_BLEND_DESC::default();
    desc.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
        BlendEnable: true.into(),
        SrcBlend: D3D11_BLEND_ONE,
        DestBlend: D3D11_BLEND_ONE,
        BlendOp: D3D11_BLEND_OP_ADD,
        SrcBlendAlpha: D3D11_BLEND_ONE,
        DestBlendAlpha: D3D11_BLEND_ONE,
        BlendOpAlpha: D3D11_BLEND_OP_ADD,
        RenderTargetWriteMask: D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8,
    };

    let mut state = None;
    unsafe { device.CreateBlendState(&desc, Some(&mut state)) }
        .context("Failed to create path rasterization blend state")?;
    Ok(state)
}

fn create_path_sprite_blend_state(device: &ID3D11Device) -> Result<Option<ID3D11BlendState>> {
    let mut desc = D3D11_BLEND_DESC::default();
    desc.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
        BlendEnable: true.into(),
        SrcBlend: D3D11_BLEND_ONE,
        DestBlend: D3D11_BLEND_INV_SRC_ALPHA,
        BlendOp: D3D11_BLEND_OP_ADD,
        SrcBlendAlpha: D3D11_BLEND_ONE,
        DestBlendAlpha: D3D11_BLEND_INV_SRC_ALPHA,
        BlendOpAlpha: D3D11_BLEND_OP_ADD,
        RenderTargetWriteMask: D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8,
    };

    let mut state = None;
    unsafe { device.CreateBlendState(&desc, Some(&mut state)) }
        .context("Failed to create path sprite blend state")?;
    Ok(state)
}

fn set_rasterizer_state(device: &ID3D11Device, context: &ID3D11DeviceContext) -> Result<()> {
    let desc = D3D11_RASTERIZER_DESC {
        FillMode: D3D11_FILL_SOLID,
        CullMode: D3D11_CULL_NONE,
        FrontCounterClockwise: false.into(),
        DepthBias: 0,
        DepthBiasClamp: 0.0,
        SlopeScaledDepthBias: 0.0,
        DepthClipEnable: false.into(),
        ScissorEnable: false.into(),
        MultisampleEnable: false.into(),
        AntialiasedLineEnable: false.into(),
    };

    let mut state = None;
    unsafe {
        device.CreateRasterizerState(&desc, Some(&mut state))?;
        if let Some(ref state) = state {
            context.RSSetState(state);
        }
    }

    Ok(())
}

fn create_buffer(
    device: &ID3D11Device,
    element_size: usize,
    capacity: usize,
) -> Result<ID3D11Buffer> {
    let desc = D3D11_BUFFER_DESC {
        ByteWidth: (element_size * capacity) as u32,
        Usage: D3D11_USAGE_DYNAMIC,
        BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
        CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
        MiscFlags: D3D11_RESOURCE_MISC_BUFFER_STRUCTURED.0 as u32,
        StructureByteStride: element_size as u32,
    };

    let mut buffer = None;
    unsafe { device.CreateBuffer(&desc, None, Some(&mut buffer)) }
        .context("Failed to create buffer")?;
    buffer.context("Buffer is None")
}

fn create_buffer_view(
    device: &ID3D11Device,
    buffer: &ID3D11Buffer,
) -> Result<Option<ID3D11ShaderResourceView>> {
    let mut view = None;
    unsafe { device.CreateShaderResourceView(buffer, None, Some(&mut view)) }
        .context("Failed to create buffer view")?;
    Ok(view)
}

fn update_buffer<T>(
    context: &ID3D11DeviceContext,
    buffer: &ID3D11Buffer,
    data: &[T],
) -> Result<()> {
    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
    unsafe {
        context.Map(buffer, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped))?;
        std::ptr::copy_nonoverlapping(data.as_ptr(), mapped.pData as *mut T, data.len());
        context.Unmap(buffer, 0);
    }
    Ok(())
}

fn get_font_info() -> &'static FontInfo {
    use std::sync::OnceLock;
    static FONT_INFO: OnceLock<FontInfo> = OnceLock::new();
    FONT_INFO.get_or_init(|| {
        // Default font info - in production this would come from DirectWrite
        FontInfo {
            gamma_ratios: [1.0, 1.0, 1.0, 1.0],
            grayscale_enhanced_contrast: 1.0,
        }
    })
}

// Ensure the target is Send + Sync as required by the trait
unsafe impl Send for DirectXOffScreenTarget {}
unsafe impl Sync for DirectXOffScreenTarget {}
