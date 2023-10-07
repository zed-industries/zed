use std::str::FromStr;
use std::sync::OnceLock;

use anyhow::{anyhow, Context};
use clap::builder::PossibleValue;
use clap::ValueEnum;
use gpui3::AnyElement;
use strum::{EnumIter, EnumString, IntoEnumIterator};

use ui::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ElementStory {
    Avatar,
    Icon,
    Input,
    Label,
}

impl ElementStory {
    pub fn story<S: 'static + Send + Sync + Clone>(&self) -> AnyElement<S> {
        use crate::stories::elements;

        match self {
            Self::Avatar => elements::avatar::AvatarStory::new().into_any(),
            Self::Icon => elements::icon::IconStory::new().into_any(),
            Self::Input => elements::input::InputStory::new().into_any(),
            Self::Label => elements::label::LabelStory::new().into_any(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ComponentStory {
    AssistantPanel,
    Breadcrumb,
    Buffer,
    Panel,
    ProjectPanel,
    Tab,
    TabBar,
    Terminal,
    Workspace,
}

impl ComponentStory {
    pub fn story<S: 'static + Send + Sync + Clone>(&self) -> AnyElement<S> {
        use crate::stories::components;

        match self {
            Self::AssistantPanel => {
                components::assistant_panel::AssistantPanelStory::new().into_any()
            }
            Self::Buffer => components::buffer::BufferStory::new().into_any(),
            Self::Breadcrumb => components::breadcrumb::BreadcrumbStory::new().into_any(),
            Self::Panel => components::panel::PanelStory::new().into_any(),
            Self::ProjectPanel => components::project_panel::ProjectPanelStory::new().into_any(),
            Self::Tab => components::tab::TabStory::new().into_any(),
            Self::TabBar => components::tab_bar::TabBarStory::new().into_any(),
            Self::Terminal => components::terminal::TerminalStory::new().into_any(),
            Self::Workspace => components::workspace::WorkspaceStory::new().into_any(),
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
    pub fn story<S: 'static + Send + Sync + Clone>(&self) -> AnyElement<S> {
        match self {
            Self::Element(element_story) => element_story.story(),
            Self::Component(component_story) => component_story.story(),
            Self::KitchenSink => crate::stories::kitchen_sink::KitchenSinkStory::new().into_any(),
        }
    }
}

/// The list of all stories available in the storybook.
static ALL_STORY_SELECTORS: OnceLock<Vec<StorySelector>> = OnceLock::new();

impl ValueEnum for StorySelector {
    fn value_variants<'a>() -> &'a [Self] {
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

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let value = match self {
            Self::Element(story) => format!("elements/{story}"),
            Self::Component(story) => format!("components/{story}"),
            Self::KitchenSink => "kitchen_sink".to_string(),
        };

        Some(PossibleValue::new(value))
    }
}
