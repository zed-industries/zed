use gpui::{
    color::Color, geometry::rect::RectF, scene::Shadow, AnyElement, App, Element, Entity, Quad,
    View,
};
use log::LevelFilter;
use pathfinder_geometry::vector::vec2f;
use simplelog::SimpleLogger;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_window(Default::default(), |_| CornersView);
    });
}

struct CornersView;

impl Entity for CornersView {
    type Event = ();
}

impl View for CornersView {
    fn ui_name() -> &'static str {
        "CornersView"
    }

    fn render(&mut self, _: &mut gpui::ViewContext<Self>) -> AnyElement<CornersView> {
        CornersElement.into_any()
    }
}

struct CornersElement;

impl<V: View> gpui::Element<V> for CornersElement {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        _: &mut V,
        _: &mut gpui::ViewContext<V>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn paint(
        &mut self,
        bounds: pathfinder_geometry::rect::RectF,
        _: pathfinder_geometry::rect::RectF,
        _: &mut Self::LayoutState,
        _: &mut V,
        cx: &mut gpui::PaintContext<V>,
    ) -> Self::PaintState {
        cx.scene().push_quad(Quad {
            bounds,
            background: Some(Color::white()),
            ..Default::default()
        });

        cx.scene().push_layer(None);

        cx.scene().push_quad(Quad {
            bounds: RectF::new(vec2f(100., 100.), vec2f(100., 100.)),
            background: Some(Color::red()),
            border: Default::default(),
            corner_radii: gpui::scene::CornerRadii {
                top_left: 20.,
                ..Default::default()
            },
        });

        cx.scene().push_quad(Quad {
            bounds: RectF::new(vec2f(200., 100.), vec2f(100., 100.)),
            background: Some(Color::green()),
            border: Default::default(),
            corner_radii: gpui::scene::CornerRadii {
                top_right: 20.,
                ..Default::default()
            },
        });

        cx.scene().push_quad(Quad {
            bounds: RectF::new(vec2f(100., 200.), vec2f(100., 100.)),
            background: Some(Color::blue()),
            border: Default::default(),
            corner_radii: gpui::scene::CornerRadii {
                bottom_left: 20.,
                ..Default::default()
            },
        });

        cx.scene().push_quad(Quad {
            bounds: RectF::new(vec2f(200., 200.), vec2f(100., 100.)),
            background: Some(Color::yellow()),
            border: Default::default(),
            corner_radii: gpui::scene::CornerRadii {
                bottom_right: 20.,
                ..Default::default()
            },
        });

        cx.scene().push_shadow(Shadow {
            bounds: RectF::new(vec2f(400., 100.), vec2f(100., 100.)),
            corner_radii: gpui::scene::CornerRadii {
                bottom_right: 20.,
                ..Default::default()
            },
            sigma: 20.0,
            color: Color::black(),
        });

        cx.scene().push_layer(None);
        cx.scene().push_quad(Quad {
            bounds: RectF::new(vec2f(400., 100.), vec2f(100., 100.)),
            background: Some(Color::red()),
            border: Default::default(),
            corner_radii: gpui::scene::CornerRadii {
                bottom_right: 20.,
                ..Default::default()
            },
        });

        cx.scene().pop_layer();
        cx.scene().pop_layer();
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
