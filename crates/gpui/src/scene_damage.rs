//! Diffs two consecutive `Scene`s to find a conservative region of the
//! framebuffer that changed, so a backend can redraw and present only that
//! region.
//!
//! # Approach
//!
//! A pixel's final color is fully determined by the subsequence of primitives
//! covering it, applied in draw order. So the diff aligns the two frames'
//! primitive sequences by *content* (a primitive's `order` field is a
//! per-frame counter, not an identity, and is deliberately excluded from
//! matching): any pixel covered only by matched, relative-order-preserving
//! pairs provably didn't change, so damage is the union of the bounds of
//! everything that failed to match. Relative order across primitive types is
//! validated separately in [`damage_reordered_pairs`], since matching runs
//! per type array.
//!
//! Damage may over-report (costing redundant GPU work) but must never
//! under-report (which would leave stale pixels on screen).

use crate::{
    Bounds, MonochromeSprite, PaintSurface, Path, Point, PolychromeSprite, Quad, ScaledPixels,
    Scene, Shadow, Size, SubpixelSprite, TransformationMatrix, Underline,
};
use smallvec::SmallVec;

/// The maximum number of rectangles tracked in [`SceneDamage::Rects`]. Adding
/// more coalesces the closest pair, trading precision for bounded cost.
pub const MAX_DAMAGE_RECTS: usize = 8;

/// The region of the framebuffer that changed between two frames.
#[derive(Clone, Debug)]
pub enum SceneDamage {
    /// Everything must be redrawn (e.g. the previous contents are invalid).
    Full,
    /// Only these rectangles changed, in scaled/device pixels.
    Rects(DamageRects),
    /// Nothing changed; the frame's presentation can be skipped.
    Unchanged,
}

/// A bounded set of damaged rectangles that together cover every changed
/// pixel. Rectangles are pairwise disjoint, so a renderer may draw each
/// region's pixels exactly once (overlap would double-blend translucent
/// content).
#[derive(Clone, Debug, Default)]
pub struct DamageRects(SmallVec<[Bounds<ScaledPixels>; MAX_DAMAGE_RECTS]>);

impl DamageRects {
    /// The damaged rectangles.
    pub fn as_slice(&self) -> &[Bounds<ScaledPixels>] {
        &self.0
    }

    /// Whether any damaged rectangle contains `point`.
    pub fn contains_point(&self, point: &Point<ScaledPixels>) -> bool {
        self.0.iter().any(|rect| rect.contains(point))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Adds a damaged rectangle, keeping the set pairwise disjoint by
    /// absorbing every intersecting rectangle into the new one, and
    /// coalescing with the closest rectangle when the cap is exceeded.
    /// Empty and non-finite rectangles are ignored.
    fn add(&mut self, rect: Bounds<ScaledPixels>) {
        if !(rect.size.width.0 > 0.0 && rect.size.height.0 > 0.0)
            || !rect.origin.x.0.is_finite()
            || !rect.origin.y.0.is_finite()
            || !rect.size.width.0.is_finite()
            || !rect.size.height.0.is_finite()
        {
            return;
        }
        let mut rect = rect;
        loop {
            // Absorb everything the (growing) rectangle intersects. Each
            // union can newly intersect further rectangles, so scan to a
            // fixpoint; the set shrinks every iteration.
            while let Some(index) = self
                .0
                .iter()
                .position(|existing| existing.intersects(&rect))
            {
                rect = self.0.swap_remove(index).union(&rect);
            }
            if self.0.len() < MAX_DAMAGE_RECTS {
                self.0.push(rect);
                return;
            }
            // At the cap: coalesce with the disjoint rectangle whose union
            // wastes the least area, then re-check for new intersections.
            let mut best = (0, f32::INFINITY);
            for (index, existing) in self.0.iter().enumerate() {
                let waste = area(&existing.union(&rect)) - area(existing) - area(&rect);
                if waste < best.1 {
                    best = (index, waste);
                }
            }
            rect = self.0.swap_remove(best.0).union(&rect);
        }
    }
}

fn area(rect: &Bounds<ScaledPixels>) -> f32 {
    rect.size.width.0 * rect.size.height.0
}

/// When set (`GPUI_EXPERIMENTAL_ORDER_TOLERANT_DAMAGE=1`), the scene diff
/// matches primitives by content, ignoring their per-frame `order` values.
/// When unset, `order` values are compared, so any mid-scene insertion
/// cascades damage over everything painted after it - safe but usually
/// close to full-window damage. The diff only runs at all when one of the
/// experimental rendering features (`GPUI_EXPERIMENTAL_PRESENT_SKIP`,
/// `GPUI_EXPERIMENTAL_PARTIAL_RENDER`) is enabled, and they are only
/// worthwhile in combination with this one.
pub(crate) fn order_tolerant_damage() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("GPUI_EXPERIMENTAL_ORDER_TOLERANT_DAMAGE")
            .is_ok_and(|value| value != "0" && !value.is_empty())
    })
}

impl SceneDamage {
    /// Computes a conservative region of `next` that differs from `prev`.
    ///
    /// This may over-report but should not under-report. Both scenes must be
    /// finished (sorted).
    pub fn between(prev: &Scene, next: &Scene) -> SceneDamage {
        Self::between_with_order_tolerance(prev, next, order_tolerant_damage())
    }

    /// Like [`Self::between`], with the order-tolerance mode passed
    /// explicitly rather than read from the environment, so both modes can
    /// be tested deterministically.
    pub fn between_with_order_tolerance(
        prev: &Scene,
        next: &Scene,
        order_tolerant: bool,
    ) -> SceneDamage {
        let mut acc = DamageRects::default();
        let mut matched = Vec::new();
        let strict = !order_tolerant;

        // Kind ranks mirror `PrimitiveKind`'s declaration order, which is how
        // `BatchIterator` breaks draw-order ties across primitive types.
        diff_sequence(
            &prev.shadows,
            &next.shadows,
            0,
            |a, b| shadows_equal(a, b) && (!strict || a.order == b.order),
            shadow_damage_bounds,
            |shadow| shadow.order,
            &mut matched,
            &mut acc,
        );
        diff_sequence(
            &prev.quads,
            &next.quads,
            1,
            |a, b| quads_equal(a, b) && (!strict || a.order == b.order),
            |quad| quad.bounds,
            |quad| quad.order,
            &mut matched,
            &mut acc,
        );
        diff_sequence(
            &prev.paths,
            &next.paths,
            2,
            |a, b| paths_equal(a, b) && (!strict || a.order == b.order),
            path_damage_bounds,
            |path| path.order,
            &mut matched,
            &mut acc,
        );
        diff_sequence(
            &prev.underlines,
            &next.underlines,
            3,
            |a, b| underlines_equal(a, b) && (!strict || a.order == b.order),
            |underline| underline.bounds,
            |underline| underline.order,
            &mut matched,
            &mut acc,
        );
        diff_sequence(
            &prev.monochrome_sprites,
            &next.monochrome_sprites,
            4,
            |a, b| monochrome_sprites_equal(a, b) && (!strict || a.order == b.order),
            |sprite| transformed_bounds(sprite.bounds, &sprite.transformation),
            |sprite| sprite.order,
            &mut matched,
            &mut acc,
        );
        diff_sequence(
            &prev.subpixel_sprites,
            &next.subpixel_sprites,
            5,
            |a, b| subpixel_sprites_equal(a, b) && (!strict || a.order == b.order),
            |sprite| transformed_bounds(sprite.bounds, &sprite.transformation),
            |sprite| sprite.order,
            &mut matched,
            &mut acc,
        );
        diff_sequence(
            &prev.polychrome_sprites,
            &next.polychrome_sprites,
            6,
            |a, b| polychrome_sprites_equal(a, b) && (!strict || a.order == b.order),
            |sprite| sprite.bounds,
            |sprite| sprite.order,
            &mut matched,
            &mut acc,
        );

        damage_reordered_pairs(matched, &mut acc);

        // Surface contents (e.g. video frames) can't be compared cheaply, so
        // they are always treated as damaged in both frames.
        accumulate_surface_damage(&prev.surfaces, &mut acc);
        accumulate_surface_damage(&next.surfaces, &mut acc);

        if acc.is_empty() {
            SceneDamage::Unchanged
        } else {
            SceneDamage::Rects(acc)
        }
    }

    /// Combines two damage regions into one that covers both, used to accumulate
    /// damage across frames that failed or skipped presentation.
    pub fn union(self, other: SceneDamage) -> SceneDamage {
        match (self, other) {
            (SceneDamage::Full, _) | (_, SceneDamage::Full) => SceneDamage::Full,
            (SceneDamage::Unchanged, damage) | (damage, SceneDamage::Unchanged) => damage,
            (SceneDamage::Rects(mut a), SceneDamage::Rects(b)) => {
                for rect in b.0 {
                    a.add(rect);
                }
                SceneDamage::Rects(a)
            }
        }
    }

    /// Whether the damage covers `point`.
    pub fn contains_point(&self, point: &Point<ScaledPixels>) -> bool {
        match self {
            SceneDamage::Full => true,
            SceneDamage::Rects(rects) => rects.contains_point(point),
            SceneDamage::Unchanged => false,
        }
    }
}

// The following comparisons intentionally ignore each primitive's `order`
// field: order values are assigned from a per-frame counter, so they are
// positional labels rather than content, and comparing them across frames
// makes any mid-scene insertion "change" every later primitive. Relative
// ordering between matched primitives is validated separately in
// `damage_reordered_pairs`.
//
// For types whose remaining fields all affect rendered output, equality is
// delegated to the derived `PartialEq` after normalizing `order`, so that new
// fields are automatically included in the comparison.

fn shadows_equal(a: &Shadow, b: &Shadow) -> bool {
    let mut b = *b;
    b.order = a.order;
    *a == b
}

fn underlines_equal(a: &Underline, b: &Underline) -> bool {
    let mut b = *b;
    b.order = a.order;
    *a == b
}

fn monochrome_sprites_equal(a: &MonochromeSprite, b: &MonochromeSprite) -> bool {
    let mut b = *b;
    b.order = a.order;
    *a == b
}

fn subpixel_sprites_equal(a: &SubpixelSprite, b: &SubpixelSprite) -> bool {
    let mut b = *b;
    b.order = a.order;
    *a == b
}

fn polychrome_sprites_equal(a: &PolychromeSprite, b: &PolychromeSprite) -> bool {
    let mut b = *b;
    b.order = a.order;
    *a == b
}

/// Quads compare loosely: a borderless quad's border color and style don't
/// affect any pixel, so differences in them are not damage.
fn quads_equal(a: &Quad, b: &Quad) -> bool {
    a.bounds == b.bounds
        && a.content_mask == b.content_mask
        && a.background == b.background
        && a.corner_radii == b.corner_radii
        && a.border_widths == b.border_widths
        && (!quad_has_border(a)
            || (a.border_style == b.border_style && a.border_color == b.border_color))
}

fn quad_has_border(quad: &Quad) -> bool {
    quad.border_widths.top.0 > 0.0
        || quad.border_widths.right.0 > 0.0
        || quad.border_widths.bottom.0 > 0.0
        || quad.border_widths.left.0 > 0.0
}

/// Paths compare only render-relevant fields: `id` is a per-frame slot index
/// and the remaining private fields are builder state.
fn paths_equal(a: &Path<ScaledPixels>, b: &Path<ScaledPixels>) -> bool {
    a.bounds == b.bounds
        && a.content_mask == b.content_mask
        && a.color == b.color
        && a.vertices == b.vertices
}

fn shadow_damage_bounds(shadow: &Shadow) -> Bounds<ScaledPixels> {
    shadow.bounds.dilate(shadow.blur_radius * 3.0)
}

fn path_damage_bounds(path: &Path<ScaledPixels>) -> Bounds<ScaledPixels> {
    // Paths are copied from a linearly filtered intermediate texture, which can
    // affect one pixel beyond their logical bounds.
    path.bounds.dilate(ScaledPixels(1.0))
}

fn accumulate_surface_damage(surfaces: &[PaintSurface], acc: &mut DamageRects) {
    for surface in surfaces {
        acc.add(surface.bounds.intersect(&surface.content_mask.bounds));
    }
}

/// A content-matched pair of primitives from consecutive frames. Keys encode
/// the primitive's position in each frame's global draw sequence.
struct MatchedPair {
    prev_key: u64,
    cur_key: u64,
    bounds: Bounds<ScaledPixels>,
}

/// Encodes a draw position: primary by `order`, ties broken by primitive kind,
/// mirroring `BatchIterator`'s `(order, kind)` sort.
fn draw_key(order: u32, kind_rank: u64) -> u64 {
    ((order as u64) << 3) | kind_rank
}

/// When a mismatch is found, how many elements may be skipped in total across
/// both sequences to find the next matching run before giving up and damaging
/// everything that remains (the common suffix is protected separately).
const RESYNC_LOOKAHEAD: usize = 64;

/// How many consecutive elements must match to accept a resync point,
/// avoiding coincidental alignments in scenes full of identical primitives.
/// Wrong alignments only over-report damage, so this is a precision knob, not
/// a correctness requirement.
const RESYNC_RUN: usize = 4;

/// Aligns `prev` and `cur` (each sorted by draw order) by content. Elements
/// that pair up contribute to `matched`; everything else is damaged. Matching
/// is monotone, so matched pairs always preserve their relative order within
/// one sequence.
fn diff_sequence<T>(
    prev: &[T],
    cur: &[T],
    kind_rank: u64,
    content_eq: impl Fn(&T, &T) -> bool,
    bounds_of: impl Fn(&T) -> Bounds<ScaledPixels>,
    order_of: impl Fn(&T) -> u32,
    matched: &mut Vec<MatchedPair>,
    acc: &mut DamageRects,
) {
    let mut record_match = |a: &T, b: &T, matched: &mut Vec<MatchedPair>| {
        matched.push(MatchedPair {
            prev_key: draw_key(order_of(a), kind_rank),
            cur_key: draw_key(order_of(b), kind_rank),
            bounds: bounds_of(b),
        });
    };

    // Trim the common suffix first so that a change region larger than the
    // resync lookahead doesn't cascade damage over the unchanged tail.
    let mut prev_end = prev.len();
    let mut cur_end = cur.len();
    while prev_end > 0 && cur_end > 0 && content_eq(&prev[prev_end - 1], &cur[cur_end - 1]) {
        prev_end -= 1;
        cur_end -= 1;
        record_match(&prev[prev_end], &cur[cur_end], matched);
    }
    let prev = &prev[..prev_end];
    let cur = &cur[..cur_end];

    let mut i = 0;
    let mut j = 0;
    loop {
        while i < prev.len() && j < cur.len() && content_eq(&prev[i], &cur[j]) {
            record_match(&prev[i], &cur[j], matched);
            i += 1;
            j += 1;
        }
        if i == prev.len() && j == cur.len() {
            return;
        }

        // Mismatch: find the smallest total skip that realigns the sequences.
        let mut resync = None;
        'search: for total_skip in 1..=RESYNC_LOOKAHEAD {
            for skip_prev in 0..=total_skip {
                let skip_cur = total_skip - skip_prev;
                if i + skip_prev > prev.len() || j + skip_cur > cur.len() {
                    continue;
                }
                if aligned(&prev[i + skip_prev..], &cur[j + skip_cur..], &content_eq) {
                    resync = Some((skip_prev, skip_cur));
                    break 'search;
                }
            }
        }
        match resync {
            Some((skip_prev, skip_cur)) => {
                for primitive in &prev[i..i + skip_prev] {
                    acc.add(bounds_of(primitive));
                }
                for primitive in &cur[j..j + skip_cur] {
                    acc.add(bounds_of(primitive));
                }
                i += skip_prev;
                j += skip_cur;
            }
            None => {
                for primitive in &prev[i..] {
                    acc.add(bounds_of(primitive));
                }
                for primitive in &cur[j..] {
                    acc.add(bounds_of(primitive));
                }
                return;
            }
        }
    }
}

/// Whether the heads of two sequences match well enough to accept a resync.
fn aligned<T>(prev: &[T], cur: &[T], content_eq: &impl Fn(&T, &T) -> bool) -> bool {
    let run = RESYNC_RUN.min(prev.len()).min(cur.len());
    if run == 0 {
        // Aligned only if both sequences are exhausted; otherwise the
        // remaining elements of the longer side still need damaging.
        return prev.is_empty() && cur.is_empty();
    }
    (0..run).all(|k| content_eq(&prev[k], &cur[k]))
}

/// Damages matched pairs whose relative draw order changed between frames.
///
/// Per-type matching preserves relative order within each type array, but a
/// primitive can still move relative to primitives of *other* types (e.g. a
/// quad rising above a sprite), which changes blending wherever they overlap.
/// Scanning all pairs in previous-frame draw order and flagging any whose
/// current-frame position sinks below an already-seen position catches every
/// such inversion; the changed pixels of an inverted pair lie within the
/// overlap of the two primitives, which is contained in the flagged one's
/// bounds.
fn damage_reordered_pairs(mut matched: Vec<MatchedPair>, acc: &mut DamageRects) {
    matched.sort_unstable_by_key(|pair| pair.prev_key);
    let mut running_max = 0;
    for pair in matched {
        if pair.cur_key < running_max {
            acc.add(pair.bounds);
        } else {
            running_max = pair.cur_key;
        }
    }
}

/// Axis-aligned bounds of a sprite after its transformation, so rotated sprites
/// damage their full painted extent.
fn transformed_bounds(
    bounds: Bounds<ScaledPixels>,
    transform: &TransformationMatrix,
) -> Bounds<ScaledPixels> {
    let rotation_scale = transform.rotation_scale;
    let translation = transform.translation;
    let x0 = bounds.origin.x.0;
    let y0 = bounds.origin.y.0;
    let x1 = x0 + bounds.size.width.0;
    let y1 = y0 + bounds.size.height.0;
    let mut min = (f32::MAX, f32::MAX);
    let mut max = (f32::MIN, f32::MIN);
    for (x, y) in [(x0, y0), (x1, y0), (x0, y1), (x1, y1)] {
        // Matches the shader: transpose(rotation_scale) * position + translation.
        let transformed_x = rotation_scale[0][0] * x + rotation_scale[1][0] * y + translation[0];
        let transformed_y = rotation_scale[0][1] * x + rotation_scale[1][1] * y + translation[1];
        min.0 = min.0.min(transformed_x);
        min.1 = min.1.min(transformed_y);
        max.0 = max.0.max(transformed_x);
        max.1 = max.1.max(transformed_y);
    }
    Bounds {
        origin: Point {
            x: ScaledPixels(min.0),
            y: ScaledPixels(min.1),
        },
        size: Size {
            width: ScaledPixels(max.0 - min.0),
            height: ScaledPixels(max.1 - min.1),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Background, ContentMask, Hsla, PaintSurface, Primitive, Quad, Underline, px};

    fn rect(x: f32, y: f32, width: f32, height: f32) -> Bounds<ScaledPixels> {
        Bounds {
            origin: Point {
                x: ScaledPixels(x),
                y: ScaledPixels(y),
            },
            size: Size {
                width: ScaledPixels(width),
                height: ScaledPixels(height),
            },
        }
    }

    fn quad(bounds: Bounds<ScaledPixels>, lightness: f32) -> Quad {
        Quad {
            bounds,
            content_mask: ContentMask {
                bounds: rect(0., 0., 1000., 1000.),
            },
            background: Hsla {
                h: 0.,
                s: 0.,
                l: lightness,
                a: 1.,
            }
            .into(),
            ..Default::default()
        }
    }

    fn quad_with_order(bounds: Bounds<ScaledPixels>, lightness: f32, order: u32) -> Quad {
        Quad {
            order,
            ..quad(bounds, lightness)
        }
    }

    fn scene_of(quads: &[Quad]) -> Scene {
        let mut scene = Scene::default();
        for quad in quads {
            scene.insert_primitive(*quad);
        }
        scene.finish();
        scene
    }

    /// Builds a scene from primitives with explicit `order` fields, bypassing
    /// `insert_primitive`'s order assignment.
    fn scene_of_ordered(quads: &[Quad], underlines: &[Underline]) -> Scene {
        let mut scene = Scene::default();
        scene.quads.extend_from_slice(quads);
        scene.underlines.extend_from_slice(underlines);
        scene.finish();
        scene
    }

    fn scene_of_primitives(primitives: &[Primitive]) -> Scene {
        let mut scene = Scene::default();
        for primitive in primitives {
            scene.insert_primitive(primitive.clone());
        }
        scene.finish();
        scene
    }

    fn path_primitive() -> Primitive {
        let start = Point {
            x: px(10.0),
            y: px(20.0),
        };
        let mut path = Path::new(start);
        path.color = Background::from(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 1.0,
        });
        path.content_mask = ContentMask {
            bounds: Bounds {
                origin: Point {
                    x: px(0.0),
                    y: px(0.0),
                },
                size: Size {
                    width: px(1000.0),
                    height: px(1000.0),
                },
            },
        };
        path.push_triangle(
            (
                start,
                Point {
                    x: px(20.0),
                    y: px(20.0),
                },
                Point {
                    x: px(10.0),
                    y: px(40.0),
                },
            ),
            (
                Point::new(0.0, 0.0),
                Point::new(0.5, 0.0),
                Point::new(1.0, 1.0),
            ),
        );
        Primitive::Path(path.scale(1.0))
    }

    fn underline_with_order(bounds: Bounds<ScaledPixels>, lightness: f32, order: u32) -> Underline {
        Underline {
            order,
            pad: 0,
            bounds,
            content_mask: ContentMask {
                bounds: rect(0., 0., 1000., 1000.),
            },
            color: Hsla {
                h: 0.,
                s: 0.,
                l: lightness,
                a: 1.,
            },
            thickness: ScaledPixels(1.),
            wavy: false.into(),
        }
    }

    fn scene_with_surface(surface_bounds: Bounds<ScaledPixels>) -> Scene {
        let mut scene = Scene::default();
        scene.insert_primitive(PaintSurface {
            order: 0,
            bounds: surface_bounds,
            content_mask: ContentMask {
                bounds: rect(0., 0., 1000., 1000.),
            },
            #[cfg(target_os = "macos")]
            image_buffer: dummy_surface_buffer(),
        });
        scene.finish();
        scene
    }

    #[cfg(target_os = "macos")]
    fn dummy_surface_buffer() -> core_video::pixel_buffer::CVPixelBuffer {
        use core_video::pixel_buffer::{CVPixelBuffer, kCVPixelFormatType_32BGRA};
        CVPixelBuffer::new(1, 1, kCVPixelFormatType_32BGRA, None).unwrap()
    }

    /// Whether `region` is entirely covered by the union of `covers`, computed
    /// exactly by rectangle subtraction.
    fn rect_covered_by(region: Bounds<ScaledPixels>, covers: &[Bounds<ScaledPixels>]) -> bool {
        if region.size.width.0 <= 0.0 || region.size.height.0 <= 0.0 {
            return true;
        }
        let Some(index) = covers.iter().position(|cover| {
            let overlap = cover.intersect(&region);
            overlap.size.width.0 > 0.0 && overlap.size.height.0 > 0.0
        }) else {
            return false;
        };
        let overlap = covers[index].intersect(&region);
        let mut remaining = covers.to_vec();
        remaining.swap_remove(index);

        let region_right = region.origin.x.0 + region.size.width.0;
        let region_bottom = region.origin.y.0 + region.size.height.0;
        let overlap_right = overlap.origin.x.0 + overlap.size.width.0;
        let overlap_bottom = overlap.origin.y.0 + overlap.size.height.0;
        let residuals = [
            // Above the overlap.
            rect(
                region.origin.x.0,
                region.origin.y.0,
                region.size.width.0,
                overlap.origin.y.0 - region.origin.y.0,
            ),
            // Below the overlap.
            rect(
                region.origin.x.0,
                overlap_bottom,
                region.size.width.0,
                region_bottom - overlap_bottom,
            ),
            // Left of the overlap, within its vertical span.
            rect(
                region.origin.x.0,
                overlap.origin.y.0,
                overlap.origin.x.0 - region.origin.x.0,
                overlap.size.height.0,
            ),
            // Right of the overlap, within its vertical span.
            rect(
                overlap_right,
                overlap.origin.y.0,
                region_right - overlap_right,
                overlap.size.height.0,
            ),
        ];
        residuals
            .into_iter()
            .all(|residual| rect_covered_by(residual, &remaining))
    }

    fn damage_covers(damage: &SceneDamage, region: Bounds<ScaledPixels>) -> bool {
        match damage {
            SceneDamage::Full => true,
            SceneDamage::Unchanged => region.size.width.0 <= 0.0 || region.size.height.0 <= 0.0,
            SceneDamage::Rects(rects) => rect_covered_by(region, rects.as_slice()),
        }
    }

    fn damage_within(damage: &SceneDamage, region: Bounds<ScaledPixels>) -> bool {
        match damage {
            SceneDamage::Full => false,
            SceneDamage::Unchanged => true,
            SceneDamage::Rects(rects) => rects
                .as_slice()
                .iter()
                .all(|rect| rect_covered_by(*rect, &[region])),
        }
    }

    #[test]
    fn identical_scenes_are_unchanged() {
        let quads = [
            quad(rect(0., 0., 100., 100.), 0.2),
            quad(rect(50., 50., 100., 100.), 0.8),
        ];
        let a = scene_of(&quads);
        let b = scene_of(&quads);
        assert!(matches!(
            SceneDamage::between_with_order_tolerance(&a, &b, true),
            SceneDamage::Unchanged
        ));
    }

    #[test]
    fn strict_order_mode_reports_order_shifts_as_damage() {
        let before = scene_of_ordered(&[quad_with_order(rect(0., 0., 50., 50.), 0.2, 1)], &[]);
        let after = scene_of_ordered(&[quad_with_order(rect(0., 0., 50., 50.), 0.2, 5)], &[]);
        // Identical content, shifted order: tolerant mode sees no change,
        // strict mode conservatively damages the shifted primitive.
        assert!(matches!(
            SceneDamage::between_with_order_tolerance(&before, &after, true),
            SceneDamage::Unchanged
        ));
        let strict = SceneDamage::between_with_order_tolerance(&before, &after, false);
        assert!(
            damage_covers(&strict, rect(0., 0., 50., 50.)),
            "strict damage {strict:?}"
        );
    }

    #[test]
    fn pure_order_shift_is_unchanged() {
        let before = scene_of_ordered(
            &[
                quad_with_order(rect(0., 0., 50., 50.), 0.2, 1),
                quad_with_order(rect(100., 100., 50., 50.), 0.8, 2),
            ],
            &[],
        );
        let after = scene_of_ordered(
            &[
                quad_with_order(rect(0., 0., 50., 50.), 0.2, 5),
                quad_with_order(rect(100., 100., 50., 50.), 0.8, 9),
            ],
            &[],
        );
        assert!(matches!(
            SceneDamage::between_with_order_tolerance(&before, &after, true),
            SceneDamage::Unchanged
        ));
    }

    #[test]
    fn changed_quad_damages_only_its_bounds() {
        let unchanged = quad(rect(0., 0., 100., 100.), 0.1);
        let changed_bounds = rect(200., 200., 10., 20.);
        let before = scene_of(&[unchanged, quad(changed_bounds, 0.5)]);
        let after = scene_of(&[unchanged, quad(changed_bounds, 0.9)]);
        let damage = SceneDamage::between_with_order_tolerance(&before, &after, true);
        assert!(damage_covers(&damage, changed_bounds), "damage {damage:?}");
        assert!(damage_within(&damage, changed_bounds), "damage {damage:?}");
    }

    #[test]
    fn changed_path_damage_includes_intermediate_texture_filtering() {
        let before = scene_of_primitives(&[path_primitive()]);
        let after = scene_of_primitives(&[]);

        let damage = SceneDamage::between_with_order_tolerance(&before, &after, true);
        assert!(
            damage_covers(&damage, rect(9.0, 19.0, 12.0, 22.0)),
            "damage {damage:?}"
        );
    }

    #[test]
    fn inserted_quad_damages_only_inserted_bounds() {
        let a = quad(rect(0., 0., 50., 50.), 0.1);
        let cursor_bounds = rect(60., 0., 2., 20.);
        let cursor = quad(cursor_bounds, 0.5);
        let c = quad(rect(100., 0., 50., 50.), 0.9);
        let before = scene_of(&[a, c]);
        let after = scene_of(&[a, cursor, c]);
        let damage = SceneDamage::between_with_order_tolerance(&before, &after, true);
        assert!(damage_covers(&damage, cursor_bounds), "damage {damage:?}");
        assert!(damage_within(&damage, cursor_bounds), "damage {damage:?}");
    }

    #[test]
    fn removed_quad_damages_only_removed_bounds() {
        let a = quad(rect(0., 0., 50., 50.), 0.1);
        let cursor_bounds = rect(60., 0., 2., 20.);
        let cursor = quad(cursor_bounds, 0.5);
        let c = quad(rect(100., 0., 50., 50.), 0.9);
        let before = scene_of(&[a, cursor, c]);
        let after = scene_of(&[a, c]);
        let damage = SceneDamage::between_with_order_tolerance(&before, &after, true);
        assert!(damage_covers(&damage, cursor_bounds), "damage {damage:?}");
        assert!(damage_within(&damage, cursor_bounds), "damage {damage:?}");
    }

    #[test]
    fn overlapping_insert_damages_only_inserted_bounds() {
        let a = quad(rect(0., 0., 50., 50.), 0.1);
        let inserted_bounds = rect(10., 10., 50., 50.);
        let inserted = quad(inserted_bounds, 0.5);
        let c = quad(rect(40., 40., 50., 50.), 0.9);
        let before = scene_of(&[a, c]);
        let after = scene_of(&[a, inserted, c]);
        let damage = SceneDamage::between_with_order_tolerance(&before, &after, true);
        assert!(damage_covers(&damage, inserted_bounds), "damage {damage:?}");
        assert!(damage_within(&damage, inserted_bounds), "damage {damage:?}");
    }

    #[test]
    fn two_change_loci_damage_stays_localized() {
        let mut background: Vec<Quad> = (0..20)
            .map(|i| quad(rect(i as f32 * 30.0, 300.0, 20.0, 20.0), 0.3))
            .collect();
        let spinner_a_bounds = rect(0., 0., 10., 10.);
        let spinner_b_bounds = rect(900., 900., 10., 10.);
        background.insert(3, quad(spinner_a_bounds, 0.1));
        background.insert(15, quad(spinner_b_bounds, 0.1));
        let before = scene_of(&background);
        background[3] = quad(spinner_a_bounds, 0.6);
        background[15] = quad(spinner_b_bounds, 0.6);
        let after = scene_of(&background);

        let damage = SceneDamage::between_with_order_tolerance(&before, &after, true);
        assert!(
            damage_covers(&damage, spinner_a_bounds),
            "damage {damage:?}"
        );
        assert!(
            damage_covers(&damage, spinner_b_bounds),
            "damage {damage:?}"
        );
        // The unchanged strip between the two loci must not be damaged.
        assert!(
            !damage.contains_point(&Point {
                x: ScaledPixels(310.0),
                y: ScaledPixels(310.0),
            }),
            "damage {damage:?}"
        );
    }

    #[test]
    fn cross_type_reorder_damages_the_moved_primitive() {
        let quad_bounds = rect(0., 0., 50., 50.);
        let underline_bounds = rect(25., 25., 50., 10.);
        // The underline starts above the quad, then sinks below it.
        let before = scene_of_ordered(
            &[quad_with_order(quad_bounds, 0.2, 1)],
            &[underline_with_order(underline_bounds, 0.8, 2)],
        );
        let after = scene_of_ordered(
            &[quad_with_order(quad_bounds, 0.2, 2)],
            &[underline_with_order(underline_bounds, 0.8, 1)],
        );
        let damage = SceneDamage::between_with_order_tolerance(&before, &after, true);
        let overlap = quad_bounds.intersect(&underline_bounds);
        assert!(damage_covers(&damage, overlap), "damage {damage:?}");
    }

    #[test]
    fn surfaces_damage_their_previous_and_current_bounds() {
        let before = scene_with_surface(rect(10., 10., 20., 20.));
        let after = scene_with_surface(rect(30., 30., 20., 20.));
        let damage = SceneDamage::between_with_order_tolerance(&before, &after, true);
        assert!(damage_covers(&damage, rect(10., 10., 20., 20.)));
        assert!(damage_covers(&damage, rect(30., 30., 20., 20.)));
    }

    #[test]
    fn union_combines_damage() {
        assert!(matches!(
            SceneDamage::Full.union(SceneDamage::Unchanged),
            SceneDamage::Full
        ));
        assert!(matches!(
            SceneDamage::Unchanged.union(SceneDamage::Unchanged),
            SceneDamage::Unchanged
        ));

        let mut a = DamageRects::default();
        a.add(rect(0., 0., 10., 10.));
        let mut b = DamageRects::default();
        b.add(rect(20., 20., 10., 10.));
        let merged = SceneDamage::Rects(a).union(SceneDamage::Rects(b));
        assert!(damage_covers(&merged, rect(0., 0., 10., 10.)));
        assert!(damage_covers(&merged, rect(20., 20., 10., 10.)));
    }

    #[cfg(not(target_family = "wasm"))]
    mod properties {
        use super::*;
        use proptest::{prelude::*, test_runner::TestCaseError};

        fn arbitrary_rect() -> impl Strategy<Value = Bounds<ScaledPixels>> {
            (-100i32..100, -100i32..100, 0i32..100, 0i32..100)
                .prop_map(|(x, y, w, h)| rect(x as f32, y as f32, w as f32, h as f32))
        }

        fn arbitrary_damage() -> impl Strategy<Value = SceneDamage> {
            prop_oneof![
                1 => Just(SceneDamage::Full),
                2 => Just(SceneDamage::Unchanged),
                5 => proptest::collection::vec(arbitrary_rect(), 0..6).prop_map(|rects| {
                    let mut acc = DamageRects::default();
                    for rect in rects {
                        acc.add(rect);
                    }
                    if acc.is_empty() {
                        SceneDamage::Unchanged
                    } else {
                        SceneDamage::Rects(acc)
                    }
                }),
            ]
        }

        /// Any region covered by any input damage must be covered by the
        /// merged damage. (The converse is not required: expanding damage
        /// is always safe.)
        #[gpui::property_test]
        fn merged_damage_covers_every_input_region(
            #[strategy = proptest::collection::vec(arbitrary_damage(), 0..8)] damages: Vec<
                SceneDamage,
            >,
        ) -> Result<(), TestCaseError> {
            let merged = damages
                .iter()
                .cloned()
                .fold(SceneDamage::Unchanged, SceneDamage::union);
            for damage in &damages {
                match damage {
                    SceneDamage::Full => {
                        prop_assert!(matches!(merged, SceneDamage::Full));
                    }
                    SceneDamage::Unchanged => {}
                    SceneDamage::Rects(rects) => {
                        for rect in rects.as_slice() {
                            prop_assert!(
                                damage_covers(&merged, *rect),
                                "merged damage {merged:?} does not cover input rect {rect:?}",
                            );
                        }
                    }
                }
            }
            Ok(())
        }

        /// Renderers draw each damage rectangle's pixels separately, so the
        /// rectangles must never overlap or translucent content would be
        /// blended twice.
        #[gpui::property_test]
        fn accumulator_rects_are_pairwise_disjoint(
            #[strategy = proptest::collection::vec(arbitrary_rect(), 0..40)] rects: Vec<
                Bounds<ScaledPixels>,
            >,
        ) -> Result<(), TestCaseError> {
            let mut acc = DamageRects::default();
            for rect in &rects {
                acc.add(*rect);
            }
            let rects = acc.as_slice();
            for (index, a) in rects.iter().enumerate() {
                for b in &rects[index + 1..] {
                    let overlap = a.intersect(b);
                    prop_assert!(
                        overlap.size.width.0 <= 0.0 || overlap.size.height.0 <= 0.0,
                        "accumulator rects overlap: {a:?} and {b:?}",
                    );
                }
            }
            Ok(())
        }

        /// Every rectangle added to an accumulator must remain covered by
        /// the accumulator's rectangles, no matter how many additions and
        /// coalesces happen.
        #[gpui::property_test]
        fn accumulator_covers_every_added_rect(
            #[strategy = proptest::collection::vec(arbitrary_rect(), 0..40)] rects: Vec<
                Bounds<ScaledPixels>,
            >,
        ) -> Result<(), TestCaseError> {
            let mut acc = DamageRects::default();
            for rect in &rects {
                acc.add(*rect);
            }
            for rect in &rects {
                prop_assert!(
                    rect_covered_by(*rect, acc.as_slice()),
                    "accumulator {acc:?} does not cover added rect {rect:?}",
                );
            }
            Ok(())
        }
    }
}
