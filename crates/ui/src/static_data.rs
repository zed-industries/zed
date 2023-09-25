use crate::{
    list_item, palette_item, IconAsset, Label, LabelColor, ListItem, PaletteItem, ToggleState,
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

pub fn example_editor_actions() -> Vec<PaletteItem> {
    vec![
        palette_item("New File", Some("Ctrl+N")),
        palette_item("Open File", Some("Ctrl+O")),
        palette_item("Save File", Some("Ctrl+S")),
        palette_item("Cut", Some("Ctrl+X")),
        palette_item("Copy", Some("Ctrl+C")),
        palette_item("Paste", Some("Ctrl+V")),
        palette_item("Undo", Some("Ctrl+Z")),
        palette_item("Redo", Some("Ctrl+Shift+Z")),
        palette_item("Find", Some("Ctrl+F")),
        palette_item("Replace", Some("Ctrl+R")),
        palette_item("Jump to Line", None),
        palette_item("Select All", None),
        palette_item("Deselect All", None),
        palette_item("Switch Document", None),
        palette_item("Insert Line Below", None),
        palette_item("Insert Line Above", None),
        palette_item("Move Line Up", None),
        palette_item("Move Line Down", None),
        palette_item("Toggle Comment", None),
        palette_item("Delete Line", None),
    ]
}
