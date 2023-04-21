use gpui::{
    color::Color,
    fonts::{Properties, Weight},
    text_layout::RunStyle,
    Drawable, Element, Quad, SceneBuilder, View, ViewContext,
};
use log::LevelFilter;
use pathfinder_geometry::rect::RectF;
use simplelog::SimpleLogger;
use std::ops::Range;

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

    fn render(&mut self, _: &mut gpui::ViewContext<Self>) -> Element<TextView> {
        TextElement.into_element()
    }
}

impl<V: View> Drawable<V> for TextElement {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        _: &mut V,
        _: &mut ViewContext<V>,
    ) -> (pathfinder_geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        _: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState {
        let font_size = 12.;
        let family = cx
            .font_cache
            .load_family(&["SF Pro Display"], &Default::default())
            .unwrap();
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
        let line = cx.text_layout_cache().layout_str(
            text,
            font_size,
            &[
                (1, normal),
                (1, bold),
                (1, normal),
                (1, bold),
                (text.len() - 4, normal),
            ],
        );

        scene.push_quad(Quad {
            bounds,
            background: Some(Color::white()),
            ..Default::default()
        });
        line.paint(scene, bounds.origin(), visible_bounds, bounds.height(), cx);
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &V,
        _: &ViewContext<V>,
    ) -> gpui::json::Value {
        todo!()
    }
}
