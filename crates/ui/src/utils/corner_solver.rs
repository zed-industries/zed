use gpui::Pixels;

/// Calculates corner radii for nested elements in both directions.
///
/// ## Forward calculation (parent → child)
/// Given a parent's corner radius, calculates the child's corner radius:
/// ```
/// child_radius = max(0, parent_radius - parent_border - parent_padding + child_border)
/// ```
///
/// ## Inverse calculation (child → parent)
/// Given a child's desired corner radius, calculates the required parent radius:
/// ```
/// parent_radius = child_radius + parent_border + parent_padding - child_border
/// ```
pub struct CornerSolver;

impl CornerSolver {
    /// Calculates the child's corner radius given the parent's properties.
    ///
    /// # Arguments
    /// - `parent_radius`: Outer corner radius of the parent element
    /// - `parent_border`: Border width of the parent element
    /// - `parent_padding`: Padding of the parent element
    /// - `child_border`: Border width of the child element
    pub fn child_radius(
        parent_radius: Pixels,
        parent_border: Pixels,
        parent_padding: Pixels,
        child_border: Pixels,
    ) -> Pixels {
        (parent_radius - parent_border - parent_padding + child_border).max(Pixels::ZERO)
    }

    /// Calculates the required parent radius to achieve a desired child radius.
    ///
    /// # Arguments
    /// - `child_radius`: Desired corner radius for the child element
    /// - `parent_border`: Border width of the parent element
    /// - `parent_padding`: Padding of the parent element
    /// - `child_border`: Border width of the child element
    pub fn parent_radius(
        child_radius: Pixels,
        parent_border: Pixels,
        parent_padding: Pixels,
        child_border: Pixels,
    ) -> Pixels {
        child_radius + parent_border + parent_padding - child_border
    }
}

/// Builder for calculating corner radii across multiple nested levels.
pub struct NestedCornerSolver {
    levels: Vec<Level>,
}

#[derive(Debug, Clone, Copy)]
struct Level {
    border: Pixels,
    padding: Pixels,
}

impl NestedCornerSolver {
    /// Creates a new nested corner solver.
    pub fn new() -> Self {
        Self { levels: Vec::new() }
    }

    /// Adds a level to the nesting hierarchy.
    ///
    /// Levels should be added from outermost to innermost.
    pub fn add_level(mut self, border: Pixels, padding: Pixels) -> Self {
        self.levels.push(Level { border, padding });
        self
    }

    /// Calculates the corner radius at a specific nesting level given the root radius.
    ///
    /// # Arguments
    /// - `root_radius`: The outermost corner radius
    /// - `level`: The nesting level (0 = first child of root, 1 = grandchild, etc.)
    pub fn radius_at_level(&self, root_radius: Pixels, level: usize) -> Pixels {
        let mut radius = root_radius;

        for i in 0..=level.min(self.levels.len().saturating_sub(1)) {
            let current_level = &self.levels[i];
            let next_border = if i < self.levels.len() - 1 {
                self.levels[i + 1].border
            } else {
                Pixels::ZERO
            };

            radius = CornerSolver::child_radius(
                radius,
                current_level.border,
                current_level.padding,
                next_border,
            );
        }

        radius
    }

    /// Calculates the required root radius to achieve a desired radius at a specific level.
    ///
    /// # Arguments
    /// - `target_radius`: The desired corner radius at the target level
    /// - `target_level`: The nesting level where the target radius should be achieved
    pub fn root_radius_for_level(&self, target_radius: Pixels, target_level: usize) -> Pixels {
        if target_level >= self.levels.len() {
            return target_radius;
        }

        let mut radius = target_radius;

        // Work backwards from the target level to the root
        for i in (0..=target_level).rev() {
            let current_level = &self.levels[i];
            let child_border = if i < self.levels.len() - 1 {
                self.levels[i + 1].border
            } else {
                Pixels::ZERO
            };

            radius = CornerSolver::parent_radius(
                radius,
                current_level.border,
                current_level.padding,
                child_border,
            );
        }

        radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_calculation() {
        let parent_radius = Pixels(20.0);
        let parent_border = Pixels(2.0);
        let parent_padding = Pixels(8.0);
        let child_border = Pixels(1.0);

        let child_radius =
            CornerSolver::child_radius(parent_radius, parent_border, parent_padding, child_border);

        assert_eq!(child_radius, Pixels(11.0)); // 20 - 2 - 8 + 1 = 11
    }

    #[test]
    fn test_inverse_calculation() {
        let child_radius = Pixels(11.0);
        let parent_border = Pixels(2.0);
        let parent_padding = Pixels(8.0);
        let child_border = Pixels(1.0);

        let parent_radius =
            CornerSolver::parent_radius(child_radius, parent_border, parent_padding, child_border);

        assert_eq!(parent_radius, Pixels(20.0)); // 11 + 2 + 8 - 1 = 20
    }

    #[test]
    fn test_nested_forward() {
        let solver = NestedCornerSolver::new()
            .add_level(Pixels(2.0), Pixels(8.0)) // Root level
            .add_level(Pixels(1.0), Pixels(4.0)) // First child
            .add_level(Pixels(1.0), Pixels(2.0)); // Second child

        let root_radius = Pixels(20.0);

        assert_eq!(solver.radius_at_level(root_radius, 0), Pixels(11.0)); // 20 - 2 - 8 + 1
        assert_eq!(solver.radius_at_level(root_radius, 1), Pixels(7.0)); // 11 - 1 - 4 + 1
        assert_eq!(solver.radius_at_level(root_radius, 2), Pixels(4.0)); // 7 - 1 - 2 + 0
    }

    #[test]
    fn test_nested_inverse() {
        let solver = NestedCornerSolver::new()
            .add_level(Pixels(2.0), Pixels(8.0)) // Root level
            .add_level(Pixels(1.0), Pixels(4.0)) // First child
            .add_level(Pixels(1.0), Pixels(2.0)); // Second child

        let target_radius = Pixels(4.0);
        let root_radius = solver.root_radius_for_level(target_radius, 2);

        assert_eq!(root_radius, Pixels(20.0));

        // Verify by calculating forward
        assert_eq!(solver.radius_at_level(root_radius, 2), target_radius);
    }
}
