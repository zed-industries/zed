use gpui2::WindowContext;

use crate::{
    Buffer, BufferRow, BufferRows, GitStatus, HighlightColor, HighlightedLine, HighlightedText,
    Icon, Keybinding, Label, LabelColor, ListEntry, ListEntrySize, ListItem, MicStatus,
    ModifierKeys, PaletteItem, Player, PlayerCallStatus, PlayerWithCallStatus, ScreenShareStatus,
    ToggleState,
};

pub fn static_players() -> Vec<Player> {
    vec![
        Player::new(
            0,
            "https://avatars.githubusercontent.com/u/1714999?v=4".into(),
            "nathansobo".into(),
        ),
        Player::new(
            1,
            "https://avatars.githubusercontent.com/u/326587?v=4".into(),
            "maxbrunsfeld".into(),
        ),
        Player::new(
            2,
            "https://avatars.githubusercontent.com/u/482957?v=4".into(),
            "as-cii".into(),
        ),
        Player::new(
            3,
            "https://avatars.githubusercontent.com/u/1714999?v=4".into(),
            "iamnbutler".into(),
        ),
        Player::new(
            4,
            "https://avatars.githubusercontent.com/u/1486634?v=4".into(),
            "maxdeviant".into(),
        ),
    ]
}

pub fn static_players_with_call_status() -> Vec<PlayerWithCallStatus> {
    let players = static_players();
    let mut player_0_status = PlayerCallStatus::new();
    let player_1_status = PlayerCallStatus::new();
    let player_2_status = PlayerCallStatus::new();
    let mut player_3_status = PlayerCallStatus::new();
    let mut player_4_status = PlayerCallStatus::new();

    player_0_status.screen_share_status = ScreenShareStatus::Shared;
    player_0_status.followers = Some(vec![players[1].clone(), players[3].clone()]);

    player_3_status.voice_activity = 0.5;
    player_4_status.mic_status = MicStatus::Muted;
    player_4_status.in_current_project = false;

    vec![
        PlayerWithCallStatus::new(players[0].clone(), player_0_status),
        PlayerWithCallStatus::new(players[1].clone(), player_1_status),
        PlayerWithCallStatus::new(players[2].clone(), player_2_status),
        PlayerWithCallStatus::new(players[3].clone(), player_3_status),
        PlayerWithCallStatus::new(players[4].clone(), player_4_status),
    ]
}

pub fn static_project_panel_project_items() -> Vec<ListItem> {
    vec![
        ListEntry::new(Label::new("zed"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(0)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new(".cargo"))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".config"))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".git").color(LabelColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".cargo"))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".idea").color(LabelColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new("assets"))
            .left_icon(Icon::Folder.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("cargo-target").color(LabelColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new("crates"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("activity_indicator"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("ai"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("audio"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("auto_update"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("breadcrumbs"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("call"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("sqlez").color(LabelColor::Modified))
            .left_icon(Icon::Folder.into())
            .indent_level(2)
            .set_toggle(ToggleState::NotToggled),
        ListEntry::new(Label::new("gpui2"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(2)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("src"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(3)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("derrive_element.rs"))
            .left_icon(Icon::FileRust.into())
            .indent_level(4),
        ListEntry::new(Label::new("storybook").color(LabelColor::Modified))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("docs").color(LabelColor::Default))
            .left_icon(Icon::Folder.into())
            .indent_level(2)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("src").color(LabelColor::Modified))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(3)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("ui").color(LabelColor::Modified))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(4)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("component").color(LabelColor::Created))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(5)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("facepile.rs").color(LabelColor::Default))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("follow_group.rs").color(LabelColor::Default))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("list_item.rs").color(LabelColor::Created))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("tab.rs").color(LabelColor::Default))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("target").color(LabelColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".dockerignore"))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(1),
        ListEntry::new(Label::new(".DS_Store").color(LabelColor::Hidden))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(1),
        ListEntry::new(Label::new("Cargo.lock"))
            .left_icon(Icon::FileLock.into())
            .indent_level(1),
        ListEntry::new(Label::new("Cargo.toml"))
            .left_icon(Icon::FileToml.into())
            .indent_level(1),
        ListEntry::new(Label::new("Dockerfile"))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(1),
        ListEntry::new(Label::new("Procfile"))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(1),
        ListEntry::new(Label::new("README.md"))
            .left_icon(Icon::FileDoc.into())
            .indent_level(1),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn static_project_panel_single_items() -> Vec<ListItem> {
    vec![
        ListEntry::new(Label::new("todo.md"))
            .left_icon(Icon::FileDoc.into())
            .indent_level(0),
        ListEntry::new(Label::new("README.md"))
            .left_icon(Icon::FileDoc.into())
            .indent_level(0),
        ListEntry::new(Label::new("config.json"))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(0),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn static_collab_panel_current_call() -> Vec<ListItem> {
    vec![
        ListEntry::new(Label::new("as-cii")).left_avatar("http://github.com/as-cii.png?s=50"),
        ListEntry::new(Label::new("nathansobo"))
            .left_avatar("http://github.com/nathansobo.png?s=50"),
        ListEntry::new(Label::new("maxbrunsfeld"))
            .left_avatar("http://github.com/maxbrunsfeld.png?s=50"),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn static_collab_panel_channels() -> Vec<ListItem> {
    vec![
        ListEntry::new(Label::new("zed"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(0),
        ListEntry::new(Label::new("community"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(1),
        ListEntry::new(Label::new("dashboards"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
        ListEntry::new(Label::new("feedback"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
        ListEntry::new(Label::new("teams-in-channels-alpha"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
        ListEntry::new(Label::new("current-projects"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(1),
        ListEntry::new(Label::new("codegen"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
        ListEntry::new(Label::new("gpui2"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
        ListEntry::new(Label::new("livestreaming"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
        ListEntry::new(Label::new("open-source"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
        ListEntry::new(Label::new("replace"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
        ListEntry::new(Label::new("semantic-index"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
        ListEntry::new(Label::new("vim"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
        ListEntry::new(Label::new("web-tech"))
            .left_icon(Icon::Hash.into())
            .size(ListEntrySize::Medium)
            .indent_level(2),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn example_editor_actions() -> Vec<PaletteItem> {
    vec![
        PaletteItem::new("New File").keybinding(Keybinding::new(
            "N".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Open File").keybinding(Keybinding::new(
            "O".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Save File").keybinding(Keybinding::new(
            "S".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Cut").keybinding(Keybinding::new(
            "X".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Copy").keybinding(Keybinding::new(
            "C".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Paste").keybinding(Keybinding::new(
            "V".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Undo").keybinding(Keybinding::new(
            "Z".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Redo").keybinding(Keybinding::new(
            "Z".to_string(),
            ModifierKeys::new().control(true).shift(true),
        )),
        PaletteItem::new("Find").keybinding(Keybinding::new(
            "F".to_string(),
            ModifierKeys::new().control(true),
        )),
        PaletteItem::new("Replace").keybinding(Keybinding::new(
            "R".to_string(),
            ModifierKeys::new().control(true),
        )),
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

pub fn empty_buffer_example<V: 'static>() -> Buffer<V> {
    Buffer::new().set_rows(Some(BufferRows::default()))
}

pub fn hello_world_rust_buffer_example<V: 'static>(cx: &WindowContext) -> Buffer<V> {
    Buffer::new()
        .set_title("hello_world.rs".to_string())
        .set_path("src/hello_world.rs".to_string())
        .set_language("rust".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: true,
            rows: hello_world_rust_buffer_rows(cx),
        }))
}

pub fn hello_world_rust_buffer_with_status_example<V: 'static>(cx: &WindowContext) -> Buffer<V> {
    Buffer::new()
        .set_title("hello_world.rs".to_string())
        .set_path("src/hello_world.rs".to_string())
        .set_language("rust".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: true,
            rows: hello_world_rust_with_status_buffer_rows(cx),
        }))
}

pub fn hello_world_rust_buffer_rows(cx: &WindowContext) -> Vec<BufferRow> {
    let show_line_number = true;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: true,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "fn ".to_string(),
                        color: HighlightColor::Keyword.hsla(cx),
                    },
                    HighlightedText {
                        text: "main".to_string(),
                        color: HighlightColor::Function.hsla(cx),
                    },
                    HighlightedText {
                        text: "() {".to_string(),
                        color: HighlightColor::Default.hsla(cx),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 2,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "    // Statements here are executed when the compiled binary is called."
                        .to_string(),
                    color: HighlightColor::Comment.hsla(cx),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 3,
            code_action: false,
            current: false,
            line: None,
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 4,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "    // Print text to the console.".to_string(),
                    color: HighlightColor::Comment.hsla(cx),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 5,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "}".to_string(),
                    color: HighlightColor::Default.hsla(cx),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
    ]
}

pub fn hello_world_rust_with_status_buffer_rows(cx: &WindowContext) -> Vec<BufferRow> {
    let show_line_number = true;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: true,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "fn ".to_string(),
                        color: HighlightColor::Keyword.hsla(cx),
                    },
                    HighlightedText {
                        text: "main".to_string(),
                        color: HighlightColor::Function.hsla(cx),
                    },
                    HighlightedText {
                        text: "() {".to_string(),
                        color: HighlightColor::Default.hsla(cx),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 2,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "// Statements here are executed when the compiled binary is called."
                        .to_string(),
                    color: HighlightColor::Comment.hsla(cx),
                }],
            }),
            cursors: None,
            status: GitStatus::Modified,
            show_line_number,
        },
        BufferRow {
            line_number: 3,
            code_action: false,
            current: false,
            line: None,
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 4,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "    // Print text to the console.".to_string(),
                    color: HighlightColor::Comment.hsla(cx),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 5,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "}".to_string(),
                    color: HighlightColor::Default.hsla(cx),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 6,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "".to_string(),
                    color: HighlightColor::Default.hsla(cx),
                }],
            }),
            cursors: None,
            status: GitStatus::Created,
            show_line_number,
        },
        BufferRow {
            line_number: 7,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "Marshall and Nate were here".to_string(),
                    color: HighlightColor::Default.hsla(cx),
                }],
            }),
            cursors: None,
            status: GitStatus::Created,
            show_line_number,
        },
    ]
}
