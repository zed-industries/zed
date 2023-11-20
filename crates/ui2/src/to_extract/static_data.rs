use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use chrono::DateTime;
use gpui::{AppContext, ViewContext};
use rand::Rng;
use theme2::ActiveTheme;

use crate::{binding, HighlightedText};
use crate::{
    Buffer, BufferRow, BufferRows, Button, EditorPane, FileSystemStatus, GitStatus,
    HighlightedLine, Icon, KeyBinding, Label, ListEntry, ListEntrySize, Livestream, MicStatus,
    Notification, PaletteItem, Player, PlayerCallStatus, PlayerWithCallStatus, PublicPlayer,
    ScreenShareStatus, Symbol, Tab, TextColor, Toggle, VideoStatus,
};
use crate::{ListItem, NotificationAction};

pub fn static_tabs_example() -> Vec<Tab> {
    vec![
        Tab::new("wip.rs")
            .title("wip.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .fs_status(FileSystemStatus::Deleted),
        Tab::new("Cargo.toml")
            .title("Cargo.toml".to_string())
            .icon(Icon::FileToml)
            .current(false)
            .git_status(GitStatus::Modified),
        Tab::new("Channels Panel")
            .title("Channels Panel".to_string())
            .icon(Icon::Hash)
            .current(false),
        Tab::new("channels_panel.rs")
            .title("channels_panel.rs".to_string())
            .icon(Icon::FileRust)
            .current(true)
            .git_status(GitStatus::Modified),
        Tab::new("workspace.rs")
            .title("workspace.rs".to_string())
            .current(false)
            .icon(Icon::FileRust)
            .git_status(GitStatus::Modified),
        Tab::new("icon_button.rs")
            .title("icon_button.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new("storybook.rs")
            .title("storybook.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .git_status(GitStatus::Created),
        Tab::new("theme.rs")
            .title("theme.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new("theme_registry.rs")
            .title("theme_registry.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new("styleable_helpers.rs")
            .title("styleable_helpers.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
    ]
}

pub fn static_tabs_1() -> Vec<Tab> {
    vec![
        Tab::new("project_panel.rs")
            .title("project_panel.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .fs_status(FileSystemStatus::Deleted),
        Tab::new("tab_bar.rs")
            .title("tab_bar.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .git_status(GitStatus::Modified),
        Tab::new("workspace.rs")
            .title("workspace.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new("tab.rs")
            .title("tab.rs".to_string())
            .icon(Icon::FileRust)
            .current(true)
            .git_status(GitStatus::Modified),
    ]
}

pub fn static_tabs_2() -> Vec<Tab> {
    vec![
        Tab::new("tab_bar.rs")
            .title("tab_bar.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .fs_status(FileSystemStatus::Deleted),
        Tab::new("static_data.rs")
            .title("static_data.rs".to_string())
            .icon(Icon::FileRust)
            .current(true)
            .git_status(GitStatus::Modified),
    ]
}

pub fn static_tabs_3() -> Vec<Tab> {
    vec![Tab::new("static_tabs_3")
        .git_status(GitStatus::Created)
        .current(true)]
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

pub fn static_new_notification_items_2() -> Vec<Notification> {
    vec![
        Notification::new_icon_message(
            "notif-1",
            "You were mentioned in a note.",
            DateTime::parse_from_rfc3339("2023-11-02T11:59:57Z")
                .unwrap()
                .naive_local(),
            Icon::AtSign,
            Arc::new(|_, _| {}),
        ),
        Notification::new_actor_with_actions(
            "notif-2",
            "as-cii sent you a contact request.",
            DateTime::parse_from_rfc3339("2023-11-02T12:09:07Z")
                .unwrap()
                .naive_local(),
            PublicPlayer::new("as-cii", "http://github.com/as-cii.png?s=50"),
            [
                NotificationAction::new(
                    Button::new("Decline"),
                    "Decline Request",
                    (Some(Icon::XCircle), "Declined"),
                ),
                NotificationAction::new(
                    Button::new("Accept").variant(crate::ButtonVariant::Filled),
                    "Accept Request",
                    (Some(Icon::Check), "Accepted"),
                ),
            ],
        ),
        Notification::new_icon_message(
            "notif-3",
            "You were mentioned #design.",
            DateTime::parse_from_rfc3339("2023-11-02T12:09:07Z")
                .unwrap()
                .naive_local(),
            Icon::MessageBubbles,
            Arc::new(|_, _| {}),
        ),
        Notification::new_actor_with_actions(
            "notif-4",
            "as-cii sent you a contact request.",
            DateTime::parse_from_rfc3339("2023-11-01T12:09:07Z")
                .unwrap()
                .naive_local(),
            PublicPlayer::new("as-cii", "http://github.com/as-cii.png?s=50"),
            [
                NotificationAction::new(
                    Button::new("Decline"),
                    "Decline Request",
                    (Some(Icon::XCircle), "Declined"),
                ),
                NotificationAction::new(
                    Button::new("Accept").variant(crate::ButtonVariant::Filled),
                    "Accept Request",
                    (Some(Icon::Check), "Accepted"),
                ),
            ],
        ),
        Notification::new_icon_message(
            "notif-5",
            "You were mentioned in a note.",
            DateTime::parse_from_rfc3339("2023-10-28T12:09:07Z")
                .unwrap()
                .naive_local(),
            Icon::AtSign,
            Arc::new(|_, _| {}),
        ),
        Notification::new_actor_with_actions(
            "notif-6",
            "as-cii sent you a contact request.",
            DateTime::parse_from_rfc3339("2022-10-25T12:09:07Z")
                .unwrap()
                .naive_local(),
            PublicPlayer::new("as-cii", "http://github.com/as-cii.png?s=50"),
            [
                NotificationAction::new(
                    Button::new("Decline"),
                    "Decline Request",
                    (Some(Icon::XCircle), "Declined"),
                ),
                NotificationAction::new(
                    Button::new("Accept").variant(crate::ButtonVariant::Filled),
                    "Accept Request",
                    (Some(Icon::Check), "Accepted"),
                ),
            ],
        ),
        Notification::new_icon_message(
            "notif-7",
            "You were mentioned in a note.",
            DateTime::parse_from_rfc3339("2022-10-14T12:09:07Z")
                .unwrap()
                .naive_local(),
            Icon::AtSign,
            Arc::new(|_, _| {}),
        ),
        Notification::new_actor_with_actions(
            "notif-8",
            "as-cii sent you a contact request.",
            DateTime::parse_from_rfc3339("2021-10-12T12:09:07Z")
                .unwrap()
                .naive_local(),
            PublicPlayer::new("as-cii", "http://github.com/as-cii.png?s=50"),
            [
                NotificationAction::new(
                    Button::new("Decline"),
                    "Decline Request",
                    (Some(Icon::XCircle), "Declined"),
                ),
                NotificationAction::new(
                    Button::new("Accept").variant(crate::ButtonVariant::Filled),
                    "Accept Request",
                    (Some(Icon::Check), "Accepted"),
                ),
            ],
        ),
        Notification::new_icon_message(
            "notif-9",
            "You were mentioned in a note.",
            DateTime::parse_from_rfc3339("2021-02-02T12:09:07Z")
                .unwrap()
                .naive_local(),
            Icon::AtSign,
            Arc::new(|_, _| {}),
        ),
        Notification::new_actor_with_actions(
            "notif-10",
            "as-cii sent you a contact request.",
            DateTime::parse_from_rfc3339("1969-07-20T00:00:00Z")
                .unwrap()
                .naive_local(),
            PublicPlayer::new("as-cii", "http://github.com/as-cii.png?s=50"),
            [
                NotificationAction::new(
                    Button::new("Decline"),
                    "Decline Request",
                    (Some(Icon::XCircle), "Declined"),
                ),
                NotificationAction::new(
                    Button::new("Accept").variant(crate::ButtonVariant::Filled),
                    "Accept Request",
                    (Some(Icon::Check), "Accepted"),
                ),
            ],
        ),
    ]
}

pub fn static_project_panel_project_items() -> Vec<ListItem> {
    vec![
        ListEntry::new(Label::new("zed"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(0)
            .toggle(Toggle::Toggled(true)),
        ListEntry::new(Label::new(".cargo"))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".config"))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".git").color(TextColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".cargo"))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".idea").color(TextColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new("assets"))
            .left_icon(Icon::Folder.into())
            .indent_level(1)
            .toggle(Toggle::Toggled(true)),
        ListEntry::new(Label::new("cargo-target").color(TextColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new("crates"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(1)
            .toggle(Toggle::Toggled(true)),
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
        ListEntry::new(Label::new("sqlez").color(TextColor::Modified))
            .left_icon(Icon::Folder.into())
            .indent_level(2)
            .toggle(Toggle::Toggled(false)),
        ListEntry::new(Label::new("gpui2"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(2)
            .toggle(Toggle::Toggled(true)),
        ListEntry::new(Label::new("src"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(3)
            .toggle(Toggle::Toggled(true)),
        ListEntry::new(Label::new("derive_element.rs"))
            .left_icon(Icon::FileRust.into())
            .indent_level(4),
        ListEntry::new(Label::new("storybook").color(TextColor::Modified))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(1)
            .toggle(Toggle::Toggled(true)),
        ListEntry::new(Label::new("docs").color(TextColor::Default))
            .left_icon(Icon::Folder.into())
            .indent_level(2)
            .toggle(Toggle::Toggled(true)),
        ListEntry::new(Label::new("src").color(TextColor::Modified))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(3)
            .toggle(Toggle::Toggled(true)),
        ListEntry::new(Label::new("ui").color(TextColor::Modified))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(4)
            .toggle(Toggle::Toggled(true)),
        ListEntry::new(Label::new("component").color(TextColor::Created))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(5)
            .toggle(Toggle::Toggled(true)),
        ListEntry::new(Label::new("facepile.rs").color(TextColor::Default))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("follow_group.rs").color(TextColor::Default))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("list_item.rs").color(TextColor::Created))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("tab.rs").color(TextColor::Default))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("target").color(TextColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".dockerignore"))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(1),
        ListEntry::new(Label::new(".DS_Store").color(TextColor::Hidden))
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

pub fn static_collab_panel_channels<V>() -> Vec<ListItem> {
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
        PaletteItem::new("New File").key_binding(KeyBinding::new(binding("cmd-n"))),
        PaletteItem::new("Open File").key_binding(KeyBinding::new(binding("cmd-o"))),
        PaletteItem::new("Save File").key_binding(KeyBinding::new(binding("cmd-s"))),
        PaletteItem::new("Cut").key_binding(KeyBinding::new(binding("cmd-x"))),
        PaletteItem::new("Copy").key_binding(KeyBinding::new(binding("cmd-c"))),
        PaletteItem::new("Paste").key_binding(KeyBinding::new(binding("cmd-v"))),
        PaletteItem::new("Undo").key_binding(KeyBinding::new(binding("cmd-z"))),
        PaletteItem::new("Redo").key_binding(KeyBinding::new(binding("cmd-shift-z"))),
        PaletteItem::new("Find").key_binding(KeyBinding::new(binding("cmd-f"))),
        PaletteItem::new("Replace").key_binding(KeyBinding::new(binding("cmd-r"))),
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

pub fn empty_editor_example(cx: &mut ViewContext<EditorPane>) -> EditorPane {
    EditorPane::new(
        cx,
        static_tabs_example(),
        PathBuf::from_str("crates/ui/src/static_data.rs").unwrap(),
        vec![],
        empty_buffer_example(),
    )
}

pub fn empty_buffer_example() -> Buffer {
    Buffer::new("empty-buffer").set_rows(Some(BufferRows::default()))
}

pub fn hello_world_rust_editor_example(cx: &mut ViewContext<EditorPane>) -> EditorPane {
    EditorPane::new(
        cx,
        static_tabs_example(),
        PathBuf::from_str("crates/ui/src/static_data.rs").unwrap(),
        vec![Symbol(vec![
            HighlightedText {
                text: "fn ".into(),
                color: cx.theme().syntax_color("keyword"),
            },
            HighlightedText {
                text: "main".into(),
                color: cx.theme().syntax_color("function"),
            },
        ])],
        hello_world_rust_buffer_example(cx),
    )
}

pub fn hello_world_rust_buffer_example(cx: &AppContext) -> Buffer {
    Buffer::new("hello-world-rust-buffer")
        .set_title("hello_world.rs".to_string())
        .set_path("src/hello_world.rs".to_string())
        .set_language("rust".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: true,
            rows: hello_world_rust_buffer_rows(cx),
        }))
}

pub fn hello_world_rust_buffer_rows(cx: &AppContext) -> Vec<BufferRow> {
    let show_line_number = true;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: true,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "fn ".into(),
                        color: cx.theme().syntax_color("keyword"),
                    },
                    HighlightedText {
                        text: "main".into(),
                        color: cx.theme().syntax_color("function"),
                    },
                    HighlightedText {
                        text: "() {".into(),
                        color: cx.theme().colors().text,
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
                        .into(),
                    color: cx.theme().syntax_color("comment"),
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
                    text: "    // Print text to the console.".into(),
                    color: cx.theme().syntax_color("comment"),
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
                        text: "    println!(".into(),
                        color: cx.theme().colors().text,
                    },
                    HighlightedText {
                        text: "\"Hello, world!\"".into(),
                        color: cx.theme().syntax_color("string"),
                    },
                    HighlightedText {
                        text: ");".into(),
                        color: cx.theme().colors().text,
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
                    text: "}".into(),
                    color: cx.theme().colors().text,
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
    ]
}

pub fn hello_world_rust_editor_with_status_example(cx: &mut ViewContext<EditorPane>) -> EditorPane {
    EditorPane::new(
        cx,
        static_tabs_example(),
        PathBuf::from_str("crates/ui/src/static_data.rs").unwrap(),
        vec![Symbol(vec![
            HighlightedText {
                text: "fn ".into(),
                color: cx.theme().syntax_color("keyword"),
            },
            HighlightedText {
                text: "main".into(),
                color: cx.theme().syntax_color("function"),
            },
        ])],
        hello_world_rust_buffer_with_status_example(cx),
    )
}

pub fn hello_world_rust_buffer_with_status_example(cx: &AppContext) -> Buffer {
    Buffer::new("hello-world-rust-buffer-with-status")
        .set_title("hello_world.rs".to_string())
        .set_path("src/hello_world.rs".to_string())
        .set_language("rust".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: true,
            rows: hello_world_rust_with_status_buffer_rows(cx),
        }))
}

pub fn hello_world_rust_with_status_buffer_rows(cx: &AppContext) -> Vec<BufferRow> {
    let show_line_number = true;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: true,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "fn ".into(),
                        color: cx.theme().syntax_color("keyword"),
                    },
                    HighlightedText {
                        text: "main".into(),
                        color: cx.theme().syntax_color("function"),
                    },
                    HighlightedText {
                        text: "() {".into(),
                        color: cx.theme().colors().text,
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
                        .into(),
                    color: cx.theme().syntax_color("comment"),
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
                    text: "    // Print text to the console.".into(),
                    color: cx.theme().syntax_color("comment"),
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
                        text: "    println!(".into(),
                        color: cx.theme().colors().text,
                    },
                    HighlightedText {
                        text: "\"Hello, world!\"".into(),
                        color: cx.theme().syntax_color("string"),
                    },
                    HighlightedText {
                        text: ");".into(),
                        color: cx.theme().colors().text,
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
                    text: "}".into(),
                    color: cx.theme().colors().text,
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
                    text: "".into(),
                    color: cx.theme().colors().text,
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
                    text: "// Marshall and Nate were here".into(),
                    color: cx.theme().syntax_color("comment"),
                }],
            }),
            cursors: None,
            status: GitStatus::Created,
            show_line_number,
        },
    ]
}

pub fn terminal_buffer(cx: &AppContext) -> Buffer {
    Buffer::new("terminal")
        .set_title(Some("zed — fish".into()))
        .set_rows(Some(BufferRows {
            show_line_numbers: false,
            rows: terminal_buffer_rows(cx),
        }))
}

pub fn terminal_buffer_rows(cx: &AppContext) -> Vec<BufferRow> {
    let show_line_number = false;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "maxdeviant ".into(),
                        color: cx.theme().syntax_color("keyword"),
                    },
                    HighlightedText {
                        text: "in ".into(),
                        color: cx.theme().colors().text,
                    },
                    HighlightedText {
                        text: "profaned-capital ".into(),
                        color: cx.theme().syntax_color("function"),
                    },
                    HighlightedText {
                        text: "in ".into(),
                        color: cx.theme().colors().text,
                    },
                    HighlightedText {
                        text: "~/p/zed ".into(),
                        color: cx.theme().syntax_color("function"),
                    },
                    HighlightedText {
                        text: "on ".into(),
                        color: cx.theme().colors().text,
                    },
                    HighlightedText {
                        text: " gpui2-ui ".into(),
                        color: cx.theme().syntax_color("keyword"),
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
                    text: "λ ".into(),
                    color: cx.theme().syntax_color("string"),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
    ]
}
