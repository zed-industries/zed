use crate::{Bounds, Half};
use std::{
    cmp,
    fmt::Debug,
    ops::{Add, Sub},
};

#[derive(Debug)]
pub struct BoundsTree<U: Default + Clone + Debug> {
    root: Option<usize>,
    nodes: Vec<Node<U>>,
    stack: Vec<usize>,
}

impl<U> BoundsTree<U>
where
    U: Clone + Debug + PartialOrd + Add<U, Output = U> + Sub<Output = U> + Half + Default,
{
    pub fn new() -> Self {
        BoundsTree {
            root: None,
            nodes: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn insert(&mut self, new_bounds: Bounds<U>) -> u32 {
        // If the tree is empty, make the root the new leaf.
        if self.root.is_none() {
            let new_node = self.push_leaf(new_bounds, 1);
            self.root = Some(new_node);
            return 1;
        }

        // Search for the best place to add the new leaf based on heuristics.
        let mut max_intersecting_ordering = 0;
        let mut index = self.root.unwrap();
        while let Node::Internal {
            left,
            right,
            bounds: node_bounds,
            ..
        } = self.node_mut(index)
        {
            let left = *left;
            let right = *right;
            *node_bounds = node_bounds.union(&new_bounds);
            self.stack.push(index);

            // Descend to the best-fit child, based on which one would increase
            // the surface area the least. This attempts to keep the tree balanced
            // in terms of surface area. If there is an intersection with the other child,
            // add its keys to the intersections vector.
            let left_cost = new_bounds.union(self.node(left).bounds()).half_perimeter();
            let right_cost = new_bounds.union(self.node(right).bounds()).half_perimeter();
            if left_cost < right_cost {
                max_intersecting_ordering =
                    self.find_max_ordering(right, &new_bounds, max_intersecting_ordering);
                index = left;
            } else {
                max_intersecting_ordering =
                    self.find_max_ordering(left, &new_bounds, max_intersecting_ordering);
                index = right;
            }
        }

        // We've found a leaf ('index' now refers to a leaf node).
        // We'll insert a new parent node above the leaf and attach our new leaf to it.
        let sibling = index;

        // Check for collision with the located leaf node
        let Node::Leaf {
            bounds: sibling_bounds,
            order: sibling_ordering,
            ..
        } = self.node(index)
        else {
            unreachable!();
        };
        if sibling_bounds.intersects(&new_bounds) {
            max_intersecting_ordering = cmp::max(max_intersecting_ordering, *sibling_ordering);
        }

        let ordering = max_intersecting_ordering + 1;
        let new_node = self.push_leaf(new_bounds, ordering);
        let new_parent = self.push_internal(sibling, new_node);

        // If there was an old parent, we need to update its children indices.
        if let Some(old_parent) = self.stack.last().copied() {
            let Node::Internal { left, right, .. } = self.node_mut(old_parent) else {
                unreachable!();
            };

            if *left == sibling {
                *left = new_parent;
            } else {
                *right = new_parent;
            }
        } else {
            // If the old parent was the root, the new parent is the new root.
            self.root = Some(new_parent);
        }

        for node_index in self.stack.drain(..) {
            let Node::Internal { max_ordering, .. } = &mut self.nodes[node_index] else {
                unreachable!()
            };
            *max_ordering = cmp::max(*max_ordering, ordering);
        }

        ordering
    }

    fn find_max_ordering(&self, index: usize, bounds: &Bounds<U>, mut max_ordering: u32) -> u32 {
        match self.node(index) {
            Node::Leaf {
                bounds: node_bounds,
                order: ordering,
                ..
            } => {
                if bounds.intersects(node_bounds) {
                    max_ordering = cmp::max(*ordering, max_ordering);
                }
            }
            Node::Internal {
                left,
                right,
                bounds: node_bounds,
                max_ordering: node_max_ordering,
                ..
            } => {
                if bounds.intersects(node_bounds) && max_ordering < *node_max_ordering {
                    let left_max_ordering = self.node(*left).max_ordering();
                    let right_max_ordering = self.node(*right).max_ordering();
                    if left_max_ordering > right_max_ordering {
                        max_ordering = self.find_max_ordering(*left, bounds, max_ordering);
                        max_ordering = self.find_max_ordering(*right, bounds, max_ordering);
                    } else {
                        max_ordering = self.find_max_ordering(*right, bounds, max_ordering);
                        max_ordering = self.find_max_ordering(*left, bounds, max_ordering);
                    }
                }
            }
        }
        max_ordering
    }

    fn push_leaf(&mut self, bounds: Bounds<U>, order: u32) -> usize {
        self.nodes.push(Node::Leaf { bounds, order });
        self.nodes.len() - 1
    }

    fn push_internal(&mut self, left: usize, right: usize) -> usize {
        let left_node = self.node(left);
        let right_node = self.node(right);
        let new_bounds = left_node.bounds().union(right_node.bounds());
        let max_ordering = cmp::max(left_node.max_ordering(), right_node.max_ordering());
        self.nodes.push(Node::Internal {
            bounds: new_bounds,
            left,
            right,
            max_ordering,
        });
        self.nodes.len() - 1
    }

    #[inline(always)]
    fn node(&self, index: usize) -> &Node<U> {
        &self.nodes[index]
    }

    #[inline(always)]
    fn node_mut(&mut self, index: usize) -> &mut Node<U> {
        &mut self.nodes[index]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Primitive<U: Clone + Default + Debug> {
    bounds: Bounds<U>,
    order: u32,
}

#[derive(Debug)]
enum Node<U: Clone + Default + Debug> {
    Leaf {
        bounds: Bounds<U>,
        order: u32,
    },
    Internal {
        left: usize,
        right: usize,
        bounds: Bounds<U>,
        max_ordering: u32,
    },
}

impl<U> Node<U>
where
    U: Clone + Default + Debug,
{
    fn bounds(&self) -> &Bounds<U> {
        match self {
            Node::Leaf { bounds, .. } => bounds,
            Node::Internal { bounds, .. } => bounds,
        }
    }

    fn max_ordering(&self) -> u32 {
        match self {
            Node::Leaf {
                order: ordering, ..
            } => *ordering,
            Node::Internal { max_ordering, .. } => *max_ordering,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Size};
    use rand::{Rng, SeedableRng};
    use std::{fs, path::Path};

    #[test]
    fn test_bounds_insertion_with_two_bounds() {
        let mut tree = BoundsTree::new();
        let bounds1 = Bounds {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };
        let bounds2 = Bounds {
            origin: Point { x: 5.0, y: 5.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };

        // Insert the first Bounds.
        assert_eq!(tree.insert(bounds1), 1);

        // Insert the second Bounds, which overlaps with the first.
        assert_eq!(tree.insert(bounds2), 2);
    }

    #[test]
    fn test_adjacent_bounds() {
        let mut tree = BoundsTree::new();
        let bounds1 = Bounds {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };
        let bounds2 = Bounds {
            origin: Point { x: 10.0, y: 0.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };

        // Insert the first bounds.
        assert_eq!(tree.insert(bounds1), 1);

        // Insert the second bounds, which is adjacent to the first but not overlapping.
        assert_eq!(tree.insert(bounds2), 1);
    }

    #[test]
    fn test_random_iterations() {
        let max_bounds = 100;

        let mut actual_intersections: Vec<usize> = Vec::new();
        for seed in 1..=1000 {
            // let seed = 44;
            let debug = false;
            if debug {
                let svg_path = Path::new("./svg");
                if svg_path.exists() {
                    fs::remove_dir_all("./svg").unwrap();
                }
                fs::create_dir_all("./svg").unwrap();
            }

            dbg!(seed);

            let mut tree = BoundsTree::new();
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
            let mut expected_quads: Vec<Primitive<f32>> = Vec::new();

            let mut insert_time = std::time::Duration::ZERO;

            // Insert a random number of random Bounds into the tree.
            let num_bounds = rng.gen_range(1..=max_bounds);
            for quad_id in 0..num_bounds {
                let min_x: f32 = rng.gen_range(-100.0..100.0);
                let min_y: f32 = rng.gen_range(-100.0..100.0);
                let max_x: f32 = rng.gen_range(min_x..min_x + 50.0);
                let max_y: f32 = rng.gen_range(min_y..min_y + 50.0);
                let bounds = Bounds {
                    origin: Point { x: min_x, y: min_y },
                    size: Size {
                        width: max_x - min_x,
                        height: max_y - min_y,
                    },
                };

                let expected_ordering = expected_quads
                    .iter()
                    .filter_map(|quad| {
                        (quad.bounds.origin.x < bounds.origin.x + bounds.size.width
                            && quad.bounds.origin.x + quad.bounds.size.width > bounds.origin.x
                            && quad.bounds.origin.y < bounds.origin.y + bounds.size.height
                            && quad.bounds.origin.y + quad.bounds.size.height > bounds.origin.y)
                            .then_some(quad.order)
                    })
                    .max()
                    .unwrap_or(0)
                    + 1;
                expected_quads.push(Primitive {
                    bounds,
                    order: expected_ordering,
                });
                if debug {
                    println!("inserting {} with Bounds: {:?}", quad_id, bounds);
                    draw_bounds(
                        format!("./svg/expected_bounds_after_{}.svg", quad_id),
                        &expected_quads,
                    );
                }

                // Insert the Bounds into the tree and collect intersections.
                actual_intersections.clear();
                let t0 = std::time::Instant::now();
                let actual_ordering = tree.insert(bounds);
                insert_time += t0.elapsed();
                assert_eq!(actual_ordering, expected_ordering);

                if debug {
                    tree.draw(format!("./svg/bounds_tree_after_{}.svg", quad_id));
                }
            }
        }
    }

    fn draw_bounds(svg_path: impl AsRef<Path>, bounds: &[Primitive<f32>]) {
        let mut svg_content = String::from(
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="-100 -100 200 200" style="border:1px solid black;">"#,
        );

        for quad in bounds {
            svg_content.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" style="fill:none;stroke:black;stroke-width:1" />"#,
                quad.bounds.origin.x,
                quad.bounds.origin.y,
                quad.bounds.size.width,
                quad.bounds.size.height
            ));
            svg_content.push_str(&format!(
                r#"<text x="{}" y="{}" font-size="3" text-anchor="middle" alignment-baseline="central"></text>"#,
                quad.bounds.origin.x + quad.bounds.size.width / 2.0,
                quad.bounds.origin.y + quad.bounds.size.height / 2.0,
            ));
        }

        svg_content.push_str("</svg>");
        fs::write(svg_path, &svg_content).unwrap();
    }

    impl BoundsTree<f32> {
        fn draw(&self, svg_path: impl AsRef<std::path::Path>) {
            let root_bounds = self.node(self.root.unwrap()).bounds();

            let mut svg_content = format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" style="border:1px solid black;" viewBox="{} {} {} {}">"#,
                root_bounds.origin.x,
                root_bounds.origin.y,
                root_bounds.size.width,
                root_bounds.size.height
            );

            fn draw_node(svg_content: &mut String, nodes: &[Node<f32>], index: usize) {
                match &nodes[index] {
                    Node::Internal {
                        bounds,
                        left,
                        right,
                        ..
                    } => {
                        svg_content.push_str(&format!(
                            r#"<rect x="{}" y="{}" width="{}" height="{}" style="fill:rgba({},{},{},0.1);stroke:rgba({},{},{},1);stroke-width:1" />"#,
                            bounds.origin.x,
                            bounds.origin.y,
                            bounds.size.width,
                            bounds.size.height,
                            (index * 50) % 255, // Red component
                            (index * 120) % 255, // Green component
                            (index * 180) % 255, // Blue component
                            (index * 50) % 255, // Red stroke
                            (index * 120) % 255, // Green stroke
                            (index * 180) % 255  // Blue stroke
                        ));
                        draw_node(svg_content, nodes, *left);
                        draw_node(svg_content, nodes, *right);
                    }
                    Node::Leaf { bounds, .. } => {
                        svg_content.push_str(&format!(
                            r#"<rect x="{}" y="{}" width="{}" height="{}" style="fill:none;stroke:black;stroke-width:1" />"#,
                            bounds.origin.x,
                            bounds.origin.y,
                            bounds.size.width,
                            bounds.size.height
                        ));
                    }
                }
            }

            if let Some(root) = self.root {
                draw_node(&mut svg_content, &self.nodes, root);
            }

            svg_content.push_str("</svg>");
            std::fs::write(svg_path, &svg_content).unwrap();
        }
    }
}
