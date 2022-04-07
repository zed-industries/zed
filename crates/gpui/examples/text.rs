use gpui::{
    color::Color,
    fonts::{Properties, Weight},
    text_layout::RunStyle,
    DebugContext, Element as _, Quad,
};
use log::LevelFilter;
use pathfinder_geometry::rect::RectF;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_window(Default::default(), |_| TextView);
    });
}

struct TextView;
struct TextElement;

impl gpui::Entity for TextView {
    type Event = ();
}

impl gpui::View for TextView {
    fn ui_name() -> &'static str {
        "View"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<Self>) -> gpui::ElementBox {
        TextElement.boxed()
    }
}

impl gpui::Element for TextElement {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        _: &mut gpui::LayoutContext,
    ) -> (pathfinder_geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut gpui::PaintContext,
    ) -> Self::PaintState {
        let font_size = 12.;
        let family = cx.font_cache.load_family(&["SF Pro Display"]).unwrap();
        let normal = RunStyle {
            font_id: cx
                .font_cache
                .select_font(family, &Default::default())
                .unwrap(),
            color: Color::default(),
            underline: Default::default(),
        };
        let bold = RunStyle {
            font_id: cx
                .font_cache
                .select_font(
                    family,
                    &Properties {
                        weight: Weight::BOLD,
                        ..Default::default()
                    },
                )
                .unwrap(),
            color: Color::default(),
            underline: Default::default(),
        };

        let text = "Hello world!";
        let line = cx.text_layout_cache.layout_str(
            text,
            font_size,
            &[
                (1, normal.clone()),
                (1, bold.clone()),
                (1, normal.clone()),
                (1, bold.clone()),
                (text.len() - 4, normal.clone()),
            ],
        );

        cx.scene.push_quad(Quad {
            bounds,
            background: Some(Color::white()),
            ..Default::default()
        });
        line.paint(bounds.origin(), visible_bounds, bounds.height(), cx);
    }

    fn dispatch_event(
        &mut self,
        _: &gpui::Event,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut gpui::EventContext,
    ) -> bool {
        false
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &DebugContext,
    ) -> gpui::json::Value {
        todo!()
    }
}
