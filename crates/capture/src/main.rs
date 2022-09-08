mod live_kit_token;

use futures::StreamExt;
use gpui::{
    actions,
    elements::{Canvas, *},
    keymap::Binding,
    platform::current::Surface,
    Menu, MenuItem, ViewContext,
};
use live_kit::{LocalVideoTrack, Room};
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

        let live_kit_url = std::env::var("LIVE_KIT_URL").unwrap();
        let live_kit_key = std::env::var("LIVE_KIT_KEY").unwrap();
        let live_kit_secret = std::env::var("LIVE_KIT_SECRET").unwrap();

        cx.spawn(|mut cx| async move {
            let user1_token = live_kit_token::create_token(
                &live_kit_key,
                &live_kit_secret,
                "test-room",
                "test-participant-1",
            )
            .unwrap();
            let room1 = Room::new();
            room1.connect(&live_kit_url, &user1_token).await.unwrap();

            let user2_token = live_kit_token::create_token(
                &live_kit_key,
                &live_kit_secret,
                "test-room",
                "test-participant-2",
            )
            .unwrap();
            let room2 = Room::new();
            room2.connect(&live_kit_url, &user2_token).await.unwrap();
            cx.add_window(Default::default(), |cx| ScreenCaptureView::new(room2, cx));

            let windows = live_kit::list_windows();
            let window = windows
                .iter()
                .find(|w| w.owner_name.as_deref() == Some("Safari"))
                .unwrap();
            let track = LocalVideoTrack::screen_share_for_window(window.id);
            room1.publish_video_track(&track).await.unwrap();
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
        let mut remote_video_tracks = room.remote_video_tracks();
        cx.spawn_weak(|this, mut cx| async move {
            if let Some(video_track) = remote_video_tracks.next().await {
                let (mut frames_tx, mut frames_rx) = watch::channel_with(None);
                video_track.add_renderer(move |frame| *frames_tx.borrow_mut() = Some(frame));

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
        Canvas::new(move |bounds, _, cx| {
            if let Some(image_buffer) = image_buffer.clone() {
                cx.scene.push_surface(Surface {
                    bounds,
                    image_buffer,
                });
            }
        })
        .boxed()
    }
}

fn quit(_: &Quit, cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}
