use std::sync::Arc;

use call::Room;
use client::ChannelId;
use gpui::{Entity, TestAppContext};

mod channel_buffer_tests;
mod channel_guest_tests;
mod channel_message_tests;
mod channel_tests;
// mod debug_panel_tests;
mod editor_tests;
mod following_tests;
mod git_tests;
mod integration_tests;
mod notification_tests;
mod random_channel_buffer_tests;
mod random_project_collaboration_tests;
mod randomized_test_helpers;
mod remote_editing_collaboration_tests;
mod test_server;

use language::{Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};
pub use randomized_test_helpers::{
    RandomizedTest, TestError, UserTestPlan, run_randomized_test, save_randomized_test_plan,
};
pub use test_server::{TestClient, TestServer};

#[derive(Debug, Eq, PartialEq)]
struct RoomParticipants {
    remote: Vec<String>,
    pending: Vec<String>,
}

fn room_participants(room: &Entity<Room>, cx: &mut TestAppContext) -> RoomParticipants {
    room.read_with(cx, |room, _| {
        let mut remote = room
            .remote_participants()
            .iter()
            .map(|(_, participant)| participant.user.github_login.clone())
            .collect::<Vec<_>>();
        let mut pending = room
            .pending_participants()
            .iter()
            .map(|user| user.github_login.clone())
            .collect::<Vec<_>>();
        remote.sort();
        pending.sort();
        RoomParticipants { remote, pending }
    })
}

fn channel_id(room: &Entity<Room>, cx: &mut TestAppContext) -> Option<ChannelId> {
    cx.read(|cx| room.read(cx).channel_id())
}

fn rust_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ))
}
