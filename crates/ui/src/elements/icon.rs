use std::sync::Arc;

use gpui2::elements::svg;
use gpui2::{Element, ViewContext};
use gpui2::{Hsla, IntoElement};
use strum::EnumIter;

use crate::prelude::*;
use crate::theme::theme;
use crate::Theme;

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    Small,
    #[default]
    Large,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconColor {
    #[default]
    Default,
    Muted,
    Disabled,
    Placeholder,
    Accent,
    Error,
    Warning,
    Success,
    Info,
}

impl IconColor {
    pub fn color(self, theme: Arc<Theme>) -> Hsla {
        match self {
            IconColor::Default => theme.lowest.base.default.foreground,
            IconColor::Muted => theme.lowest.variant.default.foreground,
            IconColor::Disabled => theme.lowest.base.disabled.foreground,
            IconColor::Placeholder => theme.lowest.base.disabled.foreground,
            IconColor::Accent => theme.lowest.accent.default.foreground,
            IconColor::Error => theme.lowest.negative.default.foreground,
            IconColor::Warning => theme.lowest.warning.default.foreground,
            IconColor::Success => theme.lowest.positive.default.foreground,
            IconColor::Info => theme.lowest.accent.default.foreground,
        }
    }
}

#[derive(Default, PartialEq, Copy, Clone, EnumIter)]
pub enum IconAsset {
    Ai,
    ArrowLeft,
    ArrowRight,
    ArrowUpRight,
    AudioOff,
    AudioOn,
    Bolt,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    Close,
    ExclamationTriangle,
    File,
    FileDoc,
    FileGit,
    FileLock,
    FileRust,
    FileToml,
    FileTree,
    Folder,
    FolderOpen,
    FolderX,
    #[default]
    Hash,
    InlayHint,
    MagicWand,
    MagnifyingGlass,
    MessageBubbles,
    Mic,
    MicMute,
    Plus,
    Screen,
    Split,
    Terminal,
    XCircle,
    Copilot,
    Envelope,
}

impl IconAsset {
    pub fn path(self) -> &'static str {
        match self {
            IconAsset::Ai => "icons/ai.svg",
            IconAsset::ArrowLeft => "icons/arrow_left.svg",
            IconAsset::ArrowRight => "icons/arrow_right.svg",
            IconAsset::ArrowUpRight => "icons/arrow_up_right.svg",
            IconAsset::AudioOff => "icons/speaker-off.svg",
            IconAsset::AudioOn => "icons/speaker-loud.svg",
            IconAsset::Bolt => "icons/bolt.svg",
            IconAsset::ChevronDown => "icons/chevron_down.svg",
            IconAsset::ChevronLeft => "icons/chevron_left.svg",
            IconAsset::ChevronRight => "icons/chevron_right.svg",
            IconAsset::ChevronUp => "icons/chevron_up.svg",
            IconAsset::Close => "icons/x.svg",
            IconAsset::ExclamationTriangle => "icons/warning.svg",
            IconAsset::File => "icons/file_icons/file.svg",
            IconAsset::FileDoc => "icons/file_icons/book.svg",
            IconAsset::FileGit => "icons/file_icons/git.svg",
            IconAsset::FileLock => "icons/file_icons/lock.svg",
            IconAsset::FileRust => "icons/file_icons/rust.svg",
            IconAsset::FileToml => "icons/file_icons/toml.svg",
            IconAsset::FileTree => "icons/project.svg",
            IconAsset::Folder => "icons/file_icons/folder.svg",
            IconAsset::FolderOpen => "icons/file_icons/folder_open.svg",
            IconAsset::FolderX => "icons/stop_sharing.svg",
            IconAsset::Hash => "icons/hash.svg",
            IconAsset::InlayHint => "icons/inlay_hint.svg",
            IconAsset::MagicWand => "icons/magic-wand.svg",
            IconAsset::MagnifyingGlass => "icons/magnifying_glass.svg",
            IconAsset::MessageBubbles => "icons/conversations.svg",
            IconAsset::Mic => "icons/mic.svg",
            IconAsset::MicMute => "icons/mic-mute.svg",
            IconAsset::Plus => "icons/plus.svg",
            IconAsset::Screen => "icons/desktop.svg",
            IconAsset::Split => "icons/split.svg",
            IconAsset::Terminal => "icons/terminal.svg",
            IconAsset::XCircle => "icons/error.svg",
            IconAsset::Copilot => "icons/copilot.svg",
            IconAsset::Envelope => "icons/feedback.svg",
        }
    }
}

#[derive(Element, Clone)]
pub struct Icon {
    asset: IconAsset,
    color: IconColor,
    size: IconSize,
}

impl Icon {
    pub fn new(asset: IconAsset) -> Self {
        Self {
            asset,
            color: IconColor::default(),
            size: IconSize::default(),
        }
    }

    pub fn color(mut self, color: IconColor) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let fill = self.color.color(theme);

        let sized_svg = match self.size {
            IconSize::Small => svg().size_3p5(),
            IconSize::Large => svg().size_4(),
        };

        sized_svg.flex_none().path(self.asset.path()).fill(fill)
    }
}
