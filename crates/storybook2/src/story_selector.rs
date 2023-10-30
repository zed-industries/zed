use std::str::FromStr;
use std::sync::OnceLock;

use crate::stories::*;
use anyhow::anyhow;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use gpui2::{AnyView, VisualContext};
use strum::{EnumIter, EnumString, IntoEnumIterator};
use ui::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ElementStory {
    Avatar,
    Button,
    Colors,
    Details,
    Focus,
    Icon,
    Input,
    Label,
    Scroll,
    Text,
    ZIndex,
}

impl ElementStory {
    pub fn story(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            Self::Avatar => { cx.build_view(|cx| (), |_, _| ui::AvatarStory.render()) }.into_any(),
            Self::Button => { cx.build_view(|cx| (), |_, _| ui::ButtonStory.render()) }.into_any(),
            Self::Colors => { cx.build_view(|cx| (), |_, _| ColorsStory.render()) }.into_any(),
            Self::Details => {
                { cx.build_view(|cx| (), |_, _| ui::DetailsStory.render()) }.into_any()
            }
            Self::Focus => FocusStory::view(cx).into_any(),
            Self::Icon => { cx.build_view(|cx| (), |_, _| ui::IconStory.render()) }.into_any(),
            Self::Input => { cx.build_view(|cx| (), |_, _| ui::InputStory.render()) }.into_any(),
            Self::Label => { cx.build_view(|cx| (), |_, _| ui::LabelStory.render()) }.into_any(),
            Self::Scroll => ScrollStory::view(cx).into_any(),
            Self::Text => TextStory::view(cx).into_any(),
            Self::ZIndex => { cx.build_view(|cx| (), |_, _| ZIndexStory.render()) }.into_any(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ComponentStory {
    AssistantPanel,
    Breadcrumb,
    Buffer,
    ChatPanel,
    CollabPanel,
    CommandPalette,
    Copilot,
    ContextMenu,
    Facepile,
    Keybinding,
    LanguageSelector,
    MultiBuffer,
    NotificationsPanel,
    Palette,
    Panel,
    ProjectPanel,
    RecentProjects,
    Tab,
    TabBar,
    Terminal,
    ThemeSelector,
    TitleBar,
    Toast,
    Toolbar,
    TrafficLights,
    Workspace,
}

impl ComponentStory {
    pub fn story(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            Self::AssistantPanel => {
                { cx.build_view(|cx| (), |_, _| ui::AssistantPanelStory.render()) }.into_any()
            }
            Self::Buffer => { cx.build_view(|cx| (), |_, _| ui::BufferStory.render()) }.into_any(),
            Self::Breadcrumb => {
                { cx.build_view(|cx| (), |_, _| ui::BreadcrumbStory.render()) }.into_any()
            }
            Self::ChatPanel => {
                { cx.build_view(|cx| (), |_, _| ui::ChatPanelStory.render()) }.into_any()
            }
            Self::CollabPanel => {
                { cx.build_view(|cx| (), |_, _| ui::CollabPanelStory.render()) }.into_any()
            }
            Self::CommandPalette => {
                { cx.build_view(|cx| (), |_, _| ui::CommandPaletteStory.render()) }.into_any()
            }
            Self::ContextMenu => {
                { cx.build_view(|cx| (), |_, _| ui::ContextMenuStory.render()) }.into_any()
            }
            Self::Facepile => {
                { cx.build_view(|cx| (), |_, _| ui::FacepileStory.render()) }.into_any()
            }
            Self::Keybinding => {
                { cx.build_view(|cx| (), |_, _| ui::KeybindingStory.render()) }.into_any()
            }
            Self::LanguageSelector => {
                { cx.build_view(|cx| (), |_, _| ui::LanguageSelectorStory.render()) }.into_any()
            }
            Self::MultiBuffer => {
                { cx.build_view(|cx| (), |_, _| ui::MultiBufferStory.render()) }.into_any()
            }
            Self::NotificationsPanel => {
                { cx.build_view(|cx| (), |_, _| ui::NotificationsPanelStory.render()) }.into_any()
            }
            Self::Palette => {
                { cx.build_view(|cx| (), |_, _| ui::PaletteStory.render()) }.into_any()
            }
            Self::Panel => { cx.build_view(|cx| (), |_, _| ui::PanelStory.render()) }.into_any(),
            Self::ProjectPanel => {
                { cx.build_view(|cx| (), |_, _| ui::ProjectPanelStory.render()) }.into_any()
            }
            Self::RecentProjects => {
                { cx.build_view(|cx| (), |_, _| ui::RecentProjectsStory.render()) }.into_any()
            }
            Self::Tab => { cx.build_view(|cx| (), |_, _| ui::TabStory.render()) }.into_any(),
            Self::TabBar => { cx.build_view(|cx| (), |_, _| ui::TabBarStory.render()) }.into_any(),
            Self::Terminal => {
                { cx.build_view(|cx| (), |_, _| ui::TerminalStory.render()) }.into_any()
            }
            Self::ThemeSelector => {
                { cx.build_view(|cx| (), |_, _| ui::ThemeSelectorStory.render()) }.into_any()
            }
            Self::TitleBar => ui::TitleBarStory::view(cx).into_any(),
            Self::Toast => { cx.build_view(|cx| (), |_, _| ui::ToastStory.render()) }.into_any(),
            Self::Toolbar => {
                { cx.build_view(|cx| (), |_, _| ui::ToolbarStory.render()) }.into_any()
            }
            Self::TrafficLights => {
                { cx.build_view(|cx| (), |_, _| ui::TrafficLightsStory.render()) }.into_any()
            }
            Self::Copilot => {
                { cx.build_view(|cx| (), |_, _| ui::CopilotModalStory.render()) }.into_any()
            }
            Self::Workspace => ui::WorkspaceStory::view(cx).into_any(),
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
        use anyhow::Context;

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
    pub fn story(&self, cx: &mut WindowContext) -> AnyView {
        match self {
            Self::Element(element_story) => element_story.story(cx),
            Self::Component(component_story) => component_story.story(cx),
            Self::KitchenSink => KitchenSinkStory::view(cx).into_any(),
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
