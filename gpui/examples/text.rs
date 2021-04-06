use gpui::{
    color::ColorU,
    fonts::{Properties, Weight},
    platform::{current as platform, Runner},
    Border, Element as _, Quad,
};
use log::LevelFilter;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    let mut app = gpui::App::new(()).unwrap();
    platform::runner()
        .on_finish_launching(move || {
            app.platform().activate(true);
            app.add_window(|_| TextView);
        })
        .run();
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

    fn render<'a>(&self, app: &gpui::AppContext) -> gpui::ElementBox {
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

    fn after_layout(
        &mut self,
        _: pathfinder_geometry::vector::Vector2F,
        _: &mut Self::LayoutState,
        _: &mut gpui::AfterLayoutContext,
    ) {
    }

    fn paint(
        &mut self,
        bounds: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        ctx: &mut gpui::PaintContext,
    ) -> Self::PaintState {
        let font_size = 12.;
        let family = ctx.font_cache.load_family(&["SF Pro Display"]).unwrap();
        let normal = ctx
            .font_cache
            .select_font(family, &Default::default())
            .unwrap();
        let bold = ctx
            .font_cache
            .select_font(
                family,
                &Properties {
                    weight: Weight::BOLD,
                    ..Default::default()
                },
            )
            .unwrap();

        let text = "Hello world!";
        let line = ctx.text_layout_cache.layout_str(
            text,
            font_size,
            &[
                (0..1, normal),
                (1..2, bold),
                (2..3, normal),
                (3..4, bold),
                (4..text.len(), normal),
            ],
        );

        ctx.scene.push_quad(Quad {
            bounds: bounds,
            background: Some(ColorU::white()),
            ..Default::default()
        });
        line.paint(
            bounds.origin(),
            bounds,
            &[(0..text.len(), ColorU::black())],
            ctx,
        );
    }

    fn dispatch_event(
        &mut self,
        _: &gpui::Event,
        _: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut gpui::EventContext,
    ) -> bool {
        false
    }
}
