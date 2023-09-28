use std::str::FromStr;
use std::sync::OnceLock;

use anyhow::{anyhow, Context};
use clap::builder::PossibleValue;
use clap::ValueEnum;
use strum::{EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ElementStory {
    Avatar,
    Button,
    Icon,
    Input,
    Label,
}

#[derive(Debug, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ComponentStory {
    Breadcrumb,
    ChatPanel,
    CollabPanel,
    Facepile,
    Keybinding,
    Palette,
    ProjectPanel,
    StatusBar,
    TabBar,
    Terminal,
    TitleBar,
    Toolbar,
    TrafficLights,
}

#[derive(Debug, Clone, Copy)]
pub enum StorySelector {
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

/// The list of all stories available in the storybook.
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
