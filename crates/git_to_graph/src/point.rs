//! Point types for graph paths.

use super::types::PointType;
use serde::{Serialize, Serializer};
use std::cell::RefCell;
use std::rc::Rc;

/// Trait for points in a path.
pub trait Point: std::fmt::Debug {
    /// Get the x coordinate.
    fn get_x(&self) -> i32;
    /// Set the x coordinate.
    fn set_x(&mut self, v: i32);
    /// Get the y coordinate.
    fn get_y(&self) -> i32;
    /// Get the point type.
    fn get_type(&self) -> PointType;
    /// Check if two points are equal.
    fn equal(&self, other: &dyn Point) -> bool;
    /// Convert to string representation.
    #[allow(dead_code)]
    fn to_string(&self) -> String;
}

/// Concrete point implementation used in the graph.
#[derive(Debug, Clone)]
pub struct PointImpl {
    x: i32,
    y: Rc<RefCell<i32>>,
    typ: PointType,
}

impl PointImpl {
    /// Create a new point.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::point::{PointImpl, Point};
    /// use git_to_graph::types::PointType;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let y = Rc::new(RefCell::new(5));
    /// let point = PointImpl::new(10, y, PointType::Pipe);
    /// assert_eq!(point.get_x(), 10);
    /// assert_eq!(point.get_y(), 5);
    /// ```
    pub fn new(x: i32, y: Rc<RefCell<i32>>, typ: PointType) -> Self {
        PointImpl { x, y, typ }
    }
}

impl Point for PointImpl {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn set_x(&mut self, v: i32) {
        self.x = v;
    }

    fn get_y(&self) -> i32 {
        *self.y.borrow()
    }

    fn get_type(&self) -> PointType {
        self.typ
    }

    fn equal(&self, other: &dyn Point) -> bool {
        self.get_x() == other.get_x()
            && self.get_y() == other.get_y()
            && self.get_type() == other.get_type()
    }

    fn to_string(&self) -> String {
        format!(
            "{{{},{},{}}}",
            self.get_x(),
            self.get_y(),
            self.get_type() as u8
        )
    }
}

impl Serialize for PointImpl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (self.x, *self.y.borrow(), self.typ as u8).serialize(serializer)
    }
}

/// Test point implementation (for testing purposes).
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct PointTest {
    x: i32,
    y: i32,
    typ: PointType,
}

impl PointTest {
    /// Create a new test point.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::point::{PointTest, Point};
    /// use git_to_graph::types::PointType;
    ///
    /// let point = PointTest::new(10, 5, PointType::Pipe);
    /// assert_eq!(point.get_x(), 10);
    /// assert_eq!(point.get_y(), 5);
    /// ```
    pub fn new(x: i32, y: i32, typ: PointType) -> Self {
        PointTest { x, y, typ }
    }
}

impl Point for PointTest {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn set_x(&mut self, v: i32) {
        self.x = v;
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn get_type(&self) -> PointType {
        self.typ
    }

    fn equal(&self, other: &dyn Point) -> bool {
        self.get_x() == other.get_x()
            && self.get_y() == other.get_y()
            && self.get_type() == other.get_type()
    }

    fn to_string(&self) -> String {
        format!("{{{},{},{}}}", self.x, self.y, self.typ as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_point_impl() {
        let y = Rc::new(RefCell::new(5));
        let mut point = PointImpl::new(10, y, PointType::Pipe);
        assert_eq!(point.get_x(), 10);
        assert_eq!(point.get_y(), 5);
        point.set_x(15);
        assert_eq!(point.get_x(), 15);
    }

    #[test]
    fn test_point_test() {
        let mut point = PointTest::new(10, 5, PointType::Fork);
        assert_eq!(point.get_x(), 10);
        assert_eq!(point.get_y(), 5);
        assert_eq!(point.get_type(), PointType::Fork);
        point.set_x(20);
        assert_eq!(point.get_x(), 20);
    }

    #[test]
    fn test_point_equal() {
        let y = Rc::new(RefCell::new(5));
        let point1 = PointImpl::new(10, y.clone(), PointType::Pipe);
        let point2 = PointImpl::new(10, y, PointType::Pipe);
        assert!(point1.equal(&point2));
    }
}
