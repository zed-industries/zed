use gpui::{actions, KeyBinding, Menu, MenuItem};
use live_kit_client::{
    create_audio_track_from_microphone, create_video_track_from_screen_capture_source,
    id::ParticipantIdentity, options::TrackPublishOptions, track::LocalTrack, Room, RoomEvent,
    RoomOptions,
};
use live_kit_server::token::{self, VideoGrant};
use log::LevelFilter;
use postage::stream::Stream as _;
use simplelog::SimpleLogger;
use std::time::Duration;

actions!(live_kit_client, [Quit]);

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new().run(|cx| {
        live_kit_client::init(cx.background_executor().dispatcher.clone());

        #[cfg(any(test, feature = "test-support"))]
        println!("USING TEST LIVEKIT");

        #[cfg(not(any(test, feature = "test-support")))]
        println!("USING REAL LIVEKIT");

        cx.activate(true);

        cx.on_action(quit);
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

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
            let (room_a, _room_a_events) =
                Room::connect(&live_kit_url, &user_a_token, RoomOptions::default())
                    .await
                    .unwrap();

            let user2_token = token::create(
                &live_kit_key,
                &live_kit_secret,
                Some("test-participant-2"),
                VideoGrant::to_join("test-room"),
            )
            .unwrap();
            let (room_b, mut room_b_events) =
                Room::connect(&live_kit_url, &user2_token, RoomOptions::default())
                    .await
                    .unwrap();

            let audio_track = create_audio_track_from_microphone().await.unwrap();
            let audio_track_publication = room_a
                .local_participant()
                .publish_track(
                    LocalTrack::Audio(audio_track.clone()),
                    TrackPublishOptions::default(),
                )
                .await
                .unwrap();

            if let RoomEvent::TrackSubscribed { participant, .. } =
                room_b_events.recv().await.unwrap()
            {
                let remote_publications = room_b
                    .remote_participants()
                    .get(&ParticipantIdentity("test-participant-1".into()))
                    .unwrap()
                    .track_publications();
                assert_eq!(remote_publications.len(), 1);
                assert_eq!(participant.identity().0, "test-participant-1");
            } else {
                panic!("unexpected message");
            }

            audio_track_publication.mute();

            println!("waiting for mute changed!");
            if let RoomEvent::TrackMuted { publication, .. } = room_b_events.recv().await.unwrap() {
                assert_eq!(publication.sid(), audio_track_publication.sid());
            } else {
                panic!("unexpected message");
            }

            audio_track_publication.unmute();

            if let RoomEvent::TrackUnmuted { publication, .. } = room_b_events.recv().await.unwrap()
            {
                assert_eq!(publication.sid(), audio_track_publication.sid());
            } else {
                panic!("unexpected message");
            }

            println!("Pausing for 5 seconds to test audio, make some noise!");
            let timer = cx.background_executor().timer(Duration::from_secs(5));
            timer.await;
            room_b
                .local_participant()
                .unpublish_track(&audio_track_publication.sid())
                .await
                .unwrap();

            // Clear out any active speakers changed messages
            let mut next = room_b_events.recv().await.unwrap();
            while let RoomEvent::ActiveSpeakersChanged { speakers } = next {
                println!("Speakers changed: {:?}", speakers);
                next = room_b_events.recv().await.unwrap();
            }

            if let RoomEvent::TrackUnsubscribed {
                publication,
                participant,
                ..
            } = next
            {
                assert_eq!(participant.identity().0, "test-participant-1");
                assert_eq!(audio_track_publication.sid(), publication.sid());
                let remote_publications = room_b
                    .remote_participants()
                    .get(&ParticipantIdentity("test-participant-1".into()))
                    .unwrap()
                    .track_publications();
                assert_eq!(remote_publications.len(), 0);
            } else {
                panic!("unexpected message");
            }

            let sources = cx
                .update(|cx| cx.screen_capture_sources())
                .unwrap()
                .await
                .unwrap()
                .unwrap();
            let source = sources.into_iter().next().unwrap();

            let (local_video_track, stream) =
                create_video_track_from_screen_capture_source(&*source)
                    .await
                    .unwrap();
            let local_video_track_publication = room_a
                .local_participant()
                .publish_track(
                    LocalTrack::Video(local_video_track),
                    TrackPublishOptions::default(),
                )
                .await
                .unwrap();

            if let RoomEvent::TrackSubscribed {
                track, participant, ..
            } = room_b_events.recv().await.unwrap()
            {
                let remote_publications = room_b
                    .remote_participants()
                    .get(&ParticipantIdentity("test-participant-1".into()))
                    .unwrap()
                    .track_publications();

                assert_eq!(remote_publications.len(), 1);
                assert_eq!(participant.identity().0, "test-participant-1");
            } else {
                panic!("unexpected message");
            }

            room_a
                .local_participant()
                .unpublish_track(&local_video_track_publication.sid())
                .await
                .unwrap();
            if let RoomEvent::TrackUnpublished {
                publication,
                participant,
            } = room_b_events.recv().await.unwrap()
            {
                assert_eq!(participant.identity().0, "test-participant-1");
                assert_eq!(publication.sid(), local_video_track_publication.sid());

                let remote_publications = room_b
                    .remote_participants()
                    .get(&ParticipantIdentity("test-participant-1".into()))
                    .unwrap()
                    .track_publications();
                assert_eq!(remote_publications.len(), 0);
            } else {
                panic!("unexpected message");
            }

            cx.update(|cx| cx.shutdown()).ok();
        })
        .detach();
    });
}

fn quit(_: &Quit, cx: &mut gpui::AppContext) {
    cx.quit();
}
