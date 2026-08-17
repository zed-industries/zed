use super::super::*;

pub(super) fn node_left_top(n: &LayoutNode) -> (f64, f64) {
    (n.x - n.width / 2.0, n.y - n.height / 2.0)
}
