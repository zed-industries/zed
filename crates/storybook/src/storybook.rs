mod actions;
mod app_menus;
mod assets;
mod stories;
mod story_selector;

use clap::Parser;
use dialoguer::FuzzySelect;
use gpui::{
    div, px, size, AnyView, AppContext, Bounds, Render, ViewContext, VisualContext, WindowBounds,
    WindowOptions,
};
use log::LevelFilter;
use project::Project;
use settings::{KeymapFile, Settings};
use simplelog::SimpleLogger;
use strum::IntoEnumIterator;
use theme::{ThemeRegistry, ThemeSettings};
use ui::prelude::*;

use crate::app_menus::app_menus;
use crate::assets::Assets;
use crate::story_selector::{ComponentStory, StorySelector};
use actions::Quit;
pub use indoc::indoc;

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
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    menu::init();
    let args = Args::parse();

    let story_selector = args.story.unwrap_or_else(|| {
        let stories = ComponentStory::iter().collect::<Vec<_>>();

        ctrlc::set_handler(move || {}).unwrap();

        let result = FuzzySelect::new()
            .with_prompt("Choose a story to run:")
            .items(&stories)
            .interact();

        let Ok(selection) = result else {
            dialoguer::console::Term::stderr().show_cursor().unwrap();
            std::process::exit(0);
        };

        StorySelector::Component(stories[selection])
    });
    let theme_name = args.theme.unwrap_or("One Dark".to_string());

    gpui::App::new().with_assets(Assets).run(move |cx| {
        load_embedded_fonts(cx).unwrap();

        settings::init(cx);
        theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);

        let selector = story_selector;

        let theme_registry = ThemeRegistry::global(cx);
        let mut theme_settings = ThemeSettings::get_global(cx).clone();
        theme_settings.active_theme = theme_registry.get(&theme_name).unwrap();
        ThemeSettings::override_global(theme_settings, cx);

        language::init(cx);
        editor::init(cx);
        Project::init_settings(cx);
        init(cx);
        load_storybook_keymap(cx);
        cx.set_menus(app_menus());

        let size = size(px(1500.), px(780.));
        let bounds = Bounds::centered(None, size, cx);
        let _window = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |cx| {
                let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;
                cx.set_rem_size(ui_font_size);

                cx.new_view(|cx| StoryWrapper::new(selector.story(cx)))
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
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .font_family("Zed Mono")
            .child(self.story.clone())
    }
}

fn load_embedded_fonts(cx: &AppContext) -> gpui::Result<()> {
    let font_paths = cx.asset_source().list("fonts")?;
    let mut embedded_fonts = Vec::new();
    for font_path in font_paths {
        if font_path.ends_with(".ttf") {
            let font_bytes = cx
                .asset_source()
                .load(&font_path)?
                .expect("Should never be None in the storybook");
            embedded_fonts.push(font_bytes);
        }
    }

    cx.text_system().add_fonts(embedded_fonts)
}

fn load_storybook_keymap(cx: &mut AppContext) {
    KeymapFile::load_asset("keymaps/storybook.json", cx).unwrap();
}

pub fn init(cx: &mut AppContext) {
    cx.on_action(quit);
}

fn quit(_: &Quit, cx: &mut AppContext) {
    cx.spawn(|cx| async move {
        cx.update(|cx| cx.quit())?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
