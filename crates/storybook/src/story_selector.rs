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
    Vector,
}

impl ComponentStory {
    pub fn story(&self, window: &mut Window, cx: &mut AppContext) -> AnyView {
        match self {
            Self::ApplicationMenu => window
                .new_view(cx, |window, cx| title_bar::ApplicationMenuStory::new(window, cx))
                .into(),
            Self::AutoHeightEditor => AutoHeightEditorStory::new(window, cx).into(),
            Self::Avatar => window.new_view(cx, |_, _| ui::AvatarStory).into(),
            Self::Button => window.new_view(cx, |_, _| ui::ButtonStory).into(),
            Self::CollabNotification => window
                .new_view(cx, |_, _| collab_ui::notifications::CollabNotificationStory)
                .into(),
            Self::ContextMenu => window.new_view(cx, |_, _| ui::ContextMenuStory).into(),
            Self::Cursor => window.new_view(cx, |_, _| crate::stories::CursorStory).into(),
            Self::DefaultColors => DefaultColorsStory::view(window, cx).into(),
            Self::Disclosure => window.new_view(cx, |_, _| ui::DisclosureStory).into(),
            Self::Focus => FocusStory::view(window, cx).into(),
            Self::Icon => window.new_view(cx, |_, _| ui::IconStory).into(),
            Self::IconButton => window.new_view(cx, |_, _| ui::IconButtonStory).into(),
            Self::Keybinding => window.new_view(cx, |_, _| ui::KeybindingStory).into(),
            Self::Label => window.new_view(cx, |_, _| ui::LabelStory).into(),
            Self::List => window.new_view(cx, |_, _| ui::ListStory).into(),
            Self::ListHeader => window.new_view(cx, |_, _| ui::ListHeaderStory).into(),
            Self::ListItem => window.new_view(cx, |_, _| ui::ListItemStory).into(),
            Self::OverflowScroll => window.new_view(cx, |_, _| crate::stories::OverflowScrollStory).into(),
            Self::Picker => PickerStory::new(window, cx).into(),
            Self::Scroll => ScrollStory::view(window, cx).into(),
            Self::Tab => window.new_view(cx, |_, _| ui::TabStory).into(),
            Self::TabBar => window.new_view(cx, |_, _| ui::TabBarStory).into(),
            Self::Text => TextStory::view(window, cx).into(),
            Self::ToggleButton => window.new_view(cx, |_, _| ui::ToggleButtonStory).into(),
            Self::ToolStrip => window.new_view(cx, |_, _| ui::ToolStripStory).into(),
            Self::ViewportUnits => window.new_view(cx, |_, _| crate::stories::ViewportUnitsStory).into(),
            Self::WithRemSize => window.new_view(cx, |_, _| crate::stories::WithRemSizeStory).into(),
            Self::Vector => window.new_view(cx, |_, _| ui::VectorStory).into(),
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
    pub fn story(&self, window: &mut Window, cx: &mut AppContext) -> AnyView {
        match self {
            Self::Component(component_story) => component_story.story(window, cx),
            Self::KitchenSink => KitchenSinkStory::view(window, cx).into(),
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
