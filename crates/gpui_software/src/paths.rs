use std::{cell::RefCell, sync::OnceLock};

use gpui::{BackgroundTag, Path, ScaledPixels};
use vello_cpu::{
    Pixmap, RenderContext, Resources,
    color::{AlphaColor, PremulRgba8, Srgb},
    kurbo::{Affine, BezPath},
};

use crate::{
    kernels,
    lower::{IRect, gradient_affine, gradient_lut, pack_hsla},
};

pub(crate) struct PreparedPath<'a> {
    source: &'a Path<ScaledPixels>,
    prepared: OnceLock<PathPaint>,
}

struct PathPaint {
    bezier: BezPath,
    color: u32,
    gradient: Option<([u32; 256], (f32, f32, f32))>,
}

impl<'a> PreparedPath<'a> {
    pub fn new(source: &'a Path<ScaledPixels>) -> Self {
        Self {
            source,
            prepared: OnceLock::new(),
        }
    }

    fn paint(&self) -> &PathPaint {
        self.prepared.get_or_init(|| {
            let gradient = (self.source.color.tag() == BackgroundTag::LinearGradient).then(|| {
                (
                    gradient_lut(self.source.color),
                    gradient_affine(self.source.color, self.source.bounds),
                )
            });
            PathPaint {
                bezier: to_bezier_path(self.source),
                color: if gradient.is_some() {
                    0xffff_ffff
                } else {
                    pack_hsla(self.source.color.solid())
                },
                gradient,
            }
        })
    }
}

struct PathScratch {
    context: RenderContext,
    resources: Resources,
    pixmap: Pixmap,
}

thread_local! {
    // Each renderer cell is at most 64 by 32 pixels; retain bounded scratch on each worker.
    static SCRATCH: RefCell<PathScratch> = RefCell::new(PathScratch {
        context: RenderContext::new(64, 32),
        resources: Resources::new(),
        pixmap: Pixmap::new(64, 32),
    });
}

pub(crate) fn rasterize_path(
    destination: &mut [u32],
    stride: usize,
    band_y: usize,
    rect: IRect,
    path: &PreparedPath<'_>,
) {
    let Ok(width) = u16::try_from(rect.width()) else {
        return;
    };
    let Ok(height) = u16::try_from(rect.height()) else {
        return;
    };
    if width == 0 || height == 0 {
        return;
    }
    let paint = path.paint();
    if paint.bezier.is_empty() {
        return;
    }
    SCRATCH.with_borrow_mut(|scratch| {
        let PathScratch {
            context,
            resources,
            pixmap,
        } = scratch;
        context.reset_and_resize(width, height);
        pixmap.resize(width, height);
        pixmap.data_mut().fill(PremulRgba8::from_u32(0));
        let color = paint.color;
        context.set_transform(Affine::translate((
            f64::from(-rect.x0),
            f64::from(-rect.y0),
        )));
        context.set_paint(AlphaColor::<Srgb>::from_rgba8(
            ((color >> 16) & 0xff) as u8,
            ((color >> 8) & 0xff) as u8,
            (color & 0xff) as u8,
            (color >> 24) as u8,
        ));
        context.fill_path(&paint.bezier);
        context.flush();
        context.render(&mut *pixmap, resources);

        for (source_row, y) in pixmap.data().chunks(width as usize).zip(rect.y0..rect.y1) {
            let destination_start = (y as usize - band_y) * stride + rect.x0 as usize;
            let destination_row =
                &mut destination[destination_start..destination_start + width as usize];
            for ((destination, source), x) in destination_row
                .iter_mut()
                .zip(source_row)
                .zip(rect.x0..rect.x1)
            {
                if source.a == 0 {
                    continue;
                }
                if let Some((lut, (t0, dt_dx, dt_dy))) = &paint.gradient {
                    let t = t0 + (x as f32 + 0.5) * dt_dx + (y as f32 + 0.5) * dt_dy;
                    let color = lut[(t.clamp(0.0, 1.0) * 255.0).round() as usize];
                    let alpha = ((color >> 24) * u32::from(source.a) + 127) / 255;
                    *destination = kernels::blend_pixel(*destination, color, alpha as u8);
                    continue;
                }
                *destination = blend_premultiplied(*destination, *source);
            }
        }
    });
}

fn blend_premultiplied(destination: u32, source: PremulRgba8) -> u32 {
    let blend = |channel: u8, shift: u32| {
        let background = (destination >> shift) & 255;
        (u32::from(channel) + (background * (255 - u32::from(source.a)) + 127) / 255).min(255)
    };
    0xff00_0000 | (blend(source.r, 16) << 16) | (blend(source.g, 8) << 8) | blend(source.b, 0)
}

fn to_bezier_path(path: &Path<ScaledPixels>) -> BezPath {
    let mut bezier = BezPath::new();
    for vertices in path.vertices.chunks_exact(3) {
        let point = |index: usize| {
            (
                f64::from(vertices[index].xy_position.x.0),
                f64::from(vertices[index].xy_position.y.0),
            )
        };
        let points = [point(0), point(1), point(2)];
        let curved = vertices[0].st_position.x == 0.0
            && vertices[0].st_position.y == 0.0
            && vertices[1].st_position.x == 0.5
            && vertices[1].st_position.y == 0.0
            && vertices[2].st_position.x == 1.0
            && vertices[2].st_position.y == 1.0;
        let signed_area = (points[1].0 - points[0].0) * (points[2].1 - points[0].1)
            - (points[1].1 - points[0].1) * (points[2].0 - points[0].0);
        if curved && signed_area < 0.0 {
            bezier.move_to(points[2]);
            bezier.quad_to(points[1], points[0]);
        } else {
            bezier.move_to(points[0]);
            if curved {
                bezier.quad_to(points[1], points[2]);
            } else if signed_area < 0.0 {
                bezier.line_to(points[2]);
                bezier.line_to(points[1]);
            } else {
                bezier.line_to(points[1]);
                bezier.line_to(points[2]);
            }
        }
        bezier.close_path();
    }
    bezier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn premultiplied_compositing_matches_source_over() {
        for alpha in 0..=255u32 {
            for channel in 0..=alpha {
                for background in 0..=255u32 {
                    let source = PremulRgba8 {
                        r: channel as u8,
                        g: channel as u8,
                        b: channel as u8,
                        a: alpha as u8,
                    };
                    let actual = blend_premultiplied(background * 0x010101, source) & 255;
                    let expected = (f64::from(channel)
                        + f64::from(background) * (1.0 - f64::from(alpha) / 255.0))
                        .round() as u32;
                    assert_eq!(actual, expected);
                    let straight = (channel * 255 + alpha / 2)
                        .checked_div(alpha)
                        .unwrap_or(0)
                        .min(255);
                    let previous = kernels::blend_pixel(background, straight, alpha as u8) & 255;
                    assert!(
                        actual.abs_diff(previous) <= 2,
                        "alpha {alpha}, channel {channel}, background {background}"
                    );
                }
            }
        }
    }
}
