use crate::{AnyWindowHandle, AppCell, Context, PlatformInputHandler, ViewContext, WeakView};
use std::{ops::Range, rc::Weak};

pub struct WindowInputHandler<V>
where
    V: InputHandler,
{
    pub cx: Weak<AppCell>,
    pub window: AnyWindowHandle,
    pub handler: WeakView<V>,
}

impl<V: InputHandler + 'static> PlatformInputHandler for WindowInputHandler<V> {
    fn selected_text_range(&self) -> Option<std::ops::Range<usize>> {
        self.update(|view, cx| view.selected_text_range(cx))
            .flatten()
    }

    fn marked_text_range(&self) -> Option<std::ops::Range<usize>> {
        self.update(|view, cx| view.marked_text_range(cx)).flatten()
    }

    fn text_for_range(&self, range_utf16: std::ops::Range<usize>) -> Option<String> {
        self.update(|view, cx| view.text_for_range(range_utf16, cx))
            .flatten()
    }

    fn replace_text_in_range(
        &mut self,
        replacement_range: Option<std::ops::Range<usize>>,
        text: &str,
    ) {
        self.update(|view, cx| view.replace_text_in_range(replacement_range, text, cx));
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<std::ops::Range<usize>>,
        new_text: &str,
        new_selected_range: Option<std::ops::Range<usize>>,
    ) {
        self.update(|view, cx| {
            view.replace_and_mark_text_in_range(range_utf16, new_text, new_selected_range, cx)
        });
    }

    fn unmark_text(&mut self) {
        self.update(|view, cx| view.unmark_text(cx));
    }

    fn bounds_for_range(&self, range_utf16: std::ops::Range<usize>) -> Option<crate::Bounds<f32>> {
        self.update(|view, cx| view.bounds_for_range(range_utf16, cx))
            .flatten()
    }
}

impl<V: InputHandler + 'static> WindowInputHandler<V> {
    fn update<T>(&self, f: impl FnOnce(&mut V, &mut ViewContext<V>) -> T) -> Option<T> {
        let cx = self.cx.upgrade()?;
        let mut cx = cx.borrow_mut();
        cx.update_window(self.window, |_, cx| self.handler.update(cx, f).ok())
            .ok()?
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
        &self,
        range_utf16: std::ops::Range<usize>,
        cx: &mut ViewContext<Self>,
    ) -> Option<crate::Bounds<f32>>;
}
