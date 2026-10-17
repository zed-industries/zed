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
    use gpui::PlatformHeadlessRenderer as _;
    let mut renderer = crate::WgpuHeadlessRenderer::new()
        .expect("rounded overflow pixel tests require a GPU adapter");
    fixtures::render_with(cx, build, |scene, size| {
        renderer
            .render_scene_to_image(scene, size)
            .unwrap()
            .into_raw()
    })
}

mod texture_transport {
    use super::{assert_same_pixels, pixel};
    mod fixtures {
        include!("../../gpui/src/rounded_clip_fixtures.rs");
    }
    fn texture_renderer() -> crate::WgpuHeadlessRenderer {
        let mut renderer =
            crate::WgpuHeadlessRenderer::new().expect("instance texture pixel tests require a GPU");
        let core = &mut renderer.core;
        let device = &core.resources.device;
        let layouts = super::super::WgpuRendererCore::create_bind_group_layouts(device, true);
        let pipelines = super::super::WgpuRendererCore::create_pipelines(
            device,
            &layouts,
            core.target_format,
            wgpu::CompositeAlphaMode::Opaque,
            core.rendering_params.path_sample_count,
            false,
            true,
        );
        let (instances, capacity) = super::super::WgpuRendererCore::create_instance_texture(
            device,
            1024,
            device.limits().max_texture_dimension_2d,
        );
        core.resources.bind_group_layouts = layouts;
        core.resources.pipelines = pipelines;
        core.resources.instance_data = instances;
        core.instance_data_capacity = capacity;
        core.instance_data_alignment = 16;
        core.uses_webgl_instance_data = true;
        renderer
    }
    #[gpui::test]
    fn rounded_overflow_cached_child_follows_ancestor_clip(cx: &mut gpui::TestAppContext) {
        use gpui::PlatformHeadlessRenderer as _;
        let mut renderer = texture_renderer();
        fixtures::retained_frames(cx, |scene, size| {
            renderer
                .render_scene_to_image(scene, size)
                .unwrap()
                .into_raw()
        });
    }
    fn render_div_fixture(
        cx: &mut gpui::TestAppContext,
        build: fn() -> gpui::Div,
    ) -> (Vec<u8>, usize) {
        use gpui::PlatformHeadlessRenderer as _;
        let mut renderer = texture_renderer();
        fixtures::render_with(cx, build, |scene, size| {
            renderer
                .render_scene_to_image(scene, size)
                .unwrap()
                .into_raw()
        })
    }
}

#[gpui::test]
fn rounded_overflow_cached_child_follows_ancestor_clip(cx: &mut gpui::TestAppContext) {
    use gpui::PlatformHeadlessRenderer as _;
    let mut renderer =
        crate::WgpuHeadlessRenderer::new().expect("cached clipping test requires a GPU");
    fixtures::retained_frames(cx, |scene, size| {
        renderer
            .render_scene_to_image(scene, size)
            .unwrap()
            .into_raw()
    });
}
