use std::path::PathBuf;
use std::str::FromStr;

use rand::Rng;

use crate::{
    Buffer, BufferRow, BufferRows, Editor, FileSystemStatus, GitStatus, HighlightColor,
    HighlightedLine, HighlightedText, Icon, Keybinding, Label, LabelColor, ListEntry,
    ListEntrySize, ListItem, Livestream, MicStatus, ModifierKeys, PaletteItem, Player,
    PlayerCallStatus, PlayerWithCallStatus, ScreenShareStatus, Symbol, Tab, Theme, ToggleState,
    VideoStatus,
};

pub fn static_tabs_example() -> Vec<Tab> {
    vec![
        Tab::new()
            .title("wip.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .fs_status(FileSystemStatus::Deleted),
        Tab::new()
            .title("Cargo.toml".to_string())
            .icon(Icon::FileToml)
            .current(false)
            .git_status(GitStatus::Modified),
        Tab::new()
            .title("Channels Panel".to_string())
            .icon(Icon::Hash)
            .current(false),
        Tab::new()
            .title("channels_panel.rs".to_string())
            .icon(Icon::FileRust)
            .current(true)
            .git_status(GitStatus::Modified),
        Tab::new()
            .title("workspace.rs".to_string())
            .current(false)
            .icon(Icon::FileRust)
            .git_status(GitStatus::Modified),
        Tab::new()
            .title("icon_button.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new()
            .title("storybook.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .git_status(GitStatus::Created),
        Tab::new()
            .title("theme.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new()
            .title("theme_registry.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new()
            .title("styleable_helpers.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
    ]
}

pub fn static_tabs_1() -> Vec<Tab> {
    vec![
        Tab::new()
            .title("project_panel.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .fs_status(FileSystemStatus::Deleted),
        Tab::new()
            .title("tab_bar.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .git_status(GitStatus::Modified),
        Tab::new()
            .title("workspace.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new()
            .title("tab.rs".to_string())
            .icon(Icon::FileRust)
            .current(true)
            .git_status(GitStatus::Modified),
    ]
}

pub fn static_tabs_2() -> Vec<Tab> {
    vec![
        Tab::new()
            .title("tab_bar.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .fs_status(FileSystemStatus::Deleted),
        Tab::new()
            .title("static_data.rs".to_string())
            .icon(Icon::FileRust)
            .current(true)
            .git_status(GitStatus::Modified),
    ]
}

pub fn static_tabs_3() -> Vec<Tab> {
    vec![Tab::new().git_status(GitStatus::Created).current(true)]
}

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

#[derive(Debug)]
pub struct PlayerData {
    pub url: String,
    pub name: String,
}
pub fn static_player_data() -> Vec<PlayerData> {
    vec![
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/1714999?v=4".into(),
            name: "iamnbutler".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/326587?v=4".into(),
            name: "maxbrunsfeld".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/482957?v=4".into(),
            name: "as-cii".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/1789?v=4".into(),
            name: "nathansobo".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/1486634?v=4".into(),
            name: "ForLoveOfCats".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/2690773?v=4".into(),
            name: "SomeoneToIgnore".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/19867440?v=4".into(),
            name: "JosephTLyons".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/24362066?v=4".into(),
            name: "osiewicz".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/22121886?v=4".into(),
            name: "KCaverly".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/1486634?v=4".into(),
            name: "maxdeviant".into(),
        },
    ]
}
pub fn create_static_players(player_data: Vec<PlayerData>) -> Vec<Player> {
    let mut players = Vec::new();
    for data in player_data {
        players.push(Player::new(players.len(), data.url, data.name));
    }
    players
}
pub fn static_player_1(data: &Vec<PlayerData>) -> Player {
    Player::new(1, data[0].url.clone(), data[0].name.clone())
}
pub fn static_player_2(data: &Vec<PlayerData>) -> Player {
    Player::new(2, data[1].url.clone(), data[1].name.clone())
}
pub fn static_player_3(data: &Vec<PlayerData>) -> Player {
    Player::new(3, data[2].url.clone(), data[2].name.clone())
}
pub fn static_player_4(data: &Vec<PlayerData>) -> Player {
    Player::new(4, data[3].url.clone(), data[3].name.clone())
}
pub fn static_player_5(data: &Vec<PlayerData>) -> Player {
    Player::new(5, data[4].url.clone(), data[4].name.clone())
}
pub fn static_player_6(data: &Vec<PlayerData>) -> Player {
    Player::new(6, data[5].url.clone(), data[5].name.clone())
}
pub fn static_player_7(data: &Vec<PlayerData>) -> Player {
    Player::new(7, data[6].url.clone(), data[6].name.clone())
}
pub fn static_player_8(data: &Vec<PlayerData>) -> Player {
    Player::new(8, data[7].url.clone(), data[7].name.clone())
}
pub fn static_player_9(data: &Vec<PlayerData>) -> Player {
    Player::new(9, data[8].url.clone(), data[8].name.clone())
}
pub fn static_player_10(data: &Vec<PlayerData>) -> Player {
    Player::new(10, data[9].url.clone(), data[9].name.clone())
}
pub fn static_livestream() -> Livestream {
    Livestream {
        players: random_players_with_call_status(7),
        channel: Some("gpui2-ui".to_string()),
    }
}
pub fn populate_player_call_status(
    player: Player,
    followers: Option<Vec<Player>>,
) -> PlayerCallStatus {
    let mut rng = rand::thread_rng();
    let in_current_project: bool = rng.gen();
    let disconnected: bool = rng.gen();
    let voice_activity: f32 = rng.gen();
    let mic_status = if rng.gen_bool(0.5) {
        MicStatus::Muted
    } else {
        MicStatus::Unmuted
    };
    let video_status = if rng.gen_bool(0.5) {
        VideoStatus::On
    } else {
        VideoStatus::Off
    };
    let screen_share_status = if rng.gen_bool(0.5) {
        ScreenShareStatus::Shared
    } else {
        ScreenShareStatus::NotShared
    };
    PlayerCallStatus {
        mic_status,
        voice_activity,
        video_status,
        screen_share_status,
        in_current_project,
        disconnected,
        following: None,
        followers,
    }
}
pub fn random_players_with_call_status(number_of_players: usize) -> Vec<PlayerWithCallStatus> {
    let players = create_static_players(static_player_data());
    let mut player_status = vec![];
    for i in 0..number_of_players {
        let followers = if i == 0 {
            Some(vec![
                players[1].clone(),
                players[3].clone(),
                players[5].clone(),
                players[6].clone(),
            ])
        } else if i == 1 {
            Some(vec![players[2].clone(), players[6].clone()])
        } else {
            None
        };
        let call_status = populate_player_call_status(players[i].clone(), followers);
        player_status.push(PlayerWithCallStatus::new(players[i].clone(), call_status));
    }
    player_status
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
        ListEntry::new(Label::new("derive_element.rs"))
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

pub fn empty_editor_example() -> Editor {
    Editor {
        tabs: static_tabs_example(),
        path: PathBuf::from_str("crates/ui/src/static_data.rs").unwrap(),
        symbols: vec![],
        buffer: empty_buffer_example(),
    }
}

pub fn empty_buffer_example() -> Buffer {
    Buffer::new().set_rows(Some(BufferRows::default()))
}

pub fn hello_world_rust_editor_example(theme: &Theme) -> Editor {
    Editor {
        tabs: static_tabs_example(),
        path: PathBuf::from_str("crates/ui/src/static_data.rs").unwrap(),
        symbols: vec![Symbol(vec![
            HighlightedText {
                text: "fn ".to_string(),
                color: HighlightColor::Keyword.hsla(&theme),
            },
            HighlightedText {
                text: "main".to_string(),
                color: HighlightColor::Function.hsla(&theme),
            },
        ])],
        buffer: hello_world_rust_buffer_example(theme),
    }
}

pub fn hello_world_rust_buffer_example(theme: &Theme) -> Buffer {
    Buffer::new()
        .set_title("hello_world.rs".to_string())
        .set_path("src/hello_world.rs".to_string())
        .set_language("rust".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: true,
            rows: hello_world_rust_buffer_rows(theme),
        }))
}

pub fn hello_world_rust_buffer_rows(theme: &Theme) -> Vec<BufferRow> {
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
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                    HighlightedText {
                        text: "main".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "() {".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
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
                    color: HighlightColor::Comment.hsla(&theme),
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
                    color: HighlightColor::Comment.hsla(&theme),
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
                highlighted_texts: vec![
                    HighlightedText {
                        text: "    println!(".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "\"Hello, world!\"".to_string(),
                        color: HighlightColor::String.hsla(&theme),
                    },
                    HighlightedText {
                        text: ");".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                ],
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
                    text: "}".to_string(),
                    color: HighlightColor::Default.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
    ]
}

pub fn hello_world_rust_editor_with_status_example(theme: &Theme) -> Editor {
    Editor {
        tabs: static_tabs_example(),
        path: PathBuf::from_str("crates/ui/src/static_data.rs").unwrap(),
        symbols: vec![Symbol(vec![
            HighlightedText {
                text: "fn ".to_string(),
                color: HighlightColor::Keyword.hsla(&theme),
            },
            HighlightedText {
                text: "main".to_string(),
                color: HighlightColor::Function.hsla(&theme),
            },
        ])],
        buffer: hello_world_rust_buffer_with_status_example(theme),
    }
}

pub fn hello_world_rust_buffer_with_status_example(theme: &Theme) -> Buffer {
    Buffer::new()
        .set_title("hello_world.rs".to_string())
        .set_path("src/hello_world.rs".to_string())
        .set_language("rust".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: true,
            rows: hello_world_rust_with_status_buffer_rows(theme),
        }))
}

pub fn hello_world_rust_with_status_buffer_rows(theme: &Theme) -> Vec<BufferRow> {
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
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                    HighlightedText {
                        text: "main".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "() {".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
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
                    color: HighlightColor::Comment.hsla(&theme),
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
                    color: HighlightColor::Comment.hsla(&theme),
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
                highlighted_texts: vec![
                    HighlightedText {
                        text: "    println!(".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "\"Hello, world!\"".to_string(),
                        color: HighlightColor::String.hsla(&theme),
                    },
                    HighlightedText {
                        text: ");".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                ],
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
                    text: "}".to_string(),
                    color: HighlightColor::Default.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 7,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "".to_string(),
                    color: HighlightColor::Default.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::Created,
            show_line_number,
        },
        BufferRow {
            line_number: 8,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "// Marshall and Nate were here".to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::Created,
            show_line_number,
        },
    ]
}

pub fn terminal_buffer(theme: &Theme) -> Buffer {
    Buffer::new()
        .set_title("zed — fish".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: false,
            rows: terminal_buffer_rows(theme),
        }))
}

pub fn terminal_buffer_rows(theme: &Theme) -> Vec<BufferRow> {
    let show_line_number = false;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "maxdeviant ".to_string(),
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                    HighlightedText {
                        text: "in ".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "profaned-capital ".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "in ".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "~/p/zed ".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "on ".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: " gpui2-ui ".to_string(),
                        color: HighlightColor::Keyword.hsla(&theme),
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
                    text: "λ ".to_string(),
                    color: HighlightColor::String.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
    ]
}
