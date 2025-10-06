//! Path operations for git graph rendering.

use super::point::{Point, PointImpl};
use super::types::{PointType, rotate_idx};
use std::cell::RefCell;
use std::rc::Rc;

/// Path defines how to draw a line between parent and child nodes.
#[derive(Debug)]
pub struct Path {
    pub points: Vec<PointImpl>,
    pub color_idx: i32,
}

impl Path {
    /// Create a new empty path.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::path::Path;
    ///
    /// let path = Path::new();
    /// assert!(path.is_empty());
    /// ```
    pub fn new() -> Self {
        Path {
            points: Vec::new(),
            color_idx: 0,
        }
    }

    /// Get the length of the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::path::Path;
    ///
    /// let path = Path::new();
    /// assert_eq!(path.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if the path is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::path::Path;
    ///
    /// let path = Path::new();
    /// assert!(path.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if the path is valid (has at least 2 points).
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::path::Path;
    ///
    /// let path = Path::new();
    /// assert!(!path.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.len() >= 2
    }

    /// Set the color index for this path.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::path::Path;
    ///
    /// let mut path = Path::new();
    /// path.set_color(5);
    /// assert_eq!(path.color_idx, 5);
    /// ```
    pub fn set_color(&mut self, color: i32) {
        self.color_idx = color;
    }

    /// Check if this path is a Fork type.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::path::Path;
    ///
    /// let path = Path::new();
    /// assert!(!path.is_fork());
    /// ```
    pub fn is_fork(&self) -> bool {
        self.is_valid() && self.second().get_type().is_fork()
    }

    /// Check if this path is a MergeTo type.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::path::Path;
    ///
    /// let path = Path::new();
    /// assert!(!path.is_merge_to());
    /// ```
    pub fn is_merge_to(&self) -> bool {
        self.is_valid() && self.second().get_type().is_merge_to()
    }

    /// Get a point at the given index (supports negative indexing).
    pub fn get(&self, idx: i32) -> &PointImpl {
        let real_idx = rotate_idx(idx, self.len());
        &self.points[real_idx]
    }

    /// Get a mutable point at the given index (supports negative indexing).
    pub fn get_mut(&mut self, idx: i32) -> &mut PointImpl {
        let real_idx = rotate_idx(idx, self.len());
        &mut self.points[real_idx]
    }

    /// Get the first point.
    pub fn first(&self) -> &PointImpl {
        self.get(0)
    }

    /// Get the second point.
    pub fn second(&self) -> &PointImpl {
        self.get(1)
    }

    /// Get the last point.
    pub fn last(&self) -> &PointImpl {
        self.get(-1)
    }

    /// Get the second-to-last point.
    pub fn second_to_last(&self) -> &PointImpl {
        self.get(-2)
    }

    /// Get the third-to-last point.
    pub fn third_to_last(&self) -> &PointImpl {
        self.get(-3)
    }

    /// Remove the last point.
    pub fn remove_last(&mut self) {
        self.remove(-1);
    }

    /// Remove the second-to-last point.
    pub fn remove_second_to_last(&mut self) {
        self.remove(-2);
    }

    /// Remove a point at the given index.
    pub fn remove(&mut self, idx: i32) {
        let real_idx = rotate_idx(idx, self.len());
        self.points.remove(real_idx);
    }

    /// Append a point to the path (no duplicate check).
    pub fn append(&mut self, point: PointImpl) {
        self.points.push(point);
    }

    /// Insert a point at the given index.
    pub fn insert(&mut self, idx: i32, point: PointImpl) {
        let real_idx = rotate_idx(idx, self.len());
        self.points.insert(real_idx, point);
    }

    /// Append a point if it's not a duplicate of the last point.
    pub fn no_dup_append(&mut self, point: PointImpl) {
        if self.is_empty() || !self.last().equal(&point) {
            self.append(point);
        }
    }

    /// Append a point if it's not a duplicate, and remove intermediate points with same y.
    pub fn no_dup_append2(&mut self, point: PointImpl) {
        self.no_dup_append(point);
        while self.len() >= 3 && self.last().get_y() == self.third_to_last().get_y() {
            self.remove_second_to_last();
        }
    }

    /// Insert a point if it's not a duplicate of the previous point.
    pub fn no_dup_insert(&mut self, idx: i32, point: PointImpl) {
        let real_idx = rotate_idx(idx, self.len());
        if real_idx == 0 || !self.points[real_idx - 1].equal(&point) {
            self.insert(idx, point);
        }
    }

    /// Get the path x coordinate at a specific lookup index.
    ///
    /// # Examples
    ///
    /// ```
    /// use git_to_graph::path::Path;
    ///
    /// let path = Path::new();
    /// assert_eq!(path.get_height_at_idx(5), -1);
    /// ```
    pub fn get_height_at_idx(&self, lookup_idx: i32) -> i32 {
        let mut height = -1;
        if !self.is_empty() && lookup_idx >= self.first().get_y() {
            for point in &self.points {
                let point_y = point.get_y();
                if point_y >= 0 && point_y <= lookup_idx {
                    height = point.get_x();
                }
            }
        }
        height
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a new point (used throughout the codebase).
///
/// # Examples
///
/// ```
/// use git_to_graph::path::new_point;
/// use git_to_graph::types::PointType;
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// let y = Rc::new(RefCell::new(5));
/// let point = new_point(10, y, PointType::Pipe);
/// ```
pub fn new_point(x: i32, y: Rc<RefCell<i32>>, typ: PointType) -> PointImpl {
    PointImpl::new(x, y, typ)
}

/// Expand a path to ensure there's a point for every row.
///
/// # Examples
///
/// ```
/// use git_to_graph::path::{Path, expand_path};
///
/// let path = Path::new();
/// let expanded = expand_path(&path);
/// assert_eq!(expanded.len(), path.len());
/// ```
pub fn expand_path(path: &Path) -> Path {
    if path.is_empty() {
        return Path::new();
    }

    let mut np = Path::new();
    np.color_idx = path.color_idx;

    // Clone the first point
    let first = path.first();
    np.points.push(PointImpl::new(
        first.get_x(),
        Rc::new(RefCell::new(first.get_y())),
        first.get_type(),
    ));

    for i in 1..path.len() {
        let p1 = &path.points[i - 1];
        let p2 = &path.points[i];

        if p2.get_y() > p1.get_y() + 1 {
            for j in (p1.get_y() + 1)..p2.get_y() {
                np.points.push(PointImpl::new(
                    p1.get_x(),
                    Rc::new(RefCell::new(j)),
                    PointType::Pipe,
                ));
            }
        }

        // Clone p2
        np.points.push(PointImpl::new(
            p2.get_x(),
            Rc::new(RefCell::new(p2.get_y())),
            p2.get_type(),
        ));
    }

    np
}

#[cfg(test)]
mod tests {
    use super::super::types::PointType;
    use super::*;

    #[test]
    fn test_path_basic() {
        let mut path = Path::new();
        assert!(path.is_empty());
        assert!(!path.is_valid());

        let y1 = Rc::new(RefCell::new(0));
        let y2 = Rc::new(RefCell::new(1));
        path.append(new_point(0, y1, PointType::Pipe));
        path.append(new_point(0, y2, PointType::Pipe));

        assert_eq!(path.len(), 2);
        assert!(path.is_valid());
    }

    #[test]
    fn test_path_accessors() {
        let mut path = Path::new();
        let y0 = Rc::new(RefCell::new(0));
        let y1 = Rc::new(RefCell::new(1));
        let y2 = Rc::new(RefCell::new(2));

        path.append(new_point(0, y0, PointType::Pipe));
        path.append(new_point(1, y1, PointType::Fork));
        path.append(new_point(2, y2, PointType::Pipe));

        assert_eq!(path.first().get_x(), 0);
        assert_eq!(path.second().get_x(), 1);
        assert_eq!(path.last().get_x(), 2);
        assert_eq!(path.second_to_last().get_x(), 1);
    }
}
