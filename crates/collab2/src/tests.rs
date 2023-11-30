use call::Room;
use gpui::{Model, TestAppContext};

mod channel_buffer_tests;
mod channel_message_tests;
mod channel_tests;
mod editor_tests;
mod following_tests;
mod integration_tests;
mod notification_tests;
mod random_channel_buffer_tests;
mod random_project_collaboration_tests;
mod randomized_test_helpers;
mod test_server;

pub use crate as collab2;
pub use randomized_test_helpers::{
    run_randomized_test, save_randomized_test_plan, RandomizedTest, TestError, UserTestPlan,
};
pub use test_server::{TestClient, TestServer};

#[derive(Debug, Eq, PartialEq)]
struct RoomParticipants {
    remote: Vec<String>,
    pending: Vec<String>,
}

fn room_participants(room: &dyn workspace::Room, cx: &mut TestAppContext) -> RoomParticipants {
    let mut remote = cx.update(|cx| {
        room.remote_participants(cx)
            .iter()
            .map(|(_, participant)| participant.0.github_login.clone())
            .collect::<Vec<_>>()
    });
    let mut pending = cx.update(|cx| {
        room.pending_participants(cx)
            .iter()
            .map(|user| user.github_login.clone())
            .collect::<Vec<_>>()
    });
    remote.sort();
    pending.sort();
    RoomParticipants { remote, pending }
}

fn channel_id(room: &Model<Room>, cx: &mut TestAppContext) -> Option<u64> {
    cx.read(|cx| room.read(cx).channel_id())
}
