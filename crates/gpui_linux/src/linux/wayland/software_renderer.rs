use std::{
    cell::Cell,
    fs::File,
    os::fd::AsFd as _,
    rc::Rc,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};

use anyhow::{Context as _, Result};
use gpui::{Bounds, DevicePixels, GpuSpecs, PlatformAtlas, Scene, Size};
use gpui_software::{FontCorrection, Framebuffer, SoftwareRenderer};
use gpui_wgpu::{CompositorGpuHint, GpuContext, WgpuRenderer, WgpuSurfaceConfig};
use memmap2::{MmapMut, MmapOptions};
use wayland_client::{
    Proxy as _,
    protocol::{wl_buffer, wl_shm, wl_shm_pool, wl_surface},
};

use super::{client::Globals, window::RawWindow};

const BUFFER_COUNT: usize = 3;
const MAX_TEXTURE_SIZE: i32 = 16_384;
const MAX_DAMAGE_RECTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WaylandRendererKind {
    Auto,
    Wgpu,
    Software,
}

impl WaylandRendererKind {
    pub(crate) fn from_environment() -> Self {
        const GPUI_RENDERER: &str = "GPUI_RENDERER";
        match std::env::var(GPUI_RENDERER).as_deref() {
            Ok("software") => {
                log::info!("Using GPUI software renderer because GPUI_RENDERER=software");
                Self::Software
            }
            Ok("wgpu") => {
                log::info!("Using WGPU renderer because GPUI_RENDERER=wgpu");
                Self::Wgpu
            }
            Ok(value) => {
                log::warn!(
                    "Ignoring unsupported {GPUI_RENDERER} value {value:?}; expected software or wgpu"
                );
                Self::Auto
            }
            Err(std::env::VarError::NotUnicode(_)) => {
                log::warn!("Ignoring non-Unicode {GPUI_RENDERER} value");
                Self::Auto
            }
            Err(std::env::VarError::NotPresent) => Self::Auto,
        }
    }
}

pub(crate) enum WaylandRenderer {
    Wgpu(WgpuRenderer),
    Software(WaylandSoftwareRenderer),
}

impl WaylandRenderer {
    pub(crate) fn new(
        renderer_kind: &Rc<Cell<WaylandRendererKind>>,
        gpu_context: GpuContext,
        raw_window: &RawWindow,
        config: WgpuSurfaceConfig,
        compositor_gpu: Option<CompositorGpuHint>,
        globals: &Globals,
    ) -> Result<Self> {
        let size = config.size;
        match renderer_kind.get() {
            WaylandRendererKind::Wgpu => {
                WgpuRenderer::new(gpu_context, raw_window, config, compositor_gpu)
                    .map(Self::Wgpu)
                    .context("Creating WGPU renderer forced by GPUI_RENDERER=wgpu")
            }
            WaylandRendererKind::Software => {
                WaylandSoftwareRenderer::new(size, globals).map(Self::Software)
            }
            WaylandRendererKind::Auto => {
                match WgpuRenderer::new(gpu_context.clone(), raw_window, config, compositor_gpu) {
                    Ok(renderer) if !renderer.gpu_specs().is_software_emulated => {
                        log::info!("Using WGPU renderer with a hardware GPU adapter");
                        renderer_kind.set(WaylandRendererKind::Wgpu);
                        Ok(Self::Wgpu(renderer))
                    }
                    Ok(mut renderer) => {
                        let device_name = renderer.gpu_specs().device_name;
                        log::info!(
                            "Using GPUI software renderer because WGPU selected CPU adapter {device_name:?}"
                        );
                        renderer.destroy();
                        drop(renderer);
                        drop(gpu_context.borrow_mut().take());
                        renderer_kind.set(WaylandRendererKind::Software);
                        WaylandSoftwareRenderer::new(size, globals)
                            .map(Self::Software)
                            .context(
                                "Creating GPUI software renderer after rejecting CPU WGPU adapter",
                            )
                    }
                    Err(wgpu_error) => {
                        log::info!(
                            "Using GPUI software renderer because WGPU initialization failed: {wgpu_error:#}"
                        );
                        drop(gpu_context.borrow_mut().take());
                        renderer_kind.set(WaylandRendererKind::Software);
                        WaylandSoftwareRenderer::new(size, globals)
                            .map(Self::Software)
                            .with_context(|| {
                                format!(
                                    "Creating GPUI software renderer after WGPU failed: {wgpu_error:#}"
                                )
                            })
                    }
                }
            }
        }
    }

    pub(crate) fn draw(&mut self, scene: &Scene, surface: &wl_surface::WlSurface) -> bool {
        match self {
            Self::Wgpu(renderer) => renderer.draw(scene),
            Self::Software(renderer) => match renderer.draw(scene, surface) {
                Ok(presented) => presented,
                Err(error) => {
                    log::error!("Failed to present GPUI software frame: {error:#}");
                    false
                }
            },
        }
    }

    pub(crate) fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        match self {
            Self::Wgpu(renderer) => renderer.sprite_atlas().clone(),
            Self::Software(renderer) => renderer.sprite_atlas(),
        }
    }

    pub(crate) fn gpu_specs(&self) -> GpuSpecs {
        match self {
            Self::Wgpu(renderer) => renderer.gpu_specs(),
            Self::Software(renderer) => renderer.gpu_specs(),
        }
    }

    pub(crate) fn max_texture_size(&self) -> u32 {
        match self {
            Self::Wgpu(renderer) => renderer.max_texture_size(),
            Self::Software(_) => MAX_TEXTURE_SIZE as u32,
        }
    }

    pub(crate) fn update_drawable_size(&mut self, size: Size<DevicePixels>) {
        match self {
            Self::Wgpu(renderer) => renderer.update_drawable_size(size),
            Self::Software(renderer) => {
                if let Err(error) = renderer.resize(size) {
                    log::error!("Failed to resize GPUI software renderer: {error:#}");
                }
            }
        }
    }

    pub(crate) fn set_subpixel_layout(&mut self, is_bgr: bool) {
        match self {
            Self::Wgpu(renderer) => renderer.set_subpixel_layout(is_bgr),
            Self::Software(renderer) => renderer.set_subpixel_layout(is_bgr),
        }
    }

    pub(crate) fn update_transparency(&mut self, transparent: bool) {
        if let Self::Wgpu(renderer) = self {
            renderer.update_transparency(transparent);
        }
    }

    pub(crate) fn supports_subpixel_rendering(&self) -> bool {
        match self {
            Self::Wgpu(renderer) => renderer.supports_dual_source_blending(),
            Self::Software(_) => true,
        }
    }

    pub(crate) fn is_software(&self) -> bool {
        matches!(self, Self::Software(_))
    }

    pub(crate) fn device_lost(&self) -> bool {
        match self {
            Self::Wgpu(renderer) => renderer.device_lost(),
            Self::Software(_) => false,
        }
    }

    pub(crate) fn recover(&mut self, raw_window: &RawWindow) -> Result<()> {
        if let Self::Wgpu(renderer) = self {
            renderer.recover(raw_window)?;
        }
        Ok(())
    }

    pub(crate) fn needs_redraw(&mut self) -> bool {
        match self {
            Self::Wgpu(renderer) => renderer.needs_redraw(),
            Self::Software(_) => false,
        }
    }

    pub(crate) fn destroy(&mut self) {
        match self {
            Self::Wgpu(renderer) => renderer.destroy(),
            Self::Software(renderer) => renderer.destroy(),
        }
    }
}

pub(crate) struct WaylandSoftwareRenderer {
    renderer: SoftwareRenderer,
    font_correction: FontCorrection,
    buffers: Vec<ShmBuffer>,
    presentation_sequence: u64,
    pending_surface_damage: Vec<Bounds<DevicePixels>>,
    globals: Globals,
}

impl WaylandSoftwareRenderer {
    fn new(size: Size<DevicePixels>, globals: &Globals) -> Result<Self> {
        let size = normalized_size(size);
        Ok(Self {
            renderer: SoftwareRenderer::new(size, FontCorrection::default()),
            font_correction: FontCorrection::default(),
            buffers: create_buffers(size, globals)?,
            presentation_sequence: 0,
            pending_surface_damage: Vec::new(),
            globals: globals.clone(),
        })
    }

    fn draw(&mut self, scene: &Scene, surface: &wl_surface::WlSurface) -> Result<bool> {
        let damage = self.renderer.draw(scene, false);
        let size = self.renderer.framebuffer().size();
        for rect in damage.rects {
            append_damage(&mut self.pending_surface_damage, rect, size);
            for buffer in &mut self.buffers {
                append_damage(&mut buffer.pending_damage, rect, size);
            }
        }

        let Some(buffer_index) = self.acquire_buffer() else {
            damage_surface(surface, &[]);
            surface.commit();
            return Ok(false);
        };
        let present_start = Instant::now();
        let buffer = &mut self.buffers[buffer_index];
        if let Err(error) = buffer.copy_damage(self.renderer.framebuffer()) {
            buffer.released.store(true, Ordering::Release);
            return Err(error);
        }

        surface.attach(Some(&buffer.buffer), 0, 0);
        damage_surface(surface, &self.pending_surface_damage);
        surface.commit();
        self.pending_surface_damage.clear();
        self.presentation_sequence = self.presentation_sequence.wrapping_add(1);
        buffer.last_presented = self.presentation_sequence;

        if software_stats_enabled() {
            log::info!(
                "gpui_software: wayland_present={:?} buffer={buffer_index}",
                present_start.elapsed()
            );
        }
        Ok(true)
    }

    fn acquire_buffer(&mut self) -> Option<usize> {
        let newest = newest_released_buffer(self.buffers.iter().map(|buffer| {
            (
                buffer.released.load(Ordering::Acquire),
                buffer.last_presented,
            )
        }));
        if let Some(index) = newest
            && self.buffers[index]
                .released
                .compare_exchange(true, false, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        {
            return Some(index);
        }
        if self.buffers.len() < BUFFER_COUNT {
            match ShmBuffer::new(self.renderer.framebuffer().size(), &self.globals) {
                Ok(buffer) => {
                    buffer.released.store(false, Ordering::Release);
                    let index = self.buffers.len();
                    self.buffers.push(buffer);
                    return Some(index);
                }
                Err(error) => log::error!(
                    "Failed to allocate an additional Wayland software buffer: {error:#}"
                ),
            }
        }
        None
    }

    fn resize(&mut self, size: Size<DevicePixels>) -> Result<()> {
        let size = normalized_size(size);
        if self.renderer.framebuffer().size() == size {
            return Ok(());
        }
        let buffers = create_buffers(size, &self.globals)?;
        self.renderer.resize(size);
        self.buffers = buffers;
        self.presentation_sequence = 0;
        self.pending_surface_damage.clear();
        Ok(())
    }

    fn set_subpixel_layout(&mut self, is_bgr: bool) {
        if self.font_correction.is_bgr != is_bgr {
            self.font_correction.is_bgr = is_bgr;
            self.renderer.set_font_correction(self.font_correction);
        }
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.renderer.atlas()
    }

    fn gpu_specs(&self) -> GpuSpecs {
        self.renderer.gpu_specs()
    }

    fn destroy(&mut self) {
        self.buffers.clear();
    }
}

fn damage_surface(surface: &wl_surface::WlSurface, damage: &[Bounds<DevicePixels>]) {
    if surface.version() >= wl_surface::REQ_DAMAGE_BUFFER_SINCE {
        if damage.is_empty() {
            surface.damage_buffer(0, 0, 1, 1);
        } else {
            for rect in damage {
                surface.damage_buffer(
                    rect.origin.x.0,
                    rect.origin.y.0,
                    rect.size.width.0,
                    rect.size.height.0,
                );
            }
        }
    } else if damage.is_empty() {
        surface.damage(0, 0, 1, 1);
    } else {
        surface.damage(0, 0, i32::MAX, i32::MAX);
    }
}

struct ShmBuffer {
    buffer: wl_buffer::WlBuffer,
    pool: wl_shm_pool::WlShmPool,
    _file: File,
    mapping: MmapMut,
    released: Arc<AtomicBool>,
    pending_damage: Vec<Bounds<DevicePixels>>,
    last_presented: u64,
}

impl ShmBuffer {
    fn new(size: Size<DevicePixels>, globals: &Globals) -> Result<Self> {
        let width = size.width.0;
        let height = size.height.0;
        let stride = width
            .checked_mul(4)
            .context("Wayland software buffer stride overflow")?;
        let byte_length = stride
            .checked_mul(height)
            .context("Wayland software buffer size overflow")?;
        let file = tempfile::tempfile().context("Creating Wayland shared-memory file")?;
        file.set_len(byte_length as u64)
            .context("Sizing Wayland shared-memory file")?;
        // The file remains alive and unchanged in size for the lifetime of the mapping.
        let mapping = unsafe { MmapOptions::new().len(byte_length as usize).map_mut(&file) }
            .context("Mapping Wayland shared-memory file")?;
        let pool = globals
            .shm
            .create_pool(file.as_fd(), byte_length, &globals.qh, ());
        let released = Arc::new(AtomicBool::new(true));
        let buffer = pool.create_buffer(
            0,
            width,
            height,
            stride,
            wl_shm::Format::Xrgb8888,
            &globals.qh,
            released.clone(),
        );
        Ok(Self {
            buffer,
            pool,
            _file: file,
            mapping,
            released,
            pending_damage: vec![Bounds::new(Default::default(), size)],
            last_presented: 0,
        })
    }

    fn copy_damage(&mut self, framebuffer: &Framebuffer) -> Result<()> {
        let size = framebuffer.size();
        let width = usize::try_from(size.width.0).context("Negative software framebuffer width")?;
        let height =
            usize::try_from(size.height.0).context("Negative software framebuffer height")?;
        let stride = width
            .checked_mul(4)
            .context("Software framebuffer stride overflow")?;
        let source = bytemuck::cast_slice(framebuffer.pixels());
        for rect in &self.pending_damage {
            let left = (rect.origin.x.0.max(0) as usize).min(width);
            let top = (rect.origin.y.0.max(0) as usize).min(height);
            let right =
                (rect.origin.x.0.saturating_add(rect.size.width.0).max(0) as usize).min(width);
            let bottom =
                (rect.origin.y.0.saturating_add(rect.size.height.0).max(0) as usize).min(height);
            if left >= right || top >= bottom {
                continue;
            }
            let byte_left = left.checked_mul(4).context("Damage offset overflow")?;
            let byte_right = right.checked_mul(4).context("Damage offset overflow")?;
            for row in top..bottom {
                let row_start = row.checked_mul(stride).context("Damage row overflow")?;
                let start = row_start
                    .checked_add(byte_left)
                    .context("Damage start overflow")?;
                let end = row_start
                    .checked_add(byte_right)
                    .context("Damage end overflow")?;
                let source_row = source
                    .get(start..end)
                    .context("Damage exceeds software framebuffer")?;
                let destination_row = self
                    .mapping
                    .get_mut(start..end)
                    .context("Damage exceeds Wayland shared-memory buffer")?;
                destination_row.copy_from_slice(source_row);
            }
        }
        self.pending_damage.clear();
        Ok(())
    }
}

impl Drop for ShmBuffer {
    fn drop(&mut self) {
        self.buffer.destroy();
        self.pool.destroy();
    }
}

fn normalized_size(size: Size<DevicePixels>) -> Size<DevicePixels> {
    Size {
        width: DevicePixels(size.width.0.clamp(1, MAX_TEXTURE_SIZE)),
        height: DevicePixels(size.height.0.clamp(1, MAX_TEXTURE_SIZE)),
    }
}

fn create_buffers(size: Size<DevicePixels>, globals: &Globals) -> Result<Vec<ShmBuffer>> {
    Ok(vec![ShmBuffer::new(size, globals)?])
}

fn newest_released_buffer(buffers: impl Iterator<Item = (bool, u64)>) -> Option<usize> {
    buffers
        .enumerate()
        .filter(|(_, (released, _))| *released)
        .max_by_key(|(_, (_, last_presented))| *last_presented)
        .map(|(index, _)| index)
}

fn append_damage(
    damage: &mut Vec<Bounds<DevicePixels>>,
    rect: Bounds<DevicePixels>,
    size: Size<DevicePixels>,
) {
    let full = Bounds::new(Default::default(), size);
    if damage.first() == Some(&full) {
        return;
    }
    if rect == full || damage.len() >= MAX_DAMAGE_RECTS {
        damage.clear();
        damage.push(full);
    } else if !damage.contains(&rect) {
        damage.push(rect);
    }
}

fn software_stats_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("GPUI_SOFTWARE_STATS").is_ok_and(|value| value == "1" || value == "true")
    })
}

#[cfg(test)]
mod tests {
    use gpui::{DevicePixels, bounds, point, size};

    use super::{MAX_DAMAGE_RECTS, append_damage, newest_released_buffer};

    #[test]
    fn newest_released_buffer_never_reuses_a_busy_buffer() {
        assert_eq!(
            newest_released_buffer([(false, 9), (true, 8), (true, 3)].into_iter()),
            Some(1)
        );
        assert_eq!(
            newest_released_buffer([(true, 10), (true, 8), (true, 3)].into_iter()),
            Some(0)
        );
        assert_eq!(
            newest_released_buffer([(false, 9), (false, 8)].into_iter()),
            None
        );
        assert_eq!(newest_released_buffer([].into_iter()), None);
    }

    #[test]
    fn damage_collapses_to_the_full_buffer() {
        let framebuffer_size = size(DevicePixels(256), DevicePixels(128));
        let mut damage = Vec::new();
        for x in 0..=MAX_DAMAGE_RECTS {
            append_damage(
                &mut damage,
                bounds(
                    point(DevicePixels(x as i32), DevicePixels(0)),
                    size(DevicePixels(1), DevicePixels(1)),
                ),
                framebuffer_size,
            );
        }
        assert_eq!(
            damage,
            vec![bounds(
                point(DevicePixels(0), DevicePixels(0)),
                framebuffer_size
            )]
        );
    }
}
