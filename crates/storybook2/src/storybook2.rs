mod stories;
mod story_selector;
use crate::story_selector::{ComponentStory, StorySelector};
use assets::Assets;
use clap::Parser;
use dialoguer::FuzzySelect;
use gpui::{
    div, px, size, AnyView, Bounds, Div, Render, ViewContext, VisualContext, WindowBounds,
    WindowOptions,
};
pub use indoc::indoc;
use log::LevelFilter;
use settings2::Settings;
use simplelog::SimpleLogger;
use std::sync::Arc;
use strum::IntoEnumIterator;
use theme2::{ThemeRegistry, ThemeSettings};
use ui::prelude::*;

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

    let story_selector = args.story.clone().unwrap_or_else(|| {
        let stories = ComponentStory::iter().collect::<Vec<_>>();

        let selection = FuzzySelect::new()
            .with_prompt("Choose a story to rungit :")
            .items(&stories)
            .interact()
            .unwrap();

        StorySelector::Component(stories[selection])
    });
    let theme_name = args.theme.unwrap_or("One Dark".to_string());

    let assets = Arc::new(Assets);
    gpui::App::production(assets.clone()).run(move |cx| {
        assets.load_embedded_fonts(cx);
        settings2::init(cx);
        theme2::init(theme2::LoadThemes::All, cx);

        let selector = story_selector;

        let theme_registry = cx.global::<ThemeRegistry>();
        let mut theme_settings = ThemeSettings::get_global(cx).clone();
        theme_settings.active_theme = theme_registry.get(&theme_name).unwrap();
        ThemeSettings::override_global(theme_settings, cx);

        language::init(cx);
        editor::init(cx);

        let _window = cx.open_window(
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
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        div()
            .flex()
            .flex_col()
            .size_full()
            .font("Zed Mono")
            .child(self.story.clone())
    }
}
