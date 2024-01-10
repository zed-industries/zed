use crate::{
    AsyncWindowContext, Bounds, Pixels, PlatformInputHandler, View, ViewContext, WindowContext,
};
use std::ops::Range;

/// Implement this trait to allow views to handle textual input when implementing an editor, field, etc.
///
/// Once your view `V` implements this trait, you can use it to construct an [`ElementInputHandler<V>`].
/// This input handler can then be assigned during paint by calling [`WindowContext::handle_input`].
pub trait InputHandler: 'static + Sized {
    fn text_for_range(&mut self, range: Range<usize>, cx: &mut ViewContext<Self>)
        -> Option<String>;
    fn selected_text_range(&mut self, cx: &mut ViewContext<Self>) -> Option<Range<usize>>;
    fn marked_text_range(&self, cx: &mut ViewContext<Self>) -> Option<Range<usize>>;
    fn unmark_text(&mut self, cx: &mut ViewContext<Self>);
    fn replace_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        cx: &mut ViewContext<Self>,
    );
    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        cx: &mut ViewContext<Self>,
    );
    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        element_bounds: Bounds<Pixels>,
        cx: &mut ViewContext<Self>,
    ) -> Option<Bounds<Pixels>>;
}

/// The canonical implementation of [`PlatformInputHandler`]. Call [`WindowContext::handle_input`]
/// with an instance during your element's paint.
pub struct ElementInputHandler<V> {
    view: View<V>,
    element_bounds: Bounds<Pixels>,
    cx: AsyncWindowContext,
}

impl<V: 'static> ElementInputHandler<V> {
    /// Used in [`Element::paint`][element_paint] with the element's bounds and a view context for its
    /// containing view.
    ///
    /// [element_paint]: crate::Element::paint
    pub fn new(element_bounds: Bounds<Pixels>, view: View<V>, cx: &mut WindowContext) -> Self {
        ElementInputHandler {
            view,
            element_bounds,
            cx: cx.to_async(),
        }
    }
}

impl<V: InputHandler> PlatformInputHandler for ElementInputHandler<V> {
    fn selected_text_range(&mut self) -> Option<Range<usize>> {
        self.view
            .update(&mut self.cx, |view, cx| view.selected_text_range(cx))
            .ok()
            .flatten()
    }

    fn marked_text_range(&mut self) -> Option<Range<usize>> {
        self.view
            .update(&mut self.cx, |view, cx| view.marked_text_range(cx))
            .ok()
            .flatten()
    }

    fn text_for_range(&mut self, range_utf16: Range<usize>) -> Option<String> {
        self.view
            .update(&mut self.cx, |view, cx| {
                view.text_for_range(range_utf16, cx)
            })
            .ok()
            .flatten()
    }

    fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str) {
        self.view
            .update(&mut self.cx, |view, cx| {
                view.replace_text_in_range(replacement_range, text, cx)
            })
            .ok();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
    ) {
        self.view
            .update(&mut self.cx, |view, cx| {
                view.replace_and_mark_text_in_range(range_utf16, new_text, new_selected_range, cx)
            })
            .ok();
    }

    fn unmark_text(&mut self) {
        self.view
            .update(&mut self.cx, |view, cx| view.unmark_text(cx))
            .ok();
    }

    fn bounds_for_range(&mut self, range_utf16: Range<usize>) -> Option<Bounds<Pixels>> {
        self.view
            .update(&mut self.cx, |view, cx| {
                view.bounds_for_range(range_utf16, self.element_bounds, cx)
            })
            .ok()
            .flatten()
    }
}
