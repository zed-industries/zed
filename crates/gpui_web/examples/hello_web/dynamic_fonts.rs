use anyhow::{Result, ensure};
use futures::AsyncReadExt;
use gpui::{
    App, Bounds, Context, Render, Subscription, Task, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpui_util::ResultExt;
use std::borrow::Cow;

// Pin the raw TTF rather than using Google Fonts CSS, which may supply WOFF2
// subsets that cannot be passed directly to Cosmic Text.
const FONT_URL: &str = "https://fonts.gstatic.com/s/notosansdevanagari/v30/TuGoUUFzXI5FBtUq5a8bjKYTZjtRU6Sgv3NaV_SNmI0b8QQCQmHn6B2OHjbL_08AlXQly-A.ttf";
const SAMPLE: &str = "नमस्ते 😀 दुनिया 👨‍👩‍👧‍👦 नमस्ते ❤️";

struct DynamicFonts {
    status: String,
    download: Option<Task<()>>,
    _missing_glyphs: Subscription,
}

impl DynamicFonts {
    fn new(cx: &mut Context<Self>) -> Self {
        let view = cx.weak_entity();
        let subscription = cx.on_missing_glyphs(move |missing, cx| {
            if missing.iter().any(|glyph| {
                glyph
                    .grapheme()
                    .chars()
                    .any(|character| ('\u{0900}'..='\u{097f}').contains(&character))
            }) {
                view.update(cx, |view, cx| view.load_font(cx)).log_err();
            }
        });
        Self {
            status: "Waiting for a missing Devanagari glyph…".into(),
            download: None,
            _missing_glyphs: subscription,
        }
    }

    fn load_font(&mut self, cx: &mut Context<Self>) {
        // Many missing graphemes can request the same font, including after a failed
        // download. Keep the task so this demo makes only one attempt per page load.
        if self.download.is_some() {
            return;
        }
        self.status = "Downloading Noto Sans Devanagari (219 KB)…".into();
        cx.notify();
        let http_client = cx.http_client();
        self.download = Some(cx.spawn(async move |view, cx| {
            let result: Result<Vec<u8>> = async {
                let mut response = http_client.get(FONT_URL, Default::default(), true).await?;
                ensure!(
                    response.status().is_success(),
                    "Font request failed: HTTP {}",
                    response.status()
                );
                let mut bytes = Vec::new();
                response.body_mut().read_to_end(&mut bytes).await?;
                ensure!(!bytes.is_empty(), "Font response was empty");
                Ok(bytes)
            }
            .await;
            view.update(cx, |view, cx| {
                let result =
                    result.and_then(|bytes| cx.text_system().add_fonts(vec![Cow::Owned(bytes)]));
                view.status = match result {
                    Ok(()) => "Font loaded. The same text is now shaped by Cosmic Text.".into(),
                    Err(error) => format!("Font loading failed: {error:#}. Reload to retry."),
                };
                cx.refresh_windows();
                cx.notify();
            })
            .log_err();
        }));
    }
}

impl Render for DynamicFonts {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_4()
            .p_8()
            .bg(rgb(0x202030))
            .text_color(rgb(0xffffff))
            .font_family("IBM Plex Sans")
            .child(div().text_2xl().child("On-demand font loading"))
            .child("Only IBM Plex Sans is bundled. Missing Devanagari triggers the font download.")
            .child(div().text_size(px(40.)).child(SAMPLE))
            .child(
                div()
                    .text_size(px(32.))
                    .child("Emoji still uses Canvas: 😀 👨‍👩‍👧‍👦"),
            )
            .child(self.status.clone())
            .child("Source: Google Fonts CDN · Noto Sans Devanagari · SIL Open Font License")
            .child(
                "Use the browser Network panel to throttle or block the font request, then reload.",
            )
    }
}

fn main() {
    gpui_platform::web_init();
    gpui_platform::application().run(|cx: &mut App| {
        if cx
            .text_system()
            .add_fonts(vec![Cow::Borrowed(include_bytes!(
                "../../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf"
            ))])
            .log_err()
            .is_none()
        {
            return;
        }
        let bounds = Bounds::centered(None, size(px(900.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(DynamicFonts::new),
        )
        .log_err();
        cx.activate(true);
    });
}
