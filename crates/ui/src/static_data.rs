use crate::{
    label, list_item, palette_item, IconAsset, LabelColor, ListItem, PaletteItem, ToggleState,
};

pub fn static_project_panel_project_items() -> Vec<ListItem> {
    vec![
        list_item(label("zed"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(0)
            .set_toggle(ToggleState::Toggled),
        list_item(label(".cargo"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(label(".config"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(label(".git").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(label(".cargo"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(label(".idea").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(label("assets"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        list_item(label("cargo-target").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(label("crates"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        list_item(label("activity_indicator"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(label("ai"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(label("audio"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(label("auto_update"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(label("breadcrumbs"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(label("call"))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2),
        list_item(label("sqlez").color(LabelColor::Modified))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2)
            .set_toggle(ToggleState::NotToggled),
        list_item(label("gpui2"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(2)
            .set_toggle(ToggleState::Toggled),
        list_item(label("src"))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(3)
            .set_toggle(ToggleState::Toggled),
        list_item(label("derrive_element.rs"))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(4),
        list_item(label("storybook").color(LabelColor::Modified))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        list_item(label("docs").color(LabelColor::Default))
            .left_icon(IconAsset::Folder.into())
            .indent_level(2)
            .set_toggle(ToggleState::Toggled),
        list_item(label("src").color(LabelColor::Modified))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(3)
            .set_toggle(ToggleState::Toggled),
        list_item(label("ui").color(LabelColor::Modified))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(4)
            .set_toggle(ToggleState::Toggled),
        list_item(label("component").color(LabelColor::Created))
            .left_icon(IconAsset::FolderOpen.into())
            .indent_level(5)
            .set_toggle(ToggleState::Toggled),
        list_item(label("facepile.rs").color(LabelColor::Default))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        list_item(label("follow_group.rs").color(LabelColor::Default))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        list_item(label("list_item.rs").color(LabelColor::Created))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        list_item(label("tab.rs").color(LabelColor::Default))
            .left_icon(IconAsset::FileRust.into())
            .indent_level(6),
        list_item(label("target").color(LabelColor::Hidden))
            .left_icon(IconAsset::Folder.into())
            .indent_level(1),
        list_item(label(".dockerignore"))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        list_item(label(".DS_Store").color(LabelColor::Hidden))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        list_item(label("Cargo.lock"))
            .left_icon(IconAsset::FileLock.into())
            .indent_level(1),
        list_item(label("Cargo.toml"))
            .left_icon(IconAsset::FileToml.into())
            .indent_level(1),
        list_item(label("Dockerfile"))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        list_item(label("Procfile"))
            .left_icon(IconAsset::File.into())
            .indent_level(1),
        list_item(label("README.md"))
            .left_icon(IconAsset::FileDoc.into())
            .indent_level(1),
    ]
}

pub fn static_project_panel_single_items() -> Vec<ListItem> {
    vec![
        list_item(label("todo.md"))
            .left_icon(IconAsset::FileDoc.into())
            .indent_level(0),
        list_item(label("README.md"))
            .left_icon(IconAsset::FileDoc.into())
            .indent_level(0),
        list_item(label("config.json"))
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
