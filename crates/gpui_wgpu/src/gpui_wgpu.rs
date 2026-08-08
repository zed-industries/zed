mod cosmic_text_system;
mod wgpu_atlas;
mod wgpu_context;
mod wgpu_renderer;

pub use cosmic_text_system::*;
pub use wgpu;
pub use wgpu_atlas::*;
pub use wgpu_context::*;
#[cfg(all(not(target_family = "wasm"), any(test, feature = "test-support")))]
pub use wgpu_renderer::WgpuHeadlessRenderer;
pub use wgpu_renderer::{GpuContext, WgpuRenderer, WgpuSurfaceConfig};

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use gpui::{
        App, Context, HeadlessAppContext, IntoElement, Render, Window, div, prelude::*, px, rgb,
        size,
    };
    use std::{path::PathBuf, sync::Arc};

    struct HeadlessTestView;

    impl Render for HeadlessTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .w(px(320.0))
                        .h(px(176.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(rgb(0x172033))
                        .font_family("DejaVu Sans")
                        .child(
                            div()
                                .w(px(280.0))
                                .h(px(112.0))
                                .p_5()
                                .rounded_lg()
                                .shadow_lg()
                                .bg(rgb(0xf4f7fb))
                                .text_color(rgb(0x182230))
                                .child(
                                    div()
                                        .flex()
                                        .gap_2()
                                        .mb_4()
                                        .child(div().size(px(20.0)).rounded_sm().bg(rgb(0xef4444)))
                                        .child(div().size(px(20.0)).rounded_sm().bg(rgb(0x22c55e)))
                                        .child(div().size(px(20.0)).rounded_sm().bg(rgb(0x3b82f6))),
                                )
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .child("Deterministic wgpu rendering"),
                                ),
                        ),
                )
        }
    }

    #[test]
    fn test_headless_rendering_is_deterministic() -> anyhow::Result<()> {
        if let Err(error) = env_logger::builder()
            .is_test(true)
            .filter_module("gpui_wgpu", log::LevelFilter::Info)
            .try_init()
        {
            log::debug!("Test logger was already initialized: {error}");
        }

        let text_system = Arc::new(CosmicTextSystem::new("DejaVu Sans"));
        let mut cx = HeadlessAppContext::with_platform(text_system, Arc::new(()), || {
            Some(Box::new(
                WgpuHeadlessRenderer::new().expect("failed to create wgpu headless renderer"),
            ))
        });
        let window = cx.open_window(size(px(324.0), px(180.0)), |_, cx: &mut App| {
            cx.new(|_| HeadlessTestView)
        })?;
        let window = window.into();
        cx.update_window(window, |_, window, cx| {
            window.draw(cx).clear(cx);
        })?;

        let first = cx.capture_screenshot(window)?;
        let second = cx.capture_screenshot(window)?;

        assert_eq!(first.as_raw(), second.as_raw());
        assert_eq!(first.dimensions(), (648, 360));
        assert_eq!(first.get_pixel(0, 0).0, [0, 0, 0, 255]);
        assert_eq!(first.get_pixel(10, 10).0, [0x17, 0x20, 0x33, 255]);
        assert_eq!(first.get_pixel(104, 128).0, [0xef, 0x44, 0x44, 255]);
        assert_eq!(first.get_pixel(160, 128).0, [0x22, 0xc5, 0x5e, 255]);
        assert_eq!(first.get_pixel(216, 128).0, [0x3b, 0x82, 0xf6, 255]);

        let text_ink_pixels = {
            let image = &first;
            (180..250)
                .flat_map(|y| (90..590).map(move |x| image.get_pixel(x, y).0))
                .filter(|pixel| pixel[0] < 100 && pixel[1] < 100 && pixel[2] < 100)
                .count()
        };
        assert!(text_ink_pixels > 100, "rendered text was missing");

        let background = [0x17, 0x20, 0x33, 255];
        let shadow_pixels = {
            let image = &first;
            (294..320)
                .flat_map(|y| (44..604).map(move |x| image.get_pixel(x, y).0))
                .filter(|pixel| *pixel != background)
                .count()
        };
        assert!(shadow_pixels > 100, "rendered shadow was missing");

        if let Some(path) = std::env::var_os("GPUI_HEADLESS_EVIDENCE_PATH") {
            let path = PathBuf::from(path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            first.save(path)?;
        }

        Ok(())
    }
}
