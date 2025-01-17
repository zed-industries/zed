use std::{cell::RefCell, rc::Rc};

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

#[derive(Debug, Clone)]
pub struct TerminalScrollHandle(Rc<RefCell<ScrollHandleState>>);

impl TerminalScrollHandle {
    pub fn new(terminal: &Terminal) -> Self {
        Self(Rc::new(RefCell::new(ScrollHandleState::new(terminal))))
    }

    pub fn update(&self, terminal: &Terminal) {
        *self.0.borrow_mut() = ScrollHandleState::new(terminal);
    }
}

impl ScrollableHandle for TerminalScrollHandle {
    fn content_size(&self) -> Option<ContentSize> {
        let data = self.0.borrow();
        Some(ContentSize {
            size: size(px(0.), data.total_height),
            scroll_adjustment: Some(Point::new(px(0.), px(0.))),
        })
    }

    fn offset(&self) -> Point<Pixels> {
        let data = self.0.borrow();
        Point::new(px(0.), -data.scroll_offset)
    }

    fn set_offset(&self, point: Point<Pixels>) {
        // todo
    }

    fn viewport(&self) -> Bounds<Pixels> {
        let data = self.0.borrow();
        Bounds::new(
            Point::new(px(0.), px(0.)),
            size(px(0.), data.viewport_height),
        )
    }
}
