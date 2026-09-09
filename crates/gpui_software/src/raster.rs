use gpui::AtlasTextureKind;
use rayon::prelude::*;

use crate::{
    atlas::SoftwareAtlasState,
    bin_pass::BinGrid,
    damage::Damage,
    framebuffer::{BAND_HEIGHT, Framebuffer},
    kernels,
    lower::{LoweredFrame, Op},
    paths,
};

pub(crate) fn rasterize(
    framebuffer: &mut Framebuffer,
    frame: &LoweredFrame,
    bins: &BinGrid,
    damage: &Damage,
    atlas: &SoftwareAtlasState,
) {
    let stride = framebuffer.size().width.0.max(0) as usize;
    if stride == 0 {
        return;
    }
    let render_band = |(band, pixels): (usize, &mut [u32])| {
        let band_y = band * BAND_HEIGHT;
        for column in 0..bins.columns() {
            let dirty_index = band * damage.columns + column;
            if !damage
                .dirty_cells
                .get(dirty_index)
                .copied()
                .unwrap_or(false)
            {
                continue;
            }
            let cell = bins.cell(band, column);
            let cell_rect = bins.cell_rect(band, column);
            if !cell.has_opaque_cover {
                kernels::fill_opaque(pixels, stride, band_y, cell_rect, 0xff00_0000);
            }
            for op_index in &cell.ops {
                let Some(op) = frame.ops.get(*op_index as usize) else {
                    continue;
                };
                let rect = op.rect().intersect(cell_rect);
                if rect.is_empty() {
                    continue;
                }
                match op {
                    Op::FillOpaque { color, .. } => {
                        kernels::fill_opaque(pixels, stride, band_y, rect, *color)
                    }
                    Op::FillBlend { color, .. } => {
                        kernels::fill_blend(pixels, stride, band_y, rect, *color)
                    }
                    Op::FillGradient {
                        gradient,
                        t0,
                        dt_dx,
                        dt_dy,
                        ..
                    } => kernels::fill_gradient(
                        pixels,
                        stride,
                        band_y,
                        rect,
                        &frame.cache.gradients[*gradient],
                        *t0,
                        *dt_dx,
                        *dt_dy,
                    ),
                    Op::BlitMono {
                        destination,
                        untransformed,
                        tile,
                        color,
                        lut,
                        inverse,
                        ..
                    } => {
                        if let Some(texture) =
                            atlas.texture_for_tile(*tile, AtlasTextureKind::Monochrome)
                        {
                            kernels::blit_mono(
                                pixels,
                                stride,
                                band_y,
                                rect,
                                *destination,
                                *untransformed,
                                *inverse,
                                texture,
                                (tile.bounds.origin.x.0, tile.bounds.origin.y.0),
                                (tile.bounds.size.width.0, tile.bounds.size.height.0),
                                *color,
                                &frame.cache.luts.mono_luts[*lut],
                            );
                        } else {
                            log::error!(
                                "software renderer referenced an invalid or retired monochrome atlas texture"
                            );
                        }
                    }
                    Op::BlitSubpixel {
                        destination,
                        untransformed,
                        tile,
                        color,
                        lut,
                        inverse,
                        is_bgr,
                        ..
                    } => {
                        if let Some(texture) =
                            atlas.texture_for_tile(*tile, AtlasTextureKind::Subpixel)
                        {
                            kernels::blit_subpixel(
                                pixels,
                                stride,
                                band_y,
                                rect,
                                *destination,
                                *untransformed,
                                *inverse,
                                texture,
                                (tile.bounds.origin.x.0, tile.bounds.origin.y.0),
                                (tile.bounds.size.width.0, tile.bounds.size.height.0),
                                *color,
                                &frame.cache.luts.subpixel_luts[*lut],
                                *is_bgr,
                            );
                        } else {
                            log::error!(
                                "software renderer referenced an invalid or retired subpixel atlas texture"
                            );
                        }
                    }
                    Op::BlitPolychrome {
                        destination,
                        tile,
                        opacity,
                        grayscale,
                        ..
                    } => {
                        if let Some(texture) =
                            atlas.texture_for_tile(*tile, AtlasTextureKind::Polychrome)
                        {
                            kernels::blit_polychrome(
                                pixels,
                                stride,
                                band_y,
                                rect,
                                *destination,
                                texture,
                                (tile.bounds.origin.x.0, tile.bounds.origin.y.0),
                                (tile.bounds.size.width.0, tile.bounds.size.height.0),
                                *opacity,
                                *grayscale,
                            );
                        } else {
                            log::error!(
                                "software renderer referenced an invalid or retired polychrome atlas texture"
                            );
                        }
                    }
                    Op::Path { path, .. } => {
                        paths::rasterize_path(pixels, stride, band_y, rect, &frame.paths[*path])
                    }
                }
            }
        }
    };
    // Waking the worker pool costs more than rasterizing a small edit on the calling thread.
    if damage
        .dirty_cells
        .iter()
        .filter(|dirty| **dirty)
        .take(33)
        .count()
        <= 32
    {
        framebuffer
            .pixels_mut()
            .chunks_mut(stride * BAND_HEIGHT)
            .enumerate()
            .for_each(render_band);
    } else {
        framebuffer
            .pixels_mut()
            .par_chunks_mut(stride * BAND_HEIGHT)
            .enumerate()
            .for_each(render_band);
    }
}
