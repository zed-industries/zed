#![allow(dead_code, unused_variables)]

mod stories;
mod story;
mod story_selector;

use std::sync::Arc;

use ::theme as legacy_theme;
use clap::Parser;
use gpui2::{serde_json, vec2f, view, Element, IntoElement, RectF, ViewContext, WindowBounds};
use legacy_theme::{ThemeRegistry, ThemeSettings};
use log::LevelFilter;
use settings::{default_settings, SettingsStore};
use simplelog::SimpleLogger;
use ui::{ElementExt, Theme, WorkspaceElement};

use crate::story_selector::{ComponentStory, ElementStory, StorySelector};

gpui2::actions! {
    storybook,
    [ToggleInspector]
}

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

    let args = Args::parse();

    gpui2::App::new(Assets).unwrap().run(move |cx| {
        let mut store = SettingsStore::default();
        store
            .set_default_settings(default_settings().as_ref(), cx)
            .unwrap();
        cx.set_global(store);
        legacy_theme::init(Assets, cx);
        // load_embedded_fonts(cx.platform().as_ref());

        let theme_registry = cx.global::<Arc<ThemeRegistry>>();

        let theme_override = args
            .theme
            .and_then(|theme| {
                theme_registry
                    .list_names(true)
                    .find(|known_theme| theme == *known_theme)
            })
            .and_then(|theme_name| theme_registry.get(&theme_name).ok());

        cx.add_window(
            gpui2::WindowOptions {
                bounds: WindowBounds::Fixed(RectF::new(vec2f(0., 0.), vec2f(1700., 980.))),
                center: true,
                ..Default::default()
            },
            |cx| match args.story {
                Some(StorySelector::Element(ElementStory::Avatar)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::elements::avatar::AvatarStory::default(),
                    )
                }),
                Some(StorySelector::Element(ElementStory::Button)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::elements::button::ButtonStory::default(),
                    )
                }),
                Some(StorySelector::Element(ElementStory::Icon)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::elements::icon::IconStory::default(),
                    )
                }),
                Some(StorySelector::Element(ElementStory::Input)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::elements::input::InputStory::default(),
                    )
                }),
                Some(StorySelector::Element(ElementStory::Label)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::elements::label::LabelStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::AssistantPanel)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::assistant_panel::AssistantPanelStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Breadcrumb)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::breadcrumb::BreadcrumbStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Buffer)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::buffer::BufferStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::ChatPanel)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::chat_panel::ChatPanelStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::CollabPanel)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::collab_panel::CollabPanelStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Facepile)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::facepile::FacepileStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Keybinding)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::keybinding::KeybindingStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Palette)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::palette::PaletteStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Panel)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::panel::PanelStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::ProjectPanel)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::project_panel::ProjectPanelStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::StatusBar)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::status_bar::StatusBarStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Tab)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::tab::TabStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::TabBar)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::tab_bar::TabBarStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Terminal)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::terminal::TerminalStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::TitleBar)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::title_bar::TitleBarStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Toolbar)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::toolbar::ToolbarStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::TrafficLights)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::traffic_lights::TrafficLightsStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::ContextMenu)) => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        stories::components::context_menu::ContextMenuStory::default(),
                    )
                }),
                None => view(move |cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        theme_override.clone(),
                        WorkspaceElement::default(),
                    )
                }),
            },
        );
        cx.platform().activate(true);
    });
}

fn render_story<V: 'static, S: IntoElement<V>>(
    cx: &mut ViewContext<V>,
    theme_override: Option<Arc<legacy_theme::Theme>>,
    story: S,
) -> impl Element<V> {
    let theme = current_theme(cx, theme_override);

    story.into_element().themed(theme)
}

fn current_theme<V: 'static>(
    cx: &mut ViewContext<V>,
    theme_override: Option<Arc<legacy_theme::Theme>>,
) -> Theme {
    let legacy_theme =
        theme_override.unwrap_or_else(|| settings::get::<ThemeSettings>(cx).theme.clone());

    let new_theme: Theme = serde_json::from_value(legacy_theme.base_theme.clone()).unwrap();

    add_base_theme_to_legacy_theme(&legacy_theme, new_theme)
}

// Nathan: During the transition to gpui2, we will include the base theme on the legacy Theme struct.
fn add_base_theme_to_legacy_theme(legacy_theme: &legacy_theme::Theme, new_theme: Theme) -> Theme {
    legacy_theme
        .deserialized_base_theme
        .lock()
        .get_or_insert_with(|| Box::new(new_theme))
        .downcast_ref::<Theme>()
        .unwrap()
        .clone()
}

use anyhow::{anyhow, Result};
use gpui2::AssetSource;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "themes/**/*"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<std::borrow::Cow<[u8]>> {
        Self::get(path)
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Vec<std::borrow::Cow<'static, str>> {
        Self::iter().filter(|p| p.starts_with(path)).collect()
    }
}

// fn load_embedded_fonts(platform: &dyn gpui2::Platform) {
//     let font_paths = Assets.list("fonts");
//     let mut embedded_fonts = Vec::new();
//     for font_path in &font_paths {
//         if font_path.ends_with(".ttf") {
//             let font_path = &*font_path;
//             let font_bytes = Assets.load(font_path).unwrap().to_vec();
//             embedded_fonts.push(Arc::from(font_bytes));
//         }
//     }
//     platform.fonts().add_fonts(&embedded_fonts).unwrap();
// }
