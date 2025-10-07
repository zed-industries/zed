mod actions;
mod app_menus;
mod assets;
mod stories;
mod story_selector;

use std::sync::Arc;

use clap::Parser;
use dialoguer::FuzzySelect;
use gpui::{
    AnyView, App, Bounds, Context, Render, Window, WindowBounds, WindowOptions,
    colors::{Colors, GlobalColors},
    div, px, size,
};
use log::LevelFilter;
use project::Project;
use reqwest_client::ReqwestClient;
use settings::{KeymapFile, Settings};
use simplelog::SimpleLogger;
use strum::IntoEnumIterator;
use theme::ThemeSettings;
use ui::prelude::*;
use workspace;

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

    gpui::Application::new().with_assets(Assets).run(move |cx| {
        load_embedded_fonts(cx).unwrap();

        cx.set_global(GlobalColors(Arc::new(Colors::default())));

        let http_client = ReqwestClient::user_agent("zed_storybook").unwrap();
        cx.set_http_client(Arc::new(http_client));

        settings::init(cx);
        theme::init(theme::LoadThemes::All(Box::new(Assets)), cx);

        let selector = story_selector;

        let mut theme_settings = ThemeSettings::get_global(cx).clone();
        theme_settings.theme =
            theme::ThemeSelection::Static(settings::ThemeName(theme_name.into()));
        ThemeSettings::override_global(theme_settings, cx);

        language::init(cx);
        editor::init(cx);
        Project::init_settings(cx);
        workspace::init_settings(cx);
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
            move |window, cx| {
                theme::setup_ui_font(window, cx);

                cx.new(|cx| StoryWrapper::new(selector.story(window, cx)))
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .font_family(".ZedMono")
            .child(self.story.clone())
    }
}

fn load_embedded_fonts(cx: &App) -> anyhow::Result<()> {
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

fn load_storybook_keymap(cx: &mut App) {
    cx.bind_keys(KeymapFile::load_asset("keymaps/storybook.json", None, cx).unwrap());
}

pub fn init(cx: &mut App) {
    cx.on_action(quit);
}

fn quit(_: &Quit, cx: &mut App) {
    cx.spawn(async move |cx| {
        cx.update(|cx| cx.quit())?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
