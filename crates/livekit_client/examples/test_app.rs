#![cfg_attr(windows, allow(unused))]
// TODO: For some reason mac build complains about import of postage::stream::Stream, but removal of
// it causes compile errors.
#![cfg_attr(target_os = "macos", allow(unused_imports))]

use gpui::{
    actions, bounds, div, point,
    prelude::{FluentBuilder as _, IntoElement},
    px, rgb, size, AppContext, AsyncAppContext, Bounds, Context, InteractiveElement, KeyBinding,
    Menu, MenuItem, Model, ParentElement, Pixels, Render, ScreenCaptureStream, SharedString,
    StatefulInteractiveElement as _, Styled, Task, WindowBounds, WindowHandle, WindowOptions,
};
#[cfg(not(target_os = "windows"))]
use livekit_client::{
    capture_local_audio_track, capture_local_video_track,
    id::ParticipantIdentity,
    options::{TrackPublishOptions, VideoCodec},
    participant::{Participant, RemoteParticipant},
    play_remote_audio_track,
    publication::{LocalTrackPublication, RemoteTrackPublication},
    track::{LocalTrack, RemoteTrack, RemoteVideoTrack, TrackSource},
    AudioStream, RemoteVideoTrackView, Room, RoomEvent, RoomOptions,
};
#[cfg(not(target_os = "windows"))]
use postage::stream::Stream;

#[cfg(target_os = "windows")]
use livekit_client::{
    participant::{Participant, RemoteParticipant},
    publication::{LocalTrackPublication, RemoteTrackPublication},
    track::{LocalTrack, RemoteTrack, RemoteVideoTrack},
    AudioStream, RemoteVideoTrackView, Room, RoomEvent,
};

use livekit_server::token::{self, VideoGrant};
use log::LevelFilter;
use simplelog::SimpleLogger;

actions!(livekit_client, [Quit]);

#[cfg(windows)]
fn main() {}

#[cfg(not(windows))]
fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new().run(|cx| {
        livekit_client::init(
            cx.background_executor().dispatcher.clone(),
            cx.http_client(),
        );

        #[cfg(any(test, feature = "test-support"))]
        println!("USING TEST LIVEKIT");

        #[cfg(not(any(test, feature = "test-support")))]
        println!("USING REAL LIVEKIT");

        cx.activate(true);
        cx.on_action(quit);
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Zed".into(),
            items: vec![MenuItem::Action {
                name: "Quit".into(),
                action: Box::new(Quit),
                os_action: None,
            }],
        }]);

        let livekit_url = std::env::var("LIVEKIT_URL").unwrap_or("http://localhost:7880".into());
        let livekit_key = std::env::var("LIVEKIT_KEY").unwrap_or("devkey".into());
        let livekit_secret = std::env::var("LIVEKIT_SECRET").unwrap_or("secret".into());
        let height = px(800.);
        let width = px(800.);

        cx.spawn(|cx| async move {
            let mut windows = Vec::new();
            for i in 0..2 {
                let token = token::create(
                    &livekit_key,
                    &livekit_secret,
                    Some(&format!("test-participant-{i}")),
                    VideoGrant::to_join("test-room"),
                )
                .unwrap();

                let bounds = bounds(point(width * i, px(0.0)), size(width, height));
                let window =
                    LivekitWindow::new(livekit_url.as_str(), token.as_str(), bounds, cx.clone())
                        .await;
                windows.push(window);
            }
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
    #[cfg(not(target_os = "windows"))]
    remote_participants: Vec<(ParticipantIdentity, ParticipantState)>,
    _events_task: Task<()>,
}

#[derive(Default)]
struct ParticipantState {
    audio_output_stream: Option<(RemoteTrackPublication, AudioStream)>,
    muted: bool,
    screen_share_output_view: Option<(RemoteVideoTrack, gpui::Model<RemoteVideoTrackView>)>,
    speaking: bool,
}

#[cfg(not(windows))]
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
                |model, window, cx| {
                    let _events_task = model.spawn(cx, |this, mut cx| async move {
                        while let Some(event) = events.recv().await {
                            this.update(&mut cx, |this: &mut LivekitWindow, model, cx| {
                                this.handle_room_event(event, model, cx)
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
                },
            )
            .unwrap()
        })
        .unwrap()
    }

    fn handle_room_event(
        &mut self,
        event: RoomEvent,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) {
        eprintln!("event: {event:?}");

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
                    .map_or(false, |(track, _)| track.sid() == unpublish_sid)
                {
                    output.audio_output_stream.take();
                }
                if output
                    .screen_share_output_view
                    .as_ref()
                    .map_or(false, |(track, _)| track.sid() == unpublish_sid)
                {
                    output.screen_share_output_view.take();
                }
                model.notify(cx);
            }

            RoomEvent::TrackSubscribed {
                publication,
                participant,
                track,
            } => {
                let output = self.remote_participant(participant);
                match track {
                    RemoteTrack::Audio(track) => {
                        output.audio_output_stream = Some((
                            publication.clone(),
                            play_remote_audio_track(&track, cx.background_executor()).unwrap(),
                        ));
                    }
                    RemoteTrack::Video(track) => {
                        output.screen_share_output_view = Some((
                            track.clone(),
                            cx.new_model(|model, cx| {
                                RemoteVideoTrackView::new(track, model, window, cx)
                            }),
                        ));
                    }
                }
                model.notify(cx);
            }

            RoomEvent::TrackMuted { participant, .. } => {
                if let Participant::Remote(participant) = participant {
                    self.remote_participant(participant).muted = true;
                    model.notify(cx);
                }
            }

            RoomEvent::TrackUnmuted { participant, .. } => {
                if let Participant::Remote(participant) = participant {
                    self.remote_participant(participant).muted = false;
                    model.notify(cx);
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
                model.notify(cx);
            }

            _ => {}
        }

        model.notify(cx);
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

    fn toggle_mute(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        if let Some(track) = &self.microphone_track {
            if track.is_muted() {
                track.unmute();
            } else {
                track.mute();
            }
            model.notify(cx);
        } else {
            let participant = self.room.local_participant();
            model
                .spawn(cx, |this, mut cx| async move {
                    let (track, stream) =
                        capture_local_audio_track(cx.background_executor())?.await;
                    let publication = participant
                        .publish_track(
                            LocalTrack::Audio(track),
                            TrackPublishOptions {
                                source: TrackSource::Microphone,
                                ..Default::default()
                            },
                        )
                        .await
                        .unwrap();
                    this.update(&mut cx, |this, model, cx| {
                        this.microphone_track = Some(publication);
                        this.microphone_stream = Some(stream);
                        model.notify(cx);
                    })
                })
                .detach();
        }
    }

    fn toggle_screen_share(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        if let Some(track) = self.screen_share_track.take() {
            self.screen_share_stream.take();
            let participant = self.room.local_participant();
            cx.background_executor()
                .spawn(async move {
                    participant.unpublish_track(&track.sid()).await.unwrap();
                })
                .detach();
            model.notify(cx);
        } else {
            let participant = self.room.local_participant();
            let sources = cx.screen_capture_sources();
            model
                .spawn(cx, |this, mut cx| async move {
                    let sources = sources.await.unwrap()?;
                    let source = sources.into_iter().next().unwrap();
                    let (track, stream) = capture_local_video_track(&*source).await?;
                    let publication = participant
                        .publish_track(
                            LocalTrack::Video(track),
                            TrackPublishOptions {
                                source: TrackSource::Screenshare,
                                video_codec: VideoCodec::H264,
                                ..Default::default()
                            },
                        )
                        .await
                        .unwrap();
                    this.update(&mut cx, |this, model, cx| {
                        this.screen_share_track = Some(publication);
                        this.screen_share_stream = Some(stream);
                        model.notify(cx);
                    })
                })
                .detach();
        }
    }

    fn toggle_remote_audio_for_participant(
        &mut self,
        identity: &ParticipantIdentity,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> Option<()> {
        let participant = self.remote_participants.iter().find_map(|(id, state)| {
            if id == identity {
                Some(state)
            } else {
                None
            }
        })?;
        let publication = &participant.audio_output_stream.as_ref()?.0;
        publication.set_enabled(!publication.is_enabled());
        model.notify(cx);
        Some(())
    }
}

#[cfg(not(windows))]
impl Render for LivekitWindow {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        fn button() -> gpui::Div {
            div()
                .w(px(180.0))
                .h(px(30.0))
                .px_2()
                .m_2()
                .bg(rgb(0x8888ff))
        }

        div()
            .bg(rgb(0xffffff))
            .size_full()
            .flex()
            .flex_col()
            .child(
                div().bg(rgb(0xffd4a8)).flex().flex_row().children([
                    button()
                        .id("toggle-mute")
                        .child(if let Some(track) = &self.microphone_track {
                            if track.is_muted() {
                                "Unmute"
                            } else {
                                "Mute"
                            }
                        } else {
                            "Publish mic"
                        })
                        .on_click(
                            model
                                .listener(|this, _, model, window, cx| this.toggle_mute(model, cx)),
                        ),
                    button()
                        .id("toggle-screen-share")
                        .child(if self.screen_share_track.is_none() {
                            "Share screen"
                        } else {
                            "Unshare screen"
                        })
                        .on_click(model.listener(|this, _, model, window, cx| {
                            this.toggle_screen_share(model, cx)
                        })),
                ]),
            )
            .child(
                div()
                    .id("remote-participants")
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .children(self.remote_participants.iter().map(|(identity, state)| {
                        div()
                            .h(px(300.0))
                            .flex()
                            .flex_col()
                            .m_2()
                            .px_2()
                            .bg(rgb(0x8888ff))
                            .child(SharedString::from(if state.speaking {
                                format!("{} (speaking)", &identity.0)
                            } else if state.muted {
                                format!("{} (muted)", &identity.0)
                            } else {
                                identity.0.clone()
                            }))
                            .when_some(state.audio_output_stream.as_ref(), |el, state| {
                                el.child(
                                    button()
                                        .id(SharedString::from(identity.0.clone()))
                                        .child(if state.0.is_enabled() {
                                            "Deafen"
                                        } else {
                                            "Undeafen"
                                        })
                                        .on_click(model.listener({
                                            let identity = identity.clone();
                                            move |this, _, model, window, cx| {
                                                this.toggle_remote_audio_for_participant(
                                                    &identity, model, cx,
                                                );
                                            }
                                        })),
                                )
                            })
                            .children(state.screen_share_output_view.as_ref().map(|e| e.1.clone()))
                    })),
            )
    }
}
