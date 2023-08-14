#![allow(dead_code, unused_variables)]
use element::{AnyElement, Element};
use frame::frame;
use log::LevelFilter;
use simplelog::SimpleLogger;
use taffy::tree::NodeId;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);

        // cx.add_window(
        //     Default::default(),
        //     // |_| view(|_| Playground::new()),
        // );
    });
}

use themes::{rose_pine, ThemeColors};

mod adapter;
mod color;
mod element;
mod frame;
mod style;
mod themes;

pub struct Playground<V: 'static>(AnyElement<V>);

impl<V: 'static> gpui::Element<V> for Playground<V> {
    type LayoutState = NodeId;

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        view: &mut V,
        cx: &mut gpui::LayoutContext<V>,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        todo!()
    }

    fn paint(
        &mut self,
        scene: &mut gpui::SceneBuilder,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut gpui::PaintContext<V>,
    ) -> Self::PaintState {
        todo!()
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> Option<gpui::geometry::rect::RectF> {
        todo!()
    }

    fn debug(
        &self,
        bounds: gpui::geometry::rect::RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &gpui::ViewContext<V>,
    ) -> gpui::serde_json::Value {
        todo!()
    }
}

impl<V> Playground<V> {
    pub fn new() -> Self {
        Self(workspace(&rose_pine::moon()).into_any())
    }
}

fn workspace<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
    frame()
}
//     todo!()
//     // column()
//     // .size(auto())
//     // .fill(theme.base(0.5))
//     // .text_color(theme.text(0.5))
//     // .child(title_bar(theme))
//     // .child(stage(theme))
//     // .child(status_bar(theme))
// }

// fn title_bar<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
//     row()
//         .fill(theme.base(0.2))
//         .justify(0.)
//         .width(auto())
//         .child(text("Zed Playground"))
// }

// fn stage<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
//     row().fill(theme.surface(0.9))
// }

// fn status_bar<V: 'static>(theme: &ThemeColors) -> impl Element<V> {
//     row().fill(theme.surface(0.1))
// }
