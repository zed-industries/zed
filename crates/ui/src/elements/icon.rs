use gpui2::elements::svg;
use gpui2::style::StyleHelpers;
use gpui2::IntoElement;
use gpui2::{Element, ViewContext};

use crate::theme;

// Icon::Hash
// icon(IconAsset::Hash).color(IconColor::Warning)
// Icon::new(IconAsset::Hash).color(IconColor::Warning)

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconAsset {
    Ai,
    ArrowLeft,
    ArrowRight,
    #[default]
    ArrowUpRight,
    Bolt,
    Hash,
    File,
    Folder,
    FolderOpen,
    ChevronDown,
    ChevronUp,
    ChevronLeft,
    ChevronRight,
}

impl IconAsset {
    pub fn path(self) -> &'static str {
        match self {
            IconAsset::Ai => "icons/ai.svg",
            IconAsset::ArrowLeft => "icons/arrow_left.svg",
            IconAsset::ArrowRight => "icons/arrow_right.svg",
            IconAsset::ArrowUpRight => "icons/arrow_up_right.svg",
            IconAsset::Bolt => "icons/bolt.svg",
            IconAsset::Hash => "icons/hash.svg",
            IconAsset::ChevronDown => "icons/chevron_down.svg",
            IconAsset::ChevronUp => "icons/chevron_up.svg",
            IconAsset::ChevronLeft => "icons/chevron_left.svg",
            IconAsset::ChevronRight => "icons/chevron_right.svg",
            IconAsset::File => "icons/file_icons/file.svg",
            IconAsset::Folder => "icons/file_icons/folder.svg",
            IconAsset::FolderOpen => "icons/file_icons/folder_open.svg",
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

// impl Icon {
//     pub fn new(asset: IconAsset) -> Icon {
//         Icon { asset }
//     }
// }

impl Icon {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        svg()
            .path(self.asset.path())
            .size_4()
            .fill(theme.lowest.base.default.foreground)
    }
}
