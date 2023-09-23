#![allow(dead_code, unused_variables)]

mod collab_panel;
mod stories;
mod story;
mod story_selector;
mod workspace;

use ::theme as legacy_theme;
use clap::Parser;
use gpui2::{serde_json, vec2f, view, Element, IntoElement, RectF, ViewContext, WindowBounds};
use legacy_theme::ThemeSettings;
use log::LevelFilter;
use settings::{default_settings, SettingsStore};
use simplelog::SimpleLogger;
use ui::{ElementExt, Theme};

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

        cx.add_window(
            gpui2::WindowOptions {
                bounds: WindowBounds::Fixed(RectF::new(vec2f(0., 0.), vec2f(1600., 900.))),
                center: true,
                ..Default::default()
            },
            |cx| match args.story {
                Some(StorySelector::Element(ElementStory::Avatar)) => view(|cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        stories::elements::avatar::AvatarStory::default(),
                    )
                }),
                Some(StorySelector::Element(ElementStory::TextButton)) => view(|cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        stories::elements::text_button::TextButtonStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Breadcrumb)) => view(|cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        stories::components::breadcrumb::BreadcrumbStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Facepile)) => view(|cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        stories::components::facepile::FacepileStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Palette)) => view(|cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        stories::components::palette::PaletteStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::Toolbar)) => view(|cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        stories::components::toolbar::ToolbarStory::default(),
                    )
                }),
                Some(StorySelector::Component(ComponentStory::TrafficLights)) => view(|cx| {
                    render_story(
                        &mut ViewContext::new(cx),
                        stories::components::traffic_lights::TrafficLightsStory::default(),
                    )
                }),
                None => {
                    view(|cx| render_story(&mut ViewContext::new(cx), WorkspaceElement::default()))
                }
            },
        );
        cx.platform().activate(true);
    });
}

fn render_story<V: 'static, S: IntoElement<V>>(
    cx: &mut ViewContext<V>,
    story: S,
) -> impl Element<V> {
    story.into_element().themed(current_theme(cx))
}

// Nathan: During the transition to gpui2, we will include the base theme on the legacy Theme struct.
fn current_theme<V: 'static>(cx: &mut ViewContext<V>) -> Theme {
    settings::get::<ThemeSettings>(cx)
        .theme
        .deserialized_base_theme
        .lock()
        .get_or_insert_with(|| {
            let theme: Theme =
                serde_json::from_value(settings::get::<ThemeSettings>(cx).theme.base_theme.clone())
                    .unwrap();
            Box::new(theme)
        })
        .downcast_ref::<Theme>()
        .unwrap()
        .clone()
}

use anyhow::{anyhow, Result};
use gpui2::AssetSource;
use rust_embed::RustEmbed;
use workspace::WorkspaceElement;

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
