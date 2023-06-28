use std::time::Duration;

use futures::StreamExt;
use gpui::{actions, keymap_matcher::Binding, Menu, MenuItem};
use live_kit_client::{
    LocalAudioTrack, LocalVideoTrack, RemoteAudioTrackUpdate, RemoteVideoTrackUpdate, Room,
};
use live_kit_server::token::{self, VideoGrant};
use log::LevelFilter;
use simplelog::SimpleLogger;

actions!(capture, [Quit]);

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        #[cfg(any(test, feature = "test-support"))]
        println!("USING TEST LIVEKIT");

        #[cfg(not(any(test, feature = "test-support")))]
        println!("USING REAL LIVEKIT");

        cx.platform().activate(true);
        cx.add_global_action(quit);

        cx.add_bindings([Binding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Zed",
            items: vec![MenuItem::Action {
                name: "Quit",
                action: Box::new(Quit),
                os_action: None,
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

            let mut audio_track_updates = room_b.remote_audio_track_updates();
            let audio_track = LocalAudioTrack::create();
            let audio_track_publication = room_a.publish_audio_track(&audio_track).await.unwrap();

            if let RemoteAudioTrackUpdate::Subscribed(track) =
                audio_track_updates.next().await.unwrap()
            {
                let remote_tracks = room_b.remote_audio_tracks("test-participant-1");
                assert_eq!(remote_tracks.len(), 1);
                assert_eq!(remote_tracks[0].publisher_id(), "test-participant-1");
                assert_eq!(track.publisher_id(), "test-participant-1");
            } else {
                panic!("unexpected message");
            }

            audio_track_publication.set_mute(true).await.unwrap();

            println!("waiting for mute changed!");
            if let RemoteAudioTrackUpdate::MuteChanged { track_id, muted } =
                audio_track_updates.next().await.unwrap()
            {
                let remote_tracks = room_b.remote_audio_tracks("test-participant-1");
                assert_eq!(remote_tracks[0].sid(), track_id);
                assert_eq!(muted, true);
            } else {
                panic!("unexpected message");
            }

            audio_track_publication.set_mute(false).await.unwrap();

            if let RemoteAudioTrackUpdate::MuteChanged { track_id, muted } =
                audio_track_updates.next().await.unwrap()
            {
                let remote_tracks = room_b.remote_audio_tracks("test-participant-1");
                assert_eq!(remote_tracks[0].sid(), track_id);
                assert_eq!(muted, false);
            } else {
                panic!("unexpected message");
            }

            println!("Pausing for 5 seconds to test audio, make some noise!");
            let timer = cx.background().timer(Duration::from_secs(5));
            timer.await;
            let remote_audio_track = room_b
                .remote_audio_tracks("test-participant-1")
                .pop()
                .unwrap();
            room_a.unpublish_track(audio_track_publication);

            // Clear out any active speakers changed messages
            let mut next = audio_track_updates.next().await.unwrap();
            while let RemoteAudioTrackUpdate::ActiveSpeakersChanged {
                speakers
            } = next
            {
                println!("Speakers changed: {:?}", speakers);
                next = audio_track_updates.next().await.unwrap();
            }

            if let RemoteAudioTrackUpdate::Unsubscribed {
                publisher_id,
                track_id,
            } = next
            {
                assert_eq!(publisher_id, "test-participant-1");
                assert_eq!(remote_audio_track.sid(), track_id);
                assert_eq!(room_b.remote_audio_tracks("test-participant-1").len(), 0);
            } else {
                panic!("unexpected message");
            }

            let mut video_track_updates = room_b.remote_video_track_updates();
            let displays = room_a.display_sources().await.unwrap();
            let display = displays.into_iter().next().unwrap();

            let local_video_track = LocalVideoTrack::screen_share_for_display(&display);
            let local_video_track_publication = room_a
                .publish_video_track(&local_video_track)
                .await
                .unwrap();

            if let RemoteVideoTrackUpdate::Subscribed(track) =
                video_track_updates.next().await.unwrap()
            {
                let remote_video_tracks = room_b.remote_video_tracks("test-participant-1");
                assert_eq!(remote_video_tracks.len(), 1);
                assert_eq!(remote_video_tracks[0].publisher_id(), "test-participant-1");
                assert_eq!(track.publisher_id(), "test-participant-1");
            } else {
                panic!("unexpected message");
            }

            let remote_video_track = room_b
                .remote_video_tracks("test-participant-1")
                .pop()
                .unwrap();
            room_a.unpublish_track(local_video_track_publication);
            if let RemoteVideoTrackUpdate::Unsubscribed {
                publisher_id,
                track_id,
            } = video_track_updates.next().await.unwrap()
            {
                assert_eq!(publisher_id, "test-participant-1");
                assert_eq!(remote_video_track.sid(), track_id);
                assert_eq!(room_b.remote_video_tracks("test-participant-1").len(), 0);
            } else {
                panic!("unexpected message");
            }

            cx.platform().quit();
        })
        .detach();
    });
}

fn quit(_: &Quit, cx: &mut gpui::AppContext) {
    cx.platform().quit();
}
