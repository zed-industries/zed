use call::Room;
use gpui::{ModelHandle, TestAppContext};

mod channel_buffer_tests;
mod channel_message_tests;
mod channel_tests;
mod following_tests;
mod integration_tests;
mod random_channel_buffer_tests;
mod random_project_collaboration_tests;
mod randomized_test_helpers;
mod test_server;

pub use randomized_test_helpers::{
    run_randomized_test, save_randomized_test_plan, RandomizedTest, TestError, UserTestPlan,
};
pub use test_server::{TestClient, TestServer};

#[derive(Debug, Eq, PartialEq)]
struct RoomParticipants {
    remote: Vec<String>,
    pending: Vec<String>,
}

fn room_participants(room: &ModelHandle<Room>, cx: &mut TestAppContext) -> RoomParticipants {
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
