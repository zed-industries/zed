use gpui::Pixels;

/// Calculates the child’s outer border-box corner radius for a single nested level.
///
/// child_radius = max(0, parent_radius − parent_border − parent_padding)
pub fn inner_corner_radius(
    parent_radius: Pixels,
    parent_border: Pixels,
    parent_padding: Pixels,
) -> Pixels {
    (parent_radius - parent_border - parent_padding).max(Pixels::ZERO)
}

/// Solver for arbitrarily deep nested corner radii.
///
/// For each level i:
///   Rᵢ = max(0, Rᵢ₋₁ − Bᵢ₋₁ − Pᵢ₋₁)
/// where R₀ = root outer radius, B₀/P₀ = root border/padding,
/// and children store (border, padding) for subsequent levels.
pub struct CornerSolver {
    root_radius: Pixels,
    root_border: Pixels,
    root_padding: Pixels,
    children: Vec<(Pixels, Pixels)>,
}

impl CornerSolver {
    pub fn new(root_radius: Pixels, root_border: Pixels, root_padding: Pixels) -> Self {
        Self {
            root_radius,
            root_border,
            root_padding,
            children: Vec::new(),
        }
    }

    pub fn add_child(mut self, border: Pixels, padding: Pixels) -> Self {
        self.children.push((border, padding));
        self
    }

    pub fn corner_radius(&self, level: usize) -> Pixels {
        let mut r = inner_corner_radius(self.root_radius, self.root_border, self.root_padding);
        if level == 0 {
            return r;
        }
        if level >= self.children.len() {
            return Pixels::ZERO;
        }
        for i in 0..level {
            let (b, p) = self.children[i];
            r = inner_corner_radius(r, b, p);
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::px;

    #[test]
    fn test_inner_corner_radius() {
        assert_eq!(inner_corner_radius(px(10.0), px(2.0), px(3.0)), px(5.0));
        assert_eq!(inner_corner_radius(px(4.0), px(2.0), px(3.0)), px(0.0));
    }

    #[test]
    fn test_corner_solver_single() {
        let solver = CornerSolver::new(px(10.0), px(2.0), px(3.0));
        assert_eq!(solver.corner_radius(0), px(5.0));
    }

    #[test]
    fn test_corner_solver_nested() {
        let solver = CornerSolver::new(px(20.0), px(2.0), px(3.0))
            .add_child(px(1.0), px(2.0))
            .add_child(px(1.0), px(1.0))
            .add_child(px(2.0), px(2.0));

        assert_eq!(solver.corner_radius(0), px(15.0));
        assert_eq!(solver.corner_radius(1), px(12.0));
        assert_eq!(solver.corner_radius(2), px(10.0));
        assert_eq!(solver.corner_radius(3), px(0.0));
    }
}
