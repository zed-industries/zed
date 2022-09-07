mod live_kit_token;

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

        let live_kit_key = std::env::var("LIVE_KIT_KEY").unwrap();
        let live_kit_secret = std::env::var("LIVE_KIT_SECRET").unwrap();

        let token = live_kit_token::create_token(
            &live_kit_key,
            &live_kit_secret,
            "test-room",
            "test-participant",
        )
        .unwrap();

        let room = Room::new();
        cx.foreground()
            .spawn(async move {
                println!("connecting...");
                room.connect("wss://zed.livekit.cloud", &token).await;
                let windows = live_kit::list_windows();
                println!("connected! {:?}", windows);

                let window_id = windows.iter().next().unwrap().id;
                let track = LocalVideoTrack::screen_share_for_window(window_id);
                room.publish_video_track(&track).await;
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
