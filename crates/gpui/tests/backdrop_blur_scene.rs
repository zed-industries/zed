use gpui::{
    BackdropBlurRect, Bounds, ContentMask, MAX_BACKDROP_BLUR_KERNEL_LEVELS, PrimitiveBatch, Quad,
    ScaledPixels, Scene, Size, point,
};

fn test_bounds(x: f32) -> Bounds<ScaledPixels> {
    Bounds {
        origin: point(ScaledPixels(x), ScaledPixels(0.)),
        size: Size {
            width: ScaledPixels(10.),
            height: ScaledPixels(10.),
        },
    }
}

fn test_content_mask() -> ContentMask<ScaledPixels> {
    ContentMask {
        bounds: Bounds {
            origin: point(ScaledPixels(-100.), ScaledPixels(-100.)),
            size: Size {
                width: ScaledPixels(1000.),
                height: ScaledPixels(1000.),
            },
        },
    }
}

fn test_quad(x: f32) -> Quad {
    Quad {
        bounds: test_bounds(x),
        content_mask: test_content_mask(),
        ..Default::default()
    }
}

fn test_backdrop_blur_rect(x: f32) -> BackdropBlurRect {
    BackdropBlurRect {
        bounds: test_bounds(x),
        content_mask: test_content_mask(),
        blur_radius: ScaledPixels(12.),
        opacity: 1.,
        ..Default::default()
    }
}

#[test]
fn backdrop_blur_rects_interleave_with_quads() {
    let mut scene = Scene::default();
    scene.insert_primitive(test_quad(0.));
    scene.insert_primitive(test_backdrop_blur_rect(0.));
    scene.insert_primitive(test_quad(0.));
    scene.finish();

    let batches = scene.batches().collect::<Vec<_>>();

    assert!(matches!(batches[0], PrimitiveBatch::Quads(ref range) if range == &(0..1)));
    assert!(matches!(
        batches[1],
        PrimitiveBatch::BackdropBlurRects(ref range) if range == &(0..1)
    ));
    assert!(matches!(batches[2], PrimitiveBatch::Quads(ref range) if range == &(1..2)));
}

#[test]
fn backdrop_blur_rects_preserve_same_layer_paint_order() {
    let mut scene = Scene::default();
    scene.push_layer(test_bounds(0.));
    scene.insert_primitive(test_quad(0.));
    scene.insert_primitive(test_backdrop_blur_rect(0.));
    scene.insert_primitive(test_quad(0.));
    scene.pop_layer();
    scene.finish();

    let batches = scene.batches().collect::<Vec<_>>();

    assert!(matches!(batches[0], PrimitiveBatch::Quads(ref range) if range == &(0..1)));
    assert!(matches!(
        batches[1],
        PrimitiveBatch::BackdropBlurRects(ref range) if range == &(0..1)
    ));
    assert!(matches!(batches[2], PrimitiveBatch::Quads(ref range) if range == &(1..2)));
}

#[test]
fn adjacent_backdrop_blur_rects_coalesce() {
    let mut scene = Scene::default();
    scene.insert_primitive(test_backdrop_blur_rect(0.));
    scene.insert_primitive(test_backdrop_blur_rect(0.));
    scene.insert_primitive(test_quad(0.));
    scene.finish();

    let batches = scene.batches().collect::<Vec<_>>();

    assert!(matches!(
        batches[0],
        PrimitiveBatch::BackdropBlurRects(ref range) if range == &(0..2)
    ));
    assert!(matches!(batches[1], PrimitiveBatch::Quads(ref range) if range == &(0..1)));
}

#[test]
fn backdrop_blur_rect_replays() {
    let mut prev_scene = Scene::default();
    prev_scene.insert_primitive(test_backdrop_blur_rect(0.));

    let mut scene = Scene::default();
    scene.replay(0..prev_scene.len(), &prev_scene);
    scene.finish();

    assert_eq!(1, scene.backdrop_blur_rects.len());
    assert!(matches!(
        scene.batches().next(),
        Some(PrimitiveBatch::BackdropBlurRects(ref range)) if range == &(0..1)
    ));
}

#[test]
fn clipped_backdrop_blur_rect_is_ignored() {
    let mut scene = Scene::default();
    let mut backdrop_blur_rect = test_backdrop_blur_rect(0.);
    backdrop_blur_rect.content_mask.bounds = test_bounds(100.);

    scene.insert_primitive(backdrop_blur_rect);

    assert!(scene.backdrop_blur_rects.is_empty());
}

#[test]
fn backdrop_blur_rect_radius_maps_to_kernel_levels() {
    let mut backdrop_blur_rect = test_backdrop_blur_rect(0.);

    backdrop_blur_rect.blur_radius = ScaledPixels(2.);
    assert_eq!(1, backdrop_blur_rect.effective_kernel_levels());

    backdrop_blur_rect.blur_radius = ScaledPixels(18.);
    assert_eq!(4, backdrop_blur_rect.effective_kernel_levels());
}

#[test]
fn backdrop_blur_rect_default_is_visible() {
    assert_eq!(1., BackdropBlurRect::default().opacity);
}

#[test]
fn backdrop_blur_rect_kernel_levels_are_clamped() {
    let mut backdrop_blur_rect = test_backdrop_blur_rect(0.);
    backdrop_blur_rect.blur_radius = ScaledPixels(1000.);

    assert_eq!(
        MAX_BACKDROP_BLUR_KERNEL_LEVELS,
        backdrop_blur_rect.effective_kernel_levels()
    );
}

#[test]
fn zero_radius_backdrop_blur_rect_has_no_effective_kernel_levels() {
    let mut backdrop_blur_rect = test_backdrop_blur_rect(0.);
    backdrop_blur_rect.blur_radius = ScaledPixels(0.);

    assert_eq!(0, backdrop_blur_rect.effective_kernel_levels());
}

#[test]
fn backdrop_blur_rect_gpu_layout_matches_hlsl() {
    assert_eq!(80, std::mem::size_of::<BackdropBlurRect>());
    assert_eq!(0, std::mem::offset_of!(BackdropBlurRect, order));
    assert_eq!(4, std::mem::offset_of!(BackdropBlurRect, pad));
    assert_eq!(8, std::mem::offset_of!(BackdropBlurRect, bounds));
    assert_eq!(24, std::mem::offset_of!(BackdropBlurRect, content_mask));
    assert_eq!(40, std::mem::offset_of!(BackdropBlurRect, corner_radii));
    assert_eq!(56, std::mem::offset_of!(BackdropBlurRect, blur_radius));
    assert_eq!(60, std::mem::offset_of!(BackdropBlurRect, opacity));
    assert_eq!(64, std::mem::offset_of!(BackdropBlurRect, tint));
}
