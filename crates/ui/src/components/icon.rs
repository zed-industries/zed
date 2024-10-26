#![allow(missing_docs)]
use gpui::{svg, AnimationElement, Hsla, IntoElement, Rems, Transformation};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoStaticStr};
use ui_macros::DerivePathStr;

use crate::{prelude::*, Indicator};

#[derive(IntoElement)]
pub enum AnyIcon {
    Icon(Icon),
    AnimatedIcon(AnimationElement<Icon>),
}

impl AnyIcon {
    /// Returns a new [`AnyIcon`] after applying the given mapping function
    /// to the contained [`Icon`].
    pub fn map(self, f: impl FnOnce(Icon) -> Icon) -> Self {
        match self {
            Self::Icon(icon) => Self::Icon(f(icon)),
            Self::AnimatedIcon(animated_icon) => Self::AnimatedIcon(animated_icon.map_element(f)),
        }
    }
}

impl From<Icon> for AnyIcon {
    fn from(value: Icon) -> Self {
        Self::Icon(value)
    }
}

impl From<AnimationElement<Icon>> for AnyIcon {
    fn from(value: AnimationElement<Icon>) -> Self {
        Self::AnimatedIcon(value)
    }
}

impl RenderOnce for AnyIcon {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self {
            Self::Icon(icon) => icon.into_any_element(),
            Self::AnimatedIcon(animated_icon) => animated_icon.into_any_element(),
        }
    }
}

/// The decoration for an icon.
///
/// For example, this can show an indicator, an "x",
/// or a diagonal strikethrough to indicate something is disabled.
#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum IconDecoration {
    Strikethrough,
    IndicatorDot,
    X,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    /// 10px
    Indicator,
    /// 12px
    XSmall,
    /// 14px
    Small,
    #[default]
    /// 16px
    Medium,
}

impl IconSize {
    pub fn rems(self) -> Rems {
        match self {
            IconSize::Indicator => rems_from_px(10.),
            IconSize::XSmall => rems_from_px(12.),
            IconSize::Small => rems_from_px(14.),
            IconSize::Medium => rems_from_px(16.),
        }
    }

    /// Returns the individual components of the square that contains this [`IconSize`].
    ///
    /// The returned tuple contains:
    ///   1. The length of one side of the square
    ///   2. The padding of one side of the square
    pub fn square_components(&self, cx: &mut WindowContext) -> (Pixels, Pixels) {
        let icon_size = self.rems() * cx.rem_size();
        let padding = match self {
            IconSize::Indicator => Spacing::None.px(cx),
            IconSize::XSmall => Spacing::XSmall.px(cx),
            IconSize::Small => Spacing::XSmall.px(cx),
            IconSize::Medium => Spacing::XSmall.px(cx),
        };

        (icon_size, padding)
    }

    /// Returns the length of a side of the square that contains this [`IconSize`], with padding.
    pub fn square(&self, cx: &mut WindowContext) -> Pixels {
        let (icon_size, padding) = self.square_components(cx);

        icon_size + padding * 2.
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Copy,
    Clone,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Serialize,
    Deserialize,
    DerivePathStr,
)]
#[strum(serialize_all = "snake_case")]
#[path_str(prefix = "icons", suffix = ".svg")]
pub enum IconName {
    Ai,
    AiAnthropic,
    AiAnthropicHosted,
    AiGoogle,
    AiOllama,
    AiOpenAi,
    AiZed,
    ArrowCircle,
    ArrowDown,
    ArrowDownFromLine,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowUpFromLine,
    ArrowUpRight,
    AtSign,
    AudioOff,
    AudioOn,
    Backspace,
    Bell,
    BellDot,
    BellOff,
    BellRing,
    Bolt,
    Book,
    BookCopy,
    BookPlus,
    CaseSensitive,
    Check,
    ChevronDown,
    ChevronDownSmall, // This chevron indicates a popover menu.
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronUpDown,
    Close,
    Code,
    Command,
    Context,
    Control,
    Copilot,
    CopilotDisabled,
    CopilotError,
    CopilotInit,
    Copy,
    CountdownTimer,
    CursorIBeam,
    TextSnippet,
    Dash,
    DatabaseZap,
    Delete,
    Diff,
    Disconnected,
    Download,
    Ellipsis,
    EllipsisVertical,
    Envelope,
    Escape,
    Exit,
    ExpandVertical,
    ExternalLink,
    Eye,
    File,
    FileCode,
    FileDoc,
    FileGeneric,
    FileGit,
    FileLock,
    FileRust,
    FileText,
    FileToml,
    FileTree,
    Filter,
    Folder,
    FolderOpen,
    FolderX,
    Font,
    FontSize,
    FontWeight,
    GenericClose,
    GenericMaximize,
    GenericMinimize,
    GenericRestore,
    Github,
    Hash,
    HistoryRerun,
    Indicator,
    IndicatorX,
    InlayHint,
    Library,
    LineHeight,
    Link,
    ListTree,
    MagnifyingGlass,
    MailOpen,
    Maximize,
    Menu,
    MessageBubbles,
    Mic,
    MicMute,
    Microscope,
    Minimize,
    Option,
    PageDown,
    PageUp,
    Pencil,
    Person,
    Pin,
    Play,
    Plus,
    PocketKnife,
    Public,
    PullRequest,
    Quote,
    Regex,
    ReplNeutral,
    Replace,
    ReplaceAll,
    ReplaceNext,
    ReplyArrowRight,
    Rerun,
    Return,
    Reveal,
    RotateCcw,
    RotateCw,
    Route,
    Save,
    Screen,
    SearchCode,
    SearchSelection,
    SelectAll,
    Server,
    Settings,
    SettingsAlt,
    Shift,
    Slash,
    SlashSquare,
    Sliders,
    SlidersVertical,
    Snip,
    Space,
    Sparkle,
    SparkleAlt,
    SparkleFilled,
    Spinner,
    Split,
    Star,
    StarFilled,
    Stop,
    Strikethrough,
    Supermaven,
    SupermavenDisabled,
    SupermavenError,
    SupermavenInit,
    Tab,
    Terminal,
    Trash,
    TrashAlt,
    TriangleRight,
    Undo,
    Unpin,
    Update,
    UserGroup,
    Visible,
    Warning,
    WholeWord,
    XCircle,
    ZedAssistant,
    ZedAssistantFilled,
    ZedXCopilot,
}

#[derive(IntoElement)]
pub struct Icon {
    path: SharedString,
    color: Color,
    size: Rems,
    transformation: Transformation,
}

impl Icon {
    pub fn new(icon: IconName) -> Self {
        Self {
            path: icon.path().into(),
            color: Color::default(),
            size: IconSize::default().rems(),
            transformation: Transformation::default(),
        }
    }

    pub fn from_path(path: impl Into<SharedString>) -> Self {
        Self {
            path: path.into(),
            color: Color::default(),
            size: IconSize::default().rems(),
            transformation: Transformation::default(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size.rems();
        self
    }

    /// Sets a custom size for the icon, in [`Rems`].
    ///
    /// Not to be exposed outside of the `ui` crate.
    pub(crate) fn custom_size(mut self, size: Rems) -> Self {
        self.size = size;
        self
    }

    pub fn transform(mut self, transformation: Transformation) -> Self {
        self.transformation = transformation;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        svg()
            .with_transformation(self.transformation)
            .size(self.size)
            .flex_none()
            .path(self.path)
            .text_color(self.color.color(cx))
    }
}

#[derive(IntoElement)]
pub struct DecoratedIcon {
    icon: Icon,
    decoration: IconDecoration,
    decoration_color: Color,
    parent_background: Option<Hsla>,
}

impl DecoratedIcon {
    pub fn new(icon: Icon, decoration: IconDecoration) -> Self {
        Self {
            icon,
            decoration,
            decoration_color: Color::Default,
            parent_background: None,
        }
    }

    pub fn decoration_color(mut self, color: Color) -> Self {
        self.decoration_color = color;
        self
    }

    pub fn parent_background(mut self, background: Option<Hsla>) -> Self {
        self.parent_background = background;
        self
    }
}

impl RenderOnce for DecoratedIcon {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let background = self
            .parent_background
            .unwrap_or(cx.theme().colors().background);

        let size = self.icon.size;

        let decoration_icon = match self.decoration {
            IconDecoration::Strikethrough => IconName::Strikethrough,
            IconDecoration::IndicatorDot => IconName::Indicator,
            IconDecoration::X => IconName::IndicatorX,
        };

        let decoration_svg = |icon: IconName| {
            svg()
                .absolute()
                .top_0()
                .left_0()
                .path(icon.path())
                .size(size)
                .flex_none()
                .text_color(self.decoration_color.color(cx))
        };

        let decoration_knockout = |icon: IconName| {
            svg()
                .absolute()
                .top(-rems_from_px(2.))
                .left(-rems_from_px(3.))
                .path(icon.path())
                .size(size + rems_from_px(2.))
                .flex_none()
                .text_color(background)
        };

        div()
            .relative()
            .size(self.icon.size)
            .child(self.icon)
            .child(decoration_knockout(decoration_icon))
            .child(decoration_svg(decoration_icon))
    }
}

#[derive(IntoElement)]
pub struct IconWithIndicator {
    icon: Icon,
    indicator: Option<Indicator>,
    indicator_border_color: Option<Hsla>,
}

impl IconWithIndicator {
    pub fn new(icon: Icon, indicator: Option<Indicator>) -> Self {
        Self {
            icon,
            indicator,
            indicator_border_color: None,
        }
    }

    pub fn indicator(mut self, indicator: Option<Indicator>) -> Self {
        self.indicator = indicator;
        self
    }

    pub fn indicator_color(mut self, color: Color) -> Self {
        if let Some(indicator) = self.indicator.as_mut() {
            indicator.color = color;
        }
        self
    }

    pub fn indicator_border_color(mut self, color: Option<Hsla>) -> Self {
        self.indicator_border_color = color;
        self
    }
}

impl RenderOnce for IconWithIndicator {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let indicator_border_color = self
            .indicator_border_color
            .unwrap_or_else(|| cx.theme().colors().elevated_surface_background);

        div()
            .relative()
            .child(self.icon)
            .when_some(self.indicator, |this, indicator| {
                this.child(
                    div()
                        .absolute()
                        .size_2p5()
                        .border_2()
                        .border_color(indicator_border_color)
                        .rounded_full()
                        .bottom_neg_0p5()
                        .right_neg_0p5()
                        .child(indicator),
                )
            })
    }
}
