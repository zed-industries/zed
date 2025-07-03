use gpui::Pixels;

/// Calculates the child’s content-corner radius for a single nested level.
///
/// child_content_radius = max(0, parent_radius - parent_border - parent_padding + self_border)
///
/// - parent_radius: outer corner radius of the parent element
/// - parent_border: border width of the parent element
/// - parent_padding: padding of the parent element
/// - self_border: border width of this child element (for content inset)
pub fn inner_corner_radius(
    parent_radius: Pixels,
    parent_border: Pixels,
    parent_padding: Pixels,
    self_border: Pixels,
) -> Pixels {
    (parent_radius - parent_border - parent_padding + self_border).max(Pixels::ZERO)
}

/// Solver for arbitrarily deep nested corner radii.
///
/// Each nested level’s outer border-box radius is:
///   R₀ = max(0, root_radius - root_border - root_padding)
///   Rᵢ = max(0, Rᵢ₋₁ - childᵢ₋₁_border - childᵢ₋₁_padding) for i > 0
pub struct CornerSolver {
    root_radius: Pixels,
    root_border: Pixels,
    root_padding: Pixels,
    children: Vec<(Pixels, Pixels)>, // (border, padding)
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
        if level == 0 {
            return (self.root_radius - self.root_border - self.root_padding).max(Pixels::ZERO);
        }
        if level >= self.children.len() {
            return Pixels::ZERO;
        }
        let mut r = (self.root_radius - self.root_border - self.root_padding).max(Pixels::ZERO);
        for i in 0..level {
            let (b, p) = self.children[i];
            r = (r - b - p).max(Pixels::ZERO);
        }
        r
    }
}
