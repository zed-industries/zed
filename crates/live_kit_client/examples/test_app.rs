use core_foundation::base::CFRetain;
use futures::StreamExt;
use gpui::{
    actions,
    elements::{Canvas, *},
    keymap::Binding,
    platform::current::Surface,
    Menu, MenuItem, ViewContext,
};
use live_kit_client::{LocalVideoTrack, RemoteVideoTrackUpdate, Room};
use live_kit_server::token::{self, VideoGrant};
use log::LevelFilter;
use media::core_video::CVImageBuffer;
use postage::watch;
use simplelog::SimpleLogger;
use std::sync::Arc;

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

            let display = live_kit_client::display_source().await.unwrap();

            let track_a = LocalVideoTrack::screen_share_for_display(&display);
            room_a.publish_video_track(&track_a).await.unwrap();

            let next_update = track_changes.next().await.unwrap();

            if let RemoteVideoTrackUpdate::Subscribed(track) = next_update {
                println!("A !!!!!!!!!!!!");
                let remote_tracks = room_b.remote_video_tracks("test-participant-1");
                println!("B !!!!!!!!!!!!");
                assert_eq!(remote_tracks.len(), 1);
                println!("C !!!!!!!!!!!!");
                assert_eq!(remote_tracks[0].publisher_id(), "test-participant-1");
                println!("D !!!!!!!!!!!!");
                // dbg!(track.id());
                // assert_eq!(track.id(), "test-participant-1");
            } else {
                panic!("unexpected message")
            }
            println!("E !!!!!!!!!!!!");

            cx.platform().quit();
        })
        .detach();
    });
}

struct ScreenCaptureView {
    image_buffer: Option<CVImageBuffer>,
    _room: Arc<Room>,
}

impl gpui::Entity for ScreenCaptureView {
    type Event = ();
}

impl ScreenCaptureView {
    pub fn new(room: Arc<Room>, cx: &mut ViewContext<Self>) -> Self {
        let mut remote_video_tracks = room.remote_video_track_updates();
        cx.spawn_weak(|this, mut cx| async move {
            if let Some(video_track) = remote_video_tracks.next().await {
                let (mut frames_tx, mut frames_rx) = watch::channel_with(None);
                // video_track.add_renderer(move |frame| *frames_tx.borrow_mut() = Some(frame));

                while let Some(frame) = frames_rx.next().await {
                    if let Some(this) = this.upgrade(&cx) {
                        this.update(&mut cx, |this, cx| {
                            this.image_buffer = frame;
                            cx.notify();
                        });
                    } else {
                        break;
                    }
                }
            }
        })
        .detach();

        Self {
            image_buffer: None,
            _room: room,
        }
    }
}

impl gpui::View for ScreenCaptureView {
    fn ui_name() -> &'static str {
        "View"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<Self>) -> gpui::ElementBox {
        let image_buffer = self.image_buffer.clone();
        let canvas = Canvas::new(move |bounds, _, cx| {
            if let Some(image_buffer) = image_buffer.clone() {
                cx.scene.push_surface(Surface {
                    bounds,
                    image_buffer,
                });
            }
        });

        if let Some(image_buffer) = self.image_buffer.as_ref() {
            canvas
                .constrained()
                .with_width(image_buffer.width() as f32)
                .with_height(image_buffer.height() as f32)
                .aligned()
                .boxed()
        } else {
            canvas.boxed()
        }
    }
}

fn quit(_: &Quit, cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}
