use std::{cell::RefCell, ops::Range, rc::Rc};

use pathfinder_geometry::rect::RectF;

use crate::{platform::InputHandler, window::WindowContext, AnyView, AppContext};

pub struct WindowInputHandler {
    pub app: Rc<RefCell<AppContext>>,
    pub window_id: usize,
}

impl WindowInputHandler {
    fn read_focused_view<T, F>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&dyn AnyView, &WindowContext) -> T,
    {
        // Input-related application hooks are sometimes called by the OS during
        // a call to a window-manipulation API, like prompting the user for file
        // paths. In that case, the AppContext will already be borrowed, so any
        // InputHandler methods need to fail gracefully.
        //
        // See https://github.com/zed-industries/community/issues/444
        let mut app = self.app.try_borrow_mut().ok()?;
        app.update_window(self.window_id, |cx| {
            let view_id = cx.window.focused_view_id?;
            let view = cx.views.get(&(self.window_id, view_id))?;
            let result = f(view.as_ref(), &cx);
            Some(result)
        })
        .flatten()
    }

    fn update_focused_view<T, F>(&mut self, f: F) -> Option<T>
    where
        F: FnOnce(&mut dyn AnyView, &mut WindowContext, usize) -> T,
    {
        let mut app = self.app.try_borrow_mut().ok()?;
        app.update_window(self.window_id, |cx| {
            let view_id = cx.window.focused_view_id?;
            cx.update_any_view(view_id, |view, cx| f(view, cx, view_id))
        })
        .flatten()
    }
}

impl InputHandler for WindowInputHandler {
    fn text_for_range(&self, range: Range<usize>) -> Option<String> {
        self.read_focused_view(|view, cx| view.text_for_range(range.clone(), cx))
            .flatten()
    }

    fn selected_text_range(&self) -> Option<Range<usize>> {
        self.read_focused_view(|view, cx| view.selected_text_range(cx))
            .flatten()
    }

    fn replace_text_in_range(&mut self, range: Option<Range<usize>>, text: &str) {
        self.update_focused_view(|view, cx, view_id| {
            view.replace_text_in_range(range, text, cx, view_id);
        });
    }

    fn marked_text_range(&self) -> Option<Range<usize>> {
        self.read_focused_view(|view, cx| view.marked_text_range(cx))
            .flatten()
    }

    fn unmark_text(&mut self) {
        self.update_focused_view(|view, cx, view_id| {
            view.unmark_text(cx, view_id);
        });
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        new_text: &str,
        new_selected_range: Option<Range<usize>>,
    ) {
        self.update_focused_view(|view, cx, view_id| {
            view.replace_and_mark_text_in_range(range, new_text, new_selected_range, cx, view_id);
        });
    }

    fn rect_for_range(&self, range_utf16: Range<usize>) -> Option<RectF> {
        self.app
            .borrow()
            .read_window(self.window_id, |cx| cx.rect_for_text_range(range_utf16))
            .flatten()
    }
}
