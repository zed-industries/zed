use gpui::{rems, svg, IntoElement, Svg};
use strum::EnumIter;

use crate::prelude::*;

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    Small,
    #[default]
    Medium,
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum Icon {
    Ai,
    ArrowLeft,
    ArrowUp,
    ArrowDown,
    ArrowRight,
    ArrowUpRight,
    AtSign,
    AudioOff,
    AudioOn,
    Bell,
    BellOff,
    BellRing,
    Bolt,
    CaseSensitive,
    Check,
    Copy,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    Close,
    Collab,
    Copilot,
    CopilotInit,
    CopilotError,
    CopilotDisabled,
    Dash,
    Envelope,
    ExclamationTriangle,
    Exit,
    File,
    FileDoc,
    FileGeneric,
    FileGit,
    FileLock,
    FileRust,
    FileToml,
    FileTree,
    Folder,
    FolderOpen,
    FolderX,
    Hash,
    InlayHint,
    MagicWand,
    MagnifyingGlass,
    MailOpen,
    Maximize,
    Menu,
    MessageBubbles,
    Mic,
    MicMute,
    Plus,
    Public,
    Quote,
    Replace,
    ReplaceAll,
    Screen,
    SelectAll,
    Split,
    SplitMessage,
    Terminal,
    WholeWord,
    XCircle,
    Command,
    Control,
    Shift,
    Option,
    Return,
}

impl Icon {
    pub fn path(self) -> &'static str {
        match self {
            Icon::Ai => "icons/ai.svg",
            Icon::ArrowLeft => "icons/arrow_left.svg",
            Icon::ArrowRight => "icons/arrow_right.svg",
            Icon::ArrowUp => "icons/arrow_up.svg",
            Icon::ArrowDown => "icons/arrow_down.svg",
            Icon::ArrowUpRight => "icons/arrow_up_right.svg",
            Icon::AtSign => "icons/at-sign.svg",
            Icon::AudioOff => "icons/speaker-off.svg",
            Icon::AudioOn => "icons/speaker-loud.svg",
            Icon::Bell => "icons/bell.svg",
            Icon::BellOff => "icons/bell-off.svg",
            Icon::BellRing => "icons/bell-ring.svg",
            Icon::Bolt => "icons/bolt.svg",
            Icon::CaseSensitive => "icons/case_insensitive.svg",
            Icon::Check => "icons/check.svg",
            Icon::Copy => "icons/copy.svg",
            Icon::ChevronDown => "icons/chevron_down.svg",
            Icon::ChevronLeft => "icons/chevron_left.svg",
            Icon::ChevronRight => "icons/chevron_right.svg",
            Icon::ChevronUp => "icons/chevron_up.svg",
            Icon::Close => "icons/x.svg",
            Icon::Collab => "icons/user_group_16.svg",
            Icon::Copilot => "icons/copilot.svg",
            Icon::CopilotInit => "icons/copilot_init.svg",
            Icon::CopilotError => "icons/copilot_error.svg",
            Icon::CopilotDisabled => "icons/copilot_disabled.svg",
            Icon::Dash => "icons/dash.svg",
            Icon::Envelope => "icons/feedback.svg",
            Icon::ExclamationTriangle => "icons/warning.svg",
            Icon::Exit => "icons/exit.svg",
            Icon::File => "icons/file.svg",
            Icon::FileDoc => "icons/file_icons/book.svg",
            Icon::FileGeneric => "icons/file_icons/file.svg",
            Icon::FileGit => "icons/file_icons/git.svg",
            Icon::FileLock => "icons/file_icons/lock.svg",
            Icon::FileRust => "icons/file_icons/rust.svg",
            Icon::FileToml => "icons/file_icons/toml.svg",
            Icon::FileTree => "icons/project.svg",
            Icon::Folder => "icons/file_icons/folder.svg",
            Icon::FolderOpen => "icons/file_icons/folder_open.svg",
            Icon::FolderX => "icons/stop_sharing.svg",
            Icon::Hash => "icons/hash.svg",
            Icon::InlayHint => "icons/inlay_hint.svg",
            Icon::MagicWand => "icons/magic-wand.svg",
            Icon::MagnifyingGlass => "icons/magnifying_glass.svg",
            Icon::MailOpen => "icons/mail-open.svg",
            Icon::Maximize => "icons/maximize.svg",
            Icon::Menu => "icons/menu.svg",
            Icon::MessageBubbles => "icons/conversations.svg",
            Icon::Mic => "icons/mic.svg",
            Icon::MicMute => "icons/mic-mute.svg",
            Icon::Plus => "icons/plus.svg",
            Icon::Public => "icons/public.svg",
            Icon::Quote => "icons/quote.svg",
            Icon::Replace => "icons/replace.svg",
            Icon::ReplaceAll => "icons/replace_all.svg",
            Icon::Screen => "icons/desktop.svg",
            Icon::SelectAll => "icons/select-all.svg",
            Icon::Split => "icons/split.svg",
            Icon::SplitMessage => "icons/split_message.svg",
            Icon::Terminal => "icons/terminal.svg",
            Icon::WholeWord => "icons/word_search.svg",
            Icon::XCircle => "icons/error.svg",
            Icon::Command => "icons/command.svg",
            Icon::Control => "icons/control.svg",
            Icon::Shift => "icons/shift.svg",
            Icon::Option => "icons/option.svg",
            Icon::Return => "icons/return.svg",
        }
    }
}

#[derive(IntoElement)]
pub struct IconElement {
    path: SharedString,
    color: Color,
    size: IconSize,
}

impl RenderOnce for IconElement {
    type Rendered = Svg;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let svg_size = match self.size {
            IconSize::Small => rems(14. / 16.),
            IconSize::Medium => rems(16. / 16.),
        };

        svg()
            .size(svg_size)
            .flex_none()
            .path(self.path)
            .text_color(self.color.color(cx))
    }
}

impl IconElement {
    pub fn new(icon: Icon) -> Self {
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
