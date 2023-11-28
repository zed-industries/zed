use std::str::FromStr;
use std::sync::OnceLock;

use crate::stories::*;
use anyhow::anyhow;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use gpui::{AnyView, VisualContext};
use strum::{EnumIter, EnumString, IntoEnumIterator};
use ui::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ComponentStory {
    Avatar,
    Button,
    Checkbox,
    ContextMenu,
    Focus,
    Icon,
    Input,
    Keybinding,
    Label,
    ListItem,
    Scroll,
    Text,
    ZIndex,
    Picker,
}

impl ComponentStory {
    pub fn story(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            Self::Avatar => cx.build_view(|_| ui::AvatarStory).into(),
            Self::Button => cx.build_view(|_| ui::ButtonStory).into(),
            Self::Checkbox => cx.build_view(|_| ui::CheckboxStory).into(),
            Self::ContextMenu => cx.build_view(|_| ui::ContextMenuStory).into(),
            Self::Focus => FocusStory::view(cx).into(),
            Self::Icon => cx.build_view(|_| ui::IconStory).into(),
            Self::Input => cx.build_view(|_| ui::InputStory).into(),
            Self::Keybinding => cx.build_view(|_| ui::KeybindingStory).into(),
            Self::Label => cx.build_view(|_| ui::LabelStory).into(),
            Self::ListItem => cx.build_view(|_| ui::ListItemStory).into(),
            Self::Scroll => ScrollStory::view(cx).into(),
            Self::Text => TextStory::view(cx).into(),
            Self::ZIndex => cx.build_view(|_| ZIndexStory).into(),
            Self::Picker => PickerStory::new(cx).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StorySelector {
    Component(ComponentStory),
    KitchenSink,
}

impl FromStr for StorySelector {
    type Err = anyhow::Error;

    fn from_str(raw_story_name: &str) -> std::result::Result<Self, Self::Err> {
        use anyhow::Context;

        let story = raw_story_name.to_ascii_lowercase();

        if story == "kitchen_sink" {
            return Ok(Self::KitchenSink);
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
    pub fn story(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            Self::Component(component_story) => component_story.story(cx),
            Self::KitchenSink => KitchenSinkStory::view(cx).into(),
        }
    }
}

/// The list of all stories available in the storybook.
static ALL_STORY_SELECTORS: OnceLock<Vec<StorySelector>> = OnceLock::new();

impl ValueEnum for StorySelector {
    fn value_variants<'a>() -> &'a [Self] {
        let stories = ALL_STORY_SELECTORS.get_or_init(|| {
            let component_stories = ComponentStory::iter().map(StorySelector::Component);

            component_stories
                .chain(std::iter::once(StorySelector::KitchenSink))
                .collect::<Vec<_>>()
        });

        stories
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let value = match self {
            Self::Component(story) => format!("components/{story}"),
            Self::KitchenSink => "kitchen_sink".to_string(),
        };

        Some(PossibleValue::new(value))
    }
}
