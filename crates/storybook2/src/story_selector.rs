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
    Button,
    Details,
    Icon,
    Input,
    Label,
    ZIndex,
}

impl ElementStory {
    pub fn story<S: 'static + Send + Sync + Clone>(&self) -> AnyElement<S> {
        match self {
            Self::Avatar => ui::AvatarStory::new().into_any(),
            Self::Button => ui::ButtonStory::new().into_any(),
            Self::Details => ui::DetailsStory::new().into_any(),
            Self::Icon => ui::IconStory::new().into_any(),
            Self::Input => ui::InputStory::new().into_any(),
            Self::Label => ui::LabelStory::new().into_any(),
            Self::ZIndex => crate::stories::z_index::ZIndexStory::new().into_any(),
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
    pub fn story<S: 'static + Send + Sync + Clone>(&self, cx: &mut WindowContext) -> AnyElement<S> {
        match self {
            Self::AssistantPanel => ui::AssistantPanelStory::new().into_any(),
            Self::Buffer => ui::BufferStory::new().into_any(),
            Self::Breadcrumb => ui::BreadcrumbStory::new().into_any(),
            Self::ChatPanel => ui::ChatPanelStory::new().into_any(),
            Self::CollabPanel => ui::CollabPanelStory::new().into_any(),
            Self::CommandPalette => ui::CommandPaletteStory::new().into_any(),
            Self::ContextMenu => ui::ContextMenuStory::new().into_any(),
            Self::Facepile => ui::FacepileStory::new().into_any(),
            Self::Keybinding => ui::KeybindingStory::new().into_any(),
            Self::LanguageSelector => ui::LanguageSelectorStory::new().into_any(),
            Self::MultiBuffer => ui::MultiBufferStory::new().into_any(),
            Self::Palette => ui::PaletteStory::new().into_any(),
            Self::Panel => ui::PanelStory::new().into_any(),
            Self::ProjectPanel => ui::ProjectPanelStory::new().into_any(),
            Self::RecentProjects => ui::RecentProjectsStory::new().into_any(),
            Self::Tab => ui::TabStory::new().into_any(),
            Self::TabBar => ui::TabBarStory::new().into_any(),
            Self::Terminal => ui::TerminalStory::new().into_any(),
            Self::ThemeSelector => ui::ThemeSelectorStory::new().into_any(),
            Self::TitleBar => ui::TitleBarStory::new().into_any(),
            Self::Toast => ui::ToastStory::new().into_any(),
            Self::Toolbar => ui::ToolbarStory::new().into_any(),
            Self::TrafficLights => ui::TrafficLightsStory::new().into_any(),
            Self::Workspace => todo!(),
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
    pub fn story<S: 'static + Send + Sync + Clone>(&self, cx: &mut WindowContext) -> AnyElement<S> {
        match self {
            Self::Element(element_story) => element_story.story(),
            Self::Component(component_story) => component_story.story(cx),
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
