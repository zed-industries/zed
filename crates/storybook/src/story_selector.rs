use std::str::FromStr;
use std::sync::OnceLock;

use crate::stories::*;
use anyhow::anyhow;
use clap::ValueEnum;
use clap::builder::PossibleValue;
use gpui::AnyView;
use strum::{EnumIter, EnumString, IntoEnumIterator};
use ui::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ComponentStory {
    ApplicationMenu,
    AutoHeightEditor,
    CollabNotification,
    ContextMenu,
    Cursor,
    DefaultColors,
    Focus,
    IconButton,
    Keybinding,
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
    ViewportUnits,
    WithRemSize,
}

impl ComponentStory {
    pub fn story(&self, window: &mut Window, cx: &mut App) -> AnyView {
        match self {
            Self::ApplicationMenu => cx
                .new(|cx| title_bar::ApplicationMenuStory::new(window, cx))
                .into(),
            Self::AutoHeightEditor => AutoHeightEditorStory::new(window, cx).into(),
            Self::CollabNotification => cx
                .new(|_| collab_ui::notifications::CollabNotificationStory)
                .into(),
            Self::ContextMenu => cx.new(|_| ui::ContextMenuStory).into(),
            Self::Cursor => cx.new(|_| crate::stories::CursorStory).into(),
            Self::DefaultColors => DefaultColorsStory::model(cx).into(),
            Self::Focus => FocusStory::model(window, cx).into(),
            Self::IconButton => cx.new(|_| ui::IconButtonStory).into(),
            Self::Keybinding => cx.new(|_| ui::KeybindingStory).into(),
            Self::List => cx.new(|_| ui::ListStory).into(),
            Self::ListHeader => cx.new(|_| ui::ListHeaderStory).into(),
            Self::ListItem => cx.new(|_| ui::ListItemStory).into(),
            Self::OverflowScroll => cx.new(|_| crate::stories::OverflowScrollStory).into(),
            Self::Picker => PickerStory::new(window, cx).into(),
            Self::Scroll => ScrollStory::model(cx).into(),
            Self::Tab => cx.new(|_| ui::TabStory).into(),
            Self::TabBar => cx.new(|_| ui::TabBarStory).into(),
            Self::Text => TextStory::model(cx).into(),
            Self::ToggleButton => cx.new(|_| ui::ToggleButtonStory).into(),
            Self::ViewportUnits => cx.new(|_| crate::stories::ViewportUnitsStory).into(),
            Self::WithRemSize => cx.new(|_| crate::stories::WithRemSizeStory).into(),
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
        use anyhow::Context as _;

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
    pub fn story(&self, window: &mut Window, cx: &mut App) -> AnyView {
        match self {
            Self::Component(component_story) => component_story.story(window, cx),
            Self::KitchenSink => KitchenSinkStory::model(cx).into(),
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
