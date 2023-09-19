use std::borrow::Cow;

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
    #[default]
    ArrowUpRight,
    Bolt,
    Hash,
    File,
    Folder,
    FolderOpen,
}

pub fn icon_asset(asset: IconAsset) -> impl Into<Cow<'static, str>> {
    match asset {
        IconAsset::Ai => "icons/ai.svg",
        IconAsset::ArrowLeft => "icons/arrow_left.svg",
        IconAsset::ArrowRight => "icons/arrow_right.svg",
        IconAsset::ArrowUpRight => "icons/arrow_up_right.svg",
        IconAsset::Bolt => "icons/bolt.svg",
        IconAsset::Hash => "icons/hash.svg",
        IconAsset::File => "icons/file_icons/file.svg",
        IconAsset::Folder => "icons/file_icons/folder.svg",
        IconAsset::FolderOpen => "icons/file_icons/folder_open.svg",
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
            .path(icon_asset(self.asset))
            .size_4()
            .fill(theme.lowest.base.default.foreground)
    }
}
