use gpui::{
    color::Color,
    elements::Text,
    fonts::{HighlightStyle, TextStyle},
    platform::{CursorStyle, MouseButton},
    AnyElement, CursorRegion, Element, MouseRegion,
};
use log::LevelFilter;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_window(Default::default(), |_| TextView);
    });
}

struct TextView;

impl gpui::Entity for TextView {
    type Event = ();
}

impl gpui::View for TextView {
    fn ui_name() -> &'static str {
        "View"
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> AnyElement<TextView> {
        let font_size = 12.;
        let family = cx
            .font_cache
            .load_family(&["Monaco"], &Default::default())
            .unwrap();
        let font_id = cx
            .font_cache
            .select_font(family, &Default::default())
            .unwrap();
        let view_id = cx.view_id();

        let underline = HighlightStyle {
            underline: Some(gpui::fonts::Underline {
                thickness: 1.0.into(),
                ..Default::default()
            }),
            ..Default::default()
        };

        Text::new(
            "The text:\nHello, beautiful world, hello!",
            TextStyle {
                font_id,
                font_size,
                color: Color::red(),
                font_family_name: "".into(),
                font_family_id: family,
                underline: Default::default(),
                font_properties: Default::default(),
            },
        )
        .with_highlights(vec![(17..26, underline), (34..40, underline)])
        .with_custom_runs(vec![(17..26), (34..40)], move |ix, bounds, scene, _| {
            scene.push_cursor_region(CursorRegion {
                bounds,
                style: CursorStyle::PointingHand,
            });
            scene.push_mouse_region(
                MouseRegion::new::<Self>(view_id, ix, bounds).on_click::<Self, _>(
                    MouseButton::Left,
                    move |_, _, _| {
                        eprintln!("clicked link {ix}");
                    },
                ),
            );
        })
        .into_any()
    }
}
