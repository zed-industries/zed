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

#[derive(Debug, Clone)]
pub struct TerminalScrollHandle(Rc<RefCell<ScrollHandleState>>);

impl TerminalScrollHandle {
    pub fn new(terminal: &Terminal) -> Self {
        let line_height = terminal.last_content().size.line_height;
        Self(Rc::new(RefCell::new(ScrollHandleState {
            line_height,
            total_height: px(line_height.0 * terminal.total_lines() as f32),
            viewport_height: px(line_height.0 * terminal.viewport_lines() as f32),
            scroll_offset: px(-(terminal.last_content().display_offset as f32 * line_height.0)),
        })))
    }

    pub fn update(
        &self,
        line_height: Pixels,
        total_lines: usize,
        visible_lines: usize,
        display_offset: usize,
    ) {
        let mut data = self.0.borrow_mut();
        data.line_height = line_height;
        data.total_height = px(line_height.0 * total_lines as f32);
        data.viewport_height = px(line_height.0 * visible_lines as f32);
        data.scroll_offset = px(-(display_offset as f32 * line_height.0));
    }
}

impl ScrollableHandle for TerminalScrollHandle {
    fn content_size(&self) -> Option<ContentSize> {
        let data = self.0.borrow();
        Some(ContentSize {
            size: size(px(0.), data.total_height),
            scroll_adjustment: None,
        })
    }

    fn offset(&self) -> Point<Pixels> {
        let data = self.0.borrow();
        Point::new(px(0.), data.scroll_offset)
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
