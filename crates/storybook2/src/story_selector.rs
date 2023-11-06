use std::str::FromStr;
use std::sync::OnceLock;

use crate::stories::*;
use anyhow::anyhow;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use gpui::{AnyView, VisualContext};
use strum::{EnumIter, EnumString, IntoEnumIterator};
use ui::prelude::*;
use ui::{AvatarStory, ButtonStory, DetailsStory, IconStory, InputStory, LabelStory};

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ComponentStory {
    AssistantPanel,
    Avatar,
    Breadcrumb,
    Buffer,
    Button,
    ChatPanel,
    Checkbox,
    CollabPanel,
    Colors,
    CommandPalette,
    ContextMenu,
    Copilot,
    Details,
    Facepile,
    Focus,
    Icon,
    Input,
    Keybinding,
    Label,
    LanguageSelector,
    MultiBuffer,
    NotificationsPanel,
    Palette,
    Panel,
    ProjectPanel,
    RecentProjects,
    Scroll,
    Tab,
    TabBar,
    Terminal,
    Text,
    ThemeSelector,
    TitleBar,
    Toast,
    Toolbar,
    TrafficLights,
    Workspace,
    ZIndex,
    Picker,
}

impl ComponentStory {
    pub fn story(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            Self::AssistantPanel => cx.build_view(|_| ui::AssistantPanelStory).into(),
            Self::Avatar => cx.build_view(|_| AvatarStory).into(),
            Self::Breadcrumb => cx.build_view(|_| ui::BreadcrumbStory).into(),
            Self::Buffer => cx.build_view(|_| ui::BufferStory).into(),
            Self::Button => cx.build_view(|_| ButtonStory).into(),
            Self::ChatPanel => cx.build_view(|_| ui::ChatPanelStory).into(),
            Self::Checkbox => cx.build_view(|_| ui::CheckboxStory).into(),
            Self::CollabPanel => cx.build_view(|_| ui::CollabPanelStory).into(),
            Self::Colors => cx.build_view(|_| ColorsStory).into(),
            Self::CommandPalette => cx.build_view(|_| ui::CommandPaletteStory).into(),
            Self::ContextMenu => cx.build_view(|_| ui::ContextMenuStory).into(),
            Self::Copilot => cx.build_view(|_| ui::CopilotModalStory).into(),
            Self::Details => cx.build_view(|_| DetailsStory).into(),
            Self::Facepile => cx.build_view(|_| ui::FacepileStory).into(),
            Self::Focus => FocusStory::view(cx).into(),
            Self::Icon => cx.build_view(|_| IconStory).into(),
            Self::Input => cx.build_view(|_| InputStory).into(),
            Self::Keybinding => cx.build_view(|_| ui::KeybindingStory).into(),
            Self::Label => cx.build_view(|_| LabelStory).into(),
            Self::LanguageSelector => cx.build_view(|_| ui::LanguageSelectorStory).into(),
            Self::MultiBuffer => cx.build_view(|_| ui::MultiBufferStory).into(),
            Self::NotificationsPanel => cx.build_view(|cx| ui::NotificationsPanelStory).into(),
            Self::Palette => cx.build_view(|cx| ui::PaletteStory).into(),
            Self::Panel => cx.build_view(|cx| ui::PanelStory).into(),
            Self::ProjectPanel => cx.build_view(|_| ui::ProjectPanelStory).into(),
            Self::RecentProjects => cx.build_view(|_| ui::RecentProjectsStory).into(),
            Self::Scroll => ScrollStory::view(cx).into(),
            Self::Tab => cx.build_view(|_| ui::TabStory).into(),
            Self::TabBar => cx.build_view(|_| ui::TabBarStory).into(),
            Self::Terminal => cx.build_view(|_| ui::TerminalStory).into(),
            Self::Text => TextStory::view(cx).into(),
            Self::ThemeSelector => cx.build_view(|_| ui::ThemeSelectorStory).into(),
            Self::TitleBar => ui::TitleBarStory::view(cx).into(),
            Self::Toast => cx.build_view(|_| ui::ToastStory).into(),
            Self::Toolbar => cx.build_view(|_| ui::ToolbarStory).into(),
            Self::TrafficLights => cx.build_view(|_| ui::TrafficLightsStory).into(),
            Self::Workspace => ui::WorkspaceStory::view(cx).into(),
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
