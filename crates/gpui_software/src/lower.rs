use std::hash::{Hash, Hasher};

use gpui::{
    AtlasTile, Background, BackgroundTag, Bounds, ColorSpace, DevicePixels, Hsla, Path,
    PrimitiveBatch, Rgba, ScaledPixels, Scene, TransformationMatrix,
};

use crate::text_correction::{FontCorrection, LutCache};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct IRect {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

impl IRect {
    pub fn width(self) -> i32 {
        self.x1 - self.x0
    }

    pub fn height(self) -> i32 {
        self.y1 - self.y0
    }

    pub fn is_empty(self) -> bool {
        self.x1 <= self.x0 || self.y1 <= self.y0
    }

    pub fn intersect(self, other: Self) -> Self {
        Self {
            x0: self.x0.max(other.x0),
            y0: self.y0.max(other.y0),
            x1: self.x1.min(other.x1),
            y1: self.y1.min(other.y1),
        }
    }

    pub fn contains(self, other: Self) -> bool {
        self.x0 <= other.x0 && self.y0 <= other.y0 && self.x1 >= other.x1 && self.y1 >= other.y1
    }
}

#[derive(Clone)]
pub(crate) enum Op {
    FillOpaque {
        rect: IRect,
        color: u32,
    },
    FillBlend {
        rect: IRect,
        color: u32,
    },
    FillGradient {
        rect: IRect,
        gradient: usize,
        content_hash: u64,
        t0: f32,
        dt_dx: f32,
        dt_dy: f32,
    },
    BlitMono {
        rect: IRect,
        destination: IRect,
        untransformed: IRect,
        tile: AtlasTile,
        color: u32,
        lut: usize,
        inverse: Option<[f32; 6]>,
    },
    BlitSubpixel {
        rect: IRect,
        destination: IRect,
        untransformed: IRect,
        tile: AtlasTile,
        color: u32,
        lut: usize,
        inverse: Option<[f32; 6]>,
        is_bgr: bool,
    },
    BlitPolychrome {
        rect: IRect,
        destination: IRect,
        tile: AtlasTile,
        opacity: u8,
        grayscale: bool,
    },
    Path {
        rect: IRect,
        path: usize,
        content_hash: u64,
    },
}

impl Op {
    pub fn rect(&self) -> IRect {
        match self {
            Self::FillOpaque { rect, .. }
            | Self::FillBlend { rect, .. }
            | Self::FillGradient { rect, .. }
            | Self::BlitMono { rect, .. }
            | Self::BlitSubpixel { rect, .. }
            | Self::BlitPolychrome { rect, .. }
            | Self::Path { rect, .. } => *rect,
        }
    }

    pub fn is_opaque_rectangle(&self) -> bool {
        matches!(self, Self::FillOpaque { .. })
    }

    pub fn hash(&self, atlas: &crate::atlas::SoftwareAtlasState) -> u64 {
        let mut hasher = collections::FxHasher::default();
        std::mem::discriminant(self).hash(&mut hasher);
        let rect = self.rect();
        [rect.x0, rect.y0, rect.x1, rect.y1].hash(&mut hasher);
        match self {
            Self::FillOpaque { color, .. } | Self::FillBlend { color, .. } => {
                color.hash(&mut hasher)
            }
            Self::FillGradient {
                content_hash,
                t0,
                dt_dx,
                dt_dy,
                ..
            } => {
                content_hash.hash(&mut hasher);
                t0.to_bits().hash(&mut hasher);
                dt_dx.to_bits().hash(&mut hasher);
                dt_dy.to_bits().hash(&mut hasher);
            }
            Self::BlitMono {
                destination,
                untransformed,
                tile,
                color,
                inverse,
                ..
            }
            | Self::BlitSubpixel {
                destination,
                untransformed,
                tile,
                color,
                inverse,
                ..
            } => {
                hash_rect(*destination, &mut hasher);
                hash_rect(*untransformed, &mut hasher);
                hash_tile(*tile, &mut hasher);
                atlas.texture_generation(tile.texture_id).hash(&mut hasher);
                color.hash(&mut hasher);
                if let Some(inverse) = inverse {
                    for value in inverse {
                        value.to_bits().hash(&mut hasher);
                    }
                }
            }
            Self::BlitPolychrome {
                destination,
                tile,
                opacity,
                grayscale,
                ..
            } => {
                hash_rect(*destination, &mut hasher);
                hash_tile(*tile, &mut hasher);
                atlas.texture_generation(tile.texture_id).hash(&mut hasher);
                opacity.hash(&mut hasher);
                grayscale.hash(&mut hasher);
            }
            Self::Path { content_hash, .. } => {
                content_hash.hash(&mut hasher);
            }
        }
        hasher.finish()
    }
}

fn hash_rect(rect: IRect, hasher: &mut impl Hasher) {
    [rect.x0, rect.y0, rect.x1, rect.y1].hash(hasher);
}

fn hash_tile(tile: AtlasTile, hasher: &mut impl Hasher) {
    tile.texture_id.index.hash(hasher);
    (tile.texture_id.kind as u32).hash(hasher);
    tile.tile_id.0.hash(hasher);
    [
        tile.bounds.origin.x.0,
        tile.bounds.origin.y.0,
        tile.bounds.size.width.0,
        tile.bounds.size.height.0,
    ]
    .hash(hasher);
}

#[derive(Default)]
pub(crate) struct LoweringCache {
    pub spare_ops: Vec<Op>,
    correction: FontCorrection,
    pub luts: LutCache,
    gradient_indices: collections::FxHashMap<[u32; 9], usize>,
    pub gradients: Vec<[u32; 256]>,
    gradient_hashes: Vec<u64>,
}

impl LoweringCache {
    fn gradient(&mut self, background: Background) -> usize {
        let colors = background.colors();
        let mut key = [0; 9];
        key[0] = background.color_space_value() as u32;
        for (color, key) in colors.iter().zip(key[1..].chunks_exact_mut(4)) {
            key.copy_from_slice(&[
                color.color.h.to_bits(),
                color.color.s.to_bits(),
                color.color.l.to_bits(),
                color.color.a.to_bits(),
            ]);
        }
        if let Some(index) = self.gradient_indices.get(&key) {
            return *index;
        }
        let index = self.gradients.len();
        let lut = gradient_lut(background);
        let mut hasher = collections::FxHasher::default();
        lut.hash(&mut hasher);
        self.gradient_hashes.push(hasher.finish());
        self.gradients.push(lut);
        self.gradient_indices.insert(key, index);
        index
    }

    pub fn trim(&mut self) {
        self.luts.trim();
        if self.gradients.len() > 256 {
            self.gradients = Vec::new();
            self.gradient_hashes = Vec::new();
            self.gradient_indices = Default::default();
        }
    }
}

pub(crate) struct LoweredFrame<'a> {
    pub ops: Vec<Op>,
    pub cache: LoweringCache,
    pub paths: Vec<crate::paths::PreparedPath<'a>>,
}

pub(crate) fn lower_scene(
    scene: &Scene,
    correction: FontCorrection,
    mut cache: LoweringCache,
) -> LoweredFrame<'_> {
    if cache.correction != correction {
        cache.luts = LutCache::default();
        cache.correction = correction;
    }
    let mut ops = std::mem::take(&mut cache.spare_ops);
    ops.clear();
    ops.reserve(scene.len());
    let mut lowerer = Lowerer {
        ops,
        cache,
        paths: Vec::new(),
    };
    for batch in scene.batches() {
        match batch {
            PrimitiveBatch::Shadows(_) | PrimitiveBatch::Surfaces(_) => {}
            PrimitiveBatch::Quads(range) => {
                for quad in &scene.quads[range] {
                    lowerer.quad(quad);
                }
            }
            PrimitiveBatch::Paths(range) => {
                for path in &scene.paths[range] {
                    lowerer.path(path);
                }
            }
            PrimitiveBatch::Underlines(range) => {
                for underline in &scene.underlines[range] {
                    let bounds = snapped(underline.bounds);
                    let center = (bounds.y0 + bounds.y1) as f32 * 0.5;
                    let thickness = underline.thickness.0.max(1.0);
                    let rect = IRect {
                        x0: bounds.x0,
                        x1: bounds.x1,
                        y0: (center - thickness * 0.5).round() as i32,
                        y1: (center + thickness * 0.5).round() as i32,
                    }
                    .intersect(snapped(underline.content_mask.bounds));
                    lowerer.fill(rect, pack_hsla(underline.color));
                }
            }
            PrimitiveBatch::MonochromeSprites { range, .. } => {
                for sprite in &scene.monochrome_sprites[range] {
                    let (destination, inverse) =
                        transformed_bounds(sprite.bounds, sprite.transformation);
                    let rect = destination.intersect(snapped(sprite.content_mask.bounds));
                    if rect.is_empty() {
                        continue;
                    }
                    let color = pack_hsla(sprite.color);
                    let lut = lowerer.cache.luts.mono(color, correction);
                    lowerer.ops.push(Op::BlitMono {
                        rect,
                        destination,
                        untransformed: snapped(sprite.bounds),
                        tile: sprite.tile,
                        color,
                        lut,
                        inverse,
                    });
                }
            }
            PrimitiveBatch::SubpixelSprites { range, .. } => {
                for sprite in &scene.subpixel_sprites[range] {
                    let (destination, inverse) =
                        transformed_bounds(sprite.bounds, sprite.transformation);
                    let rect = destination.intersect(snapped(sprite.content_mask.bounds));
                    if rect.is_empty() {
                        continue;
                    }
                    let color = pack_hsla(sprite.color);
                    let lut = lowerer.cache.luts.subpixel(color, correction);
                    lowerer.ops.push(Op::BlitSubpixel {
                        rect,
                        destination,
                        untransformed: snapped(sprite.bounds),
                        tile: sprite.tile,
                        color,
                        lut,
                        inverse,
                        is_bgr: correction.is_bgr,
                    });
                }
            }
            PrimitiveBatch::PolychromeSprites { range, .. } => {
                for sprite in &scene.polychrome_sprites[range] {
                    let destination = snapped(sprite.bounds);
                    let rect = destination.intersect(snapped(sprite.content_mask.bounds));
                    if rect.is_empty() {
                        continue;
                    }
                    lowerer.ops.push(Op::BlitPolychrome {
                        rect,
                        destination,
                        tile: sprite.tile,
                        opacity: (sprite.opacity.clamp(0.0, 1.0) * 255.0).round() as u8,
                        grayscale: sprite.grayscale.as_bool(),
                    });
                }
            }
        }
    }
    LoweredFrame {
        ops: lowerer.ops,
        cache: lowerer.cache,
        paths: lowerer.paths,
    }
}

struct Lowerer<'a> {
    ops: Vec<Op>,
    cache: LoweringCache,
    paths: Vec<crate::paths::PreparedPath<'a>>,
}

impl<'a> Lowerer<'a> {
    fn quad(&mut self, quad: &gpui::Quad) {
        let bounds = snapped(quad.bounds);
        let clip = snapped(quad.content_mask.bounds);
        let clipped_bounds = bounds.intersect(clip);
        if clipped_bounds.is_empty() {
            return;
        }
        match quad.background.tag() {
            BackgroundTag::Solid | BackgroundTag::Checkerboard => {
                self.fill(clipped_bounds, pack_hsla(quad.background.solid()));
            }
            BackgroundTag::PatternSlash => {
                let mut color = Rgba::from(quad.background.solid());
                color.a *= 0.5;
                self.fill(clipped_bounds, pack_rgba(color));
            }
            BackgroundTag::LinearGradient => {
                let (t0, dt_dx, dt_dy) = gradient_affine(quad.background, quad.bounds);
                let gradient = self.cache.gradient(quad.background);
                let content_hash = self.cache.gradient_hashes[gradient];
                self.ops.push(Op::FillGradient {
                    rect: clipped_bounds,
                    gradient,
                    content_hash,
                    t0,
                    dt_dx,
                    dt_dy,
                });
            }
        }

        let border_color = pack_hsla(quad.border_color);
        let top = (quad.border_widths.top.0.round().max(0.0) as i32).min(bounds.height());
        let bottom =
            (quad.border_widths.bottom.0.round().max(0.0) as i32).min(bounds.height() - top);
        let left = (quad.border_widths.left.0.round().max(0.0) as i32).min(bounds.width());
        let right = (quad.border_widths.right.0.round().max(0.0) as i32).min(bounds.width() - left);
        self.fill(
            IRect {
                y1: bounds.y0 + top,
                ..bounds
            }
            .intersect(clip),
            border_color,
        );
        self.fill(
            IRect {
                x0: bounds.x1 - right,
                y0: bounds.y0 + top,
                y1: bounds.y1 - bottom,
                ..bounds
            }
            .intersect(clip),
            border_color,
        );
        self.fill(
            IRect {
                y0: bounds.y1 - bottom,
                ..bounds
            }
            .intersect(clip),
            border_color,
        );
        self.fill(
            IRect {
                x1: bounds.x0 + left,
                y0: bounds.y0 + top,
                y1: bounds.y1 - bottom,
                ..bounds
            }
            .intersect(clip),
            border_color,
        );
    }

    fn path(&mut self, path: &'a Path<ScaledPixels>) {
        let rect = IRect {
            x0: path.bounds.origin.x.0.floor() as i32,
            y0: path.bounds.origin.y.0.floor() as i32,
            x1: path.bounds.bottom_right().x.0.ceil() as i32,
            y1: path.bounds.bottom_right().y.0.ceil() as i32,
        }
        .intersect(snapped(path.content_mask.bounds));
        if rect.is_empty() {
            return;
        }
        let path_index = self.paths.len();
        let content_hash = hash_path(path);
        self.paths.push(crate::paths::PreparedPath::new(path));
        self.ops.push(Op::Path {
            rect,
            path: path_index,
            content_hash,
        });
    }

    fn fill(&mut self, rect: IRect, color: u32) {
        if rect.is_empty() || color >> 24 == 0 {
            return;
        }
        if color >> 24 == 255 {
            self.ops.push(Op::FillOpaque { rect, color });
        } else {
            self.ops.push(Op::FillBlend { rect, color });
        }
    }
}

fn hash_path(path: &Path<ScaledPixels>) -> u64 {
    let mut hasher = collections::FxHasher::default();
    for component in [
        path.bounds.origin.x.0,
        path.bounds.origin.y.0,
        path.bounds.size.width.0,
        path.bounds.size.height.0,
    ] {
        component.to_bits().hash(&mut hasher);
    }
    (path.color.tag() as u32).hash(&mut hasher);
    (path.color.color_space_value() as u32).hash(&mut hasher);
    path.color.solid().h.to_bits().hash(&mut hasher);
    path.color.solid().s.to_bits().hash(&mut hasher);
    path.color.solid().l.to_bits().hash(&mut hasher);
    path.color.solid().a.to_bits().hash(&mut hasher);
    path.color
        .gradient_angle_or_pattern_height()
        .to_bits()
        .hash(&mut hasher);
    for stop in path.color.colors() {
        stop.percentage.to_bits().hash(&mut hasher);
        stop.color.h.to_bits().hash(&mut hasher);
        stop.color.s.to_bits().hash(&mut hasher);
        stop.color.l.to_bits().hash(&mut hasher);
        stop.color.a.to_bits().hash(&mut hasher);
    }
    for vertex in &path.vertices {
        vertex.xy_position.x.0.to_bits().hash(&mut hasher);
        vertex.xy_position.y.0.to_bits().hash(&mut hasher);
        vertex.st_position.x.to_bits().hash(&mut hasher);
        vertex.st_position.y.to_bits().hash(&mut hasher);
    }
    hasher.finish()
}

pub(crate) fn snapped(bounds: Bounds<ScaledPixels>) -> IRect {
    IRect {
        x0: bounds.origin.x.0.round() as i32,
        y0: bounds.origin.y.0.round() as i32,
        x1: (bounds.origin.x.0 + bounds.size.width.0).round() as i32,
        y1: (bounds.origin.y.0 + bounds.size.height.0).round() as i32,
    }
}

fn transformed_bounds(
    bounds: Bounds<ScaledPixels>,
    transformation: TransformationMatrix,
) -> (IRect, Option<[f32; 6]>) {
    if transformation == TransformationMatrix::unit() {
        return (snapped(bounds), None);
    }
    let corners = [
        [bounds.origin.x.0, bounds.origin.y.0],
        [bounds.origin.x.0 + bounds.size.width.0, bounds.origin.y.0],
        [bounds.origin.x.0, bounds.origin.y.0 + bounds.size.height.0],
        [
            bounds.origin.x.0 + bounds.size.width.0,
            bounds.origin.y.0 + bounds.size.height.0,
        ],
    ];
    let transformed = corners.map(|point| {
        [
            transformation.rotation_scale[0][0] * point[0]
                + transformation.rotation_scale[0][1] * point[1]
                + transformation.translation[0],
            transformation.rotation_scale[1][0] * point[0]
                + transformation.rotation_scale[1][1] * point[1]
                + transformation.translation[1],
        ]
    });
    let minimum_x = transformed
        .iter()
        .map(|point| point[0])
        .fold(f32::INFINITY, f32::min);
    let maximum_x = transformed
        .iter()
        .map(|point| point[0])
        .fold(f32::NEG_INFINITY, f32::max);
    let minimum_y = transformed
        .iter()
        .map(|point| point[1])
        .fold(f32::INFINITY, f32::min);
    let maximum_y = transformed
        .iter()
        .map(|point| point[1])
        .fold(f32::NEG_INFINITY, f32::max);
    let matrix = transformation.rotation_scale;
    let determinant = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    if determinant == 0.0 || !determinant.is_finite() {
        return (IRect::default(), None);
    }
    let inverse = {
        let a = matrix[1][1] / determinant;
        let b = -matrix[0][1] / determinant;
        let c = -matrix[1][0] / determinant;
        let d = matrix[0][0] / determinant;
        Some([
            a,
            b,
            c,
            d,
            -(a * transformation.translation[0] + b * transformation.translation[1]),
            -(c * transformation.translation[0] + d * transformation.translation[1]),
        ])
    };
    (
        IRect {
            x0: minimum_x.round() as i32,
            y0: minimum_y.round() as i32,
            x1: maximum_x.round() as i32,
            y1: maximum_y.round() as i32,
        },
        inverse,
    )
}

pub(crate) fn pack_hsla(color: Hsla) -> u32 {
    pack_rgba(Rgba::from(color))
}

pub(crate) fn pack_rgba(color: Rgba) -> u32 {
    let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0 + 0.5) as u32;
    (channel(color.a) << 24) | (channel(color.r) << 16) | (channel(color.g) << 8) | channel(color.b)
}

pub(crate) fn gradient_affine(
    background: Background,
    bounds: Bounds<ScaledPixels>,
) -> (f32, f32, f32) {
    let angle = (background.gradient_angle_or_pattern_height() % 360.0 - 90.0).to_radians();
    let mut direction = [angle.cos(), angle.sin()];
    let width = bounds.size.width.0.max(f32::EPSILON);
    let height = bounds.size.height.0.max(f32::EPSILON);
    if width > height {
        direction[1] *= height / width;
    } else {
        direction[0] *= width / height;
    }
    let length = direction[0].hypot(direction[1]).max(f32::EPSILON);
    let denominator = if direction[0].abs() > direction[1].abs() {
        width
    } else {
        height
    };
    let stops = background.colors();
    let stop_span = stops[1].percentage - stops[0].percentage;
    let stop_span = if stop_span.abs() < f32::EPSILON {
        f32::EPSILON.copysign(stop_span)
    } else {
        stop_span
    };
    let dt_dx = direction[0] / length / denominator / stop_span;
    let dt_dy = direction[1] / length / denominator / stop_span;
    let center_x = bounds.origin.x.0 + width * 0.5;
    let center_y = bounds.origin.y.0 + height * 0.5;
    let t0 =
        0.5 - stops[0].percentage - center_x * dt_dx * stop_span - center_y * dt_dy * stop_span;
    (t0 / stop_span, dt_dx, dt_dy)
}

pub(crate) fn gradient_lut(background: Background) -> [u32; 256] {
    let colors = background.colors();
    let first = Rgba::from(colors[0].color);
    let second = Rgba::from(colors[1].color);
    std::array::from_fn(|index| {
        let t = index as f32 / 255.0;
        let color = match background.color_space_value() {
            ColorSpace::Srgb => interpolate_srgb(first, second, t),
            ColorSpace::Oklab => interpolate_oklab(first, second, t),
        };
        pack_rgba(color)
    })
}

fn interpolate_srgb(first: Rgba, second: Rgba, t: f32) -> Rgba {
    let convert = |value: f32| {
        if value <= 0.003_130_8 {
            value * 12.92
        } else {
            1.055 * value.powf(1.0 / 2.4) - 0.055
        }
    };
    let invert = |value: f32| {
        if value <= 0.04045 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    };
    Rgba {
        r: invert(convert(first.r) * (1.0 - t) + convert(second.r) * t),
        g: invert(convert(first.g) * (1.0 - t) + convert(second.g) * t),
        b: invert(convert(first.b) * (1.0 - t) + convert(second.b) * t),
        a: first.a * (1.0 - t) + second.a * t,
    }
}

fn interpolate_oklab(first: Rgba, second: Rgba, t: f32) -> Rgba {
    fn to_oklab(color: Rgba) -> [f32; 3] {
        let l = (0.412_221_46 * color.r + 0.536_332_55 * color.g + 0.051_445_995 * color.b).cbrt();
        let m = (0.211_903_5 * color.r + 0.680_699_5 * color.g + 0.107_396_96 * color.b).cbrt();
        let s = (0.088_302_46 * color.r + 0.281_718_85 * color.g + 0.629_978_7 * color.b).cbrt();
        [
            0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s,
            1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s,
            0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s,
        ]
    }
    fn from_oklab(color: [f32; 3], alpha: f32) -> Rgba {
        let l = (color[0] + 0.396_337_78 * color[1] + 0.215_803_76 * color[2]).powi(3);
        let m = (color[0] - 0.105_561_346 * color[1] - 0.063_854_17 * color[2]).powi(3);
        let s = (color[0] - 0.089_484_18 * color[1] - 1.291_485_5 * color[2]).powi(3);
        Rgba {
            r: 4.076_741_7 * l - 3.307_711_6 * m + 0.230_969_94 * s,
            g: -1.268_438 * l + 2.609_757_4 * m - 0.341_319_38 * s,
            b: -0.004_196_086_3 * l - 0.703_418_6 * m + 1.707_614_7 * s,
            a: alpha,
        }
    }
    let first_lab = to_oklab(first);
    let second_lab = to_oklab(second);
    from_oklab(
        std::array::from_fn(|channel| first_lab[channel] * (1.0 - t) + second_lab[channel] * t),
        first.a * (1.0 - t) + second.a * t,
    )
}

pub(crate) fn framebuffer_rect(size: gpui::Size<DevicePixels>) -> IRect {
    IRect {
        x0: 0,
        y0: 0,
        x1: size.width.0.max(0),
        y1: size.height.0.max(0),
    }
}

#[cfg(test)]
mod tests {
    use gpui::{
        BorderStyle, ContentMask, Corners, Edges, Quad, Underline, bounds, hsla, point, size,
    };

    use super::*;

    #[test]
    fn lookup_caches_are_bounded_and_invalidate_font_settings() {
        let mut cache = LoweringCache::default();
        for index in 0..300 {
            let background = gpui::linear_gradient(
                90.0,
                gpui::linear_color_stop(hsla(index as f32 / 300.0, 0.6, 0.5, 0.8), 0.0),
                gpui::linear_color_stop(hsla(0.6, 0.8, 0.7, 1.0), 1.0),
            );
            let gradient = cache.gradient(background);
            assert_eq!(cache.gradients[gradient], gradient_lut(background));
            assert_eq!(cache.gradient(background), gradient);
            cache.luts.mono(index, FontCorrection::default());
            cache.luts.subpixel(index, FontCorrection::default());
        }
        cache.trim();
        assert!(cache.gradients.is_empty());
        assert!(cache.luts.mono_luts.is_empty());
        assert!(cache.luts.subpixel_luts.is_empty());
        cache.luts.mono(0xff808080, FontCorrection::default());
        let scene = Scene::default();
        let lowered = lower_scene(
            &scene,
            FontCorrection {
                grayscale_enhanced_contrast: 0.5,
                ..Default::default()
            },
            cache,
        );
        assert!(lowered.cache.luts.mono_luts.is_empty());
    }

    #[test]
    fn lowers_quad_and_underline_to_integer_rectangles() {
        let mut scene = Scene::default();
        let clip = ContentMask {
            bounds: bounds(
                point(0.0.into(), 0.0.into()),
                size(100.0.into(), 100.0.into()),
            ),
        };
        scene.insert_primitive(Quad {
            bounds: bounds(point(1.4.into(), 2.6.into()), size(10.0.into(), 8.0.into())),
            content_mask: clip,
            background: hsla(0.0, 0.0, 1.0, 1.0).into(),
            border_color: hsla(0.0, 0.0, 0.0, 1.0),
            border_widths: Edges {
                top: 1.0.into(),
                right: 1.0.into(),
                bottom: 1.0.into(),
                left: 1.0.into(),
            },
            corner_radii: Corners::default(),
            border_style: BorderStyle::Dashed,
            ..Default::default()
        });
        scene.insert_primitive(Underline {
            order: 0,
            pad: 0,
            bounds: bounds(point(2.0.into(), 20.0.into()), size(8.0.into(), 4.0.into())),
            content_mask: clip,
            color: hsla(0.0, 1.0, 0.5, 1.0),
            thickness: 2.0.into(),
            wavy: true.into(),
        });
        scene.finish();

        let lowered = lower_scene(&scene, FontCorrection::default(), LoweringCache::default());
        assert_eq!(lowered.ops.len(), 6);
        assert_eq!(
            lowered.ops[0].rect(),
            IRect {
                x0: 1,
                y0: 3,
                x1: 11,
                y1: 11
            }
        );
        assert_eq!(
            lowered.ops[5].rect(),
            IRect {
                x0: 2,
                y0: 21,
                x1: 10,
                y1: 23
            }
        );
    }
}
