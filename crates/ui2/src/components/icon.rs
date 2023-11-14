use gpui::{rems, svg, Hsla};
use strum::EnumIter;

use crate::{prelude::*, LabelColor};

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    Small,
    #[default]
    Medium,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconColor {
    #[default]
    Default,
    Accent,
    Created,
    Deleted,
    Disabled,
    Error,
    Hidden,
    Info,
    Modified,
    Muted,
    Placeholder,
    Player(u32),
    Selected,
    Success,
    Warning,
}

impl IconColor {
    pub fn color(self, cx: &WindowContext) -> Hsla {
        match self {
            IconColor::Default => cx.theme().colors().icon,
            IconColor::Muted => cx.theme().colors().icon_muted,
            IconColor::Disabled => cx.theme().colors().icon_disabled,
            IconColor::Placeholder => cx.theme().colors().icon_placeholder,
            IconColor::Accent => cx.theme().colors().icon_accent,
            IconColor::Error => cx.theme().status().error,
            IconColor::Warning => cx.theme().status().warning,
            IconColor::Success => cx.theme().status().success,
            IconColor::Info => cx.theme().status().info,
            IconColor::Selected => cx.theme().colors().icon_accent,
            IconColor::Player(i) => cx.theme().styles.player.0[i.clone() as usize].cursor,
            IconColor::Created => cx.theme().status().created,
            IconColor::Modified => cx.theme().status().modified,
            IconColor::Deleted => cx.theme().status().deleted,
            IconColor::Hidden => cx.theme().status().hidden,
        }
    }
}

impl From<LabelColor> for IconColor {
    fn from(label: LabelColor) -> Self {
        match label {
            LabelColor::Default => IconColor::Default,
            LabelColor::Muted => IconColor::Muted,
            LabelColor::Disabled => IconColor::Disabled,
            LabelColor::Placeholder => IconColor::Placeholder,
            LabelColor::Accent => IconColor::Accent,
            LabelColor::Error => IconColor::Error,
            LabelColor::Warning => IconColor::Warning,
            LabelColor::Success => IconColor::Success,
            LabelColor::Info => IconColor::Info,
            LabelColor::Selected => IconColor::Selected,
            LabelColor::Player(i) => IconColor::Player(i),
            LabelColor::Created => IconColor::Created,
            LabelColor::Modified => IconColor::Modified,
            LabelColor::Deleted => IconColor::Deleted,
            LabelColor::Hidden => IconColor::Hidden,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum Icon {
    Ai,
    ArrowLeft,
    ArrowRight,
    ArrowUpRight,
    AudioOff,
    AudioOn,
    Bolt,
    Check,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    Close,
    Dash,
    Exit,
    ExclamationTriangle,
    File,
    FileGeneric,
    FileDoc,
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
    Maximize,
    Menu,
    MessageBubbles,
    Mic,
    MicMute,
    Plus,
    Quote,
    Replace,
    ReplaceAll,
    Screen,
    SelectAll,
    Split,
    SplitMessage,
    Terminal,
    XCircle,
    Copilot,
    Envelope,
    Bell,
    BellOff,
    BellRing,
    MailOpen,
    AtSign,
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
            Icon::Check => "icons/check.svg",
            Icon::ChevronDown => "icons/chevron_down.svg",
            Icon::ChevronLeft => "icons/chevron_left.svg",
            Icon::ChevronRight => "icons/chevron_right.svg",
            Icon::ChevronUp => "icons/chevron_up.svg",
            Icon::Close => "icons/x.svg",
            Icon::Dash => "icons/dash.svg",
            Icon::Exit => "icons/exit.svg",
            Icon::ExclamationTriangle => "icons/warning.svg",
            Icon::File => "icons/file.svg",
            Icon::FileGeneric => "icons/file_icons/file.svg",
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
            Icon::Maximize => "icons/maximize.svg",
            Icon::Menu => "icons/menu.svg",
            Icon::MessageBubbles => "icons/conversations.svg",
            Icon::Mic => "icons/mic.svg",
            Icon::MicMute => "icons/mic-mute.svg",
            Icon::Plus => "icons/plus.svg",
            Icon::Quote => "icons/quote.svg",
            Icon::Replace => "icons/replace.svg",
            Icon::ReplaceAll => "icons/replace_all.svg",
            Icon::Screen => "icons/desktop.svg",
            Icon::SelectAll => "icons/select-all.svg",
            Icon::Split => "icons/split.svg",
            Icon::SplitMessage => "icons/split_message.svg",
            Icon::Terminal => "icons/terminal.svg",
            Icon::XCircle => "icons/error.svg",
            Icon::Copilot => "icons/copilot.svg",
            Icon::Envelope => "icons/feedback.svg",
            Icon::Bell => "icons/bell.svg",
            Icon::BellOff => "icons/bell-off.svg",
            Icon::BellRing => "icons/bell-ring.svg",
            Icon::MailOpen => "icons/mail-open.svg",
            Icon::AtSign => "icons/at-sign.svg",
        }
    }
}

#[derive(Component)]
pub struct IconElement {
    icon: Icon,
    color: IconColor,
    size: IconSize,
}

impl IconElement {
    pub fn new(icon: Icon) -> Self {
        Self {
            icon,
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

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let svg_size = match self.size {
            IconSize::Small => rems(0.75),
            IconSize::Medium => rems(0.9375),
        };

        svg()
            .size(svg_size)
            .flex_none()
            .path(self.icon.path())
            .text_color(self.color.color(cx))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui::{Div, Render};
    use strum::IntoEnumIterator;

    use crate::Story;

    use super::*;

    pub struct IconStory;

    impl Render for IconStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            let icons = Icon::iter();

            Story::container(cx)
                .child(Story::title_for::<_, IconElement>(cx))
                .child(Story::label(cx, "All Icons"))
                .child(div().flex().gap_3().children(icons.map(IconElement::new)))
        }
    }
}
