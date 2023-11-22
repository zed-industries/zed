#![allow(dead_code, unused_variables)]

mod assets;
mod stories;
mod story_selector;

use std::sync::Arc;

use clap::Parser;
use gpui::{
    div, px, size, AnyView, AppContext, Bounds, Div, Render, ViewContext, VisualContext,
    WindowBounds, WindowOptions,
};
use log::LevelFilter;
use settings2::{default_settings, Settings, SettingsStore};
use simplelog::SimpleLogger;
use theme2::{ThemeRegistry, ThemeSettings};
use ui::prelude::*;

use crate::assets::Assets;
use crate::story_selector::StorySelector;

// gpui::actions! {
//     storybook,
//     [ToggleInspector]
// }

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    story: Option<StorySelector>,

    /// The name of the theme to use in the storybook.
    ///
    /// If not provided, the default theme will be used.
    #[arg(long)]
    theme: Option<String>,
}

fn main() {
    // unsafe { backtrace_on_stack_overflow::enable() };

    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    let args = Args::parse();

    let story_selector = args.story.clone();
    let theme_name = args.theme.unwrap_or("One Dark".to_string());

    let asset_source = Arc::new(Assets);
    gpui::App::production(asset_source).run(move |cx| {
        load_embedded_fonts(cx).unwrap();

        let mut store = SettingsStore::default();
        store
            .set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);

        theme2::init(theme2::LoadThemes::All, cx);

        let selector = story_selector.unwrap_or(StorySelector::KitchenSink);

        let theme_registry = cx.global::<ThemeRegistry>();
        let mut theme_settings = ThemeSettings::get_global(cx).clone();
        theme_settings.active_theme = theme_registry.get(&theme_name).unwrap();
        ThemeSettings::override_global(theme_settings, cx);

        language::init(cx);
        editor::init(cx);

        let window = cx.open_window(
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

                cx.build_view(|cx| StoryWrapper::new(selector.story(cx)))
            },
        );

        cx.activate(true);
    });
}

#[derive(Clone)]
pub struct StoryWrapper {
    story: AnyView,
}

impl StoryWrapper {
    pub(crate) fn new(story: AnyView) -> Self {
        Self { story }
    }
}

impl Render for StoryWrapper {
    type Output = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Output {
        div()
            .flex()
            .flex_col()
            .size_full()
            .font("Zed Mono")
            .child(self.story.clone())
    }
}

fn load_embedded_fonts(cx: &AppContext) -> gpui::Result<()> {
    let font_paths = cx.asset_source().list("fonts")?;
    let mut embedded_fonts = Vec::new();
    for font_path in font_paths {
        if font_path.ends_with(".ttf") {
            let font_bytes = cx.asset_source().load(&font_path)?.to_vec();
            embedded_fonts.push(Arc::from(font_bytes));
        }
    }

    cx.text_system().add_fonts(&embedded_fonts)
}
