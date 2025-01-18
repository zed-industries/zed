use std::{
    any::Any,
    cell::{Cell, RefCell},
    rc::Rc,
};

use gpui::{size, Bounds, Point};
use terminal::Terminal;
use ui::{px, ContentSize, Pixels, ScrollableHandle};

#[derive(Debug)]
struct ScrollHandleState {
    line_height: Pixels,
    total_height: Pixels,
    viewport_height: Pixels,
    scroll_offset: Pixels,
}

impl ScrollHandleState {
    fn new(terminal: &Terminal) -> Self {
        let line_height = terminal.last_content().size.line_height;
        let viewport_lines = terminal.viewport_lines();
        let total_lines = terminal.total_lines();
        let display_offset = terminal.last_content().display_offset;

        let scroll_offset = total_lines - viewport_lines - display_offset;

        Self {
            line_height,
            total_height: px(total_lines as f32 * line_height.0),
            viewport_height: px(viewport_lines as f32 * line_height.0),
            scroll_offset: px(scroll_offset as f32 * line_height.0),
        }
    }
}

#[derive(Debug)]
pub struct TerminalScrollHandle {
    state: Rc<RefCell<ScrollHandleState>>,
    pub future_display_offset: Rc<Cell<Option<usize>>>,
}

impl TerminalScrollHandle {
    pub fn new(terminal: &Terminal) -> Self {
        Self {
            state: Rc::new(RefCell::new(ScrollHandleState::new(terminal))),
            future_display_offset: Rc::new(Cell::new(None)),
        }
    }

    pub fn update(&self, terminal: &Terminal) {
        *self.state.borrow_mut() = ScrollHandleState::new(terminal);
    }
}

impl ScrollableHandle for TerminalScrollHandle {
    fn content_size(&self) -> Option<ContentSize> {
        Some(ContentSize {
            size: size(px(0.), self.state.borrow().total_height),
            scroll_adjustment: Some(Point::new(px(0.), px(0.))),
        })
    }

    fn offset(&self) -> Point<Pixels> {
        Point::new(px(0.), -self.state.borrow().scroll_offset)
    }

    fn set_offset(&self, point: Point<Pixels>) {
        let total_lines =
            (self.state.borrow().total_height.0 / self.state.borrow().line_height.0) as usize;
        let visible_lines =
            (self.state.borrow().viewport_height.0 / self.state.borrow().line_height.0) as usize;

        let offset_delta = (point.y.0 / self.state.borrow().line_height.0).round() as i32;

        let max_offset = total_lines - visible_lines;
        let display_offset = ((max_offset as i32 + offset_delta) as usize).min(max_offset);

        self.future_display_offset.set(Some(display_offset));
    }

    fn viewport(&self) -> Bounds<Pixels> {
        Bounds::new(
            Point::new(px(0.), px(0.)),
            size(px(0.), self.state.borrow().viewport_height),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
