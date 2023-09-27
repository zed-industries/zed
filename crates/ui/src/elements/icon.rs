use std::sync::Arc;

use gpui2::elements::svg;
use gpui2::Hsla;
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
pub enum Icon {
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

impl Icon {
    pub fn path(self) -> &'static str {
        match self {
            Icon::Ai => "icons/ai.svg",
            Icon::ArrowLeft => "icons/arrow_left.svg",
            Icon::ArrowRight => "icons/arrow_right.svg",
            Icon::ArrowUpRight => "icons/arrow_up_right.svg",
            Icon::AudioOff => "icons/speaker-off.svg",
            Icon::AudioOn => "icons/speaker-loud.svg",
            Icon::Bolt => "icons/bolt.svg",
            Icon::ChevronDown => "icons/chevron_down.svg",
            Icon::ChevronLeft => "icons/chevron_left.svg",
            Icon::ChevronRight => "icons/chevron_right.svg",
            Icon::ChevronUp => "icons/chevron_up.svg",
            Icon::Close => "icons/x.svg",
            Icon::ExclamationTriangle => "icons/warning.svg",
            Icon::File => "icons/file_icons/file.svg",
            Icon::FileDoc => "icons/file_icons/book.svg",
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
            Icon::MessageBubbles => "icons/conversations.svg",
            Icon::Mic => "icons/mic.svg",
            Icon::MicMute => "icons/mic-mute.svg",
            Icon::Plus => "icons/plus.svg",
            Icon::Screen => "icons/desktop.svg",
            Icon::Split => "icons/split.svg",
            Icon::Terminal => "icons/terminal.svg",
            Icon::XCircle => "icons/error.svg",
            Icon::Copilot => "icons/copilot.svg",
            Icon::Envelope => "icons/feedback.svg",
        }
    }
}

mod macros {
    macro_rules! icon_constructor {
        ($name:ident, $asset:ident) => {
            pub fn $name() -> Self {
                Self::new(Icon::$asset)
            }
        };
    }

    macro_rules! icon_constructors {
        () => {
            icon_constructor!(ai, Ai);
            icon_constructor!(arrow_left, ArrowLeft);
            icon_constructor!(arrow_right, ArrowRight);
            icon_constructor!(arrow_up_right, ArrowUpRight);
            icon_constructor!(audio_off, AudioOff);
            icon_constructor!(audio_on, AudioOn);
            icon_constructor!(bolt, Bolt);
            icon_constructor!(chevron_down, ChevronDown);
            icon_constructor!(chevron_left, ChevronLeft);
            icon_constructor!(chevron_right, ChevronRight);
            icon_constructor!(chevron_up, ChevronUp);
            icon_constructor!(close, Close);
            icon_constructor!(exclamation_triangle, ExclamationTriangle);
            icon_constructor!(file, File);
            icon_constructor!(file_doc, FileDoc);
            icon_constructor!(file_git, FileGit);
            icon_constructor!(file_lock, FileLock);
            icon_constructor!(file_rust, FileRust);
            icon_constructor!(file_toml, FileToml);
            icon_constructor!(file_tree, FileTree);
            icon_constructor!(folder, Folder);
            icon_constructor!(folder_open, FolderOpen);
            icon_constructor!(folder_x, FolderX);
            icon_constructor!(hash, Hash);
            icon_constructor!(inlay_hint, InlayHint);
            icon_constructor!(magic_wand, MagicWand);
            icon_constructor!(magnifying_glass, MagnifyingGlass);
            icon_constructor!(message_bubbles, MessageBubbles);
            icon_constructor!(mic, Mic);
            icon_constructor!(mic_mute, MicMute);
            icon_constructor!(plus, Plus);
            icon_constructor!(screen, Screen);
            icon_constructor!(split, Split);
            icon_constructor!(terminal, Terminal);
            icon_constructor!(x_circle, XCircle);
            icon_constructor!(copilot, Copilot);
            icon_constructor!(envelope, Envelope);
        };
    }

    pub(crate) use icon_constructor;
    pub(crate) use icon_constructors;
}

pub(crate) use macros::{icon_constructor, icon_constructors};

#[derive(Element, Clone)]
pub struct IconElement {
    asset: Icon,
    color: IconColor,
    size: IconSize,
}

impl IconElement {
    pub fn new(asset: Icon) -> Self {
        Self {
            asset,
            color: IconColor::default(),
            size: IconSize::default(),
        }
    }

    icon_constructors!();

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
