#![forbid(unsafe_code)]

pub type Unit = euclid::UnknownUnit;

pub type Point = euclid::Point2D<f64, Unit>;
pub type Vector = euclid::Vector2D<f64, Unit>;
pub type Size = euclid::Size2D<f64, Unit>;
pub type Rect = euclid::Rect<f64, Unit>;
pub type Transform = euclid::Transform2D<f64, Unit, Unit>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box2(pub euclid::Box2D<f64, Unit>);

impl Box2 {
    pub fn from_min_max(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self(euclid::Box2D::new(point(min_x, min_y), point(max_x, max_y)))
    }

    pub fn from_center(x: f64, y: f64, width: f64, height: f64) -> Self {
        let hw = width / 2.0;
        let hh = height / 2.0;
        Self(euclid::Box2D::new(
            point(x - hw, y - hh),
            point(x + hw, y + hh),
        ))
    }

    pub fn width(&self) -> f64 {
        self.0.max.x - self.0.min.x
    }

    pub fn height(&self) -> f64 {
        self.0.max.y - self.0.min.y
    }

    pub fn min_x(&self) -> f64 {
        self.0.min.x
    }

    pub fn min_y(&self) -> f64 {
        self.0.min.y
    }

    pub fn max_x(&self) -> f64 {
        self.0.max.x
    }

    pub fn max_y(&self) -> f64 {
        self.0.max.y
    }

    pub fn center(&self) -> (f64, f64) {
        (
            (self.0.min.x + self.0.max.x) / 2.0,
            (self.0.min.y + self.0.max.y) / 2.0,
        )
    }

    pub fn center_point(&self) -> Point {
        let (x, y) = self.center();
        point(x, y)
    }

    pub fn union(&mut self, other: Self) {
        let min_x = self.0.min.x.min(other.0.min.x);
        let min_y = self.0.min.y.min(other.0.min.y);
        let max_x = self.0.max.x.max(other.0.max.x);
        let max_y = self.0.max.y.max(other.0.max.y);
        self.0 = euclid::Box2D::new(point(min_x, min_y), point(max_x, max_y));
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        let d = vector(dx, dy);
        self.0 = euclid::Box2D::new(self.0.min + d, self.0.max + d);
    }

    pub fn pad(&mut self, padding: f64) {
        self.0.min.x -= padding;
        self.0.min.y -= padding;
        self.0.max.x += padding;
        self.0.max.y += padding;
    }

    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.min_x() && x <= self.max_x() && y >= self.min_y() && y <= self.max_y()
    }
}

pub fn point(x: f64, y: f64) -> Point {
    euclid::point2(x, y)
}

pub fn vector(x: f64, y: f64) -> Vector {
    euclid::vec2(x, y)
}
