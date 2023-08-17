use crate::element::{Element, ElementMetadata, EventHandler, IntoElement};
use gpui::{geometry::Size, text_layout::LineLayout, RenderContext};
use parking_lot::Mutex;
use std::sync::Arc;

impl<V: 'static, S: Into<ArcCow<'static, str>>> IntoElement<V> for S {
    type Element = Text<V>;

    fn into_element(self) -> Self::Element {
        Text {
            text: self.into(),
            metadata: Default::default(),
        }
    }
}

pub struct Text<V> {
    text: ArcCow<'static, str>,
    metadata: ElementMetadata<V>,
}

impl<V: 'static> Element<V> for Text<V> {
    type Layout = Arc<Mutex<Option<TextLayout>>>;

    fn style_mut(&mut self) -> &mut crate::style::ElementStyle {
        &mut self.metadata.style
    }

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut gpui::LayoutContext<V>,
    ) -> anyhow::Result<(taffy::tree::NodeId, Self::Layout)> {
        let rem_size = cx.rem_pixels();
        let fonts = cx.platform().fonts();
        let text_style = cx.text_style();
        let line_height = cx.font_cache().line_height(text_style.font_size);
        let layout_engine = cx.layout_engine().expect("no layout engine present");
        let text = self.text.clone();
        let layout = Arc::new(Mutex::new(None));

        let node_id = layout_engine.add_measured_node(self.metadata.style.to_taffy(rem_size), {
            let layout = layout.clone();
            move |params| {
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
            }
        })?;

        Ok((node_id, layout))
    }

    fn paint<'a>(
        &mut self,
        layout: crate::element::Layout<Arc<Mutex<Option<TextLayout>>>>,
        view: &mut V,
        cx: &mut crate::element::PaintContext<V>,
    ) -> anyhow::Result<()> {
        let element_layout_lock = layout.from_element.lock();
        let element_layout = element_layout_lock
            .as_ref()
            .expect("layout has not been performed");
        let line_layout = element_layout.line_layout.clone();
        let line_height = element_layout.line_height;
        drop(element_layout_lock);

        let text_style = cx.text_style();
        let line =
            gpui::text_layout::Line::new(line_layout, &[(self.text.len(), text_style.to_run())]);
        line.paint(
            cx.scene,
            layout.from_engine.bounds.origin(),
            layout.from_engine.bounds,
            line_height,
            cx.legacy_cx,
        );
        Ok(())
    }

    fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>> {
        &mut self.metadata.handlers
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
