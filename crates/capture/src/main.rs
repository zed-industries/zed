mod live_kit_token;

use std::time::Duration;

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

        let live_kit_url = std::env::var("LIVE_KIT_URL").unwrap();
        let live_kit_key = std::env::var("LIVE_KIT_KEY").unwrap();
        let live_kit_secret = std::env::var("LIVE_KIT_SECRET").unwrap();

        let background = cx.background().clone();
        cx.foreground()
            .spawn(async move {
                println!("connecting...");
                let user1_token = live_kit_token::create_token(
                    &live_kit_key,
                    &live_kit_secret,
                    "test-room",
                    "test-participant-1",
                )
                .unwrap();
                let room1 = Room::new("user-1 room");
                room1.connect(&live_kit_url, &user1_token).await.unwrap();

                let user2_token = live_kit_token::create_token(
                    &live_kit_key,
                    &live_kit_secret,
                    "test-room",
                    "test-participant-2",
                )
                .unwrap();
                let room2 = Room::new("user-2 room");
                room2.connect(&live_kit_url, &user2_token).await.unwrap();

                let windows = live_kit::list_windows();
                println!("connected! {:?}", windows);

                let window_id = windows.iter().next().unwrap().id;
                let track = LocalVideoTrack::screen_share_for_window(window_id);
                room1.publish_video_track(&track).await.unwrap();

                background.timer(Duration::from_secs(120)).await;
            })
            .detach();

        // cx.add_window(Default::default(), |cx| ScreenCaptureView::new(cx));
    });
}

struct ScreenCaptureView {
    image_buffer: Option<CVImageBuffer>,
}

impl gpui::Entity for ScreenCaptureView {
    type Event = ();
}

impl ScreenCaptureView {
    pub fn new(_: &mut ViewContext<Self>) -> Self {
        Self { image_buffer: None }
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
