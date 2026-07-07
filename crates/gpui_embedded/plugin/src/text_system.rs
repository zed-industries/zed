use crate::wit;
use anyhow::{Result, anyhow};
use gpui::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTile, Bounds, DevicePixels, Font, FontId,
    FontMetrics, FontRun, FontStyle, GlyphId, LineLayout, Pixels, PlatformAtlas,
    PlatformTextSystem, Point, RenderGlyphParams, ShapedGlyph, ShapedRun, Size, TextRenderingMode,
    point, px, size,
};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn to_wit_glyph_params(params: &RenderGlyphParams) -> wit::GlyphParams {
    wit::GlyphParams {
        font_id: params.font_id.0 as u32,
        glyph_id: params.glyph_id.0,
        font_size: f32::from(params.font_size),
        subpixel_variant_x: params.subpixel_variant.x,
        subpixel_variant_y: params.subpixel_variant.y,
        scale_factor: params.scale_factor,
        is_emoji: params.is_emoji,
    }
}

fn wit_bounds_to_device(bounds: &wit::DeviceBounds) -> Bounds<DevicePixels> {
    Bounds {
        origin: point(DevicePixels(bounds.origin_x), DevicePixels(bounds.origin_y)),
        size: size(DevicePixels(bounds.width), DevicePixels(bounds.height)),
    }
}

/// Cache key for [`PluginTextSystem::font_id`]. The weight is stored as its raw bits so the key is
/// hashable and comparable without relying on `f32` equality.
type FontKey = (gpui::SharedString, u32, bool);

pub struct PluginTextSystem {
    font_ids: Mutex<HashMap<FontKey, FontId>>,
    metrics: Mutex<HashMap<FontId, FontMetrics>>,
    raster_bounds: Mutex<HashMap<RenderGlyphParams, Bounds<DevicePixels>>>,
}

impl PluginTextSystem {
    pub fn new() -> Self {
        Self {
            font_ids: Mutex::new(HashMap::new()),
            metrics: Mutex::new(HashMap::new()),
            raster_bounds: Mutex::new(HashMap::new()),
        }
    }

    fn cached_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        if let Some(bounds) = lock(&self.raster_bounds).get(params) {
            return Ok(*bounds);
        }
        let device_bounds = wit::glyph_raster_bounds(to_wit_glyph_params(params));
        let bounds = wit_bounds_to_device(&device_bounds);
        lock(&self.raster_bounds).insert(params.clone(), bounds);
        Ok(bounds)
    }
}

impl PlatformTextSystem for PluginTextSystem {
    fn add_fonts(&self, _fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        Ok(())
    }

    fn all_font_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        let italic = font.style == FontStyle::Italic;
        let key = (font.family.clone(), font.weight.0.to_bits(), italic);
        if let Some(font_id) = lock(&self.font_ids).get(&key) {
            return Ok(*font_id);
        }
        let descriptor = wit::FontDescriptor {
            family: font.family.to_string(),
            weight: font.weight.0,
            italic,
        };
        let font_id = FontId(wit::resolve_font(&descriptor) as usize);
        lock(&self.font_ids).insert(key, font_id);
        Ok(font_id)
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        if let Some(metrics) = lock(&self.metrics).get(&font_id) {
            return *metrics;
        }
        let wit_metrics = wit::font_metrics_for(font_id.0 as u32);
        let metrics = FontMetrics {
            units_per_em: wit_metrics.units_per_em,
            ascent: wit_metrics.ascent,
            descent: wit_metrics.descent,
            line_gap: wit_metrics.line_gap,
            underline_position: wit_metrics.underline_position,
            underline_thickness: wit_metrics.underline_thickness,
            cap_height: wit_metrics.cap_height,
            x_height: wit_metrics.x_height,
            bounding_box: Bounds {
                origin: point(
                    wit_metrics.bounding_box.origin.x,
                    wit_metrics.bounding_box.origin.y,
                ),
                size: size(
                    wit_metrics.bounding_box.size.width,
                    wit_metrics.bounding_box.size.height,
                ),
            },
        };
        lock(&self.metrics).insert(font_id, metrics);
        metrics
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        let bounds = wit::typographic_bounds(font_id.0 as u32, glyph_id.0);
        Ok(Bounds {
            origin: point(bounds.origin.x, bounds.origin.y),
            size: size(bounds.size.width, bounds.size.height),
        })
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        let advance = wit::advance(font_id.0 as u32, glyph_id.0);
        Ok(size(advance.width, advance.height))
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        wit::glyph_for_char(font_id.0 as u32, ch).map(GlyphId)
    }

    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        self.cached_raster_bounds(params)
    }

    fn rasterize_glyph(
        &self,
        _params: &RenderGlyphParams,
        _raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        Err(anyhow!("gpui_plugin guests do not rasterize glyphs"))
    }

    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
        let wit_runs: Vec<wit::FontRun> = runs
            .iter()
            .map(|run| wit::FontRun {
                len: run.len as u32,
                font_id: run.font_id.0 as u32,
            })
            .collect();
        let layout = wit::layout_line(text, f32::from(font_size), &wit_runs);
        let shaped_runs = layout
            .runs
            .into_iter()
            .map(|run| ShapedRun {
                font_id: FontId(run.font_id as usize),
                glyphs: run
                    .glyphs
                    .into_iter()
                    .map(|glyph| ShapedGlyph {
                        id: GlyphId(glyph.id),
                        position: point(px(glyph.position.x), px(glyph.position.y)),
                        index: glyph.index as usize,
                        is_emoji: glyph.is_emoji,
                    })
                    .collect(),
            })
            .collect();
        LineLayout {
            font_size: px(layout.font_size),
            width: px(layout.width),
            ascent: px(layout.ascent),
            descent: px(layout.descent),
            runs: shaped_runs,
            len: layout.len as usize,
        }
    }

    fn recommended_rendering_mode(
        &self,
        _font_id: FontId,
        _font_size: Pixels,
    ) -> TextRenderingMode {
        TextRenderingMode::Grayscale
    }
}

/// An alpha mask rasterized in the guest (SVGs), waiting to be tinted and shipped.
struct AlphaMaskTile {
    size: Size<DevicePixels>,
    alpha_bytes: std::sync::Arc<Vec<u8>>,
    /// Payload ids already assigned per tint color (raw HSLA bits).
    tinted_payloads: HashMap<[u32; 4], u64>,
}

/// A full-color bitmap (images) with a stable payload id for the wire.
struct BitmapTile {
    payload_id: u64,
}

/// What a fabricated tile stands for; consulted by the scene serializer.
pub enum TileContent {
    Glyph(RenderGlyphParams, Point<DevicePixels>),
    AlphaMask,
    Bitmap,
}

#[derive(Default)]
struct AtlasState {
    tiles: HashMap<AtlasKey, AtlasTile>,
    glyphs: HashMap<u32, (RenderGlyphParams, Point<DevicePixels>)>,
    alpha_masks: HashMap<u32, AlphaMaskTile>,
    bitmaps: HashMap<u32, BitmapTile>,
    /// Payload bytes not yet shipped to the host, drained per display list.
    pending_payloads: Vec<wit::ImagePayload>,
    next_id: u32,
}

fn next_payload_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    // Global so payload ids never collide across the plugin's windows: the host caches
    // payloads per instance, not per view.
    static NEXT_PAYLOAD_ID: AtomicU64 = AtomicU64::new(1);
    NEXT_PAYLOAD_ID.fetch_add(1, Ordering::Relaxed)
}

/// A sprite atlas that fabricates tiles instead of rasterizing.
///
/// The host owns the real atlas and rasterizer, so the guest never runs the `build` callback that
/// [`PlatformAtlas::get_or_insert_with`] normally invokes. It only needs stable tile identifiers
/// and the glyph's raster bounds; the scene serializer later turns each glyph sprite back into a
/// symbolic `glyph` primitive using the remembered [`RenderGlyphParams`].
pub struct PluginAtlas {
    text_system: std::sync::Arc<PluginTextSystem>,
    state: Mutex<AtlasState>,
}

impl PluginAtlas {
    pub fn new(text_system: std::sync::Arc<PluginTextSystem>) -> Self {
        Self {
            text_system,
            state: Mutex::new(AtlasState::default()),
        }
    }

    /// What kind of content a fabricated tile stands for.
    pub fn tile_content(&self, tile_id: u32) -> Option<TileContent> {
        let state = lock(&self.state);
        if let Some((params, origin)) = state.glyphs.get(&tile_id) {
            Some(TileContent::Glyph(params.clone(), *origin))
        } else if state.alpha_masks.contains_key(&tile_id) {
            Some(TileContent::AlphaMask)
        } else if state.bitmaps.contains_key(&tile_id) {
            Some(TileContent::Bitmap)
        } else {
            None
        }
    }

    /// The wire payload id for a full-color bitmap tile.
    pub fn bitmap_payload(&self, tile_id: u32) -> Option<u64> {
        lock(&self.state).bitmaps.get(&tile_id).map(|tile| tile.payload_id)
    }

    /// The wire payload id for an alpha-mask tile tinted with `color`. The first request per
    /// (tile, color) bakes the tint into a premultiplied BGRA payload for the host.
    pub fn tinted_payload(&self, tile_id: u32, color: gpui::Hsla) -> Option<u64> {
        let mut state = lock(&self.state);
        let mask = state.alpha_masks.get_mut(&tile_id)?;
        let color_key = [
            color.h.to_bits(),
            color.s.to_bits(),
            color.l.to_bits(),
            color.a.to_bits(),
        ];
        if let Some(payload_id) = mask.tinted_payloads.get(&color_key) {
            return Some(*payload_id);
        }
        let payload_id = next_payload_id();
        mask.tinted_payloads.insert(color_key, payload_id);

        let rgba = gpui::Rgba::from(color);
        let mut bytes = Vec::with_capacity(mask.alpha_bytes.len() * 4);
        for mask_alpha in mask.alpha_bytes.iter() {
            let alpha = (*mask_alpha as f32 / 255.0) * rgba.a;
            // Premultiplied BGRA, matching gpui's RenderImage frame layout.
            bytes.push((rgba.b * alpha * 255.0) as u8);
            bytes.push((rgba.g * alpha * 255.0) as u8);
            bytes.push((rgba.r * alpha * 255.0) as u8);
            bytes.push((alpha * 255.0) as u8);
        }
        let size = mask.size;
        state.pending_payloads.push(wit::ImagePayload {
            id: payload_id,
            width: size.width.0 as u32,
            height: size.height.0 as u32,
            bytes,
        });
        Some(payload_id)
    }

    /// Drain the image payloads that still need to reach the host, in insertion order.
    pub fn take_pending_payloads(&self) -> Vec<wit::ImagePayload> {
        std::mem::take(&mut lock(&self.state).pending_payloads)
    }
}

fn fabricate_tile(state: &mut AtlasState, kind: AtlasTextureKind, size: Size<DevicePixels>) -> AtlasTile {
    let sequence = state.next_id;
    state.next_id += 1;
    AtlasTile {
        texture_id: AtlasTextureId {
            index: sequence,
            kind,
        },
        tile_id: gpui::TileId(sequence),
        padding: 0,
        bounds: Bounds {
            origin: point(DevicePixels(0), DevicePixels(0)),
            size,
        },
    }
}

impl PlatformAtlas for PluginAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<Option<(Size<DevicePixels>, Cow<'a, [u8]>)>>,
    ) -> Result<Option<AtlasTile>> {
        if let Some(tile) = lock(&self.state).tiles.get(key) {
            return Ok(Some(*tile));
        }
        match key {
            // Glyphs stay symbolic: the host shapes and rasterizes them, so the build
            // callback (which would call rasterize_glyph) is never invoked.
            AtlasKey::Glyph(params) => {
                let raster_bounds = self.text_system.glyph_raster_bounds(params)?;
                let mut state = lock(&self.state);
                if let Some(tile) = state.tiles.get(key) {
                    return Ok(Some(*tile));
                }
                let kind = if params.is_emoji {
                    AtlasTextureKind::Polychrome
                } else {
                    AtlasTextureKind::Monochrome
                };
                let tile = fabricate_tile(&mut state, kind, raster_bounds.size);
                state.tiles.insert(key.clone(), tile);
                state
                    .glyphs
                    .insert(tile.tile_id.0, (params.clone(), raster_bounds.origin));
                Ok(Some(tile))
            }
            // SVGs are rasterized in the guest (resvg is part of gpui) to an alpha mask;
            // the serializer bakes the sprite's tint color in before shipping.
            AtlasKey::Svg(_) => {
                let Some((size, bytes)) = build()? else {
                    return Ok(None);
                };
                let mut state = lock(&self.state);
                let tile = fabricate_tile(&mut state, AtlasTextureKind::Monochrome, size);
                state.tiles.insert(key.clone(), tile);
                state.alpha_masks.insert(
                    tile.tile_id.0,
                    AlphaMaskTile {
                        size,
                        alpha_bytes: std::sync::Arc::new(bytes.into_owned()),
                        tinted_payloads: HashMap::new(),
                    },
                );
                Ok(Some(tile))
            }
            // Images pass through untouched: the build callback yields the same
            // premultiplied BGRA bytes the host's own atlas upload would use.
            AtlasKey::Image(_) => {
                let Some((size, bytes)) = build()? else {
                    return Ok(None);
                };
                let mut state = lock(&self.state);
                let tile = fabricate_tile(&mut state, AtlasTextureKind::Polychrome, size);
                let payload_id = next_payload_id();
                state.tiles.insert(key.clone(), tile);
                state.pending_payloads.push(wit::ImagePayload {
                    id: payload_id,
                    width: size.width.0 as u32,
                    height: size.height.0 as u32,
                    bytes: bytes.into_owned(),
                });
                state
                    .bitmaps
                    .insert(tile.tile_id.0, BitmapTile { payload_id });
                Ok(Some(tile))
            }
        }
    }

    fn remove(&self, key: &AtlasKey) {
        let mut state = lock(&self.state);
        if let Some(tile) = state.tiles.remove(key) {
            state.glyphs.remove(&tile.tile_id.0);
            state.alpha_masks.remove(&tile.tile_id.0);
            state.bitmaps.remove(&tile.tile_id.0);
        }
    }
}
