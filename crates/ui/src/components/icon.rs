use gpui::{svg, AnimationElement, Hsla, IntoElement, Rems, Transformation};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoStaticStr};

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
/// or a diagonal strkethrough to indicate something is disabled.
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
    Debug, PartialEq, Eq, Copy, Clone, EnumIter, EnumString, IntoStaticStr, Serialize, Deserialize,
)]
pub enum IconName {
    Ai,
    AiAnthropic,
    AiAnthropicHosted,
    AiOpenAi,
    AiGoogle,
    AiOllama,
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
    /// This chevron indicates a popover menu.
    ChevronDownSmall,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronUpDown,
    Close,
    Code,
    Collab,
    Command,
    Context,
    Control,
    Copilot,
    CopilotDisabled,
    CopilotError,
    CopilotInit,
    Copy,
    CountdownTimer,
    Dash,
    DatabaseZap,
    Delete,
    Disconnected,
    Download,
    Ellipsis,
    EllipsisVertical,
    Envelope,
    Escape,
    ExclamationTriangle,
    Exit,
    ExpandVertical,
    ExternalLink,
    Eye,
    File,
    FileDoc,
    FileGeneric,
    FileGit,
    FileLock,
    FileRust,
    FileToml,
    FileTree,
    FileText,
    FileCode,
    Filter,
    Folder,
    FolderOpen,
    FolderX,
    Font,
    FontSize,
    FontWeight,
    Github,
    GenericMinimize,
    GenericMaximize,
    GenericClose,
    GenericRestore,
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
    Route,
    RotateCcw,
    RotateCw,
    Save,
    Screen,
    SearchSelection,
    SearchCode,
    SelectAll,
    Server,
    Settings,
    Shift,
    Slash,
    SlashSquare,
    Sliders,
    SlidersAlt,
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
    TextCursor,
    TextSelect,
    Trash,
    TriangleRight,
    Undo,
    Unpin,
    Update,
    WholeWord,
    XCircle,
    ZedAssistant,
    ZedAssistantFilled,
    ZedXCopilot,
    Visible,
}

impl IconName {
    pub fn path(self) -> &'static str {
        match self {
            IconName::Ai => "icons/ai.svg",
            IconName::AiAnthropic => "icons/ai_anthropic.svg",
            IconName::AiAnthropicHosted => "icons/ai_anthropic_hosted.svg",
            IconName::AiOpenAi => "icons/ai_open_ai.svg",
            IconName::AiGoogle => "icons/ai_google.svg",
            IconName::AiOllama => "icons/ai_ollama.svg",
            IconName::AiZed => "icons/ai_zed.svg",
            IconName::ArrowCircle => "icons/arrow_circle.svg",
            IconName::ArrowDown => "icons/arrow_down.svg",
            IconName::ArrowDownFromLine => "icons/arrow_down_from_line.svg",
            IconName::ArrowLeft => "icons/arrow_left.svg",
            IconName::ArrowRight => "icons/arrow_right.svg",
            IconName::ArrowUp => "icons/arrow_up.svg",
            IconName::ArrowUpFromLine => "icons/arrow_up_from_line.svg",
            IconName::ArrowUpRight => "icons/arrow_up_right.svg",
            IconName::AtSign => "icons/at_sign.svg",
            IconName::AudioOff => "icons/speaker_off.svg",
            IconName::AudioOn => "icons/speaker_loud.svg",
            IconName::Backspace => "icons/backspace.svg",
            IconName::Bell => "icons/bell.svg",
            IconName::BellDot => "icons/bell_dot.svg",
            IconName::BellOff => "icons/bell_off.svg",
            IconName::BellRing => "icons/bell_ring.svg",
            IconName::Bolt => "icons/bolt.svg",
            IconName::Book => "icons/book.svg",
            IconName::BookCopy => "icons/book_copy.svg",
            IconName::BookPlus => "icons/book_plus.svg",
            IconName::CaseSensitive => "icons/case_insensitive.svg",
            IconName::Check => "icons/check.svg",
            IconName::ChevronDown => "icons/chevron_down.svg",
            IconName::ChevronDownSmall => "icons/chevron_down_small.svg",
            IconName::ChevronLeft => "icons/chevron_left.svg",
            IconName::ChevronRight => "icons/chevron_right.svg",
            IconName::ChevronUp => "icons/chevron_up.svg",
            IconName::ChevronUpDown => "icons/chevron_up_down.svg",
            IconName::Close => "icons/x.svg",
            IconName::Code => "icons/code.svg",
            IconName::Collab => "icons/user_group_16.svg",
            IconName::Command => "icons/command.svg",
            IconName::Context => "icons/context.svg",
            IconName::Control => "icons/control.svg",
            IconName::Copilot => "icons/copilot.svg",
            IconName::CopilotDisabled => "icons/copilot_disabled.svg",
            IconName::CopilotError => "icons/copilot_error.svg",
            IconName::CopilotInit => "icons/copilot_init.svg",
            IconName::Copy => "icons/copy.svg",
            IconName::CountdownTimer => "icons/countdown_timer.svg",
            IconName::Dash => "icons/dash.svg",
            IconName::DatabaseZap => "icons/database_zap.svg",
            IconName::Delete => "icons/delete.svg",
            IconName::Disconnected => "icons/disconnected.svg",
            IconName::Download => "icons/download.svg",
            IconName::Ellipsis => "icons/ellipsis.svg",
            IconName::EllipsisVertical => "icons/ellipsis_vertical.svg",
            IconName::Envelope => "icons/feedback.svg",
            IconName::Escape => "icons/escape.svg",
            IconName::ExclamationTriangle => "icons/warning.svg",
            IconName::Exit => "icons/exit.svg",
            IconName::ExpandVertical => "icons/expand_vertical.svg",
            IconName::ExternalLink => "icons/external_link.svg",
            IconName::Eye => "icons/eye.svg",
            IconName::File => "icons/file.svg",
            IconName::FileDoc => "icons/file_icons/book.svg",
            IconName::FileGeneric => "icons/file_icons/file.svg",
            IconName::FileGit => "icons/file_icons/git.svg",
            IconName::FileLock => "icons/file_icons/lock.svg",
            IconName::FileRust => "icons/file_icons/rust.svg",
            IconName::FileToml => "icons/file_icons/toml.svg",
            IconName::FileTree => "icons/project.svg",
            IconName::FileCode => "icons/file_code.svg",
            IconName::FileText => "icons/file_text.svg",
            IconName::Filter => "icons/filter.svg",
            IconName::Folder => "icons/file_icons/folder.svg",
            IconName::FolderOpen => "icons/file_icons/folder_open.svg",
            IconName::FolderX => "icons/stop_sharing.svg",
            IconName::Font => "icons/font.svg",
            IconName::FontSize => "icons/font_size.svg",
            IconName::FontWeight => "icons/font_weight.svg",
            IconName::Github => "icons/github.svg",
            IconName::GenericMinimize => "icons/generic_minimize.svg",
            IconName::GenericMaximize => "icons/generic_maximize.svg",
            IconName::GenericClose => "icons/generic_close.svg",
            IconName::GenericRestore => "icons/generic_restore.svg",
            IconName::Hash => "icons/hash.svg",
            IconName::HistoryRerun => "icons/history_rerun.svg",
            IconName::Indicator => "icons/indicator.svg",
            IconName::IndicatorX => "icons/indicator_x.svg",
            IconName::InlayHint => "icons/inlay_hint.svg",
            IconName::Library => "icons/library.svg",
            IconName::LineHeight => "icons/line_height.svg",
            IconName::Link => "icons/link.svg",
            IconName::ListTree => "icons/list_tree.svg",
            IconName::MagnifyingGlass => "icons/magnifying_glass.svg",
            IconName::MailOpen => "icons/mail_open.svg",
            IconName::Maximize => "icons/maximize.svg",
            IconName::Menu => "icons/menu.svg",
            IconName::MessageBubbles => "icons/conversations.svg",
            IconName::Mic => "icons/mic.svg",
            IconName::MicMute => "icons/mic_mute.svg",
            IconName::Microscope => "icons/microscope.svg",
            IconName::Minimize => "icons/minimize.svg",
            IconName::Option => "icons/option.svg",
            IconName::PageDown => "icons/page_down.svg",
            IconName::PageUp => "icons/page_up.svg",
            IconName::Pencil => "icons/pencil.svg",
            IconName::Person => "icons/person.svg",
            IconName::Pin => "icons/pin.svg",
            IconName::Play => "icons/play.svg",
            IconName::Plus => "icons/plus.svg",
            IconName::Public => "icons/public.svg",
            IconName::PullRequest => "icons/pull_request.svg",
            IconName::Quote => "icons/quote.svg",
            IconName::Regex => "icons/regex.svg",
            IconName::ReplNeutral => "icons/repl_neutral.svg",
            IconName::Replace => "icons/replace.svg",
            IconName::ReplaceAll => "icons/replace_all.svg",
            IconName::ReplaceNext => "icons/replace_next.svg",
            IconName::ReplyArrowRight => "icons/reply_arrow_right.svg",
            IconName::Rerun => "icons/rerun.svg",
            IconName::Return => "icons/return.svg",
            IconName::Reveal => "icons/reveal.svg",
            IconName::RotateCcw => "icons/rotate_ccw.svg",
            IconName::RotateCw => "icons/rotate_cw.svg",
            IconName::Route => "icons/route.svg",
            IconName::Save => "icons/save.svg",
            IconName::Screen => "icons/desktop.svg",
            IconName::SearchSelection => "icons/search_selection.svg",
            IconName::SearchCode => "icons/search_code.svg",
            IconName::SelectAll => "icons/select_all.svg",
            IconName::Server => "icons/server.svg",
            IconName::Settings => "icons/file_icons/settings.svg",
            IconName::Shift => "icons/shift.svg",
            IconName::Slash => "icons/slash.svg",
            IconName::SlashSquare => "icons/slash_square.svg",
            IconName::Sliders => "icons/sliders.svg",
            IconName::SlidersAlt => "icons/sliders-alt.svg",
            IconName::Snip => "icons/snip.svg",
            IconName::Space => "icons/space.svg",
            IconName::Sparkle => "icons/sparkle.svg",
            IconName::SparkleAlt => "icons/sparkle_alt.svg",
            IconName::SparkleFilled => "icons/sparkle_filled.svg",
            IconName::Spinner => "icons/spinner.svg",
            IconName::Split => "icons/split.svg",
            IconName::Star => "icons/star.svg",
            IconName::StarFilled => "icons/star_filled.svg",
            IconName::Stop => "icons/stop.svg",
            IconName::Strikethrough => "icons/strikethrough.svg",
            IconName::Supermaven => "icons/supermaven.svg",
            IconName::SupermavenDisabled => "icons/supermaven_disabled.svg",
            IconName::SupermavenError => "icons/supermaven_error.svg",
            IconName::SupermavenInit => "icons/supermaven_init.svg",
            IconName::Tab => "icons/tab.svg",
            IconName::Terminal => "icons/terminal.svg",
            IconName::TextCursor => "icons/text-cursor.svg",
            IconName::TextSelect => "icons/text_select.svg",
            IconName::Trash => "icons/trash.svg",
            IconName::TriangleRight => "icons/triangle_right.svg",
            IconName::Unpin => "icons/unpin.svg",
            IconName::Update => "icons/update.svg",
            IconName::Undo => "icons/undo.svg",
            IconName::WholeWord => "icons/word_search.svg",
            IconName::XCircle => "icons/error.svg",
            IconName::ZedAssistant => "icons/zed_assistant.svg",
            IconName::ZedAssistantFilled => "icons/zed_assistant_filled.svg",
            IconName::ZedXCopilot => "icons/zed_x_copilot.svg",
            IconName::Visible => "icons/visible.svg",
        }
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
                        .w_2()
                        .h_2()
                        .border_1()
                        .border_color(indicator_border_color)
                        .rounded_full()
                        .bottom_neg_0p5()
                        .right_neg_1()
                        .child(indicator),
                )
            })
    }
}
