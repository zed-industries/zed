use gpui::{color::Color, geometry::rect::RectF, AnyElement, App, Element, Entity, Quad, View};
use log::LevelFilter;
use pathfinder_geometry::vector::vec2f;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_window(Default::default(), |_| QuadView);
    });
}

struct QuadView;

impl Entity for QuadView {
    type Event = ();
}

impl View for QuadView {
    fn ui_name() -> &'static str {
        "QuadView"
    }

    fn render(&mut self, _: &mut gpui::ViewContext<Self>) -> AnyElement<QuadView> {
        QuadElement.into_any()
    }
}

struct QuadElement;

impl<V: View> gpui::Element<V> for QuadElement {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        _: &mut V,
        _: &mut gpui::LayoutContext<V>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn paint(
        &mut self,
        scene: &mut gpui::SceneBuilder,
        _: pathfinder_geometry::rect::RectF,
        _: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        _: &mut V,
        _: &mut gpui::PaintContext<V>,
    ) -> Self::PaintState {
        scene.push_quad(Quad {
            bounds: RectF::new(vec2f(100., 100.), vec2f(100., 100.)),
            background: Some(Color::red()),
            border: Default::default(),
            corner_radii: gpui::scene::CornerRadii {
                top_left: 20.,
                ..Default::default()
            },
        });

        scene.push_quad(Quad {
            bounds: RectF::new(vec2f(200., 100.), vec2f(100., 100.)),
            background: Some(Color::green()),
            border: Default::default(),
            corner_radii: gpui::scene::CornerRadii {
                top_right: 20.,
                ..Default::default()
            },
        });

        scene.push_quad(Quad {
            bounds: RectF::new(vec2f(100., 200.), vec2f(100., 100.)),
            background: Some(Color::blue()),
            border: Default::default(),
            corner_radii: gpui::scene::CornerRadii {
                bottom_left: 20.,
                ..Default::default()
            },
        });

        scene.push_quad(Quad {
            bounds: RectF::new(vec2f(200., 200.), vec2f(100., 100.)),
            background: Some(Color::yellow()),
            border: Default::default(),
            corner_radii: gpui::scene::CornerRadii {
                bottom_right: 20.,
                ..Default::default()
            },
        });
    }

    fn rect_for_text_range(
        &self,
        _: std::ops::Range<usize>,
        _: pathfinder_geometry::rect::RectF,
        _: pathfinder_geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &gpui::ViewContext<V>,
    ) -> Option<pathfinder_geometry::rect::RectF> {
        unimplemented!()
    }

    fn debug(
        &self,
        _: pathfinder_geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &gpui::ViewContext<V>,
    ) -> serde_json::Value {
        unimplemented!()
    }
}
