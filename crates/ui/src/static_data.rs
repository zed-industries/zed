use crate::{list_item, IconAsset, Label, LabelColor, ListItem, PaletteItem, ToggleState};

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

pub fn example_editor_actions() -> Vec<PaletteItem> {
    vec![
        PaletteItem::new("New File").keybinding(Some("Ctrl+N")),
        PaletteItem::new("Open File").keybinding(Some("Ctrl+O")),
        PaletteItem::new("Save File").keybinding(Some("Ctrl+S")),
        PaletteItem::new("Cut").keybinding(Some("Ctrl+X")),
        PaletteItem::new("Copy").keybinding(Some("Ctrl+C")),
        PaletteItem::new("Paste").keybinding(Some("Ctrl+V")),
        PaletteItem::new("Undo").keybinding(Some("Ctrl+Z")),
        PaletteItem::new("Redo").keybinding(Some("Ctrl+Shift+Z")),
        PaletteItem::new("Find").keybinding(Some("Ctrl+F")),
        PaletteItem::new("Replace").keybinding(Some("Ctrl+R")),
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
