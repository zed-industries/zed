use gpui::{
    actions, bounds, div, point, px, rgb, size, AsyncAppContext, Bounds, InteractiveElement,
    KeyBinding, Menu, MenuItem, ParentElement, Pixels, Render, SharedString,
    StatefulInteractiveElement as _, Styled, Task, ViewContext, VisualContext, WindowBounds,
    WindowHandle, WindowOptions,
};
use live_kit_client::{
    capture_local_audio_track,
    options::TrackPublishOptions,
    play_remote_audio_track,
    publication::LocalTrackPublication,
    track::{LocalTrack, RemoteTrack},
    AudioStream, Room, RoomEvent, RoomOptions,
};
use live_kit_server::token::{self, VideoGrant};
use log::LevelFilter;
use postage::stream::Stream as _;
use simplelog::SimpleLogger;
use util::ResultExt as _;

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
            let user_1_token = token::create(
                &live_kit_key,
                &live_kit_secret,
                Some("test-participant-1"),
                VideoGrant::to_join("test-room"),
            )
            .unwrap();

            let user2_token = token::create(
                &live_kit_key,
                &live_kit_secret,
                Some("test-participant-2"),
                VideoGrant::to_join("test-room"),
            )
            .unwrap();

            let bounds1 = bounds(point(px(0.0), px(0.0)), size(px(400.0), px(400.0)));
            let bounds2 = bounds(point(px(400.0), px(0.0)), size(px(400.0), px(400.0)));

            let window1 = LivekitWindow::new(
                live_kit_url.as_str(),
                user_1_token.as_str(),
                bounds1,
                cx.clone(),
            )
            .await;

            let window2 = LivekitWindow::new(
                live_kit_url.as_str(),
                user2_token.as_str(),
                bounds2,
                cx.clone(),
            )
            .await;

            // let (local_video_track, stream) =
            //     create_video_track_from_screen_capture_source(&*source)
            //         .await
            //         .unwrap();
            // let local_video_track_publication = room_a
            //     .local_participant()
            //     .publish_track(
            //         LocalTrack::Video(local_video_track),
            //         TrackPublishOptions::default(),
            //     )
            //     .await
            //     .unwrap();

            // if let RoomEvent::TrackSubscribed {
            //     track, participant, ..
            // } = room_b_events.recv().await.unwrap()
            // {
            //     let remote_publications = room_b
            //         .remote_participants()
            //         .get(&ParticipantIdentity("test-participant-1".into()))
            //         .unwrap()
            //         .track_publications();

            //     assert_eq!(remote_publications.len(), 1);
            //     assert_eq!(participant.identity().0, "test-participant-1");
            // } else {
            //     panic!("unexpected message");
            // }

            // room_a
            //     .local_participant()
            //     .unpublish_track(&local_video_track_publication.sid())
            //     .await
            //     .unwrap();
            // if let RoomEvent::TrackUnpublished {
            //     publication,
            //     participant,
            // } = room_b_events.recv().await.unwrap()
            // {
            //     assert_eq!(participant.identity().0, "test-participant-1");
            //     assert_eq!(publication.sid(), local_video_track_publication.sid());

            //     let remote_publications = room_b
            //         .remote_participants()
            //         .get(&ParticipantIdentity("test-participant-1".into()))
            //         .unwrap()
            //         .track_publications();
            //     assert_eq!(remote_publications.len(), 0);
            // } else {
            //     panic!("unexpected message");
            // }

            // cx.update(|cx| cx.shutdown()).ok();
        })
        .detach();
    });
}

fn quit(_: &Quit, cx: &mut gpui::AppContext) {
    cx.quit();
}

struct LivekitWindow {
    room: Room,
    microphone_track: Option<LocalTrackPublication>,
    microphone_stream: Option<AudioStream>,
    speaker_stream: Option<AudioStream>,
    _events_task: Task<()>,
}

impl LivekitWindow {
    async fn new(
        url: &str,
        token: &str,
        bounds: Bounds<Pixels>,
        cx: AsyncAppContext,
    ) -> WindowHandle<Self> {
        let (room, mut events) = Room::connect(url, token, RoomOptions::default())
            .await
            .unwrap();

        cx.update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |cx| {
                    cx.new_view(|cx| {
                        let _events_task = cx.spawn(|this, mut cx| async move {
                            while let Some(event) = events.recv().await {
                                this.update(&mut cx, |this: &mut LivekitWindow, cx| {
                                    this.handle_room_event(event, cx)
                                })
                                .ok();
                            }
                        });

                        Self {
                            room,
                            microphone_track: None,
                            microphone_stream: None,
                            speaker_stream: None,
                            _events_task,
                        }
                    })
                },
            )
            .unwrap()
        })
        .unwrap()
    }

    fn handle_room_event(&mut self, event: RoomEvent, cx: &mut ViewContext<Self>) {
        match event {
            RoomEvent::ParticipantConnected(participant) => {
                println!("Participant connected: {:?}", participant.identity());
            }
            RoomEvent::ParticipantDisconnected(participant) => {
                println!("Participant disconnected: {:?}", participant.identity());
            }
            RoomEvent::TrackPublished { publication, .. } => {
                println!("Track published: {:?}", publication.sid());
            }
            RoomEvent::TrackUnpublished { publication, .. } => {
                println!("Track unpublished: {:?}", publication.sid());
            }
            RoomEvent::TrackSubscribed { track, .. } => {
                println!("Track subscribed: {:?}", track.sid());

                if let RemoteTrack::Audio(track) = track {
                    let stream = play_remote_audio_track(&track, cx.background_executor());
                    self.speaker_stream = Some(stream);
                }
            }
            RoomEvent::TrackUnsubscribed { publication, .. } => {
                println!("Track unsubscribed: {:?}", publication.sid());
            }
            RoomEvent::ActiveSpeakersChanged { speakers } => {
                println!("Active speakers changed: {:?}", speakers);
            }
            RoomEvent::Disconnected { .. } => {
                println!("Room disconnected");
            }
            _ => {}
        }

        cx.notify();
    }

    fn is_muted(&self) -> bool {
        self.microphone_track
            .as_ref()
            .map(|t| t.is_muted())
            .unwrap_or(true)
    }

    fn toggle_mute(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(track) = &self.microphone_track {
            if track.is_muted() {
                track.unmute();
            } else {
                track.mute();
            }
            cx.notify();
        } else {
            let participant = self.room.local_participant();
            cx.spawn(|this, mut cx| async move {
                let track = capture_local_audio_track(cx.background_executor());
                let (track, stream) = track.await?;
                let publication = participant
                    .publish_track(LocalTrack::Audio(track), TrackPublishOptions::default())
                    .await
                    .unwrap();
                this.update(&mut cx, |this, cx| {
                    this.microphone_track = Some(publication);
                    this.microphone_stream = Some(stream);
                    cx.notify();
                })
            })
            .detach();
        }
    }
}

impl Render for LivekitWindow {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div()
            .p(px(10.0))
            .bg(rgb(0xffa8d4))
            .size_full()
            .flex()
            .flex_row()
            .justify_between()
            .child(
                div()
                    .bg(gpui::rgb(0xffd4a8))
                    .w(px(120.0))
                    .flex()
                    .flex_col()
                    .justify_around()
                    .child(
                        div()
                            .id("toggle-mute")
                            .w(px(100.0))
                            .h(px(30.0))
                            .bg(rgb(0x6666ff))
                            .justify_around()
                            .flex()
                            .flex_row()
                            .child(if self.microphone_track.is_none() {
                                "Publish mic"
                            } else if self.is_muted() {
                                "Unmute"
                            } else {
                                "Mute"
                            })
                            .on_click(cx.listener(|this, _, cx| this.toggle_mute(cx))),
                    ),
            )
            .child(
                div()
                    .bg(gpui::rgb(0xaaaaff))
                    .w(px(300.0))
                    .flex()
                    .flex_col()
                    .child(format!(
                        "mic: {:?}",
                        self.microphone_track.as_ref().map(|track| track.sid())
                    ))
                    .children(self.room.remote_participants().into_values().flat_map(
                        |participant| {
                            let identity = participant.identity();
                            participant.track_publications().into_values().map({
                                let identity = identity.clone();
                                move |publication| {
                                    SharedString::from(format!("{:?} {:?}", &identity, publication))
                                }
                            })
                        },
                    )),
            )
    }
}
