//! Diffs two consecutive `Scene`s to find a conservative region of the
//! framebuffer that changed, so a backend can redraw and present only that
//! region.

use crate::{
    Bounds, MonochromeSprite, PaintOperation, PaintSurface, Path, PathVertex, Point,
    PolychromeSprite, Primitive, Quad, ScaledPixels, Scene, Shadow, Size, SubpixelSprite,
    TransformationMatrix, Underline,
};

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
    /// Computes a conservative region of `next` that differs from `prev`.
    ///
    /// This may over-report, especially when primitive order values shift, but
    /// should not under-report. Both scenes must be finished (sorted).
    pub fn between(prev: &Scene, next: &Scene) -> SceneDamage {
        let mut acc: Option<Bounds<ScaledPixels>> = None;
        diff_primitives(
            &prev.quads,
            &next.quads,
            quads_equal,
            |quad| quad.bounds,
            &mut acc,
        );
        diff_primitives(
            &prev.shadows,
            &next.shadows,
            shadows_equal,
            |shadow| shadow.bounds.dilate(shadow.blur_radius * 3.0),
            &mut acc,
        );
        diff_primitives(
            &prev.underlines,
            &next.underlines,
            underlines_equal,
            |underline| underline.bounds,
            &mut acc,
        );
        diff_primitives(
            &prev.monochrome_sprites,
            &next.monochrome_sprites,
            monochrome_sprites_equal,
            |sprite| transformed_bounds(sprite.bounds, &sprite.transformation),
            &mut acc,
        );
        diff_primitives(
            &prev.subpixel_sprites,
            &next.subpixel_sprites,
            subpixel_sprites_equal,
            |sprite| transformed_bounds(sprite.bounds, &sprite.transformation),
            &mut acc,
        );
        diff_primitives(
            &prev.polychrome_sprites,
            &next.polychrome_sprites,
            polychrome_sprites_equal,
            |sprite| sprite.bounds,
            &mut acc,
        );
        diff_primitives(
            &prev.paths,
            &next.paths,
            paths_equal,
            |path| path.bounds,
            &mut acc,
        );
        diff_paint_operations(&prev.paint_operations, &next.paint_operations, &mut acc);
        accumulate_surface_damage(&prev.surfaces, &mut acc);
        accumulate_surface_damage(&next.surfaces, &mut acc);

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

fn quads_equal(a: &Quad, b: &Quad) -> bool {
    a.order == b.order
        && a.bounds == b.bounds
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

fn underlines_equal(a: &Underline, b: &Underline) -> bool {
    a.order == b.order
        && a.pad == b.pad
        && a.bounds == b.bounds
        && a.content_mask == b.content_mask
        && a.color == b.color
        && a.thickness == b.thickness
        && a.wavy == b.wavy
}

fn shadows_equal(a: &Shadow, b: &Shadow) -> bool {
    a.order == b.order
        && a.blur_radius == b.blur_radius
        && a.bounds == b.bounds
        && a.corner_radii == b.corner_radii
        && a.content_mask == b.content_mask
        && a.color == b.color
        && a.element_bounds == b.element_bounds
        && a.element_corner_radii == b.element_corner_radii
        && a.inset == b.inset
        && a.pad == b.pad
}

fn monochrome_sprites_equal(a: &MonochromeSprite, b: &MonochromeSprite) -> bool {
    a.order == b.order
        && a.pad == b.pad
        && a.bounds == b.bounds
        && a.content_mask == b.content_mask
        && a.color == b.color
        && a.tile == b.tile
        && a.transformation == b.transformation
}

fn subpixel_sprites_equal(a: &SubpixelSprite, b: &SubpixelSprite) -> bool {
    a.order == b.order
        && a.pad == b.pad
        && a.bounds == b.bounds
        && a.content_mask == b.content_mask
        && a.color == b.color
        && a.tile == b.tile
        && a.transformation == b.transformation
}

fn polychrome_sprites_equal(a: &PolychromeSprite, b: &PolychromeSprite) -> bool {
    a.order == b.order
        && a.pad == b.pad
        && a.grayscale == b.grayscale
        && a.opacity == b.opacity
        && a.bounds == b.bounds
        && a.content_mask == b.content_mask
        && a.corner_radii == b.corner_radii
        && a.tile == b.tile
}

fn paths_equal(a: &Path<ScaledPixels>, b: &Path<ScaledPixels>) -> bool {
    a.order == b.order
        && a.bounds == b.bounds
        && a.content_mask == b.content_mask
        && a.color == b.color
        && a.vertices.len() == b.vertices.len()
        && a.vertices
            .iter()
            .zip(&b.vertices)
            .all(|(a, b)| path_vertices_equal(a, b))
}

fn paint_operations_equal(a: &PaintOperation, b: &PaintOperation) -> bool {
    match (a, b) {
        (PaintOperation::Primitive(a), PaintOperation::Primitive(b)) => primitives_equal(a, b),
        (PaintOperation::StartLayer(a), PaintOperation::StartLayer(b)) => a == b,
        (PaintOperation::EndLayer, PaintOperation::EndLayer) => true,
        _ => false,
    }
}

fn primitives_equal(a: &Primitive, b: &Primitive) -> bool {
    match (a, b) {
        (Primitive::Shadow(a), Primitive::Shadow(b)) => shadows_equal(a, b),
        (Primitive::Quad(a), Primitive::Quad(b)) => quads_equal(a, b),
        (Primitive::Path(a), Primitive::Path(b)) => paths_equal(a, b),
        (Primitive::Underline(a), Primitive::Underline(b)) => underlines_equal(a, b),
        (Primitive::MonochromeSprite(a), Primitive::MonochromeSprite(b)) => {
            monochrome_sprites_equal(a, b)
        }
        (Primitive::SubpixelSprite(a), Primitive::SubpixelSprite(b)) => {
            subpixel_sprites_equal(a, b)
        }
        (Primitive::PolychromeSprite(a), Primitive::PolychromeSprite(b)) => {
            polychrome_sprites_equal(a, b)
        }
        (Primitive::Surface(_), Primitive::Surface(_)) => false,
        _ => false,
    }
}

fn path_vertices_equal(a: &PathVertex<ScaledPixels>, b: &PathVertex<ScaledPixels>) -> bool {
    a.xy_position == b.xy_position
        && a.st_position == b.st_position
        && a.content_mask == b.content_mask
}

fn union_into(acc: &mut Option<Bounds<ScaledPixels>>, bounds: Bounds<ScaledPixels>) {
    *acc = Some(match acc.take() {
        Some(acc) => acc.union(&bounds),
        None => bounds,
    });
}

fn accumulate_surface_damage(surfaces: &[PaintSurface], acc: &mut Option<Bounds<ScaledPixels>>) {
    for surface in surfaces {
        let bounds = surface.bounds.intersect(&surface.content_mask.bounds);
        if !bounds.is_empty() {
            union_into(acc, bounds);
        }
    }
}

fn diff_primitives<T>(
    prev: &[T],
    cur: &[T],
    eq: impl Fn(&T, &T) -> bool,
    bounds_of: impl Fn(&T) -> Bounds<ScaledPixels>,
    acc: &mut Option<Bounds<ScaledPixels>>,
) {
    diff_with(prev, cur, eq, bounds_of, acc);
}

fn diff_paint_operations(
    prev: &[PaintOperation],
    cur: &[PaintOperation],
    acc: &mut Option<Bounds<ScaledPixels>>,
) {
    let max_common = prev.len().min(cur.len());

    let mut prefix = 0;
    while prefix < max_common && paint_operations_equal(&prev[prefix], &cur[prefix]) {
        prefix += 1;
    }
    if prefix == prev.len() && prefix == cur.len() {
        return;
    }

    for operation in &prev[prefix..] {
        if let Some(bounds) = paint_operation_bounds(operation) {
            union_into(acc, bounds);
        }
    }
    for operation in &cur[prefix..] {
        if let Some(bounds) = paint_operation_bounds(operation) {
            union_into(acc, bounds);
        }
    }
}

fn paint_operation_bounds(operation: &PaintOperation) -> Option<Bounds<ScaledPixels>> {
    match operation {
        PaintOperation::Primitive(primitive) => Some(primitive_damage_bounds(primitive)),
        PaintOperation::StartLayer(bounds) => Some(*bounds),
        PaintOperation::EndLayer => None,
    }
}

fn primitive_damage_bounds(primitive: &Primitive) -> Bounds<ScaledPixels> {
    match primitive {
        Primitive::Shadow(shadow) => shadow.bounds.dilate(shadow.blur_radius * 3.0),
        Primitive::Quad(quad) => quad.bounds,
        Primitive::Path(path) => path.bounds,
        Primitive::Underline(underline) => underline.bounds,
        Primitive::MonochromeSprite(sprite) => {
            transformed_bounds(sprite.bounds, &sprite.transformation)
        }
        Primitive::SubpixelSprite(sprite) => {
            transformed_bounds(sprite.bounds, &sprite.transformation)
        }
        Primitive::PolychromeSprite(sprite) => sprite.bounds,
        Primitive::Surface(surface) => surface.bounds.intersect(&surface.content_mask.bounds),
    }
}

/// Diffs two primitive slices via common-prefix / common-suffix matching.
///
/// This is deliberately not a minimal diff. If an inserted or removed primitive
/// shifts order values for many later primitives, those primitives will compare
/// unequal and be included in the damage region.
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
        return;
    }

    let mut suffix = 0;
    while suffix < max_common - prefix
        && eq(&prev[prev.len() - 1 - suffix], &cur[cur.len() - 1 - suffix])
    {
        suffix += 1;
    }

    for primitive in &prev[prefix..prev.len() - suffix] {
        union_into(acc, bounds_of(primitive));
    }
    for primitive in &cur[prefix..cur.len() - suffix] {
        union_into(acc, bounds_of(primitive));
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
    use crate::{ContentMask, Hsla, PaintSurface, Quad};

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

    fn scene_of(quads: &[Quad]) -> Scene {
        let mut scene = Scene::default();
        for quad in quads {
            scene.insert_primitive(*quad);
        }
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

    fn underline(bounds: Bounds<ScaledPixels>, lightness: f32) -> Underline {
        Underline {
            order: 0,
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

    fn bounds_contains(container: Bounds<ScaledPixels>, contained: Bounds<ScaledPixels>) -> bool {
        container.origin.x.0 <= contained.origin.x.0
            && container.origin.y.0 <= contained.origin.y.0
            && container.origin.x.0 + container.size.width.0
                >= contained.origin.x.0 + contained.size.width.0
            && container.origin.y.0 + container.size.height.0
                >= contained.origin.y.0 + contained.size.height.0
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
        use core_foundation::{base::TCFType, number::CFNumber, string::CFString};
        use core_video::{
            pixel_buffer::{self, kCVPixelFormatType_420YpCbCr8BiPlanarFullRange},
            pixel_buffer_io_surface::kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey,
            pixel_buffer_pool,
        };

        let width_key: CFString =
            unsafe { CFString::wrap_under_get_rule(pixel_buffer::kCVPixelBufferWidthKey) };
        let height_key: CFString =
            unsafe { CFString::wrap_under_get_rule(pixel_buffer::kCVPixelBufferHeightKey) };
        let animation_key: CFString = unsafe {
            CFString::wrap_under_get_rule(kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey)
        };
        let format_key: CFString = unsafe {
            CFString::wrap_under_get_rule(pixel_buffer::kCVPixelBufferPixelFormatTypeKey)
        };

        let yes: CFNumber = 1.into();
        let width: CFNumber = 1.into();
        let height: CFNumber = 1.into();
        let format: CFNumber = (kCVPixelFormatType_420YpCbCr8BiPlanarFullRange as i64).into();

        let buffer_attributes = core_foundation::dictionary::CFDictionary::from_CFType_pairs(&[
            (width_key, width.into_CFType()),
            (height_key, height.into_CFType()),
            (animation_key, yes.into_CFType()),
            (format_key, format.into_CFType()),
        ]);

        let pool = pixel_buffer_pool::CVPixelBufferPool::new(None, Some(&buffer_attributes))
            .expect("failed to create test pixel buffer pool");
        pool.create_pixel_buffer()
            .expect("failed to create test pixel buffer")
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
    fn inserted_quad_damages_inserted_bounds() {
        let a = quad(rect(0., 0., 50., 50.), 0.1);
        let cursor = quad(rect(60., 0., 2., 20.), 0.5);
        let c = quad(rect(100., 0., 50., 50.), 0.9);
        let before = scene_of(&[a, c]);
        let after = scene_of(&[a, cursor, c]);
        match SceneDamage::between(&before, &after) {
            SceneDamage::Rect(damage) => assert!(bounds_contains(damage, rect(60., 0., 2., 20.))),
            other => panic!("expected rect damage, got {other:?}"),
        }
    }

    #[test]
    fn removed_quad_damages_removed_bounds() {
        let a = quad(rect(0., 0., 50., 50.), 0.1);
        let cursor = quad(rect(60., 0., 2., 20.), 0.5);
        let c = quad(rect(100., 0., 50., 50.), 0.9);
        let before = scene_of(&[a, cursor, c]);
        let after = scene_of(&[a, c]);
        match SceneDamage::between(&before, &after) {
            SceneDamage::Rect(damage) => assert!(bounds_contains(damage, rect(60., 0., 2., 20.))),
            other => panic!("expected rect damage, got {other:?}"),
        }
    }

    #[test]
    fn overlapping_insert_can_damage_unchanged_primitives_when_order_shifts() {
        let a = quad(rect(0., 0., 50., 50.), 0.1);
        let inserted = quad(rect(10., 10., 50., 50.), 0.5);
        let c = quad(rect(40., 40., 50., 50.), 0.9);
        let before = scene_of(&[a, c]);
        let after = scene_of(&[a, inserted, c]);
        match SceneDamage::between(&before, &after) {
            SceneDamage::Rect(damage) => assert_eq!(damage, rect(10., 10., 80., 80.)),
            other => panic!("expected rect damage, got {other:?}"),
        }
    }

    #[test]
    fn cross_type_order_changes_damage_following_primitives() {
        let a = Primitive::Quad(quad(rect(0., 0., 10., 10.), 0.1));
        let b = Primitive::Underline(underline(rect(20., 20., 10., 10.), 0.5));
        let c = Primitive::Quad(quad(rect(40., 40., 20., 20.), 0.9));
        let before = scene_of_primitives(&[a.clone(), b.clone(), c.clone()]);
        let after = scene_of_primitives(&[a, c.clone(), b.clone()]);

        match SceneDamage::between(&before, &after) {
            SceneDamage::Rect(damage) => {
                assert!(bounds_contains(damage, *b.bounds()));
                assert!(bounds_contains(damage, *c.bounds()));
            }
            other => panic!("expected rect damage, got {other:?}"),
        }
    }

    #[test]
    fn surfaces_damage_their_previous_and_current_bounds() {
        let before = scene_with_surface(rect(10., 10., 20., 20.));
        let after = scene_with_surface(rect(30., 30., 20., 20.));
        match SceneDamage::between(&before, &after) {
            SceneDamage::Rect(damage) => assert_eq!(damage, rect(10., 10., 40., 40.)),
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
