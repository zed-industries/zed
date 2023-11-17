use anyhow::Result;
use gpui::AssetSource;
use gpui::{
    div, px, size, AnyView, Bounds, Div, Render, ViewContext, VisualContext, WindowBounds,
    WindowOptions,
};
use settings::{default_settings, Settings, SettingsStore};
use std::borrow::Cow;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::{prelude::*, ContextMenuStory};

struct Assets;

impl AssetSource for Assets {
    fn load(&self, _path: &str) -> Result<Cow<[u8]>> {
        todo!();
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

fn main() {
    let asset_source = Arc::new(Assets);
    gpui::App::production(asset_source).run(move |cx| {
        let mut store = SettingsStore::default();
        store
            .set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);
        ui::settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);

        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Default::default(),
                    size: size(px(1500.), px(780.)).into(),
                }),
                ..Default::default()
            },
            move |cx| {
                let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;
                cx.set_rem_size(ui_font_size);

                cx.build_view(|cx| TestView {
                    story: cx.build_view(|_| ContextMenuStory).into(),
                })
            },
        );

        cx.activate(true);
    })
}

struct TestView {
    story: AnyView,
}

impl Render for TestView {
    type Element = Div<Self>;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        div()
            .flex()
            .flex_col()
            .size_full()
            .font("Helvetica")
            .child(self.story.clone())
    }
}
