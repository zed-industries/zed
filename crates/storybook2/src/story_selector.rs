use std::str::FromStr;
use std::sync::OnceLock;

use crate::stories::*;
use anyhow::anyhow;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use gpui3::{view, AnyView, Context};
use strum::{EnumIter, EnumString, IntoEnumIterator};
use ui::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum ElementStory {
    Avatar,
    Button,
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
            Self::Avatar => {
                view(cx.entity(|cx| ()), |_, _| ui::AvatarStory::new().into_any()).into_any()
            }
            Self::Button => {
                view(cx.entity(|cx| ()), |_, _| ui::ButtonStory::new().into_any()).into_any()
            }
            Self::Details => view(cx.entity(|cx| ()), |_, _| {
                ui::DetailsStory::new().into_any()
            })
            .into_any(),
            Self::Focus => FocusStory::view(cx).into_any(),
            Self::Icon => {
                view(cx.entity(|cx| ()), |_, _| ui::IconStory::new().into_any()).into_any()
            }
            Self::Input => {
                view(cx.entity(|cx| ()), |_, _| ui::InputStory::new().into_any()).into_any()
            }
            Self::Label => {
                view(cx.entity(|cx| ()), |_, _| ui::LabelStory::new().into_any()).into_any()
            }
            Self::Scroll => ScrollStory::view(cx).into_any(),
            Self::Text => TextStory::view(cx).into_any(),
            Self::ZIndex => {
                view(cx.entity(|cx| ()), |_, _| ZIndexStory::new().into_any()).into_any()
            }
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
    ContextMenu,
    Facepile,
    Keybinding,
    LanguageSelector,
    MultiBuffer,
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
            Self::AssistantPanel => view(cx.entity(|cx| ()), |_, _| {
                ui::AssistantPanelStory::new().into_any()
            })
            .into_any(),
            Self::Buffer => {
                view(cx.entity(|cx| ()), |_, _| ui::BufferStory::new().into_any()).into_any()
            }
            Self::Breadcrumb => view(cx.entity(|cx| ()), |_, _| {
                ui::BreadcrumbStory::new().into_any()
            })
            .into_any(),
            Self::ChatPanel => view(cx.entity(|cx| ()), |_, _| {
                ui::ChatPanelStory::new().into_any()
            })
            .into_any(),
            Self::CollabPanel => view(cx.entity(|cx| ()), |_, _| {
                ui::CollabPanelStory::new().into_any()
            })
            .into_any(),
            Self::CommandPalette => view(cx.entity(|cx| ()), |_, _| {
                ui::CommandPaletteStory::new().into_any()
            })
            .into_any(),
            Self::ContextMenu => view(cx.entity(|cx| ()), |_, _| {
                ui::ContextMenuStory::new().into_any()
            })
            .into_any(),
            Self::Facepile => view(cx.entity(|cx| ()), |_, _| {
                ui::FacepileStory::new().into_any()
            })
            .into_any(),
            Self::Keybinding => view(cx.entity(|cx| ()), |_, _| {
                ui::KeybindingStory::new().into_any()
            })
            .into_any(),
            Self::LanguageSelector => view(cx.entity(|cx| ()), |_, _| {
                ui::LanguageSelectorStory::new().into_any()
            })
            .into_any(),
            Self::MultiBuffer => view(cx.entity(|cx| ()), |_, _| {
                ui::MultiBufferStory::new().into_any()
            })
            .into_any(),
            Self::Palette => view(cx.entity(|cx| ()), |_, _| {
                ui::PaletteStory::new().into_any()
            })
            .into_any(),
            Self::Panel => {
                view(cx.entity(|cx| ()), |_, _| ui::PanelStory::new().into_any()).into_any()
            }
            Self::ProjectPanel => view(cx.entity(|cx| ()), |_, _| {
                ui::ProjectPanelStory::new().into_any()
            })
            .into_any(),
            Self::RecentProjects => view(cx.entity(|cx| ()), |_, _| {
                ui::RecentProjectsStory::new().into_any()
            })
            .into_any(),
            Self::Tab => view(cx.entity(|cx| ()), |_, _| ui::TabStory::new().into_any()).into_any(),
            Self::TabBar => {
                view(cx.entity(|cx| ()), |_, _| ui::TabBarStory::new().into_any()).into_any()
            }
            Self::Terminal => view(cx.entity(|cx| ()), |_, _| {
                ui::TerminalStory::new().into_any()
            })
            .into_any(),
            Self::ThemeSelector => view(cx.entity(|cx| ()), |_, _| {
                ui::ThemeSelectorStory::new().into_any()
            })
            .into_any(),
            Self::TitleBar => ui::TitleBarStory::view(cx).into_any(),
            Self::Toast => {
                view(cx.entity(|cx| ()), |_, _| ui::ToastStory::new().into_any()).into_any()
            }
            Self::Toolbar => view(cx.entity(|cx| ()), |_, _| {
                ui::ToolbarStory::new().into_any()
            })
            .into_any(),
            Self::TrafficLights => view(cx.entity(|cx| ()), |_, _| {
                ui::TrafficLightsStory::new().into_any()
            })
            .into_any(),
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
