use gpui::{rems, svg, IntoElement, Rems, Svg};
use strum::EnumIter;

use crate::prelude::*;

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    XSmall,
    Small,
    #[default]
    Medium,
}

impl IconSize {
    pub fn rems(self) -> Rems {
        match self {
            IconSize::XSmall => rems(12. / 16.),
            IconSize::Small => rems(14. / 16.),
            IconSize::Medium => rems(16. / 16.),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum IconPath {
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
    Bolt,
    CaseSensitive,
    Check,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    Close,
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
    Plus,
    Public,
    Quote,
    Replace,
    ReplaceAll,
    ReplaceNext,
    Return,
    Screen,
    SelectAll,
    Shift,
    Snip,
    Space,
    Split,
    Tab,
    Terminal,
    Update,
    WholeWord,
    XCircle,
    ZedXCopilot,
}

impl IconPath {
    pub fn path(self) -> &'static str {
        match self {
            IconPath::Ai => "icons/ai.svg",
            IconPath::ArrowDown => "icons/arrow_down.svg",
            IconPath::ArrowLeft => "icons/arrow_left.svg",
            IconPath::ArrowRight => "icons/arrow_right.svg",
            IconPath::ArrowUp => "icons/arrow_up.svg",
            IconPath::ArrowUpRight => "icons/arrow_up_right.svg",
            IconPath::ArrowCircle => "icons/arrow_circle.svg",
            IconPath::AtSign => "icons/at_sign.svg",
            IconPath::AudioOff => "icons/speaker_off.svg",
            IconPath::AudioOn => "icons/speaker_loud.svg",
            IconPath::Backspace => "icons/backspace.svg",
            IconPath::Bell => "icons/bell.svg",
            IconPath::BellOff => "icons/bell_off.svg",
            IconPath::BellRing => "icons/bell_ring.svg",
            IconPath::Bolt => "icons/bolt.svg",
            IconPath::CaseSensitive => "icons/case_insensitive.svg",
            IconPath::Check => "icons/check.svg",
            IconPath::ChevronDown => "icons/chevron_down.svg",
            IconPath::ChevronLeft => "icons/chevron_left.svg",
            IconPath::ChevronRight => "icons/chevron_right.svg",
            IconPath::ChevronUp => "icons/chevron_up.svg",
            IconPath::Close => "icons/x.svg",
            IconPath::Collab => "icons/user_group_16.svg",
            IconPath::Command => "icons/command.svg",
            IconPath::Control => "icons/control.svg",
            IconPath::Copilot => "icons/copilot.svg",
            IconPath::CopilotDisabled => "icons/copilot_disabled.svg",
            IconPath::CopilotError => "icons/copilot_error.svg",
            IconPath::CopilotInit => "icons/copilot_init.svg",
            IconPath::Copy => "icons/copy.svg",
            IconPath::Dash => "icons/dash.svg",
            IconPath::Delete => "icons/delete.svg",
            IconPath::Disconnected => "icons/disconnected.svg",
            IconPath::Ellipsis => "icons/ellipsis.svg",
            IconPath::Envelope => "icons/feedback.svg",
            IconPath::Escape => "icons/escape.svg",
            IconPath::ExclamationTriangle => "icons/warning.svg",
            IconPath::Exit => "icons/exit.svg",
            IconPath::ExternalLink => "icons/external_link.svg",
            IconPath::File => "icons/file.svg",
            IconPath::FileDoc => "icons/file_icons/book.svg",
            IconPath::FileGeneric => "icons/file_icons/file.svg",
            IconPath::FileGit => "icons/file_icons/git.svg",
            IconPath::FileLock => "icons/file_icons/lock.svg",
            IconPath::FileRust => "icons/file_icons/rust.svg",
            IconPath::FileToml => "icons/file_icons/toml.svg",
            IconPath::FileTree => "icons/project.svg",
            IconPath::Filter => "icons/filter.svg",
            IconPath::Folder => "icons/file_icons/folder.svg",
            IconPath::FolderOpen => "icons/file_icons/folder_open.svg",
            IconPath::FolderX => "icons/stop_sharing.svg",
            IconPath::Github => "icons/github.svg",
            IconPath::Hash => "icons/hash.svg",
            IconPath::InlayHint => "icons/inlay_hint.svg",
            IconPath::Link => "icons/link.svg",
            IconPath::MagicWand => "icons/magic_wand.svg",
            IconPath::MagnifyingGlass => "icons/magnifying_glass.svg",
            IconPath::MailOpen => "icons/mail_open.svg",
            IconPath::Maximize => "icons/maximize.svg",
            IconPath::Menu => "icons/menu.svg",
            IconPath::MessageBubbles => "icons/conversations.svg",
            IconPath::Mic => "icons/mic.svg",
            IconPath::MicMute => "icons/mic_mute.svg",
            IconPath::Minimize => "icons/minimize.svg",
            IconPath::Option => "icons/option.svg",
            IconPath::PageDown => "icons/page_down.svg",
            IconPath::PageUp => "icons/page_up.svg",
            IconPath::Plus => "icons/plus.svg",
            IconPath::Public => "icons/public.svg",
            IconPath::Quote => "icons/quote.svg",
            IconPath::Replace => "icons/replace.svg",
            IconPath::ReplaceAll => "icons/replace_all.svg",
            IconPath::ReplaceNext => "icons/replace_next.svg",
            IconPath::Return => "icons/return.svg",
            IconPath::Screen => "icons/desktop.svg",
            IconPath::SelectAll => "icons/select_all.svg",
            IconPath::Shift => "icons/shift.svg",
            IconPath::Snip => "icons/snip.svg",
            IconPath::Space => "icons/space.svg",
            IconPath::Split => "icons/split.svg",
            IconPath::Tab => "icons/tab.svg",
            IconPath::Terminal => "icons/terminal.svg",
            IconPath::Update => "icons/update.svg",
            IconPath::WholeWord => "icons/word_search.svg",
            IconPath::XCircle => "icons/error.svg",
            IconPath::ZedXCopilot => "icons/zed_x_copilot.svg",
        }
    }
}

#[derive(IntoElement)]
pub struct Icon {
    path: SharedString,
    color: Color,
    size: IconSize,
}

impl RenderOnce for Icon {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        svg()
            .size(self.size.rems())
            .flex_none()
            .path(self.path)
            .text_color(self.color.color(cx))
    }
}

impl Icon {
    pub fn new(icon: IconPath) -> Self {
        Self {
            path: icon.path().into(),
            color: Color::default(),
            size: IconSize::default(),
        }
    }

    pub fn from_path(path: impl Into<SharedString>) -> Self {
        Self {
            path: path.into(),
            color: Color::default(),
            size: IconSize::default(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }
}
