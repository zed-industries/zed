use std::collections::HashSet;

use crate::{
    list_item, IconAsset, Keybinding, Label, LabelColor, ListItem, ListItemSize, ModifierKey,
    PaletteItem, ToggleState,
};

pub fn static_project_panel_project_items() -> Vec<ListItem> {
    vec![
        list_item(Label::new("zed"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(0)
            .set_toggle(ToggleState::Toggled),
        list_item(Label::new(".cargo"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(Label::new(".config"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(Label::new(".git").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(Label::new(".cargo"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(Label::new(".idea").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(Label::new("assets"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        list_item(Label::new("cargo-target").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(Label::new("crates"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        list_item(Label::new("activity_indicator"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(Label::new("ai"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(Label::new("audio"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(Label::new("auto_update"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(Label::new("breadcrumbs"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(Label::new("call"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(Label::new("sqlez").color(LabelColor::Modified))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2)
            .set_toggle(ToggleState::NotToggled),
        list_item(Label::new("gpui2"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(2)
            .set_toggle(ToggleState::Toggled),
        list_item(Label::new("src"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(3)
            .set_toggle(ToggleState::Toggled),
        list_item(Label::new("derrive_element.rs"))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(4),
        list_item(Label::new("storybook").color(LabelColor::Modified))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        list_item(Label::new("docs").color(LabelColor::Default))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2)
            .set_toggle(ToggleState::Toggled),
        list_item(Label::new("src").color(LabelColor::Modified))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(3)
            .set_toggle(ToggleState::Toggled),
        list_item(Label::new("ui").color(LabelColor::Modified))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(4)
            .set_toggle(ToggleState::Toggled),
        list_item(Label::new("component").color(LabelColor::Created))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(5)
            .set_toggle(ToggleState::Toggled),
        list_item(Label::new("facepile.rs").color(LabelColor::Default))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        list_item(Label::new("follow_group.rs").color(LabelColor::Default))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        list_item(Label::new("list_item.rs").color(LabelColor::Created))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        list_item(Label::new("tab.rs").color(LabelColor::Default))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        list_item(Label::new("target").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(Label::new(".dockerignore"))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        list_item(Label::new(".DS_Store").color(LabelColor::Hidden))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        list_item(Label::new("Cargo.lock"))
            .left_icon(IconAsset::FileLock.into())
            .indent_level(1),
        list_item(Label::new("Cargo.toml"))
            .left_icon(IconAsset::FileToml.into())
            .indent_level(1),
        list_item(Label::new("Dockerfile"))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        list_item(Label::new("Procfile"))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        list_item(Label::new("README.md"))
            .left_icon(IconAsset::FileDoc.into())
            .indent_level(1),
    ]
}

pub fn static_project_panel_single_items() -> Vec<ListItem> {
    vec![
        list_item(Label::new("todo.md"))
            .left_icon(IconAsset::FileDoc.into())
            .indent_level(0),
        list_item(Label::new("README.md"))
            .left_icon(IconAsset::FileDoc.into())
            .indent_level(0),
        list_item(Label::new("config.json"))
            .left_icon(IconAsset::File.into())
            .indent_level(0),
    ]
}

pub fn static_collab_panel_current_call() -> Vec<ListItem> {
    vec![
        list_item(Label::new("as-cii")).left_avatar("http://github.com/as-cii.png?s=50"),
        list_item(Label::new("nathansobo")).left_avatar("http://github.com/nathansobo.png?s=50"),
        list_item(Label::new("maxbrunsfeld"))
            .left_avatar("http://github.com/maxbrunsfeld.png?s=50"),
    ]
}

pub fn static_collab_panel_channels() -> Vec<ListItem> {
    vec![
        list_item(Label::new("zed"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(0),
        list_item(Label::new("community"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(1),
        list_item(Label::new("dashboards"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        list_item(Label::new("feedback"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        list_item(Label::new("teams-in-channels-alpha"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        list_item(Label::new("current-projects"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(1),
        list_item(Label::new("codegen"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        list_item(Label::new("gpui2"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        list_item(Label::new("livestreaming"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        list_item(Label::new("open-source"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        list_item(Label::new("replace"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        list_item(Label::new("semantic-index"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        list_item(Label::new("vim"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
        list_item(Label::new("web-tech"))
            .left_icon(IconAsset::Hash.into())
            .size(ListItemSize::Medium)
            .indent_level(2),
    ]
}

pub fn example_editor_actions() -> Vec<PaletteItem> {
    vec![
        PaletteItem::new("New File").keybinding(Some(Keybinding::new(
            "N".to_string(),
            HashSet::from_iter([ModifierKey::Control]),
        ))),
        PaletteItem::new("Open File").keybinding(Some(Keybinding::new(
            "O".to_string(),
            HashSet::from_iter([ModifierKey::Control]),
        ))),
        PaletteItem::new("Save File").keybinding(Some(Keybinding::new(
            "S".to_string(),
            HashSet::from_iter([ModifierKey::Control]),
        ))),
        PaletteItem::new("Cut").keybinding(Some(Keybinding::new(
            "X".to_string(),
            HashSet::from_iter([ModifierKey::Control]),
        ))),
        PaletteItem::new("Copy").keybinding(Some(Keybinding::new(
            "C".to_string(),
            HashSet::from_iter([ModifierKey::Control]),
        ))),
        PaletteItem::new("Paste").keybinding(Some(Keybinding::new(
            "V".to_string(),
            HashSet::from_iter([ModifierKey::Control]),
        ))),
        PaletteItem::new("Undo").keybinding(Some(Keybinding::new(
            "Z".to_string(),
            HashSet::from_iter([ModifierKey::Control]),
        ))),
        PaletteItem::new("Redo").keybinding(Some(Keybinding::new(
            "Z".to_string(),
            HashSet::from_iter([ModifierKey::Control, ModifierKey::Shift]),
        ))),
        PaletteItem::new("Find").keybinding(Some(Keybinding::new(
            "F".to_string(),
            HashSet::from_iter([ModifierKey::Control]),
        ))),
        PaletteItem::new("Replace").keybinding(Some(Keybinding::new(
            "R".to_string(),
            HashSet::from_iter([ModifierKey::Control]),
        ))),
        PaletteItem::new("Jump to Line"),
        PaletteItem::new("Select All"),
        PaletteItem::new("Deselect All"),
        PaletteItem::new("Switch Document"),
        PaletteItem::new("Insert Line Below"),
        PaletteItem::new("Insert Line Above"),
        PaletteItem::new("Move Line Up"),
        PaletteItem::new("Move Line Down"),
        PaletteItem::new("Toggle Comment"),
        PaletteItem::new("Delete Line"),
    ]
}
