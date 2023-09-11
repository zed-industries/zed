use crate::{
    element::{Element, IntoElement, Layout},
    ViewContext,
};
use anyhow::Result;
use gpui::{
    geometry::{vector::Vector2F, Size},
    text_layout::LineLayout,
    LayoutId,
};
use parking_lot::Mutex;
use std::sync::Arc;
use util::arc_cow::ArcCow;

impl<V: 'static> IntoElement<V> for ArcCow<'static, str> {
    type Element = Text;

    fn into_element(self) -> Self::Element {
        Text { text: self }
    }
}

impl<V: 'static> IntoElement<V> for &'static str {
    type Element = Text;

    fn into_element(self) -> Self::Element {
        Text {
            text: ArcCow::from(self),
        }
    }
}

pub struct Text {
    text: ArcCow<'static, str>,
}

impl<V: 'static> Element<V> for Text {
    type PaintState = Arc<Mutex<Option<TextLayout>>>;

    fn layout(
        &mut self,
        _view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Result<(LayoutId, Self::PaintState)> {
        let fonts = cx.platform().fonts();
        let text_style = cx.text_style();
        let line_height = cx.font_cache().line_height(text_style.font_size);
        let text = self.text.clone();
        let paint_state = Arc::new(Mutex::new(None));

        let layout_id = cx.add_measured_layout_node(Default::default(), {
            let paint_state = paint_state.clone();
            move |_params| {
                let line_layout = fonts.layout_line(
                    text.as_ref(),
                    text_style.font_size,
                    &[(text.len(), text_style.to_run())],
                );

                let size = Size {
                    width: line_layout.width,
                    height: line_height,
                };

                paint_state.lock().replace(TextLayout {
                    line_layout: Arc::new(line_layout),
                    line_height,
                });

                size
            }
        });

        Ok((layout_id?, paint_state))
    }

    fn paint<'a>(
        &mut self,
        _view: &mut V,
        parent_origin: Vector2F,
        layout: &Layout,
        paint_state: &mut Self::PaintState,
        cx: &mut ViewContext<V>,
    ) {
        let bounds = layout.bounds + parent_origin;

        let line_layout;
        let line_height;
        {
            let paint_state = paint_state.lock();
            let paint_state = paint_state
                .as_ref()
                .expect("measurement has not been performed");
            line_layout = paint_state.line_layout.clone();
            line_height = paint_state.line_height;
        }

        let text_style = cx.text_style();
        let line =
            gpui::text_layout::Line::new(line_layout, &[(self.text.len(), text_style.to_run())]);

        // TODO: We haven't added visible bounds to the new element system yet, so this is a placeholder.
        let visible_bounds = bounds;
        line.paint(bounds.origin(), visible_bounds, line_height, cx.legacy_cx);
    }
}

impl<V: 'static> IntoElement<V> for Text {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct TextLayout {
    line_layout: Arc<LineLayout>,
    line_height: f32,
}
