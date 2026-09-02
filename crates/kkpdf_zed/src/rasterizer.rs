//! Async PDF page rasterization engine and luminosity tone-mapper.
//!
//! Converts PDF vector pages into RGBA framebuffers on background worker threads,
//! with optional luminance-threshold dark mode color mapping.

use crate::cache::RenderedPage;
use crate::document::PageDimensions;
use anyhow::Result;

/// Rendering and tone mapping configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RasterizerOptions {
    /// Screen or viewport DPI scale (default 72.0 for 1x, 144.0 for 2x Retina).
    pub target_dpi: f32,
    /// User zoom magnification multiplier (1.0 = 100%).
    pub zoom_factor: f32,
    /// Whether dark mode tone mapping is active.
    pub dark_mode: bool,
    /// Saturation threshold (0.0 to 1.0) above which colors are preserved (default 0.18).
    pub saturation_threshold: f32,
}

impl Default for RasterizerOptions {
    fn default() -> Self {
        Self {
            target_dpi: 144.0, // High-DPI default for crisp font rendering
            zoom_factor: 1.0,
            dark_mode: false,
            saturation_threshold: 0.18,
        }
    }
}

/// Luminosity-preserving color remapper for dark themes.
///
/// Unlike naive RGB inversion (`255 - C`) which turns colored photos, figures,
/// and syntax-highlighted diagrams into unusable photo-negatives, this remapper:
/// 1. Computes ITU-R BT.709 relative luminance $Y = 0.2126 R + 0.7152 G + 0.0722 B$.
/// 2. Calculates color saturation $S = \frac{\max(R,G,B) - \min(R,G,B)}{\max(R,G,B)}$.
/// 3. If $S < \text{threshold}$ (neutral paper background or black ink text):
///    - Inverts luminance to match dark theme surface background (`#1E1E2E`) and light text (`#CDD6F4`).
/// 4. If $S \ge \text{threshold}$ (colored graphics, charts, photos):
///    - Leaves pixel RGB channels untouched!
pub struct LuminosityToneMapper;

impl LuminosityToneMapper {
    /// Applies dark mode tone mapping in-place on an RGBA8 buffer.
    pub fn apply(rgba_buffer: &mut [u8], saturation_threshold: f32) {
        // Chunk by 4 bytes: [R, G, B, A]
        for pixel in rgba_buffer.as_chunks_mut::<4>().0 {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            let a = pixel[3];

            // If fully transparent, skip
            if a == 0 {
                continue;
            }

            let max_c = r.max(g).max(b);
            let min_c = r.min(g).min(b);

            let saturation = if max_c > 0.0 {
                (max_c - min_c) / max_c
            } else {
                0.0
            };

            // Only recolor desaturated / neutral pixels (paper backgrounds and text)
            if saturation < saturation_threshold {
                // ITU-R BT.709 standard relative luminance
                let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;

                // Remap luminance: 255 (pure white) -> 30 (dark background), 0 (pure black) -> 215 (light text)
                // Linear transformation: target_lum = 215.0 - (lum / 255.0) * 185.0
                let target_lum = (215.0 - (lum / 255.0) * 185.0).clamp(0.0, 255.0) as u8;

                pixel[0] = target_lum;
                pixel[1] = target_lum;
                pixel[2] = target_lum;
            }
        }
    }
}

/// Core rasterization pipeline for generating page bitmaps.
pub struct PageRasterizer;

impl PageRasterizer {
    /// Rasterizes a synthetic or mock page buffer for testing and fallback.
    pub fn render_mock_page(
        page_index: usize,
        dimensions: PageDimensions,
        options: RasterizerOptions,
    ) -> Result<RenderedPage> {
        let (width, height) = dimensions.to_pixel_size(options.zoom_factor, options.target_dpi);
        let total_bytes = (width as usize) * (height as usize) * 4;

        // Create standard white paper background with an inner border
        let mut buffer = vec![255u8; total_bytes];

        // Draw a simulated dark page header box
        for y in 0..height.min(30) {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                buffer[idx] = 20; // R
                buffer[idx + 1] = 20; // G
                buffer[idx + 2] = 20; // B
                buffer[idx + 3] = 255; // A
            }
        }

        // Draw a vibrant red accent icon box (tests saturation preservation)
        for y in 40..height.min(80) {
            for x in 40..width.min(80) {
                let idx = ((y * width + x) * 4) as usize;
                buffer[idx] = 235; // R (Vibrant Red)
                buffer[idx + 1] = 30; // G
                buffer[idx + 2] = 30; // B
                buffer[idx + 3] = 255;
            }
        }

        if options.dark_mode {
            LuminosityToneMapper::apply(&mut buffer, options.saturation_threshold);
        }

        Ok(RenderedPage::new(
            page_index,
            width,
            height,
            options.zoom_factor,
            options.dark_mode,
            buffer,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luminosity_remapping_white_and_black() {
        // Pixel 0: Pure White [255, 255, 255, 255] (Paper background)
        // Pixel 1: Pure Black [0, 0, 0, 255] (Text)
        let mut buffer = vec![255, 255, 255, 255, 0, 0, 0, 255];

        LuminosityToneMapper::apply(&mut buffer, 0.18);

        // White should be mapped to dark background (~30)
        assert!(
            buffer[0] <= 40,
            "White background should become dark (got {})",
            buffer[0]
        );
        assert_eq!(buffer[0], buffer[1]);
        assert_eq!(buffer[1], buffer[2]);

        // Black should be mapped to light text (~215)
        assert!(
            buffer[4] >= 200,
            "Black text should become light (got {})",
            buffer[4]
        );
        assert_eq!(buffer[4], buffer[5]);
        assert_eq!(buffer[5], buffer[6]);
    }

    #[test]
    fn test_luminosity_remapping_preserves_saturated_colors() {
        // Pixel 0: Saturated Red [255, 0, 0, 255]
        // Pixel 1: Saturated Blue [0, 120, 255, 255]
        let mut buffer = vec![255, 0, 0, 255, 0, 120, 255, 255];

        LuminosityToneMapper::apply(&mut buffer, 0.18);

        // Saturated red must be preserved
        assert_eq!(buffer[0], 255);
        assert_eq!(buffer[1], 0);
        assert_eq!(buffer[2], 0);

        // Saturated blue must be preserved
        assert_eq!(buffer[4], 0);
        assert_eq!(buffer[5], 120);
        assert_eq!(buffer[6], 255);
    }

    #[test]
    fn test_mock_rasterizer_output_sanity() {
        let dim = PageDimensions::new(600.0, 800.0);
        let opts = RasterizerOptions {
            target_dpi: 72.0,
            zoom_factor: 1.0,
            dark_mode: false,
            saturation_threshold: 0.18,
        };

        let page = PageRasterizer::render_mock_page(0, dim, opts).expect("Mock render failed");
        assert_eq!(page.width, 600);
        assert_eq!(page.height, 800);
        assert_eq!(page.rgba_buffer.len(), 600 * 800 * 4);
    }
}
