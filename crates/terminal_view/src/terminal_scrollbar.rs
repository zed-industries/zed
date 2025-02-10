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
    total_lines: usize,
    viewport_lines: usize,
    display_offset: usize,
}

impl ScrollHandleState {
    fn new(terminal: &Terminal) -> Self {
        Self {
            line_height: terminal.last_content().size.line_height,
            total_lines: terminal.total_lines(),
            viewport_lines: terminal.viewport_lines(),
            display_offset: terminal.last_content().display_offset,
        }
    }
}

#[derive(Debug, Clone)]
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
        let state = self.state.borrow();
        Some(ContentSize {
            size: size(px(0.), px(state.total_lines as f32 * state.line_height.0)),
            scroll_adjustment: Some(Point::new(px(0.), px(0.))),
        })
    }

    fn offset(&self) -> Point<Pixels> {
        let state = self.state.borrow();
        let scroll_offset = state.total_lines - state.viewport_lines - state.display_offset;
        Point::new(
            px(0.),
            -px(scroll_offset as f32 * self.state.borrow().line_height.0),
        )
    }

    fn set_offset(&self, point: Point<Pixels>) {
        let state = self.state.borrow();
        let offset_delta = (point.y.0 / state.line_height.0).round() as i32;

        let max_offset = state.total_lines - state.viewport_lines;
        let display_offset = ((max_offset as i32 + offset_delta) as usize).min(max_offset);

        self.future_display_offset.set(Some(display_offset));
    }

    fn viewport(&self) -> Bounds<Pixels> {
        let state = self.state.borrow();
        Bounds::new(
            Point::new(px(0.), px(0.)),
            size(
                px(0.),
                px(state.viewport_lines as f32 * state.line_height.0),
            ),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
