use crate::{
    AnyWindowHandle, AppCell, Bounds, Context, Pixels, PlatformInputHandler, View, ViewContext,
    WindowContext,
};
use std::{ops::Range, rc::Weak};

pub struct WindowInputHandler {
    pub cx: Weak<AppCell>,
    pub input_handler: Box<dyn InputHandlerView>,
    pub window: AnyWindowHandle,
    pub element_bounds: Bounds<Pixels>,
}

pub trait InputHandlerView {
    fn text_for_range(&self, range: Range<usize>, cx: &mut WindowContext) -> Option<String>;
    fn selected_text_range(&self, cx: &mut WindowContext) -> Option<Range<usize>>;
    fn marked_text_range(&self, cx: &mut WindowContext) -> Option<Range<usize>>;
    fn unmark_text(&self, cx: &mut WindowContext);
    fn replace_text_in_range(
        &self,
        range: Option<Range<usize>>,
        text: &str,
        cx: &mut WindowContext,
    );
    fn replace_and_mark_text_in_range(
        &self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        cx: &mut WindowContext,
    );
    fn bounds_for_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        element_bounds: crate::Bounds<Pixels>,
        cx: &mut WindowContext,
    ) -> Option<crate::Bounds<Pixels>>;
}

impl<V: InputHandler + 'static> InputHandlerView for View<V> {
    fn text_for_range(&self, range: Range<usize>, cx: &mut WindowContext) -> Option<String> {
        self.update(cx, |this, cx| this.text_for_range(range, cx))
    }

    fn selected_text_range(&self, cx: &mut WindowContext) -> Option<Range<usize>> {
        self.update(cx, |this, cx| this.selected_text_range(cx))
    }

    fn marked_text_range(&self, cx: &mut WindowContext) -> Option<Range<usize>> {
        self.update(cx, |this, cx| this.marked_text_range(cx))
    }

    fn unmark_text(&self, cx: &mut WindowContext) {
        self.update(cx, |this, cx| this.unmark_text(cx))
    }

    fn replace_text_in_range(
        &self,
        range: Option<Range<usize>>,
        text: &str,
        cx: &mut WindowContext,
    ) {
        self.update(cx, |this, cx| this.replace_text_in_range(range, text, cx))
    }

    fn replace_and_mark_text_in_range(
        &self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
        cx: &mut WindowContext,
    ) {
        self.update(cx, |this, cx| {
            this.replace_and_mark_text_in_range(range, new_text, new_selected_range, cx)
        })
    }

    fn bounds_for_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        element_bounds: crate::Bounds<Pixels>,
        cx: &mut WindowContext,
    ) -> Option<crate::Bounds<Pixels>> {
        self.update(cx, |this, cx| {
            this.bounds_for_range(range_utf16, element_bounds, cx)
        })
    }
}

impl PlatformInputHandler for WindowInputHandler {
    fn selected_text_range(&self) -> Option<Range<usize>> {
        self.update(|handler, cx| handler.selected_text_range(cx))
            .flatten()
    }

    fn marked_text_range(&self) -> Option<Range<usize>> {
        self.update(|handler, cx| handler.marked_text_range(cx))
            .flatten()
    }

    fn text_for_range(&self, range_utf16: Range<usize>) -> Option<String> {
        self.update(|handler, cx| handler.text_for_range(range_utf16, cx))
            .flatten()
    }

    fn replace_text_in_range(&mut self, replacement_range: Option<Range<usize>>, text: &str) {
        self.update(|handler, cx| handler.replace_text_in_range(replacement_range, text, cx));
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
    ) {
        self.update(|handler, cx| {
            handler.replace_and_mark_text_in_range(range_utf16, new_text, new_selected_range, cx)
        });
    }

    fn unmark_text(&mut self) {
        self.update(|handler, cx| handler.unmark_text(cx));
    }

    fn bounds_for_range(&self, range_utf16: Range<usize>) -> Option<Bounds<Pixels>> {
        self.update(|handler, cx| handler.bounds_for_range(range_utf16, self.element_bounds, cx))
            .flatten()
    }
}

impl WindowInputHandler {
    fn update<R>(
        &self,
        f: impl FnOnce(&dyn InputHandlerView, &mut WindowContext) -> R,
    ) -> Option<R> {
        let cx = self.cx.upgrade()?;
        let mut cx = cx.borrow_mut();
        cx.update_window(self.window, |_, cx| f(&*self.input_handler, cx))
            .ok()
    }
}

pub trait InputHandler: Sized {
    fn text_for_range(&self, range: Range<usize>, cx: &mut ViewContext<Self>) -> Option<String>;
    fn selected_text_range(&self, cx: &mut ViewContext<Self>) -> Option<Range<usize>>;
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
        range_utf16: std::ops::Range<usize>,
        element_bounds: crate::Bounds<Pixels>,
        cx: &mut ViewContext<Self>,
    ) -> Option<crate::Bounds<Pixels>>;
}
