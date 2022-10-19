use futures::StreamExt;
use gpui::{actions, keymap::Binding, Menu, MenuItem};
use live_kit_client::{LocalVideoTrack, RemoteVideoTrackUpdate, Room};
use live_kit_server::token::{self, VideoGrant};
use log::LevelFilter;
use simplelog::SimpleLogger;

actions!(capture, [Quit]);

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_global_action(quit);

        cx.add_bindings([Binding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Zed",
            items: vec![MenuItem::Action {
                name: "Quit",
                action: Box::new(Quit),
            }],
        }]);

        let live_kit_url = std::env::var("LIVE_KIT_URL").unwrap_or("http://localhost:7880".into());
        let live_kit_key = std::env::var("LIVE_KIT_KEY").unwrap_or("devkey".into());
        let live_kit_secret = std::env::var("LIVE_KIT_SECRET").unwrap_or("secret".into());

        cx.spawn(|cx| async move {
            let user_a_token = token::create(
                &live_kit_key,
                &live_kit_secret,
                Some("test-participant-1"),
                VideoGrant::to_join("test-room"),
            )
            .unwrap();
            let room_a = Room::new();
            room_a.connect(&live_kit_url, &user_a_token).await.unwrap();

            let user2_token = token::create(
                &live_kit_key,
                &live_kit_secret,
                Some("test-participant-2"),
                VideoGrant::to_join("test-room"),
            )
            .unwrap();
            let room_b = Room::new();
            room_b.connect(&live_kit_url, &user2_token).await.unwrap();

            let mut track_changes = room_b.remote_video_track_updates();

            let displays = room_a.display_sources().await.unwrap();
            let display = displays.into_iter().next().unwrap();

            let track_a = LocalVideoTrack::screen_share_for_display(&display);
            let track_a_publication = room_a.publish_video_track(&track_a).await.unwrap();

            if let RemoteVideoTrackUpdate::Subscribed(track) = track_changes.next().await.unwrap() {
                let remote_tracks = room_b.remote_video_tracks("test-participant-1");
                assert_eq!(remote_tracks.len(), 1);
                assert_eq!(remote_tracks[0].publisher_id(), "test-participant-1");
                dbg!(track.sid());
                assert_eq!(track.publisher_id(), "test-participant-1");
            } else {
                panic!("unexpected message");
            }

            let remote_track = room_b
                .remote_video_tracks("test-participant-1")
                .pop()
                .unwrap();
            room_a.unpublish_track(track_a_publication);
            if let RemoteVideoTrackUpdate::Unsubscribed {
                publisher_id,
                track_id,
            } = track_changes.next().await.unwrap()
            {
                assert_eq!(publisher_id, "test-participant-1");
                assert_eq!(remote_track.sid(), track_id);
                assert_eq!(room_b.remote_video_tracks("test-participant-1").len(), 0);
            } else {
                panic!("unexpected message");
            }

            cx.platform().quit();
        })
        .detach();
    });
}

fn quit(_: &Quit, cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}
