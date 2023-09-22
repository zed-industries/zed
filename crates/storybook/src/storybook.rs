#![allow(dead_code, unused_variables)]

mod collab_panel;
mod stories;
mod story;
mod workspace;

use std::str::FromStr;
use std::sync::OnceLock;

use ::theme as legacy_theme;
use clap::{builder::PossibleValue, Parser, ValueEnum};
use gpui2::{serde_json, vec2f, view, Element, IntoElement, RectF, ViewContext, WindowBounds};
use legacy_theme::ThemeSettings;
use log::LevelFilter;
use settings::{default_settings, SettingsStore};
use simplelog::SimpleLogger;
use stories::components::breadcrumb::BreadcrumbStory;
use stories::components::facepile::FacepileStory;
use stories::components::toolbar::ToolbarStory;
use stories::components::traffic_lights::TrafficLightsStory;
use stories::elements::avatar::AvatarStory;
use strum::{EnumIter, EnumString, IntoEnumIterator};
use ui::{ElementExt, Theme};

gpui2::actions! {
    storybook,
    [ToggleInspector]
}

#[derive(Debug, Clone, Copy)]
enum StorySelector {
    Element(ElementStory),
    Component(ComponentStory),
}

impl FromStr for StorySelector {
    type Err = anyhow::Error;

    fn from_str(raw_story_name: &str) -> std::result::Result<Self, Self::Err> {
        let story = raw_story_name.to_ascii_lowercase();

        if let Some((_, story)) = story.split_once("elements/") {
            let element_story = ElementStory::from_str(story)
                .with_context(|| format!("story not found for element '{story}'"))?;

            return Ok(Self::Element(element_story));
        }

        if let Some((_, story)) = story.split_once("components/") {
            let component_story = ComponentStory::from_str(story)
                .with_context(|| format!("story not found for component '{story}'"))?;

            return Ok(Self::Component(component_story));
        }

        Err(anyhow!("story not found for '{raw_story_name}'"))
    }
}

static ALL_STORIES: OnceLock<Vec<StorySelector>> = OnceLock::new();

impl ValueEnum for StorySelector {
    fn value_variants<'a>() -> &'a [Self] {
        let stories = ALL_STORIES.get_or_init(|| {
            let element_stories = ElementStory::iter().map(Self::Element);
            let component_stories = ComponentStory::iter().map(Self::Component);

            element_stories.chain(component_stories).collect::<Vec<_>>()
        });

        stories
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let value = match self {
            Self::Element(story) => format!("elements/{story}"),
            Self::Component(story) => format!("components/{story}"),
        };

        Some(PossibleValue::new(value))
    }
}

#[derive(Debug, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
enum ElementStory {
    Avatar,
}

#[derive(Debug, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
enum ComponentStory {
    Breadcrumb,
    Facepile,
    Toolbar,
    TrafficLights,
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
                Some(StorySelector::Element(ElementStory::Avatar)) => {
                    view(|cx| render_story(&mut ViewContext::new(cx), AvatarStory::default()))
                }
                Some(StorySelector::Component(ComponentStory::Breadcrumb)) => {
                    view(|cx| render_story(&mut ViewContext::new(cx), BreadcrumbStory::default()))
                }
                Some(StorySelector::Component(ComponentStory::Facepile)) => {
                    view(|cx| render_story(&mut ViewContext::new(cx), FacepileStory::default()))
                }
                Some(StorySelector::Component(ComponentStory::Toolbar)) => {
                    view(|cx| render_story(&mut ViewContext::new(cx), ToolbarStory::default()))
                }
                Some(StorySelector::Component(ComponentStory::TrafficLights)) => view(|cx| {
                    render_story(&mut ViewContext::new(cx), TrafficLightsStory::default())
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

use anyhow::{anyhow, Context, Result};
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
