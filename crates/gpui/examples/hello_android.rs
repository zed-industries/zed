//! Interactive **GPUI Android Gallery** — exercises the platform's input
//! routing, state-bound rendering, theme switching and runtime introspection
//! on a real device.
//!
//! Built as a `cdylib` so the JVM side (`GameActivity` →
//! `System.loadLibrary("hello_android")`) can find `android_main`. See
//! `crates/gpui_android/README.md` for the full bring-up flow + a Gradle
//! harness sits next door at `examples/android-host/`.
//!
//! Quick cycle:
//!
//! ```text
//! script/build-gpui-android-apk debug   # cargo build → APK
//! adb install -r .../app-debug.apk
//! adb shell am start -n dev.zed.gpui.gallery/.GalleryActivity
//! ```
#![cfg(target_os = "android")]

use gpui::{
    App, Bounds, ClickEvent, Context, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

/// Top-level state for the gallery view. Mutable fields drive re-renders
/// when paired with `cx.notify()` inside click listeners.
struct Gallery {
    counter: i32,
    accent: AccentColor,
    appearance: Appearance,
    last_action: SharedString,
}

/// One of a fixed palette of accent colors the user can pick.
#[derive(Clone, Copy, PartialEq, Eq)]
enum AccentColor {
    Crimson,
    Tangerine,
    Forest,
    Cobalt,
    Violet,
}

impl AccentColor {
    const ALL: &'static [AccentColor] = &[
        AccentColor::Crimson,
        AccentColor::Tangerine,
        AccentColor::Forest,
        AccentColor::Cobalt,
        AccentColor::Violet,
    ];

    fn rgb(&self) -> u32 {
        match self {
            AccentColor::Crimson => 0xef4444,
            AccentColor::Tangerine => 0xf97316,
            AccentColor::Forest => 0x10b981,
            AccentColor::Cobalt => 0x3b82f6,
            AccentColor::Violet => 0xa855f7,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            AccentColor::Crimson => "crimson",
            AccentColor::Tangerine => "tangerine",
            AccentColor::Forest => "forest",
            AccentColor::Cobalt => "cobalt",
            AccentColor::Violet => "violet",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Appearance {
    Light,
    Dark,
}

impl Appearance {
    fn surface(&self) -> u32 {
        match self {
            Appearance::Light => 0xfafafa,
            Appearance::Dark => 0x14171a,
        }
    }
    fn elevated(&self) -> u32 {
        match self {
            Appearance::Light => 0xffffff,
            Appearance::Dark => 0x21262d,
        }
    }
    fn text_primary(&self) -> u32 {
        match self {
            Appearance::Light => 0x111827,
            Appearance::Dark => 0xf3f4f6,
        }
    }
    fn text_secondary(&self) -> u32 {
        match self {
            Appearance::Light => 0x6b7280,
            Appearance::Dark => 0x9ca3af,
        }
    }
    fn border(&self) -> u32 {
        match self {
            Appearance::Light => 0xe5e7eb,
            Appearance::Dark => 0x374151,
        }
    }
}

impl Gallery {
    fn new() -> Self {
        Self {
            counter: 0,
            accent: AccentColor::Cobalt,
            appearance: Appearance::Dark,
            last_action: "tap a button".into(),
        }
    }

    fn on_increment(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.counter += 1;
        self.last_action = format!("counter +1 → {}", self.counter).into();
        cx.notify();
    }

    fn on_decrement(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.counter -= 1;
        self.last_action = format!("counter −1 → {}", self.counter).into();
        cx.notify();
    }

    fn on_reset(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.counter = 0;
        self.last_action = "counter reset".into();
        cx.notify();
    }

    fn on_toggle_appearance(
        &mut self,
        _: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.appearance = match self.appearance {
            Appearance::Dark => Appearance::Light,
            Appearance::Light => Appearance::Dark,
        };
        self.last_action = format!(
            "appearance → {}",
            match self.appearance {
                Appearance::Dark => "dark",
                Appearance::Light => "light",
            }
        )
        .into();
        cx.notify();
    }

    fn pick_color(&mut self, color: AccentColor, cx: &mut Context<Self>) {
        self.accent = color;
        self.last_action = format!("accent → {}", color.name()).into();
        cx.notify();
    }

    fn header(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_3xl()
                    .text_color(rgb(self.appearance.text_primary()))
                    .child("GPUI Android"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(self.appearance.text_secondary()))
                    .child("interactive gallery"),
            )
    }

    fn counter_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let accent = rgb(self.accent.rgb());
        section(self.appearance, "counter")
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .items_center()
                    .child(button(
                        self.appearance,
                        "−",
                        cx.listener(Self::on_decrement),
                    ))
                    .child(
                        div()
                            .min_w(px(96.))
                            .text_3xl()
                            .text_center()
                            .text_color(accent)
                            .child(format!("{}", self.counter)),
                    )
                    .child(button(
                        self.appearance,
                        "+",
                        cx.listener(Self::on_increment),
                    ))
                    .child(
                        div().w(px(8.)), // spacer
                    )
                    .child(button(
                        self.appearance,
                        "reset",
                        cx.listener(Self::on_reset),
                    )),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(self.appearance.text_secondary()))
                    .child("tap +/− on the device — touch is routed as a left-click"),
            )
    }

    fn appearance_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let label = match self.appearance {
            Appearance::Dark => "switch to light",
            Appearance::Light => "switch to dark",
        };
        section(self.appearance, "appearance")
            .child(button(
                self.appearance,
                label,
                cx.listener(Self::on_toggle_appearance),
            ))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(self.appearance.text_secondary()))
                    .child("re-renders every panel from a single state field"),
            )
    }

    fn accent_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let appearance = self.appearance;
        let current = self.accent;
        let mut row = div().flex().flex_row().gap_3().items_center();
        for color in AccentColor::ALL {
            let color = *color;
            let selected = color == current;
            row = row.child(
                div()
                    .id(SharedString::from(format!("swatch-{}", color.name())))
                    .size_10()
                    .rounded_lg()
                    .bg(rgb(color.rgb()))
                    .border_2()
                    .border_color(if selected {
                        rgb(appearance.text_primary())
                    } else {
                        rgb(appearance.border())
                    })
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| this.pick_color(color, cx))),
            );
        }
        section(appearance, "accent")
            .child(row)
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(appearance.text_secondary()))
                    .child(format!("current: {}", current.name())),
            )
    }

    fn info_section(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let lines = info_lines();
        let appearance = self.appearance;
        let mut col = div().flex().flex_col().gap_1();
        for (k, v) in lines {
            col = col.child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(
                        div()
                            .min_w(px(120.))
                            .text_xs()
                            .text_color(rgb(appearance.text_secondary()))
                            .child(k),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(appearance.text_primary()))
                            .child(v),
                    ),
            );
        }
        section(appearance, "device").child(col)
    }

    fn scroll_section(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let appearance = self.appearance;
        let accent = rgb(self.accent.rgb());
        let mut col = div().flex().flex_col().gap_2();
        for i in 0..30 {
            col = col.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .px(px(12.))
                    .py(px(10.))
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(appearance.border()))
                    .bg(rgb(appearance.surface()))
                    .child(
                        div()
                            .min_w(px(28.))
                            .text_sm()
                            .text_color(accent)
                            .child(format!("#{i:02}")),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(appearance.text_primary()))
                            .child(format!("scroll item {i} — drag to verify")),
                    ),
            );
        }
        section(appearance, "scroll test")
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(appearance.text_secondary()))
                    .child("drag a finger up/down — touch is synthesised as ScrollWheel events"),
            )
            .child(col)
    }

    fn footer(&self) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(self.appearance.text_secondary()))
            .child(format!("last action: {}", self.last_action))
    }
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let appearance = self.appearance;
        // `overflow_y_scroll` requires a stateful element (hence `.id(...)`).
        // The synthetic ScrollWheel events emitted from
        // `AndroidWindow::dispatch_input` drive the scroll position.
        div()
            .id("gallery-root")
            .size_full()
            .bg(rgb(appearance.surface()))
            .pt(px(80.)) // status-bar inset
            .pb(px(48.)) // gesture/nav-bar inset
            .px(px(20.))
            .flex()
            .flex_col()
            .gap_5()
            .overflow_y_scroll()
            .child(self.header())
            .child(self.counter_section(cx))
            .child(self.appearance_section(cx))
            .child(self.accent_section(cx))
            .child(self.scroll_section(cx))
            .child(self.info_section(cx))
            .child(self.footer())
    }
}

/// Wraps an inner panel in the surface/border/padding of a "card" with a
/// section title above it. Centralises the visual identity so individual
/// sections stay focused on their own state.
fn section(
    appearance: Appearance,
    title: &'static str,
) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .p(px(16.))
        .rounded_lg()
        .border_1()
        .border_color(rgb(appearance.border()))
        .bg(rgb(appearance.elevated()))
        .child(
            div()
                .text_xs()
                .text_color(rgb(appearance.text_secondary()))
                .child(title.to_string().to_uppercase()),
        )
}

/// Pill button rendered with the current appearance. Elevated background +
/// subtle border so it's tappable on either theme without designing a
/// separate widget set.
fn button(
    appearance: Appearance,
    label: &str,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> gpui::Stateful<gpui::Div> {
    let label_string = label.to_string();
    div()
        .id(SharedString::from(format!("btn-{label_string}")))
        .px(px(20.))
        .py(px(10.))
        .rounded_md()
        .border_1()
        .border_color(rgb(appearance.border()))
        .bg(rgb(appearance.surface()))
        .text_sm()
        .text_color(rgb(appearance.text_primary()))
        .cursor_pointer()
        .hover(|s| s.opacity(0.8))
        .child(label_string)
        .on_click(on_click)
}

/// Static facts about the host environment, captured once at first paint.
fn info_lines() -> Vec<(&'static str, String)> {
    let app = gpui_android::android_app();
    let config = app.as_ref().map(|a| a.config());
    let density = config.as_ref().and_then(|c| c.density()).unwrap_or(0);
    let scale = if density > 0 {
        density as f32 / 160.0
    } else {
        1.0
    };
    let sdk = config.as_ref().map(|c| c.sdk_version()).unwrap_or(0);
    vec![
        ("renderer", "wgpu / Vulkan".into()),
        (
            "scale factor",
            format!("{scale:.2} (density {density})"),
        ),
        ("sdk version", format!("API {sdk}")),
        ("backend", "android-activity / GameActivity".into()),
        ("text", "cosmic-text + /system/fonts".into()),
    ]
}

/// Entry point invoked by the `android-activity` glue once `GameActivity`
/// has spawned its native thread.
#[unsafe(no_mangle)]
fn android_main(app: gpui_android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    log::info!("hello_android: android_main entered");

    gpui_android::set_android_app(app);

    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(360.), px(640.)), cx);
        if let Err(error) = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Gallery::new()),
        ) {
            log::error!("failed to open Android window: {error:#}");
        }
        cx.activate(true);
    });
}
