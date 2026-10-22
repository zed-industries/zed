use crate::scene::{PaintOperation, PrimitiveKind};
use crate::{
    App, Bounds, Corners, Overflow, Pixels, ScaledPixels, Scene, Size, Style, Window, point, size,
};
use std::ops::Range;

/// The curved padding edge, in device pixels. Adjacent borders can have
/// different widths, so the inner corner is generally an ellipse.
#[derive(Clone, Copy, Debug, PartialEq)]
#[expect(missing_docs)]
pub struct RoundedClip {
    pub bounds: Bounds<ScaledPixels>,
    pub radii: Corners<Size<ScaledPixels>>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum ClipOperation {
    Begin(RoundedClip),
    End,
}

impl RoundedClip {
    /// Three float4 records consumed by the platform fragment shaders.
    pub fn gpu_data(&self) -> [[f32; 4]; 3] {
        let b = self.bounds;
        let r = self.radii;
        [
            [b.origin.x.0, b.origin.y.0, b.right().0, b.bottom().0],
            [
                r.top_left.width.0,
                r.top_right.width.0,
                r.bottom_right.width.0,
                r.bottom_left.width.0,
            ],
            [
                r.top_left.height.0,
                r.top_right.height.0,
                r.bottom_right.height.0,
                r.bottom_left.height.0,
            ],
        ]
    }

    fn for_style(style: &Style, bounds: Bounds<Pixels>, rem: Pixels, scale: f32) -> Option<Self> {
        // CSS curves overflow's padding edge only when both axes clip.
        if style.overflow.x == Overflow::Visible || style.overflow.y == Overflow::Visible {
            return None;
        }
        // Match the radii used to paint this element's background and border.
        let radii = style
            .corner_radii
            .to_pixels(rem)
            .clamp_radii_for_quad_size(bounds.size);
        if radii.max() <= Pixels::ZERO {
            return None;
        }
        let widths = style.border_widths.to_pixels(rem);
        let inner = |radius: Pixels, horizontal: Pixels, vertical: Pixels| {
            size(
                ScaledPixels(f32::from((radius - horizontal).max(Pixels::ZERO)) * scale),
                ScaledPixels(f32::from((radius - vertical).max(Pixels::ZERO)) * scale),
            )
        };
        Some(Self {
            bounds: Bounds {
                origin: (bounds.origin + point(widths.left, widths.top)).scale(scale),
                size: size(
                    (bounds.size.width - widths.left - widths.right).max(Pixels::ZERO),
                    (bounds.size.height - widths.top - widths.bottom).max(Pixels::ZERO),
                )
                .scale(scale),
            },
            radii: Corners {
                top_left: inner(radii.top_left, widths.left, widths.top),
                top_right: inner(radii.top_right, widths.right, widths.top),
                bottom_right: inner(radii.bottom_right, widths.right, widths.bottom),
                bottom_left: inner(radii.bottom_left, widths.left, widths.bottom),
            },
        })
    }

    /// CPU reference for testing the geometric intersection independently of
    /// antialiasing. Check every corner: an unusually thick opposite border
    /// can cut through a corner's quarter ellipse.
    #[cfg(test)]
    fn contains(&self, position: crate::Point<ScaledPixels>) -> bool {
        let b = self.bounds;
        if b.is_empty() || !b.contains(&position) {
            return false;
        }
        let corners = [
            (b.origin, self.radii.top_left, 1., 1.),
            (b.top_right(), self.radii.top_right, -1., 1.),
            (b.bottom_right(), self.radii.bottom_right, -1., -1.),
            (b.bottom_left(), self.radii.bottom_left, 1., -1.),
        ];
        corners.into_iter().all(|(corner, radius, dx, dy)| {
            let rx = radius.width.0;
            let ry = radius.height.0;
            if rx == 0. || ry == 0. {
                return true;
            }
            let x = (position.x.0 - corner.x.0) * dx;
            let y = (position.y.0 - corner.y.0) * dy;
            x >= rx || y >= ry || ((x - rx) / rx).powi(2) + ((y - ry) / ry).powi(2) <= 1.
        })
    }
}

/// Rounded overflow uses independent padding edges instead of merging radii
/// into ContentMask: cropping a rectangle must not move an ancestor's corners.
#[derive(Default)]
pub(crate) struct RoundedClips {
    pub(crate) stacks: Vec<Vec<RoundedClip>>,
    primitives: [Vec<usize>; 8],
}

impl RoundedClips {
    pub(crate) fn clear(&mut self) {
        self.stacks.clear();
        for indices in &mut self.primitives {
            indices.clear();
        }
    }

    pub(crate) fn compile(&mut self, operations: &[PaintOperation]) {
        self.clear();
        if !operations
            .iter()
            .any(|operation| matches!(operation, PaintOperation::RoundedClip(_)))
        {
            return;
        }
        self.stacks.push(Vec::new());
        let mut active = Vec::new();
        let mut current_clip = Some(0);
        let mut indices: [Vec<((u32, crate::TileId), usize)>; 8] = Default::default();
        for operation in operations {
            match operation {
                PaintOperation::RoundedClip(ClipOperation::Begin(shape)) => {
                    active.push(*shape);
                    current_clip = None;
                }
                PaintOperation::RoundedClip(ClipOperation::End) => {
                    active.pop();
                    current_clip = None;
                }
                PaintOperation::Primitive(primitive) => {
                    let clip = *current_clip.get_or_insert_with(|| {
                        let mut shapes = Vec::new();
                        for shape in &active {
                            if !shapes.contains(shape) {
                                shapes.push(*shape);
                            }
                        }
                        self.stacks
                            .iter()
                            .position(|stack| *stack == shapes)
                            .unwrap_or_else(|| {
                                self.stacks.push(shapes);
                                self.stacks.len() - 1
                            })
                    });
                    let (kind, order, tile) = match primitive {
                        crate::Primitive::Shadow(p) => {
                            (PrimitiveKind::Shadow, p.order, crate::TileId(0))
                        }
                        crate::Primitive::Quad(p) => {
                            (PrimitiveKind::Quad, p.order, crate::TileId(0))
                        }
                        crate::Primitive::Path(p) => {
                            (PrimitiveKind::Path, p.order, crate::TileId(0))
                        }
                        crate::Primitive::Underline(p) => {
                            (PrimitiveKind::Underline, p.order, crate::TileId(0))
                        }
                        crate::Primitive::MonochromeSprite(p) => {
                            (PrimitiveKind::MonochromeSprite, p.order, p.tile.tile_id)
                        }
                        crate::Primitive::SubpixelSprite(p) => {
                            (PrimitiveKind::SubpixelSprite, p.order, p.tile.tile_id)
                        }
                        crate::Primitive::PolychromeSprite(p) => {
                            (PrimitiveKind::PolychromeSprite, p.order, p.tile.tile_id)
                        }
                        crate::Primitive::Surface(p) => {
                            (PrimitiveKind::Surface, p.order, crate::TileId(0))
                        }
                    };
                    indices[kind as usize].push(((order, tile), clip));
                }
                _ => {}
            }
        }
        debug_assert!(active.is_empty(), "balanced rounded clip scopes");
        // Match the stable sorts in Scene::finish, including atlas tile order.
        // Side tables keep the GPU instance ABI and ContentMask unchanged.
        for (target, mut entries) in self.primitives.iter_mut().zip(indices) {
            entries.sort_by_key(|entry| entry.0);
            target.extend(entries.into_iter().map(|entry| entry.1));
        }
    }

    pub(crate) fn split<'a>(
        &'a self,
        batch: crate::PrimitiveBatch,
    ) -> impl Iterator<Item = (crate::PrimitiveBatch, usize)> + 'a {
        use crate::PrimitiveBatch;
        let (kind, mut range) = match &batch {
            PrimitiveBatch::Shadows(r) => (0, r.clone()),
            PrimitiveBatch::Quads(r) => (1, r.clone()),
            PrimitiveBatch::Paths(r) => (2, r.clone()),
            PrimitiveBatch::Underlines(r) => (3, r.clone()),
            PrimitiveBatch::MonochromeSprites { range, .. } => (4, range.clone()),
            PrimitiveBatch::SubpixelSprites { range, .. } => (5, range.clone()),
            PrimitiveBatch::PolychromeSprites { range, .. } => (6, range.clone()),
            PrimitiveBatch::Surfaces(r) => (7, r.clone()),
        };
        std::iter::from_fn(move || {
            if range.is_empty() {
                return None;
            }
            let start = range.start;
            let clip = self.primitives[kind].get(start).copied().unwrap_or(0);
            let end = if self.stacks.is_empty() {
                range.end
            } else {
                (start + 1..range.end)
                    .find(|index| self.primitives[kind][*index] != clip)
                    .unwrap_or(range.end)
            };
            range.start = end;
            let range: Range<usize> = start..end;
            let part = match &batch {
                PrimitiveBatch::Shadows(_) => PrimitiveBatch::Shadows(range),
                PrimitiveBatch::Quads(_) => PrimitiveBatch::Quads(range),
                PrimitiveBatch::Paths(_) => PrimitiveBatch::Paths(range),
                PrimitiveBatch::Underlines(_) => PrimitiveBatch::Underlines(range),
                PrimitiveBatch::MonochromeSprites { texture_id, .. } => {
                    PrimitiveBatch::MonochromeSprites {
                        texture_id: *texture_id,
                        range,
                    }
                }
                PrimitiveBatch::SubpixelSprites { texture_id, .. } => {
                    PrimitiveBatch::SubpixelSprites {
                        texture_id: *texture_id,
                        range,
                    }
                }
                PrimitiveBatch::PolychromeSprites { texture_id, .. } => {
                    PrimitiveBatch::PolychromeSprites {
                        texture_id: *texture_id,
                        range,
                    }
                }
                PrimitiveBatch::Surfaces(_) => PrimitiveBatch::Surfaces(range),
            };
            Some((part, clip))
        })
    }
}

pub(crate) fn paint_children(
    style: &Style,
    bounds: Bounds<Pixels>,
    window: &mut Window,
    cx: &mut App,
    paint: impl FnOnce(&mut Window, &mut App),
) {
    let Some(shape) =
        RoundedClip::for_style(style, bounds, window.rem_size(), window.scale_factor())
    else {
        return paint(window, cx);
    };
    window
        .next_frame
        .scene
        .paint_operations
        .push(PaintOperation::RoundedClip(ClipOperation::Begin(shape)));
    paint(window, cx);
    window
        .next_frame
        .scene
        .paint_operations
        .push(PaintOperation::RoundedClip(ClipOperation::End));
}

impl Scene {
    /// Batches split where the independently owned rounded overflow stack changes.
    pub fn batches_with_clips(&self) -> impl Iterator<Item = (crate::PrimitiveBatch, usize)> + '_ {
        self.batches()
            .flat_map(|batch| self.rounded_clips.split(batch))
    }

    /// Independent padding edges for each clip stack referenced by a batch.
    pub fn rounded_clip_stacks(&self) -> &[Vec<RoundedClip>] {
        &self.rounded_clips.stacks
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Window {
    /// The completed scene for platform renderer pixel tests.
    pub fn painted_scene(&self) -> &Scene {
        &self.rendered_frame.scene
    }
}

#[cfg(test)]
mod tests {
    use super::{ClipOperation, RoundedClip as Shape};
    use crate::{
        Bounds, Corners, Edges, Overflow, Quad, ScaledPixels, Scene, Style, point, px, size,
    };
    fn style(radius: f32) -> Style {
        Style {
            overflow: point(Overflow::Hidden, Overflow::Hidden),
            corner_radii: Corners::all(px(radius)).map(|radius| (*radius).into()),
            ..Default::default()
        }
    }
    fn shape() -> Shape {
        Shape::for_style(
            &style(24.),
            Bounds::new(point(px(0.), px(0.)), size(px(64.), px(64.))),
            px(16.),
            1.,
        )
        .unwrap()
    }
    fn at(x: f32, y: f32) -> crate::Point<ScaledPixels> {
        point(ScaledPixels(x), ScaledPixels(y))
    }
    fn quad() -> Quad {
        let bounds = shape().bounds;
        Quad {
            bounds,
            content_mask: crate::ContentMask { bounds },
            ..Default::default()
        }
    }
    fn begin(scene: &mut Scene) {
        scene
            .paint_operations
            .push(crate::scene::PaintOperation::RoundedClip(
                ClipOperation::Begin(shape()),
            ));
    }
    fn end(scene: &mut Scene) {
        scene
            .paint_operations
            .push(crate::scene::PaintOperation::RoundedClip(
                ClipOperation::End,
            ));
    }
    #[test]
    fn rounded_clip_preserves_original_geometry_under_rectangular_cropping() {
        let mut shape = shape();
        shape.bounds.size.height = ScaledPixels(200.);
        let crop = Bounds::new(at(0., 0.), size(ScaledPixels(64.), ScaledPixels(50.)));
        assert!(!shape.contains(at(1., 1.)));
        assert!(shape.contains(at(1., 49.)) && crop.contains(&at(1., 49.)));
        assert!(!crop.contains(&at(32., 51.)));
    }

    #[test]
    fn rounded_clip_does_not_give_an_inset_rect_an_ancestor_radius() {
        let parent = shape();
        let inset = Bounds::new(at(16., 16.), size(ScaledPixels(32.), ScaledPixels(32.)));
        assert!(parent.contains(at(17., 17.)) && inset.contains(&at(17., 17.)));
        assert!(
            !Shape {
                bounds: inset,
                ..parent
            }
            .contains(at(17., 17.))
        );
    }

    #[test]
    fn transparent_unequal_borders_produce_an_elliptical_padding_edge() {
        let mut style = style(24.);
        style.border_widths = Edges {
            left: px(8.).into(),
            top: px(4.).into(),
            right: px(2.).into(),
            bottom: px(6.).into(),
        };
        let shape = Shape::for_style(
            &style,
            Bounds::new(point(px(0.), px(0.)), size(px(64.), px(64.))),
            px(16.),
            2.,
        )
        .unwrap();
        assert_eq!(shape.bounds.origin, at(16., 8.));
        assert_eq!(
            shape.radii.top_left,
            size(ScaledPixels(32.), ScaledPixels(40.))
        );
        assert_eq!(
            shape.radii.bottom_right,
            size(ScaledPixels(44.), ScaledPixels(36.))
        );
        assert!(!shape.contains(at(17., 9.)));
        assert!(shape.contains(at(48., 9.)));
    }

    #[test]
    fn visible_axis_and_square_corners_need_no_rounded_clip() {
        let bounds = Bounds::new(point(px(0.), px(0.)), size(px(64.), px(64.)));
        assert!(Shape::for_style(&style(0.), bounds, px(16.), 1.).is_none());
        let mut style = style(24.);
        style.overflow.x = Overflow::Visible;
        assert!(Shape::for_style(&style, bounds, px(16.), 1.).is_none());
    }

    #[test]
    fn sorted_batches_restore_sibling_clip_ownership() {
        let mut scene = Scene::default();
        begin(&mut scene);
        scene.insert_primitive(quad());
        end(&mut scene);
        scene.insert_primitive(quad());
        begin(&mut scene);
        begin(&mut scene);
        scene.insert_primitive(quad());
        end(&mut scene);
        end(&mut scene);
        scene.finish();
        let clips: Vec<_> = scene.batches_with_clips().map(|(_, clip)| clip).collect();
        assert_eq!(clips, vec![1, 0, 1]);
        assert_eq!(scene.rounded_clip_stacks()[1], vec![shape()]);
    }
    #[test]
    fn replayed_child_uses_current_ancestor_clip() {
        let mut source = Scene::default();
        begin(&mut source);
        let start = source.len();
        source.insert_primitive(quad());
        let end_index = source.len();
        end(&mut source);
        source.finish();
        let mut target = Scene::default();
        target.replay(start..end_index, &source);
        target.finish();
        assert!(target.rounded_clip_stacks().is_empty());
        assert!(target.batches_with_clips().all(|(_, clip)| clip == 0));
        begin(&mut target);
        target.replay(start..end_index, &source);
        end(&mut target);
        target.finish();
        assert_eq!(target.rounded_clip_stacks().len(), 2);
    }
    #[test]
    fn scene_replay_preserves_nested_scopes() {
        let mut source = Scene::default();
        begin(&mut source);
        begin(&mut source);
        source.insert_primitive(quad());
        end(&mut source);
        end(&mut source);
        source.finish();
        let mut target = Scene::default();
        target.replay(0..source.len(), &source);
        target.finish();
        assert_eq!(source.rounded_clip_stacks(), target.rounded_clip_stacks());
        target.clear();
        assert!(target.rounded_clip_stacks().is_empty());
    }
    #[test]
    fn atlas_sort_keeps_clip_with_its_primitive() {
        let mut scene = Scene::default();
        let bounds = shape().bounds;
        let sprite = |tile| crate::PolychromeSprite {
            bounds,
            content_mask: crate::ContentMask { bounds },
            tile: crate::AtlasTile {
                texture_id: crate::AtlasTextureId {
                    index: 0,
                    kind: crate::AtlasTextureKind::Polychrome,
                },
                tile_id: crate::TileId(tile),
                padding: 0,
                bounds: Default::default(),
            },
            order: 0,
            pad: 0,
            grayscale: false.into(),
            opacity: 1.,
            corner_radii: Default::default(),
        };
        scene.push_layer(bounds);
        begin(&mut scene);
        scene.insert_primitive(sprite(2));
        end(&mut scene);
        scene.insert_primitive(sprite(1));
        scene.pop_layer();
        scene.finish();
        assert_eq!(scene.polychrome_sprites[0].tile.tile_id, crate::TileId(1));
        assert_eq!(
            scene
                .batches_with_clips()
                .map(|(_, clip)| clip)
                .collect::<Vec<_>>(),
            vec![0, 1]
        );
    }
}
