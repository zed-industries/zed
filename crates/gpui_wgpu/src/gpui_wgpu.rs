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
    use std::{collections::HashSet, path::PathBuf, sync::Arc};

    struct HeadlessTestView;

    impl Render for HeadlessTestView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
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
        let window = cx.open_window(size(px(320.0), px(180.0)), |_, cx: &mut App| {
            cx.new(|_| HeadlessTestView)
        })?;
        let window = window.into();
        cx.update_window(window, |_, window, cx| {
            window.draw(cx).clear(cx);
        })?;

        let first = cx.capture_screenshot(window)?;
        let second = cx.capture_screenshot(window)?;

        assert_eq!(first.as_raw(), second.as_raw());
        let unique_pixels = first.pixels().map(|pixel| pixel.0).collect::<HashSet<_>>();
        assert!(unique_pixels.len() > 8, "rendered image was blank");

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
