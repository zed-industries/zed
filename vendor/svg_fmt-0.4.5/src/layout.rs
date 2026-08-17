use crate::svg::{rectangle, Rectangle};

#[derive(Copy, Clone, Debug)]
pub struct VerticalLayout {
    pub x: f32,
    pub y: f32,
    pub start_y: f32,
    pub width: f32,
}

impl VerticalLayout {
    pub fn new(x: f32, y: f32, width: f32) -> Self {
        VerticalLayout {
            x,
            y,
            start_y: y,
            width,
        }
    }

    pub fn advance(&mut self, by: f32) {
        self.y += by;
    }

    pub fn push_rectangle(&mut self, height: f32) -> Rectangle {
        let rect = rectangle(self.x, self.y, self.width, height);

        self.y += height;

        rect
    }

    pub fn total_rectangle(&self) -> Rectangle {
        rectangle(self.x, self.start_y, self.width, self.y)
    }

    pub fn start_here(&mut self) {
        self.start_y = self.y;
    }
}
