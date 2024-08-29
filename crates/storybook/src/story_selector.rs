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
    ApplicationMenu,
    AutoHeightEditor,
    Avatar,
    Button,
    Checkbox,
    CollabNotification,
    ContextMenu,
    Cursor,
    DefaultColors,
    Disclosure,
    Focus,
    Icon,
    IconButton,
    Keybinding,
    Label,
    List,
    ListHeader,
    ListItem,
    OverflowScroll,
    Picker,
    Scroll,
    Tab,
    TabBar,
    Text,
    ToggleButton,
    ToolStrip,
    ViewportUnits,
    WithRemSize,
}

impl ComponentStory {
    pub fn story(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            Self::ApplicationMenu => cx.new_view(|_| title_bar::ApplicationMenuStory).into(),
            Self::AutoHeightEditor => AutoHeightEditorStory::new(cx).into(),
            Self::Avatar => cx.new_view(|_| ui::AvatarStory).into(),
            Self::Button => cx.new_view(|_| ui::ButtonStory).into(),
            Self::Checkbox => cx.new_view(|_| ui::CheckboxStory).into(),
            Self::CollabNotification => cx
                .new_view(|_| collab_ui::notifications::CollabNotificationStory)
                .into(),
            Self::ContextMenu => cx.new_view(|_| ui::ContextMenuStory).into(),
            Self::Cursor => cx.new_view(|_| crate::stories::CursorStory).into(),
            Self::DefaultColors => DefaultColorsStory::view(cx).into(),
            Self::Disclosure => cx.new_view(|_| ui::DisclosureStory).into(),
            Self::Focus => FocusStory::view(cx).into(),
            Self::Icon => cx.new_view(|_| ui::IconStory).into(),
            Self::IconButton => cx.new_view(|_| ui::IconButtonStory).into(),
            Self::Keybinding => cx.new_view(|_| ui::KeybindingStory).into(),
            Self::Label => cx.new_view(|_| ui::LabelStory).into(),
            Self::List => cx.new_view(|_| ui::ListStory).into(),
            Self::ListHeader => cx.new_view(|_| ui::ListHeaderStory).into(),
            Self::ListItem => cx.new_view(|_| ui::ListItemStory).into(),
            Self::OverflowScroll => cx.new_view(|_| crate::stories::OverflowScrollStory).into(),
            Self::Picker => PickerStory::new(cx).into(),
            Self::Scroll => ScrollStory::view(cx).into(),
            Self::Tab => cx.new_view(|_| ui::TabStory).into(),
            Self::TabBar => cx.new_view(|_| ui::TabBarStory).into(),
            Self::Text => TextStory::view(cx).into(),
            Self::ToggleButton => cx.new_view(|_| ui::ToggleButtonStory).into(),
            Self::ToolStrip => cx.new_view(|_| ui::ToolStripStory).into(),
            Self::ViewportUnits => cx.new_view(|_| crate::stories::ViewportUnitsStory).into(),
            Self::WithRemSize => cx.new_view(|_| crate::stories::WithRemSizeStory).into(),
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
