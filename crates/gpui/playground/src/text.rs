use crate::{
    element::{Element, IntoElement, Layout},
    layout_context::LayoutContext,
    paint_context::PaintContext,
};
use anyhow::Result;
use gpui::{geometry::Size, text_layout::LineLayout, RenderContext};
use parking_lot::Mutex;
use std::sync::Arc;

impl<V: 'static, S: Into<ArcCow<'static, str>>> IntoElement<V> for S {
    type Element = Text;

    fn into_element(self) -> Self::Element {
        Text { text: self.into() }
    }
}

pub struct Text {
    text: ArcCow<'static, str>,
}

impl<V: 'static> Element<V> for Text {
    type Layout = Arc<Mutex<Option<TextLayout>>>;

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Result<Layout<V, Self::Layout>> {
        let rem_size = cx.rem_pixels();
        let fonts = cx.platform().fonts();
        let text_style = cx.text_style();
        let line_height = cx.font_cache().line_height(text_style.font_size);
        let text = self.text.clone();
        let layout = Arc::new(Mutex::new(None));

        cx.add_measured_layout_node(Default::default(), layout.clone(), move |params| {
            let line_layout = fonts.layout_line(
                text.as_ref(),
                text_style.font_size,
                &[(text.len(), text_style.to_run())],
            );

            let size = Size {
                width: line_layout.width,
                height: line_height,
            };

            layout.lock().replace(TextLayout {
                line_layout: Arc::new(line_layout),
                line_height,
            });

            size
        })
    }

    fn paint<'a>(
        &mut self,
        view: &mut V,
        layout: &mut Layout<V, Self::Layout>,
        cx: &mut PaintContext<V>,
    ) {
        let element_layout = layout.update(|layout, element_data| element_data.clone());

        let line_layout;
        let line_height;
        {
            let element_layout = element_layout.lock();
            let element_layout = element_layout
                .as_ref()
                .expect("measurement has not been performed");
            line_layout = element_layout.line_layout.clone();
            line_height = element_layout.line_height;
        }

        let text_style = cx.text_style();
        let line =
            gpui::text_layout::Line::new(line_layout, &[(self.text.len(), text_style.to_run())]);

        let origin = layout.bounds(cx).origin();
        // TODO: We haven't added visible bounds to the new element system yet, so this is a placeholder.
        let visible_bounds = layout.bounds(cx);
        line.paint(cx.scene, origin, visible_bounds, line_height, cx.legacy_cx);
    }
}

pub struct TextLayout {
    line_layout: Arc<LineLayout>,
    line_height: f32,
}

pub enum ArcCow<'a, T: ?Sized> {
    Borrowed(&'a T),
    Owned(Arc<T>),
}

impl<'a, T: ?Sized> Clone for ArcCow<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(borrowed) => Self::Borrowed(borrowed),
            Self::Owned(owned) => Self::Owned(owned.clone()),
        }
    }
}

impl<'a, T: ?Sized> From<&'a T> for ArcCow<'a, T> {
    fn from(s: &'a T) -> Self {
        Self::Borrowed(s)
    }
}

impl<T> From<Arc<T>> for ArcCow<'_, T> {
    fn from(s: Arc<T>) -> Self {
        Self::Owned(s)
    }
}

impl From<String> for ArcCow<'_, str> {
    fn from(value: String) -> Self {
        Self::Owned(value.into())
    }
}

impl<T: ?Sized> std::ops::Deref for ArcCow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ArcCow::Borrowed(s) => s,
            ArcCow::Owned(s) => s.as_ref(),
        }
    }
}

impl<T: ?Sized> AsRef<T> for ArcCow<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            ArcCow::Borrowed(borrowed) => borrowed,
            ArcCow::Owned(owned) => owned.as_ref(),
        }
    }
}
