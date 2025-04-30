use gpui::Pixels;
use smallvec::SmallVec;

/// Calculate nested corner radii and paddings dynamically
/// for nested UI elements
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CornerSolver {
    root_radius: Pixels,
    root_border_width: Pixels,
    root_padding: Pixels,
    /// Nested children with their border width + padding.
    /// This assumes that each additional child is nested within the previous child.
    children: SmallVec<[(Pixels, Pixels); 2]>, // (border_width, padding) pairs
}

impl CornerSolver {
    /// Creates a new CornerSolver with the specified root radius, border width and padding
    fn new(root_radius: Pixels, root_border_width: Pixels, root_padding: Pixels) -> Self {
        CornerSolver {
            root_radius,
            root_border_width,
            root_padding,
            children: SmallVec::new(),
        }
    }

    /// Adds a nested child element with the specified border width and padding
    pub fn add_child(mut self, border_width: Pixels, padding: Pixels) -> Self {
        self.children.push((border_width, padding));
        self
    }

    /// Returns the corner radius of the root element
    pub fn root_corner_radius(&self) -> Pixels {
        self.root_radius
    }

    /// Returns the border width of the root element
    pub fn root_border_width(&self) -> Pixels {
        self.root_border_width
    }

    /// Returns the padding of the root element
    pub fn root_padding(&self) -> Pixels {
        self.root_padding
    }

    /// Calculates the corner radius for a child at the specified index
    ///
    /// Ensures the radius never goes below zero
    ///
    /// Index 0 represents the first level of nesting inside the root element,
    /// regardless of whether any children have been added yet.
    pub fn corner_radius(&self, child_index: usize) -> Pixels {
        let mut radius = self.root_radius;

        // For rounded corners, the radius is reduced by the border width in both
        // horizontal and vertical directions (which is why we multiply by 2)
        let root_border_reduction = self.root_border_width * 2.0;
        radius = Pixels::max(radius - root_border_reduction, Pixels::ZERO);

        if child_index == 0 {
            return radius;
        }

        if child_index > self.children.len() {
            return Pixels::ZERO;
        }

        for i in 0..(child_index) {
            if i >= self.children.len() {
                break;
            }

            let (border_width, _) = self.children[i];
            let border_reduction = border_width * 2.0;
            radius = Pixels::max(radius - border_reduction, Pixels::ZERO);
        }

        radius
    }

    /// Calculates the padding for a child at the specified index
    ///
    /// Returns zero if the index is out of bounds or the calculation would result in negative padding
    pub fn padding(&self, child_index: usize) -> Pixels {
        if child_index >= self.children.len() {
            return Pixels::ZERO;
        }

        // Borders in GPUI work like "inner" borders, they subtract from the padding
        if child_index == 0 {
            let (_, child_padding) = self.children[0];
            Pixels::max(self.root_padding - child_padding, Pixels::ZERO)
        } else {
            let previous_padding = self.padding(child_index - 1);
            let (_, child_padding) = self.children[child_index];
            Pixels::max(previous_padding - child_padding, Pixels::ZERO)
        }
    }
}

/// Calculate nested corner radii and paddings dynamically
///
/// Creates a new CornerSolver with the specified root radius, border width and padding
pub fn corner_solver(
    root_radius: Pixels,
    root_border_width: Pixels,
    root_padding: Pixels,
) -> CornerSolver {
    CornerSolver::new(root_radius, root_border_width, root_padding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::px;

    #[test]
    fn test_single_child_corner_radius() {
        // Test case 1
        let solver = corner_solver(
            px(10.0), // radius
            px(2.0),  // border_width
            px(5.0),  // padding
        );

        // Child radius should be 6 (10 - 2*2)
        assert_eq!(solver.corner_radius(0), px(6.0));

        // Test case 2
        let solver = corner_solver(
            px(10.0), // radius
            px(1.0),  // border_width
            px(3.0),  // padding
        );

        // Child radius should be 8 (10 - 1*2)
        assert_eq!(solver.corner_radius(0), px(8.0));

        // Test nested children
        let solver = corner_solver(
            px(10.0), // root radius
            px(1.0),  // root border_width
            px(4.0),  // root padding
        )
        .add_child(px(1.0), px(2.0)) // child 1: border_width=1, padding=2
        .add_child(px(1.0), px(1.0)); // child 2: border_width=1, padding=1

        // First child radius should be 8 (10 - 1*2)
        assert_eq!(solver.corner_radius(0), px(8.0));

        // Second child radius should be 6 (10 - 1*2 - 1*2)
        assert_eq!(solver.corner_radius(1), px(6.0));
    }

    #[test]
    fn test_solve_four_level_nested_radius() {
        let solver = corner_solver(
            px(20.0), // root radius
            px(2.0),  // root border_width
            px(8.0),  // root padding
        )
        .add_child(px(1.0), px(2.0)) // child 1: border_width=1, padding=2
        .add_child(px(1.0), px(2.0)) // child 2: border_width=1, padding=2
        .add_child(px(1.0), px(1.0)) // child 3: border_width=1, padding=1
        .add_child(px(1.0), px(1.0)); // child 4: border_width=1, padding=1

        assert_eq!(solver.corner_radius(0), px(16.0)); // 20 - 2*2
        assert_eq!(solver.corner_radius(1), px(14.0)); // 16 - 1*2
        assert_eq!(solver.corner_radius(2), px(12.0)); // 14 - 1*2
        assert_eq!(solver.corner_radius(3), px(10.0)); // 12 - 1*2
        assert_eq!(solver.corner_radius(4), px(8.0)); // 10 - 1*2

        assert_eq!(solver.padding(0), px(6.0)); // 8 - 2
        assert_eq!(solver.padding(1), px(4.0)); // 6 - 2
        assert_eq!(solver.padding(2), px(3.0)); // 4 - 1
        assert_eq!(solver.padding(3), px(2.0)); // 3 - 1
    }
}
