use gpui::{
    fonts::{Properties, Weight},
    platform::{current as platform, Runner},
    scene::Glyph,
    Element as _,
};
use log::LevelFilter;
use pathfinder_color::ColorU;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    let mut app = gpui::App::new(()).unwrap();
    platform::runner()
        .on_finish_launching(move || {
            app.platform().activate(true);
            app.add_window(|_| View);
        })
        .run();
}

struct View;
struct Element;

impl gpui::Entity for View {
    type Event = ();
}

impl gpui::View for View {
    fn ui_name() -> &'static str {
        "view"
    }

    fn render<'a>(&self, app: &gpui::AppContext) -> gpui::ElementBox {
        Element.boxed()
    }
}

impl gpui::Element for Element {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        ctx: &mut gpui::LayoutContext,
    ) -> (pathfinder_geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn after_layout(
        &mut self,
        size: pathfinder_geometry::vector::Vector2F,
        layout: &mut Self::LayoutState,
        ctx: &mut gpui::AfterLayoutContext,
    ) {
    }

    fn paint(
        &mut self,
        bounds: pathfinder_geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        ctx: &mut gpui::PaintContext,
    ) -> Self::PaintState {
        let font_size = 18.;
        let family = ctx.font_cache.load_family(&["Fira Code"]).unwrap();
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

        let line = ctx.text_layout_cache.layout_str(
            "xxXX",
            font_size,
            &[(0..1, normal), (1..2, bold), (2..3, normal), (3..4, bold)],
        );

        for run in line.runs {
            for glyph in run.glyphs {
                ctx.scene.push_glyph(Glyph {
                    font_id: run.font_id,
                    font_size,
                    id: glyph.id,
                    origin: glyph.position,
                    color: ColorU::black(),
                });
            }
        }
    }

    fn dispatch_event(
        &mut self,
        event: &gpui::Event,
        bounds: pathfinder_geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        paint: &mut Self::PaintState,
        ctx: &mut gpui::EventContext,
    ) -> bool {
        false
    }
}
