#![allow(missing_docs)]
use gpui::{svg, AnimationElement, Hsla, IntoElement, Point, Rems, Transformation};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};
use ui_macros::DerivePathStr;

use crate::{
    prelude::*,
    traits::component_preview::{ComponentExample, ComponentPreview},
    Indicator,
};

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
            IconSize::Indicator => DynamicSpacing::Base00.px(cx),
            IconSize::XSmall => DynamicSpacing::Base02.px(cx),
            IconSize::Small => DynamicSpacing::Base02.px(cx),
            IconSize::Medium => DynamicSpacing::Base02.px(cx),
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
    Blocks,
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
    Dash,
    DatabaseZap,
    Delete,
    Diff,
    Disconnected,
    Download,
    Ellipsis,
    EllipsisVertical,
    Envelope,
    Eraser,
    Escape,
    ExpandVertical,
    Exit,
    ExternalLink,
    Eye,
    File,
    FileCode,
    FileDoc,
    FileDiff,
    FileGeneric,
    FileGit,
    FileLock,
    FileRust,
    FileSearch,
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
    Globe,
    Hash,
    HistoryRerun,
    Indicator,
    IndicatorX,
    Info,
    InlayHint,
    Keyboard,
    Library,
    LineHeight,
    Link,
    ListTree,
    ListX,
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
    PhoneIncoming,
    Pin,
    Play,
    Plus,
    PocketKnife,
    Public,
    PullRequest,
    Quote,
    RefreshTitle,
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
    SwatchBook,
    Tab,
    Terminal,
    TextSnippet,
    ThumbsUp,
    ThumbsDown,
    Trash,
    TrashAlt,
    Triangle,
    TriangleRight,
    Undo,
    Unpin,
    Update,
    UserGroup,
    Visible,
    Wand,
    Warning,
    WholeWord,
    X,
    XCircle,
    ZedAssistant,
    ZedAssistantFilled,
    ZedXCopilot,
}

impl From<IconName> for Icon {
    fn from(icon: IconName) -> Self {
        Icon::new(icon)
    }
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

const ICON_DECORATION_SIZE: f32 = 11.0;

/// An icon silhouette used to knockout the background of an element
/// for an icon to sit on top of it, emulating a stroke/border.
#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter, EnumString, IntoStaticStr, DerivePathStr)]
#[strum(serialize_all = "snake_case")]
#[path_str(prefix = "icons/knockouts", suffix = ".svg")]
pub enum KnockoutIconName {
    // /icons/knockouts/x1.svg
    XFg,
    XBg,
    DotFg,
    DotBg,
    TriangleFg,
    TriangleBg,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter, EnumString)]
pub enum IconDecorationKind {
    // Slash,
    X,
    Dot,
    Triangle,
}

impl IconDecorationKind {
    fn fg(&self) -> KnockoutIconName {
        match self {
            Self::X => KnockoutIconName::XFg,
            Self::Dot => KnockoutIconName::DotFg,
            Self::Triangle => KnockoutIconName::TriangleFg,
        }
    }

    fn bg(&self) -> KnockoutIconName {
        match self {
            Self::X => KnockoutIconName::XBg,
            Self::Dot => KnockoutIconName::DotBg,
            Self::Triangle => KnockoutIconName::TriangleBg,
        }
    }
}

/// The decoration for an icon.
///
/// For example, this can show an indicator, an "x",
/// or a diagonal strikethrough to indicate something is disabled.
#[derive(IntoElement)]
pub struct IconDecoration {
    kind: IconDecorationKind,
    color: Hsla,
    knockout_color: Hsla,
    position: Point<Pixels>,
}

impl IconDecoration {
    /// Create a new icon decoration
    pub fn new(kind: IconDecorationKind, knockout_color: Hsla, cx: &WindowContext) -> Self {
        let color = cx.theme().colors().icon;
        let position = Point::default();

        Self {
            kind,
            color,
            knockout_color,
            position,
        }
    }

    /// Sets the kind of decoration
    pub fn kind(mut self, kind: IconDecorationKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the color of the decoration
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    /// Sets the color of the decoration's knockout
    ///
    /// Match this to the background of the element
    /// the icon will be rendered on
    pub fn knockout_color(mut self, color: Hsla) -> Self {
        self.knockout_color = color;
        self
    }

    /// Sets the position of the decoration
    pub fn position(mut self, position: Point<Pixels>) -> Self {
        self.position = position;
        self
    }
}

impl RenderOnce for IconDecoration {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .size(px(ICON_DECORATION_SIZE))
            .flex_none()
            .absolute()
            .bottom(self.position.y)
            .right(self.position.x)
            .child(
                // foreground
                svg()
                    .absolute()
                    .bottom_0()
                    .right_0()
                    .size(px(ICON_DECORATION_SIZE))
                    .path(self.kind.fg().path())
                    .text_color(self.color),
            )
            .child(
                // background
                svg()
                    .absolute()
                    .bottom_0()
                    .right_0()
                    .size(px(ICON_DECORATION_SIZE))
                    .path(self.kind.bg().path())
                    .text_color(self.knockout_color),
            )
    }
}

impl ComponentPreview for IconDecoration {
    fn examples(cx: &WindowContext) -> Vec<ComponentExampleGroup<Self>> {
        let all_kinds = IconDecorationKind::iter().collect::<Vec<_>>();

        let examples = all_kinds
            .iter()
            .map(|kind| {
                let name = format!("{:?}", kind).to_string();

                single_example(
                    name,
                    IconDecoration::new(*kind, cx.theme().colors().surface_background, cx),
                )
            })
            .collect();

        vec![example_group(examples)]
    }
}

#[derive(IntoElement)]
pub struct DecoratedIcon {
    icon: Icon,
    decoration: Option<IconDecoration>,
}

impl DecoratedIcon {
    pub fn new(icon: Icon, decoration: Option<IconDecoration>) -> Self {
        Self { icon, decoration }
    }
}

impl RenderOnce for DecoratedIcon {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .relative()
            .size(self.icon.size)
            .child(self.icon)
            .when_some(self.decoration, |this, decoration| this.child(decoration))
    }
}

impl ComponentPreview for DecoratedIcon {
    fn examples(cx: &WindowContext) -> Vec<ComponentExampleGroup<Self>> {
        let icon_1 = Icon::new(IconName::FileDoc);
        let icon_2 = Icon::new(IconName::FileDoc);
        let icon_3 = Icon::new(IconName::FileDoc);
        let icon_4 = Icon::new(IconName::FileDoc);

        let decoration_x = IconDecoration::new(
            IconDecorationKind::X,
            cx.theme().colors().surface_background,
            cx,
        )
        .color(cx.theme().status().error)
        .position(Point {
            x: px(-2.),
            y: px(-2.),
        });

        let decoration_triangle = IconDecoration::new(
            IconDecorationKind::Triangle,
            cx.theme().colors().surface_background,
            cx,
        )
        .color(cx.theme().status().error)
        .position(Point {
            x: px(-2.),
            y: px(-2.),
        });

        let decoration_dot = IconDecoration::new(
            IconDecorationKind::Dot,
            cx.theme().colors().surface_background,
            cx,
        )
        .color(cx.theme().status().error)
        .position(Point {
            x: px(-2.),
            y: px(-2.),
        });

        let examples = vec![
            single_example("no_decoration", DecoratedIcon::new(icon_1, None)),
            single_example(
                "with_decoration",
                DecoratedIcon::new(icon_2, Some(decoration_x)),
            ),
            single_example(
                "with_decoration",
                DecoratedIcon::new(icon_3, Some(decoration_triangle)),
            ),
            single_example(
                "with_decoration",
                DecoratedIcon::new(icon_4, Some(decoration_dot)),
            ),
        ];

        vec![example_group(examples)]
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

impl ComponentPreview for Icon {
    fn examples(_cx: &WindowContext) -> Vec<ComponentExampleGroup<Icon>> {
        let arrow_icons = vec![
            IconName::ArrowDown,
            IconName::ArrowLeft,
            IconName::ArrowRight,
            IconName::ArrowUp,
            IconName::ArrowCircle,
        ];

        vec![example_group_with_title(
            "Arrow Icons",
            arrow_icons
                .into_iter()
                .map(|icon| {
                    let name = format!("{:?}", icon).to_string();
                    ComponentExample::new(name, Icon::new(icon))
                })
                .collect(),
        )]
    }
}
