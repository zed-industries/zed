use std::sync::Arc;

use futures::StreamExt;
use gpui::{
    AppContext as _, AsyncApp, Bounds, Context, Entity, InteractiveElement, KeyBinding, Menu,
    MenuItem, ParentElement, Pixels, Render, ScreenCaptureStream, SharedString,
    StatefulInteractiveElement as _, Styled, Task, Window, WindowBounds, WindowHandle,
    WindowOptions, actions, bounds, div, point,
    prelude::{FluentBuilder as _, IntoElement},
    px, rgb, size,
};
use livekit_client::{
    AudioStream, LocalTrackPublication, Participant, ParticipantIdentity, RemoteParticipant,
    RemoteTrackPublication, RemoteVideoTrack, RemoteVideoTrackView, Room, RoomEvent,
};

use livekit_api::token::{self, VideoGrant};
use log::LevelFilter;
use simplelog::SimpleLogger;

actions!(livekit_client, [Quit]);

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::Application::new().run(|cx| {
        #[cfg(any(test, feature = "test-support"))]
        println!("USING TEST LIVEKIT");

        #[cfg(not(any(test, feature = "test-support")))]
        println!("USING REAL LIVEKIT");

        gpui_tokio::init(cx);

        cx.activate(true);
        cx.on_action(quit);
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Zed".into(),
            items: vec![MenuItem::Action {
                name: "Quit".into(),
                action: Box::new(Quit),
                os_action: None,
                checked: false,
            }],
        }]);

        let livekit_url = std::env::var("LIVEKIT_URL").unwrap_or("http://localhost:7880".into());
        let livekit_key = std::env::var("LIVEKIT_KEY").unwrap_or("devkey".into());
        let livekit_secret = std::env::var("LIVEKIT_SECRET").unwrap_or("secret".into());
        let height = px(800.);
        let width = px(800.);

        cx.spawn(async move |cx| {
            let mut windows = Vec::new();
            for i in 0..2 {
                let token = token::create(
                    &livekit_key,
                    &livekit_secret,
                    Some(&format!("test-participant-{i}")),
                    VideoGrant::to_join("wtej-trty"),
                )
                .unwrap();

                let bounds = bounds(point(width * i, px(0.0)), size(width, height));
                let window = LivekitWindow::new(livekit_url.clone(), token, bounds, cx).await;
                windows.push(window);
            }
        })
        .detach();
    });
}

fn quit(_: &Quit, cx: &mut gpui::App) {
    cx.quit();
}

struct LivekitWindow {
    room: Arc<livekit_client::Room>,
    microphone_track: Option<LocalTrackPublication>,
    screen_share_track: Option<LocalTrackPublication>,
    microphone_stream: Option<livekit_client::AudioStream>,
    screen_share_stream: Option<Box<dyn ScreenCaptureStream>>,
    remote_participants: Vec<(ParticipantIdentity, ParticipantState)>,
    _events_task: Task<()>,
}

#[derive(Default)]
struct ParticipantState {
    audio_output_stream: Option<(RemoteTrackPublication, AudioStream)>,
    muted: bool,
    screen_share_output_view: Option<(RemoteVideoTrack, Entity<RemoteVideoTrackView>)>,
    speaking: bool,
}

impl LivekitWindow {
    async fn new(
        url: String,
        token: String,
        bounds: Bounds<Pixels>,
        cx: &mut AsyncApp,
    ) -> WindowHandle<Self> {
        let (room, mut events) =
            Room::connect(url.clone(), token, cx)
                .await
                .unwrap_or_else(|err| {
                    eprintln!(
                        "Failed to connect to {url}: {err}.\nTry `foreman start` to run the livekit server"
                    );

                    std::process::exit(1)
                });

        cx.update(|cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    cx.new(|cx| {
                        let _events_task = cx.spawn_in(window, async move |this, cx| {
                            while let Some(event) = events.next().await {
                                cx.update(|window, cx| {
                                    this.update(cx, |this: &mut LivekitWindow, cx| {
                                        this.handle_room_event(event, window, cx)
                                    })
                                })
                                .ok();
                            }
                        });

                        Self {
                            room: Arc::new(room),
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
    }

    fn handle_room_event(&mut self, event: RoomEvent, window: &mut Window, cx: &mut Context<Self>) {
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
                    .is_some_and(|(track, _)| track.sid() == unpublish_sid)
                {
                    output.audio_output_stream.take();
                }
                if output
                    .screen_share_output_view
                    .as_ref()
                    .is_some_and(|(track, _)| track.sid() == unpublish_sid)
                {
                    output.screen_share_output_view.take();
                }
                cx.notify();
            }

            RoomEvent::TrackSubscribed {
                publication,
                participant,
                track,
            } => {
                let room = self.room.clone();
                let output = self.remote_participant(participant);
                match track {
                    livekit_client::RemoteTrack::Audio(track) => {
                        output.audio_output_stream = Some((
                            publication,
                            room.play_remote_audio_track(&track, cx).unwrap(),
                        ));
                    }
                    livekit_client::RemoteTrack::Video(track) => {
                        output.screen_share_output_view = Some((
                            track.clone(),
                            cx.new(|cx| RemoteVideoTrackView::new(track, window, cx)),
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

    fn toggle_mute(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(track) = &self.microphone_track {
            if track.is_muted() {
                track.unmute(cx);
            } else {
                track.mute(cx);
            }
            cx.notify();
        } else {
            let room = self.room.clone();
            cx.spawn_in(window, async move |this, cx| {
                let (publication, stream) = room
                    .publish_local_microphone_track("test_user".to_string(), false, cx)
                    .await
                    .unwrap();
                this.update(cx, |this, cx| {
                    this.microphone_track = Some(publication);
                    this.microphone_stream = Some(stream);
                    cx.notify();
                })
            })
            .detach();
        }
    }

    fn toggle_screen_share(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(track) = self.screen_share_track.take() {
            self.screen_share_stream.take();
            let participant = self.room.local_participant();
            cx.spawn(async move |_, cx| {
                participant.unpublish_track(track.sid(), cx).await.unwrap();
            })
            .detach();
            cx.notify();
        } else {
            let participant = self.room.local_participant();
            let sources = cx.screen_capture_sources();
            cx.spawn_in(window, async move |this, cx| {
                let sources = sources.await.unwrap()?;
                let source = sources.into_iter().next().unwrap();

                let (publication, stream) = participant
                    .publish_screenshare_track(&*source, cx)
                    .await
                    .unwrap();
                this.update(cx, |this, cx| {
                    this.screen_share_track = Some(publication);
                    this.screen_share_stream = Some(stream);
                    cx.notify();
                })
            })
            .detach();
        }
    }

    fn toggle_remote_audio_for_participant(
        &mut self,
        identity: &ParticipantIdentity,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let participant = self.remote_participants.iter().find_map(|(id, state)| {
            if id == identity { Some(state) } else { None }
        })?;
        let publication = &participant.audio_output_stream.as_ref()?.0;
        publication.set_enabled(!publication.is_enabled(), cx);
        cx.notify();
        Some(())
    }
}

impl Render for LivekitWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            if track.is_muted() { "Unmute" } else { "Mute" }
                        } else {
                            "Publish mic"
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.toggle_mute(window, cx))),
                    button()
                        .id("toggle-screen-share")
                        .child(if self.screen_share_track.is_none() {
                            "Share screen"
                        } else {
                            "Unshare screen"
                        })
                        .on_click(
                            cx.listener(|this, _, window, cx| this.toggle_screen_share(window, cx)),
                        ),
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
                            .h(px(1080.0))
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
                                        .id(identity.0.clone())
                                        .child(if state.0.is_enabled() {
                                            "Deafen"
                                        } else {
                                            "Undeafen"
                                        })
                                        .on_click(cx.listener({
                                            let identity = identity.clone();
                                            move |this, _, _, cx| {
                                                this.toggle_remote_audio_for_participant(
                                                    &identity, cx,
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
