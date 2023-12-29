use assets::Assets;
use clap::Parser;
use gpui::{px, App, Bounds, Render, Size, VisualContext, WindowBounds, WindowOptions};
use log::LevelFilter;
use settings::Settings;
use simplelog::SimpleLogger;
use std::sync::Arc;
use theme::{LoadThemes, ThemeRegistry, ThemeSettings};

#[derive(Parser)]
struct Args {
    theme: Option<String>,
}

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).unwrap();

    let args = Args::parse();
    let theme_name = args.theme.unwrap_or("One Light".to_string());

    let assets = Arc::new(Assets);
    App::production(assets.clone()).run(move |cx| {
        assets.load_embedded_fonts(cx);
        settings::init(cx);
        theme::init(LoadThemes::All, cx);

        ThemeSettings::override_global_with(cx, |settings, cx| {
            settings.active_theme = ThemeRegistry::global(cx).get(&theme_name).unwrap()
        });

        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Default::default(),
                    size: Size {
                        width: px(1500.),
                        height: px(780.),
                    }
                    .into(),
                }),
                ..Default::default()
            },
            move |cx| {
                let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;
                cx.set_rem_size(ui_font_size);
                cx.build_view(|cx| TitlebarExample)
            },
        );

        cx.activate(true);
    })
}

struct TitlebarExample;

impl Render for TitlebarExample {
    type Element = ();

    fn render(&mut self, cx: &mut ui::prelude::ViewContext<Self>) -> Self::Element {
        todo!()
    }
}
