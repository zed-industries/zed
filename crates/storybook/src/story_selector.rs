use std::str::FromStr;
use std::sync::OnceLock;

use anyhow::{anyhow, Context};
use clap::builder::PossibleValue;
use clap::ValueEnum;
use gpui2::{AnyElement, Element};
use strum::{EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ElementStory {
    Avatar,
    Button,
    Icon,
    Input,
    Label,
}

impl ElementStory {
    pub fn story<V: 'static>(&self) -> AnyElement<V> {
        use crate::stories::elements;

        match self {
            Self::Avatar => elements::avatar::AvatarStory::default().into_any(),
            Self::Button => elements::button::ButtonStory::default().into_any(),
            Self::Icon => elements::icon::IconStory::default().into_any(),
            Self::Input => elements::input::InputStory::default().into_any(),
            Self::Label => elements::label::LabelStory::default().into_any(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ComponentStory {
    AssistantPanel,
    Breadcrumb,
    Buffer,
    ContextMenu,
    ChatPanel,
    CollabPanel,
    Facepile,
    Keybinding,
    Palette,
    Panel,
    ProjectPanel,
    StatusBar,
    Tab,
    TabBar,
    Terminal,
    TitleBar,
    Toolbar,
    TrafficLights,
}

impl ComponentStory {
    pub fn story<V: 'static>(&self) -> AnyElement<V> {
        use crate::stories::components;

        match self {
            Self::AssistantPanel => {
                components::assistant_panel::AssistantPanelStory::default().into_any()
            }
            Self::Breadcrumb => components::breadcrumb::BreadcrumbStory::default().into_any(),
            Self::Buffer => components::buffer::BufferStory::default().into_any(),
            Self::ContextMenu => components::context_menu::ContextMenuStory::default().into_any(),
            Self::ChatPanel => components::chat_panel::ChatPanelStory::default().into_any(),
            Self::CollabPanel => components::collab_panel::CollabPanelStory::default().into_any(),
            Self::Facepile => components::facepile::FacepileStory::default().into_any(),
            Self::Keybinding => components::keybinding::KeybindingStory::default().into_any(),
            Self::Palette => components::palette::PaletteStory::default().into_any(),
            Self::Panel => components::panel::PanelStory::default().into_any(),
            Self::ProjectPanel => {
                components::project_panel::ProjectPanelStory::default().into_any()
            }
            Self::StatusBar => components::status_bar::StatusBarStory::default().into_any(),
            Self::Tab => components::tab::TabStory::default().into_any(),
            Self::TabBar => components::tab_bar::TabBarStory::default().into_any(),
            Self::Terminal => components::terminal::TerminalStory::default().into_any(),
            Self::TitleBar => components::title_bar::TitleBarStory::default().into_any(),
            Self::Toolbar => components::toolbar::ToolbarStory::default().into_any(),
            Self::TrafficLights => {
                components::traffic_lights::TrafficLightsStory::default().into_any()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StorySelector {
    Element(ElementStory),
    Component(ComponentStory),
    KitchenSink,
}

impl FromStr for StorySelector {
    type Err = anyhow::Error;

    fn from_str(raw_story_name: &str) -> std::result::Result<Self, Self::Err> {
        let story = raw_story_name.to_ascii_lowercase();

        if story == "kitchen_sink" {
            return Ok(Self::KitchenSink);
        }

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

impl StorySelector {
    pub fn story<V: 'static>(&self) -> Vec<AnyElement<V>> {
        match self {
            StorySelector::Element(element_story) => vec![element_story.story()],
            StorySelector::Component(component_story) => vec![component_story.story()],
            StorySelector::KitchenSink => all_story_selectors()
                .into_iter()
                // Exclude the kitchen sink to prevent `story` from recursively
                // calling itself for all eternity.
                .filter(|selector| **selector != Self::KitchenSink)
                .flat_map(|selector| selector.story())
                .collect(),
        }
    }
}

/// The list of all stories available in the storybook.
static ALL_STORY_SELECTORS: OnceLock<Vec<StorySelector>> = OnceLock::new();

fn all_story_selectors<'a>() -> &'a [StorySelector] {
    let stories = ALL_STORY_SELECTORS.get_or_init(|| {
        let element_stories = ElementStory::iter().map(StorySelector::Element);
        let component_stories = ComponentStory::iter().map(StorySelector::Component);

        element_stories
            .chain(component_stories)
            .chain(std::iter::once(StorySelector::KitchenSink))
            .collect::<Vec<_>>()
    });

    stories
}

impl ValueEnum for StorySelector {
    fn value_variants<'a>() -> &'a [Self] {
        all_story_selectors()
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let value = match self {
            Self::Element(story) => format!("elements/{story}"),
            Self::Component(story) => format!("components/{story}"),
            Self::KitchenSink => "kitchen_sink".to_string(),
        };

        Some(PossibleValue::new(value))
    }
}
