use gpui::{svg, AnimationElement, Hsla, IntoElement, Rems, Transformation};
use strum::EnumIter;

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

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    Indicator,
    XSmall,
    Small,
    #[default]
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
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum IconName {
    Ai,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowUpRight,
    ArrowCircle,
    AtSign,
    AudioOff,
    AudioOn,
    Backspace,
    Bell,
    BellOff,
    BellRing,
    BellDot,
    Bolt,
    CaseSensitive,
    Check,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ExpandVertical,
    Close,
    Code,
    Collab,
    Command,
    Control,
    Copilot,
    CopilotDisabled,
    CopilotError,
    CopilotInit,
    Copy,
    Dash,
    Delete,
    Disconnected,
    Ellipsis,
    Envelope,
    Escape,
    ExclamationTriangle,
    Exit,
    ExternalLink,
    File,
    FileDoc,
    FileGeneric,
    FileGit,
    FileLock,
    FileRust,
    FileToml,
    FileTree,
    Filter,
    Folder,
    FolderOpen,
    FolderX,
    Github,
    Hash,
    InlayHint,
    Link,
    MagicWand,
    MagnifyingGlass,
    MailOpen,
    Maximize,
    Menu,
    MessageBubbles,
    Mic,
    MicMute,
    Minimize,
    Option,
    PageDown,
    PageUp,
    Pencil,
    Person,
    Play,
    Plus,
    Public,
    Quote,
    Regex,
    Replace,
    ReplaceAll,
    ReplaceNext,
    Return,
    ReplyArrowRight,
    Settings,
    Sliders,
    Screen,
    SelectAll,
    Server,
    Shift,
    Snip,
    Space,
    Split,
    Spinner,
    Tab,
    Terminal,
    Trash,
    Update,
    WholeWord,
    XCircle,
    ZedXCopilot,
    ZedAssistant,
    PullRequest,
}

impl IconName {
    pub fn path(self) -> &'static str {
        match self {
            IconName::Ai => "icons/ai.svg",
            IconName::ArrowDown => "icons/arrow_down.svg",
            IconName::ArrowLeft => "icons/arrow_left.svg",
            IconName::ArrowRight => "icons/arrow_right.svg",
            IconName::ArrowUp => "icons/arrow_up.svg",
            IconName::ArrowUpRight => "icons/arrow_up_right.svg",
            IconName::ArrowCircle => "icons/arrow_circle.svg",
            IconName::AtSign => "icons/at_sign.svg",
            IconName::AudioOff => "icons/speaker_off.svg",
            IconName::AudioOn => "icons/speaker_loud.svg",
            IconName::Backspace => "icons/backspace.svg",
            IconName::Bell => "icons/bell.svg",
            IconName::BellOff => "icons/bell_off.svg",
            IconName::BellRing => "icons/bell_ring.svg",
            IconName::BellDot => "icons/bell_dot.svg",
            IconName::Bolt => "icons/bolt.svg",
            IconName::CaseSensitive => "icons/case_insensitive.svg",
            IconName::Check => "icons/check.svg",
            IconName::ChevronDown => "icons/chevron_down.svg",
            IconName::ChevronLeft => "icons/chevron_left.svg",
            IconName::ChevronRight => "icons/chevron_right.svg",
            IconName::ChevronUp => "icons/chevron_up.svg",
            IconName::ExpandVertical => "icons/expand_vertical.svg",
            IconName::Close => "icons/x.svg",
            IconName::Code => "icons/code.svg",
            IconName::Collab => "icons/user_group_16.svg",
            IconName::Command => "icons/command.svg",
            IconName::Control => "icons/control.svg",
            IconName::Copilot => "icons/copilot.svg",
            IconName::CopilotDisabled => "icons/copilot_disabled.svg",
            IconName::CopilotError => "icons/copilot_error.svg",
            IconName::CopilotInit => "icons/copilot_init.svg",
            IconName::Copy => "icons/copy.svg",
            IconName::Dash => "icons/dash.svg",
            IconName::Delete => "icons/delete.svg",
            IconName::Disconnected => "icons/disconnected.svg",
            IconName::Ellipsis => "icons/ellipsis.svg",
            IconName::Envelope => "icons/feedback.svg",
            IconName::Escape => "icons/escape.svg",
            IconName::ExclamationTriangle => "icons/warning.svg",
            IconName::Exit => "icons/exit.svg",
            IconName::ExternalLink => "icons/external_link.svg",
            IconName::File => "icons/file.svg",
            IconName::FileDoc => "icons/file_icons/book.svg",
            IconName::FileGeneric => "icons/file_icons/file.svg",
            IconName::FileGit => "icons/file_icons/git.svg",
            IconName::FileLock => "icons/file_icons/lock.svg",
            IconName::FileRust => "icons/file_icons/rust.svg",
            IconName::FileToml => "icons/file_icons/toml.svg",
            IconName::FileTree => "icons/project.svg",
            IconName::Filter => "icons/filter.svg",
            IconName::Folder => "icons/file_icons/folder.svg",
            IconName::FolderOpen => "icons/file_icons/folder_open.svg",
            IconName::FolderX => "icons/stop_sharing.svg",
            IconName::Github => "icons/github.svg",
            IconName::Hash => "icons/hash.svg",
            IconName::InlayHint => "icons/inlay_hint.svg",
            IconName::Link => "icons/link.svg",
            IconName::MagicWand => "icons/magic_wand.svg",
            IconName::MagnifyingGlass => "icons/magnifying_glass.svg",
            IconName::MailOpen => "icons/mail_open.svg",
            IconName::Maximize => "icons/maximize.svg",
            IconName::Menu => "icons/menu.svg",
            IconName::MessageBubbles => "icons/conversations.svg",
            IconName::Mic => "icons/mic.svg",
            IconName::MicMute => "icons/mic_mute.svg",
            IconName::Minimize => "icons/minimize.svg",
            IconName::Option => "icons/option.svg",
            IconName::PageDown => "icons/page_down.svg",
            IconName::PageUp => "icons/page_up.svg",
            IconName::Person => "icons/person.svg",
            IconName::Pencil => "icons/pencil.svg",
            IconName::Play => "icons/play.svg",
            IconName::Plus => "icons/plus.svg",
            IconName::Public => "icons/public.svg",
            IconName::Quote => "icons/quote.svg",
            IconName::Regex => "icons/regex.svg",
            IconName::Replace => "icons/replace.svg",
            IconName::ReplaceAll => "icons/replace_all.svg",
            IconName::ReplaceNext => "icons/replace_next.svg",
            IconName::Return => "icons/return.svg",
            IconName::ReplyArrowRight => "icons/reply_arrow_right.svg",
            IconName::Settings => "icons/file_icons/settings.svg",
            IconName::Sliders => "icons/sliders.svg",
            IconName::Screen => "icons/desktop.svg",
            IconName::SelectAll => "icons/select_all.svg",
            IconName::Server => "icons/server.svg",
            IconName::Shift => "icons/shift.svg",
            IconName::Snip => "icons/snip.svg",
            IconName::Space => "icons/space.svg",
            IconName::Split => "icons/split.svg",
            IconName::Spinner => "icons/spinner.svg",
            IconName::Tab => "icons/tab.svg",
            IconName::Terminal => "icons/terminal.svg",
            IconName::Trash => "icons/trash.svg",
            IconName::Update => "icons/update.svg",
            IconName::WholeWord => "icons/word_search.svg",
            IconName::XCircle => "icons/error.svg",
            IconName::ZedXCopilot => "icons/zed_x_copilot.svg",
            IconName::ZedAssistant => "icons/zed_assistant.svg",
            IconName::PullRequest => "icons/pull_request.svg",
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
                        .border()
                        .border_color(indicator_border_color)
                        .rounded_full()
                        .neg_bottom_0p5()
                        .neg_right_1()
                        .child(indicator),
                )
            })
    }
}
