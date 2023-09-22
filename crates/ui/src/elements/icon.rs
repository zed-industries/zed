use crate::theme::theme;
use gpui2::elements::svg;
use gpui2::style::StyleHelpers;
use gpui2::IntoElement;
use gpui2::{Element, ViewContext};

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconAsset {
    Ai,
    ArrowLeft,
    ArrowRight,
    ArrowUpRight,
    Bolt,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    #[default]
    File,
    FileDoc,
    FileGit,
    FileLock,
    FileRust,
    FileToml,
    Folder,
    FolderOpen,
    Hash,
    Close,
}

impl IconAsset {
    pub fn path(self) -> &'static str {
        match self {
            IconAsset::Ai => "icons/ai.svg",
            IconAsset::ArrowLeft => "icons/arrow_left.svg",
            IconAsset::ArrowRight => "icons/arrow_right.svg",
            IconAsset::ArrowUpRight => "icons/arrow_up_right.svg",
            IconAsset::Bolt => "icons/bolt.svg",
            IconAsset::ChevronDown => "icons/chevron_down.svg",
            IconAsset::ChevronLeft => "icons/chevron_left.svg",
            IconAsset::ChevronRight => "icons/chevron_right.svg",
            IconAsset::ChevronUp => "icons/chevron_up.svg",
            IconAsset::File => "icons/file_icons/file.svg",
            IconAsset::FileDoc => "icons/file_icons/book.svg",
            IconAsset::FileGit => "icons/file_icons/git.svg",
            IconAsset::FileLock => "icons/file_icons/lock.svg",
            IconAsset::FileRust => "icons/file_icons/rust.svg",
            IconAsset::FileToml => "icons/file_icons/toml.svg",
            IconAsset::Folder => "icons/file_icons/folder.svg",
            IconAsset::FolderOpen => "icons/file_icons/folder_open.svg",
            IconAsset::Hash => "icons/hash.svg",
            IconAsset::Close => "icons/x.svg",
        }
    }
}

#[derive(Element, Clone)]
pub struct Icon {
    asset: IconAsset,
}

pub fn icon(asset: IconAsset) -> Icon {
    Icon { asset }
}

impl Icon {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        svg()
            .flex_none()
            .path(self.asset.path())
            .size_4()
            .fill(theme.lowest.variant.default.foreground)
    }
}
