use std::{path::Path, sync::Arc, time::Duration};

use gpui::{
    div, img, prelude::*, pulsating_between, px, red, size, Animation, AnimationExt, App,
    AppContext, Asset, AssetSource, Bounds, ImageAsset, ImageCacheError, InteractiveElement as _,
    IntoElement, Length, ParentElement, Pixels, Render, RenderImage, SharedString, Styled,
    UriOrPath, ViewContext, VisualContext as _, WindowBounds, WindowContext, WindowOptions,
    LOADING_DELAY,
};

struct Assets {}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entry| {
                Some(SharedString::from(
                    entry.ok()?.path().to_string_lossy().to_string(),
                ))
            })
            .collect::<Vec<_>>())
    }
}

const IMAGE: &str = "examples/image/app-icon.png";

#[derive(Clone, Hash)]
struct LoadImageParameters {
    timeout: Duration,
    fail: bool,
}

struct LoadImageWithParameters {}

impl Asset for LoadImageWithParameters {
    type Source = LoadImageParameters;

    type Output = Result<Arc<RenderImage>, ImageCacheError>;

    fn load(
        parameters: Self::Source,
        cx: &mut AppContext,
    ) -> impl std::future::Future<Output = Self::Output> + Send + 'static {
        let timer = cx.background_executor().timer(parameters.timeout);
        let data = ImageAsset::load(
            UriOrPath::Path(Arc::new(Path::new(IMAGE).to_path_buf())),
            cx,
        );
        async move {
            timer.await;
            if parameters.fail {
                Err(ImageCacheError::Asset("Failed to load image".into()))
            } else {
                data.await
            }
        }
    }
}

struct ImageLoadingExample {}

impl ImageLoadingExample {
    fn loading_element() -> impl IntoElement {
        div()
            .border_1()
            .border_color(gpui::yellow())
            .size_full()
            .flex_none()
            .bg(gpui::blue())
            .with_animation(
                "pulsating-bg",
                Animation::new(Duration::from_secs(2))
                    .repeat()
                    .with_easing(pulsating_between(0.3, 0.7)),
                move |this, delta| this.bg(gpui::blue().opacity(0.8 - delta)),
            )
    }

    fn fallback_element() -> impl IntoElement {
        div()
            .border_1()
            .border_color(gpui::red())
            .size_full()
            .flex_none()
            .bg(gpui::red())
    }
}

impl Render for ImageLoadingExample {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().flex().flex_col().size_full().justify_around().child(
            div().flex().flex_row().w_full().justify_around().child(
                div()
                    .flex()
                    .bg(gpui::white())
                    .size(Length::Definite(Pixels(300.0).into()))
                    .justify_center()
                    .items_center()
                    .child(
                        // Load within the 'loading delay', should not show loading fallback
                        img(|cx: &mut WindowContext| {
                            cx.use_asset::<LoadImageWithParameters>(&LoadImageParameters {
                                timeout: LOADING_DELAY.saturating_sub(Duration::from_millis(25)),
                                fail: false,
                            })
                            .map(|result| result.map_err(|e| e.into()))
                        })
                        .id("image-1")
                        .border_1()
                        .size_12()
                        .with_fallback(|| Self::fallback_element().into_any_element())
                        .border_color(red())
                        .with_loading(|| Self::loading_element().into_any_element()),
                    )
                    .child(
                        // Load after a long delay
                        img(|cx: &mut WindowContext| {
                            cx.use_asset::<LoadImageWithParameters>(&LoadImageParameters {
                                timeout: Duration::from_secs(5),
                                fail: false,
                            })
                            .map(|result| result.map_err(|e| e.into()))
                        })
                        .id("image-2")
                        .with_fallback(|| Self::fallback_element().into_any_element())
                        .with_loading(|| Self::loading_element().into_any_element())
                        .size_12()
                        .border_1()
                        .border_color(red()),
                    )
                    .child(
                        // Fail to load after a long delay
                        img(|cx: &mut WindowContext| {
                            cx.use_asset::<LoadImageWithParameters>(&LoadImageParameters {
                                timeout: Duration::from_secs(5),
                                fail: true,
                            })
                            .map(|result| result.map_err(|e| e.into()))
                        })
                        .id("image-3")
                        .with_fallback(|| Self::fallback_element().into_any_element())
                        .with_loading(|| Self::loading_element().into_any_element())
                        .size_12()
                        .border_1()
                        .border_color(red()),
                    ),
            ),
        )
    }
}

fn main() {
    App::new()
        .with_assets(Assets {})
        .run(|cx: &mut AppContext| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(300.), Pixels(300.)),
                    cx,
                ))),
                ..Default::default()
            };
            cx.open_window(options, |cx| {
                cx.activate(false);
                cx.new_view(|_cx| ImageLoadingExample {})
            })
            .unwrap();
        });
}
