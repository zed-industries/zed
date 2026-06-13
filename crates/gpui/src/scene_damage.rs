//! Diffs two consecutive `Scene`s to find the region of the framebuffer that
//! changed, so a backend can redraw and present only that region.

use crate::{Bounds, Path, Point, ScaledPixels, Scene, Size, TransformationMatrix};

/// The region of the framebuffer that changed between two frames.
#[derive(Clone, Copy, Debug)]
pub enum SceneDamage {
    /// Everything must be redrawn (e.g. the previous contents are invalid).
    Full,
    /// Only this rectangle changed, in scaled/device pixels.
    Rect(Bounds<ScaledPixels>),
    /// Nothing changed; the frame's presentation can be skipped.
    Unchanged,
}

impl SceneDamage {
    /// Computes the region of `next` that differs from `prev`. Never reports a
    /// changed region as unchanged, but may over-report. Both scenes must be
    /// finished (sorted).
    pub fn between(prev: &Scene, next: &Scene) -> SceneDamage {
        let mut acc: Option<Bounds<ScaledPixels>> = None;
        diff_primitives(&prev.quads, &next.quads, |q| q.bounds, &mut acc);
        diff_primitives(
            &prev.shadows,
            &next.shadows,
            |s| s.bounds.dilate(s.blur_radius * 3.0),
            &mut acc,
        );
        diff_primitives(&prev.underlines, &next.underlines, |u| u.bounds, &mut acc);
        diff_primitives(
            &prev.monochrome_sprites,
            &next.monochrome_sprites,
            |s| transformed_bounds(s.bounds, &s.transformation),
            &mut acc,
        );
        diff_primitives(
            &prev.subpixel_sprites,
            &next.subpixel_sprites,
            |s| transformed_bounds(s.bounds, &s.transformation),
            &mut acc,
        );
        diff_primitives(
            &prev.polychrome_sprites,
            &next.polychrome_sprites,
            |s| s.bounds,
            &mut acc,
        );
        diff_paths(&prev.paths, &next.paths, &mut acc);

        match acc {
            Some(rect) => SceneDamage::Rect(rect),
            None => SceneDamage::Unchanged,
        }
    }

    /// Combines two damage regions into one that covers both, used to accumulate
    /// damage across frames that failed or skipped presentation.
    pub fn union(self, other: SceneDamage) -> SceneDamage {
        match (self, other) {
            (SceneDamage::Full, _) | (_, SceneDamage::Full) => SceneDamage::Full,
            (SceneDamage::Unchanged, damage) | (damage, SceneDamage::Unchanged) => damage,
            (SceneDamage::Rect(a), SceneDamage::Rect(b)) => SceneDamage::Rect(a.union(&b)),
        }
    }
}

fn union_into(acc: &mut Option<Bounds<ScaledPixels>>, b: Bounds<ScaledPixels>) {
    *acc = Some(match acc.take() {
        Some(a) => a.union(&b),
        None => b,
    });
}

fn diff_primitives<T: Copy + PartialEq>(
    prev: &[T],
    cur: &[T],
    bounds_of: impl Fn(&T) -> Bounds<ScaledPixels>,
    acc: &mut Option<Bounds<ScaledPixels>>,
) {
    diff_with(prev, cur, |a, b| a == b, bounds_of, acc);
}

fn diff_paths(
    prev: &[Path<ScaledPixels>],
    cur: &[Path<ScaledPixels>],
    acc: &mut Option<Bounds<ScaledPixels>>,
) {
    diff_with(
        prev,
        cur,
        |a, b| {
            a.order == b.order
                && &a.bounds == &b.bounds
                && &a.color == &b.color
                && a.vertices == b.vertices
        },
        |p| p.bounds,
        acc,
    );
}

/// Diffs two primitive slices via common-prefix / common-suffix matching, so a
/// single inserted or removed element (e.g. a blinking cursor, which shifts
/// every following element) only damages the changed window rather than
/// everything after it.
fn diff_with<T>(
    prev: &[T],
    cur: &[T],
    eq: impl Fn(&T, &T) -> bool,
    bounds_of: impl Fn(&T) -> Bounds<ScaledPixels>,
    acc: &mut Option<Bounds<ScaledPixels>>,
) {
    let max_common = prev.len().min(cur.len());

    let mut prefix = 0;
    while prefix < max_common && eq(&prev[prefix], &cur[prefix]) {
        prefix += 1;
    }
    if prefix == prev.len() && prefix == cur.len() {
        return; // Identical.
    }

    // Match from the end, without overlapping the prefix in either slice.
    let mut suffix = 0;
    while suffix < max_common - prefix
        && eq(&prev[prev.len() - 1 - suffix], &cur[cur.len() - 1 - suffix])
    {
        suffix += 1;
    }

    for p in &prev[prefix..prev.len() - suffix] {
        union_into(acc, bounds_of(p));
    }
    for c in &cur[prefix..cur.len() - suffix] {
        union_into(acc, bounds_of(c));
    }
}

/// Axis-aligned bounds of a sprite after its transformation, so rotated sprites
/// damage their full painted extent.
fn transformed_bounds(
    bounds: Bounds<ScaledPixels>,
    transform: &TransformationMatrix,
) -> Bounds<ScaledPixels> {
    let rs = transform.rotation_scale;
    let t = transform.translation;
    let x0 = bounds.origin.x.0;
    let y0 = bounds.origin.y.0;
    let x1 = x0 + bounds.size.width.0;
    let y1 = y0 + bounds.size.height.0;
    let mut min = (f32::MAX, f32::MAX);
    let mut max = (f32::MIN, f32::MIN);
    for (x, y) in [(x0, y0), (x1, y0), (x0, y1), (x1, y1)] {
        // Matches the shader: transpose(rotation_scale) * position + translation.
        let tx = rs[0][0] * x + rs[1][0] * y + t[0];
        let ty = rs[0][1] * x + rs[1][1] * y + t[1];
        min.0 = min.0.min(tx);
        min.1 = min.1.min(ty);
        max.0 = max.0.max(tx);
        max.1 = max.1.max(ty);
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
    use crate::{ContentMask, Hsla, Quad};

    fn rect(x: f32, y: f32, w: f32, h: f32) -> Bounds<ScaledPixels> {
        Bounds {
            origin: Point {
                x: ScaledPixels(x),
                y: ScaledPixels(y),
            },
            size: Size {
                width: ScaledPixels(w),
                height: ScaledPixels(h),
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

    fn scene_of(quads: &[Quad]) -> Scene {
        let mut scene = Scene::default();
        for q in quads {
            scene.insert_primitive(*q);
        }
        scene.finish();
        scene
    }

    #[test]
    fn identical_scenes_are_unchanged() {
        let a = scene_of(&[quad(rect(0., 0., 100., 100.), 0.5)]);
        let b = scene_of(&[quad(rect(0., 0., 100., 100.), 0.5)]);
        assert!(matches!(
            SceneDamage::between(&a, &b),
            SceneDamage::Unchanged
        ));
    }

    #[test]
    fn changed_quad_damages_its_bounds() {
        let unchanged = quad(rect(0., 0., 100., 100.), 0.1);
        let before = scene_of(&[unchanged, quad(rect(200., 200., 10., 20.), 0.5)]);
        let after = scene_of(&[unchanged, quad(rect(200., 200., 10., 20.), 0.9)]);
        match SceneDamage::between(&before, &after) {
            SceneDamage::Rect(damage) => assert_eq!(damage, rect(200., 200., 10., 20.)),
            other => panic!("expected rect damage, got {other:?}"),
        }
    }

    #[test]
    fn inserted_quad_damages_only_itself() {
        // Mirrors the cursor blinking on: one primitive appears mid-scene,
        // shifting the index of everything after it.
        let a = quad(rect(0., 0., 50., 50.), 0.1);
        let cursor = quad(rect(60., 0., 2., 20.), 0.5);
        let c = quad(rect(100., 0., 50., 50.), 0.9);
        let before = scene_of(&[a, c]);
        let after = scene_of(&[a, cursor, c]);
        match SceneDamage::between(&before, &after) {
            SceneDamage::Rect(damage) => assert_eq!(damage, rect(60., 0., 2., 20.)),
            other => panic!("expected rect damage, got {other:?}"),
        }
    }

    #[test]
    fn removed_quad_damages_only_itself() {
        // The cursor blinking off.
        let a = quad(rect(0., 0., 50., 50.), 0.1);
        let cursor = quad(rect(60., 0., 2., 20.), 0.5);
        let c = quad(rect(100., 0., 50., 50.), 0.9);
        let before = scene_of(&[a, cursor, c]);
        let after = scene_of(&[a, c]);
        match SceneDamage::between(&before, &after) {
            SceneDamage::Rect(damage) => assert_eq!(damage, rect(60., 0., 2., 20.)),
            other => panic!("expected rect damage, got {other:?}"),
        }
    }

    #[test]
    fn union_combines_damage() {
        assert!(matches!(
            SceneDamage::Full.union(SceneDamage::Rect(rect(0., 0., 1., 1.))),
            SceneDamage::Full
        ));
        assert!(matches!(
            SceneDamage::Unchanged.union(SceneDamage::Unchanged),
            SceneDamage::Unchanged
        ));
        match SceneDamage::Rect(rect(0., 0., 10., 10.))
            .union(SceneDamage::Rect(rect(20., 20., 10., 10.)))
        {
            SceneDamage::Rect(damage) => assert_eq!(damage, rect(0., 0., 30., 30.)),
            other => panic!("expected rect damage, got {other:?}"),
        }
    }
}
