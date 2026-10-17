use gpui::TestAppContext;
#[path = "../../gpui/src/rounded_clip_fixtures.rs"]
mod fixtures;
fn pixel(pixels: &[u8], width: usize, x: usize, y: usize) -> [u8; 4] {
    pixels[(y * width + x) * 4..(y * width + x + 1) * 4]
        .try_into()
        .unwrap()
}
fn assert_same_pixels(actual: &[u8], expected: &[u8], _width: usize) {
    assert_eq!(actual, expected);
}
fn render_div_fixture(cx: &mut TestAppContext, build: fn() -> gpui::Div) -> (Vec<u8>, usize) {
    let mut renderer = crate::metal_renderer::MetalRenderer::new_headless(std::sync::Arc::new(
        parking_lot::Mutex::new(Default::default()),
    ));
    fixtures::render_with(cx, build, |scene, size| {
        renderer
            .render_scene_to_image(scene, size)
            .unwrap()
            .into_raw()
    })
}

#[gpui::test]
fn rounded_overflow_cached_child_follows_ancestor_clip(cx: &mut gpui::TestAppContext) {
    let mut renderer = crate::metal_renderer::MetalRenderer::new_headless(std::sync::Arc::new(
        parking_lot::Mutex::new(Default::default()),
    ));
    fixtures::retained_frames(cx, |scene, size| {
        renderer
            .render_scene_to_image(scene, size)
            .unwrap()
            .into_raw()
    });
}
