use crate::{Bounds, InputHandler, Pixels, View, ViewContext, WindowContext};
use std::ops::Range;

/// Implement this trait to allow views to handle textual input when implementing an editor, field, etc.
///
/// Once your view `V` implements this trait, you can use it to construct an [`ElementInputHandler<V>`].
/// This input handler can then be assigned during paint by calling [`WindowContext::handle_input`].
pub trait ViewInputHandler: 'static + Sized {
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
}

impl<V: 'static> ElementInputHandler<V> {
    /// Used in [`Element::paint`][element_paint] with the element's bounds and a view context for its
    /// containing view.
    ///
    /// [element_paint]: crate::Element::paint
    pub fn new(element_bounds: Bounds<Pixels>, view: View<V>) -> Self {
        ElementInputHandler {
            view,
            element_bounds,
        }
    }
}

impl<V: ViewInputHandler> InputHandler for ElementInputHandler<V> {
    fn selected_text_range(&mut self, cx: &mut WindowContext) -> Option<Range<usize>> {
        self.view
            .update(cx, |view, cx| view.selected_text_range(cx))
    }

    fn marked_text_range(&mut self, cx: &mut WindowContext) -> Option<Range<usize>> {
        self.view.update(cx, |view, cx| view.marked_text_range(cx))
    }

    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        cx: &mut WindowContext,
    ) -> Option<String> {
        self.view
            .update(cx, |view, cx| view.text_for_range(range_utf16, cx))
    }

    fn replace_text_in_range(
        &mut self,
        replacement_range: Option<Range<usize>>,
        text: &str,
        cx: &mut WindowContext,
    ) {
        self.view.update(cx, |view, cx| {
            view.replace_text_in_range(replacement_range, text, cx)
        });
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        cx: &mut WindowContext,
    ) {
        self.view.update(cx, |view, cx| {
            view.replace_and_mark_text_in_range(range_utf16, new_text, new_selected_range, cx)
        });
    }

    fn unmark_text(&mut self, cx: &mut WindowContext) {
        self.view.update(cx, |view, cx| view.unmark_text(cx));
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        cx: &mut WindowContext,
    ) -> Option<Bounds<Pixels>> {
        self.view.update(cx, |view, cx| {
            view.bounds_for_range(range_utf16, self.element_bounds, cx)
        })
    }
}
