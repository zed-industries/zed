use gpui::{
    actions, bounds, div, point, prelude::IntoElement, px, rgb, size, AsyncAppContext, Bounds,
    InteractiveElement, KeyBinding, Menu, MenuItem, ParentElement, Pixels, Render,
    ScreenCaptureStream, SharedString, StatefulInteractiveElement as _, Styled, Task, View,
    ViewContext, VisualContext, WindowBounds, WindowHandle, WindowOptions,
};
use live_kit_client::{
    capture_local_audio_track, capture_local_video_track,
    id::TrackSid,
    options::TrackPublishOptions,
    participant::{Participant, RemoteParticipant},
    play_remote_audio_track,
    publication::LocalTrackPublication,
    track::{LocalTrack, RemoteTrack},
    AudioStream, RemoteVideoTrackView, Room, RoomEvent, RoomOptions,
};
use live_kit_server::token::{self, VideoGrant};
use livekit::id::ParticipantIdentity;
use log::LevelFilter;
use postage::stream::Stream as _;
use simplelog::SimpleLogger;

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

            let bounds1 = bounds(point(px(0.0), px(0.0)), size(px(800.0), px(800.0)));
            let bounds2 = bounds(point(px(800.0), px(0.0)), size(px(800.0), px(800.0)));

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
    screen_share_track: Option<LocalTrackPublication>,
    microphone_stream: Option<AudioStream>,
    screen_share_stream: Option<Box<dyn ScreenCaptureStream>>,
    remote_participants: Vec<(ParticipantIdentity, ParticipantState)>,
    _events_task: Task<()>,
}

#[derive(Default)]
struct ParticipantState {
    audio_output_stream: Option<(TrackSid, AudioStream)>,
    muted: bool,
    screen_share_output_view: Option<(TrackSid, View<RemoteVideoTrackView>)>,
    speaking: bool,
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
                            screen_share_track: None,
                            screen_share_stream: None,
                            remote_participants: Vec::new(),
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
        eprintln!("room event: {event:?}");

        match event {
            RoomEvent::TrackUnpublished {
                publication,
                participant,
            } => {
                let output = self.remote_participant(participant);
                let unpublish_sid = publication.sid();
                if output
                    .audio_output_stream
                    .as_ref()
                    .map_or(false, |(sid, _)| *sid == unpublish_sid)
                {
                    output.audio_output_stream.take();
                }
                if output
                    .screen_share_output_view
                    .as_ref()
                    .map_or(false, |(sid, _)| *sid == unpublish_sid)
                {
                    output.screen_share_output_view.take();
                }
                cx.notify();
            }

            RoomEvent::TrackSubscribed {
                track, participant, ..
            } => {
                let output = self.remote_participant(participant);
                match track {
                    RemoteTrack::Audio(track) => {
                        output.audio_output_stream =
                            Some((track.sid(), play_remote_audio_track(&track, cx)));
                    }
                    RemoteTrack::Video(track) => {
                        output.screen_share_output_view = Some((
                            track.sid(),
                            cx.new_view(|cx| RemoteVideoTrackView::new(track, cx)),
                        ));
                    }
                }
                cx.notify();
            }

            RoomEvent::TrackMuted { participant, .. } => {
                if let Participant::Remote(participant) = participant {
                    self.remote_participant(participant).muted = true;
                    cx.notify();
                }
            }

            RoomEvent::TrackUnmuted { participant, .. } => {
                if let Participant::Remote(participant) = participant {
                    self.remote_participant(participant).muted = false;
                    cx.notify();
                }
            }

            RoomEvent::ActiveSpeakersChanged { speakers } => {
                for (identity, output) in &mut self.remote_participants {
                    output.speaking = speakers.iter().any(|speaker| {
                        if let Participant::Remote(speaker) = speaker {
                            speaker.identity() == *identity
                        } else {
                            false
                        }
                    });
                }
                cx.notify();
            }

            _ => {}
        }

        cx.notify();
    }

    fn remote_participant(&mut self, participant: RemoteParticipant) -> &mut ParticipantState {
        match self
            .remote_participants
            .binary_search_by_key(&&participant.identity(), |row| &row.0)
        {
            Ok(ix) => &mut self.remote_participants[ix].1,
            Err(ix) => {
                self.remote_participants
                    .insert(ix, (participant.identity(), ParticipantState::default()));
                &mut self.remote_participants[ix].1
            }
        }
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
                let (track, stream) = cx.update(|cx| capture_local_audio_track(cx))??;
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

    fn toggle_screen_share(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(track) = self.screen_share_track.take() {
            self.screen_share_stream.take();
            let participant = self.room.local_participant();
            cx.background_executor()
                .spawn(async move {
                    participant.unpublish_track(&track.sid()).await.unwrap();
                })
                .detach();
            cx.notify();
        } else {
            let participant = self.room.local_participant();
            let sources = cx.screen_capture_sources();
            cx.spawn(|this, mut cx| async move {
                let sources = sources.await.unwrap()?;
                let source = sources.into_iter().next().unwrap();
                let (track, stream) = capture_local_video_track(&*source).await?;
                let publication = participant
                    .publish_track(LocalTrack::Video(track), TrackPublishOptions::default())
                    .await?;
                this.update(&mut cx, |this, cx| {
                    this.screen_share_track = Some(publication);
                    this.screen_share_stream = Some(stream);
                    cx.notify();
                })
            })
            .detach();
        }
    }
}

impl Render for LivekitWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0xffa8d4))
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .p_1()
                    .bg(rgb(0xffd4a8))
                    .h(px(80.0))
                    .flex()
                    .flex_row()
                    .children([
                        div()
                            .id("toggle-mute")
                            .w(px(100.0))
                            .h(px(30.0))
                            .bg(rgb(0x6666ff))
                            .flex()
                            .flex_row()
                            .child(if let Some(track) = &self.microphone_track {
                                if track.is_muted() {
                                    "Unmute"
                                } else {
                                    "Mute"
                                }
                            } else {
                                "Publish mic"
                            })
                            .on_click(cx.listener(|this, _, cx| this.toggle_mute(cx))),
                        div()
                            .id("toggle-screen-share")
                            .w(px(100.0))
                            .h(px(30.0))
                            .bg(rgb(0x6666ff))
                            .flex()
                            .flex_row()
                            .child(if self.screen_share_track.is_none() {
                                "Share screen"
                            } else {
                                "Unshare screen"
                            })
                            .on_click(cx.listener(|this, _, cx| this.toggle_screen_share(cx))),
                    ]),
            )
            .child(
                div()
                    .id("remote-participants")
                    .overflow_y_scroll()
                    .p_1()
                    .bg(gpui::rgb(0xaaaaff))
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .children(self.remote_participants.iter().map(|(identity, state)| {
                        div()
                            .w_full()
                            .h(px(400.0))
                            .child(SharedString::from(if state.speaking {
                                format!("{} (speaking)", &identity.0)
                            } else if state.muted {
                                format!("{} (muted)", &identity.0)
                            } else {
                                identity.0.clone()
                            }))
                            .children(state.screen_share_output_view.as_ref().map(|e| e.1.clone()))
                    })),
            )
    }
}
