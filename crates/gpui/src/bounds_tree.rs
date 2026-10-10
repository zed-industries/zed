use crate::Bounds;
use std::{fmt::Debug, ops::Add};

// Glyphs and decorations are usually a few dozen scaled pixels across. A 64-pixel
// cell keeps their duplication small without making every text line one long list.
const CELL_SIZE: f64 = 64.0;
const MAX_CELLS: usize = 256;
const NONE: usize = usize::MAX;

/// Assigns one plus the greatest order of previously inserted intersecting bounds.
#[derive(Debug)]
pub(crate) struct BoundsTree<U: Clone + Debug + Default + PartialEq> {
    columns: usize,
    rows: usize,
    cells: Vec<Cell>,
    links: Vec<Link>,
    bounds: Vec<(Bounds<U>, u32)>,
    exceptional: Vec<usize>,
    maximum: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    head: usize,
    cover: u32,
    maximum: u32,
}

impl Cell {
    const EMPTY: Self = Self {
        head: NONE,
        cover: 0,
        maximum: 0,
    };
}

#[derive(Debug)]
struct Link {
    bounds: usize,
    next: usize,
    maximum: u32,
}

#[derive(Clone, Copy)]
struct Extent {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
}

fn cell_at(coordinate: f64, count: usize) -> usize {
    ((coordinate / CELL_SIZE) as usize).min(count - 1)
}

fn cell_start(index: usize) -> f64 {
    if index == 0 {
        f64::NEG_INFINITY
    } else {
        index as f64 * CELL_SIZE
    }
}

fn cell_end(index: usize, count: usize) -> f64 {
    if index + 1 == count {
        f64::INFINITY
    } else {
        (index + 1) as f64 * CELL_SIZE
    }
}

impl<U> BoundsTree<U>
where
    U: Clone + Debug + Default + PartialEq + PartialOrd + Add<Output = U> + Into<f64>,
{
    pub fn clear(&mut self) {
        self.cells.fill(Cell::EMPTY);
        self.links.clear();
        self.bounds.clear();
        self.exceptional.clear();
        self.maximum = None;
    }

    fn extent(bounds: &Bounds<U>) -> Option<Extent> {
        let left = bounds.origin.x.clone().into();
        let top = bounds.origin.y.clone().into();
        let right = (bounds.origin.x.clone() + bounds.size.width.clone()).into();
        let bottom = (bounds.origin.y.clone() + bounds.size.height.clone()).into();
        // Zero and negative sizes can still intersect under Bounds::intersects.
        // NaNs and infinite origins also need the exact predicate, not cell covers.
        (left.is_finite() && top.is_finite() && right > left && bottom > top).then_some(Extent {
            left,
            top,
            right,
            bottom,
        })
    }

    pub fn insert(&mut self, bounds: Bounds<U>) -> u32 {
        let order = self.find_max_ordering(&bounds) + 1;
        self.register(bounds, order);
        order
    }

    fn register(&mut self, bounds: Bounds<U>, order: u32) {
        let extent = Self::extent(&bounds);
        let index = self.bounds.len();
        self.bounds.push((bounds, order));
        if self
            .maximum
            .is_none_or(|maximum| self.bounds[maximum].1 < order)
        {
            self.maximum = Some(index);
        }
        if let Some(extent) = extent {
            let needed = |end: f64, current: usize| {
                let cells = ((end / CELL_SIZE).ceil() as usize).clamp(1, MAX_CELLS);
                current.max(cells.next_power_of_two().min(MAX_CELLS))
            };
            let columns = needed(extent.right, self.columns);
            let rows = needed(extent.bottom, self.rows);
            if columns != self.columns || rows != self.rows {
                self.columns = columns;
                self.rows = rows;
                self.cells.clear();
                self.cells.resize(columns * rows, Cell::EMPTY);
                self.links.clear();
                self.exceptional.clear();
                // Growing changes the formerly unbounded edge cells.
                for index in 0..self.bounds.len() {
                    self.add_to_cells(index);
                }
                return;
            }
        }
        self.add_to_cells(index);
    }

    fn add_to_cells(&mut self, index: usize) {
        let (bounds, order) = &self.bounds[index];
        let Some(extent) = Self::extent(bounds) else {
            self.exceptional.push(index);
            return;
        };
        for row in cell_at(extent.top, self.rows)..=cell_at(extent.bottom, self.rows) {
            for column in cell_at(extent.left, self.columns)..=cell_at(extent.right, self.columns) {
                let cell = &mut self.cells[row * self.columns + column];
                cell.maximum = cell.maximum.max(*order);
                if extent.top <= cell_start(row)
                    && extent.bottom >= cell_end(row, self.rows)
                    && extent.left <= cell_start(column)
                    && extent.right >= cell_end(column, self.columns)
                {
                    cell.cover = cell.cover.max(*order);
                } else {
                    let maximum = self
                        .links
                        .get(cell.head)
                        .map_or(*order, |link| link.maximum.max(*order));
                    self.links.push(Link {
                        bounds: index,
                        next: cell.head,
                        maximum,
                    });
                    cell.head = self.links.len() - 1;
                }
            }
        }
    }

    fn find_max_ordering(&self, query: &Bounds<U>) -> u32 {
        if let Some(index) = self.maximum
            && query.intersects(&self.bounds[index].0)
        {
            return self.bounds[index].1;
        }
        let Some(extent) = Self::extent(query) else {
            return self
                .bounds
                .iter()
                .filter(|(bounds, _)| query.intersects(bounds))
                .map(|(_, order)| *order)
                .max()
                .unwrap_or(0);
        };
        let mut maximum = self
            .exceptional
            .iter()
            .map(|&index| &self.bounds[index])
            .filter(|(bounds, _)| query.intersects(bounds))
            .map(|(_, order)| *order)
            .max()
            .unwrap_or(0);
        for row in cell_at(extent.top, self.rows)..=cell_at(extent.bottom, self.rows) {
            for column in cell_at(extent.left, self.columns)..=cell_at(extent.right, self.columns) {
                let cell = &self.cells[row * self.columns + column];
                if cell.maximum <= maximum {
                    continue;
                }
                // Merely touching a cell edge does not intersect its covers.
                if extent.top < cell_end(row, self.rows)
                    && extent.bottom > cell_start(row)
                    && extent.left < cell_end(column, self.columns)
                    && extent.right > cell_start(column)
                {
                    maximum = maximum.max(cell.cover);
                }
                let mut cursor = cell.head;
                while let Some(link) = self.links.get(cursor) {
                    if link.maximum <= maximum {
                        break;
                    }
                    let (bounds, order) = &self.bounds[link.bounds];
                    if *order > maximum && query.intersects(bounds) {
                        maximum = *order;
                    }
                    cursor = link.next;
                }
            }
        }
        maximum
    }
}

impl<U: Clone + Debug + Default + PartialEq> Default for BoundsTree<U> {
    fn default() -> Self {
        Self {
            columns: 1,
            rows: 1,
            cells: vec![Cell::EMPTY],
            links: Vec::new(),
            bounds: Vec::new(),
            exceptional: Vec::new(),
            maximum: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Half, Point, Size};
    use rand::{Rng, SeedableRng};
    use std::ops::Sub;

    fn rectangle(x: f32, y: f32, width: f32, height: f32) -> Bounds<f32> {
        Bounds {
            origin: Point { x, y },
            size: Size { width, height },
        }
    }

    fn assert_orders<U>(tree: &mut BoundsTree<U>, bounds: &[Bounds<U>], label: &str)
    where
        U: Clone
            + Debug
            + Default
            + PartialEq
            + PartialOrd
            + Add<U, Output = U>
            + Sub<Output = U>
            + Half
            + Into<f64>,
    {
        let mut original: Vec<(Bounds<U>, u32)> = Vec::new();
        for (index, bounds) in bounds.iter().enumerate() {
            // Derive every expected order from earlier oracle results, never tree results.
            let expected = original
                .iter()
                .filter_map(|(previous, order)| previous.intersects(bounds).then_some(*order))
                .max()
                .unwrap_or(0)
                + 1;
            assert_eq!(
                tree.insert(bounds.clone()),
                expected,
                "{label}: insertion {index}"
            );
            original.push((bounds.clone(), expected));
        }
        for (index, query) in bounds.iter().enumerate() {
            let expected = original
                .iter()
                .filter_map(|(previous, order)| previous.intersects(query).then_some(*order))
                .max()
                .unwrap_or(0);
            assert_eq!(
                tree.find_max_ordering(query),
                expected,
                "{label}: query {index}"
            );
        }
    }

    fn capacities(tree: &BoundsTree<f32>) -> (usize, usize, usize, usize) {
        (
            tree.bounds.capacity(),
            tree.cells.capacity(),
            tree.links.capacity(),
            tree.exceptional.capacity(),
        )
    }

    #[test]
    fn test_oracle_patterns_and_clear_reuse() {
        let row: Vec<_> = (0..256)
            .map(|index| rectangle(index as f32 * 3.0, 0.0, 1.0, 1.0))
            .collect();
        let grid: Vec<_> = (0..256)
            .map(|index| {
                rectangle(
                    (index % 16) as f32 * 3.0,
                    (index / 16) as f32 * 3.0,
                    1.0,
                    1.0,
                )
            })
            .collect();
        let mut reversed_row = row.clone();
        reversed_row.reverse();
        let outside_in: Vec<_> = (0..128)
            .flat_map(|index| [row[index], row[255 - index]])
            .collect();
        let nested: Vec<_> = (0..256)
            .map(|index| {
                let inset = index as f32;
                rectangle(inset, inset, 1024.0 - inset * 2.0, 1024.0 - inset * 2.0)
            })
            .collect();
        let mut growing = nested.clone();
        growing.reverse();
        let alternating: Vec<_> = (0..512)
            .map(|index| {
                let cluster = (index % 2) as f32 * 10000.0;
                rectangle(cluster + (index % 17) as f32, (index % 13) as f32, 4.0, 4.0)
            })
            .collect();
        let overlay = rectangle(-100.0, -100.0, 2000.0, 2000.0);
        let mut overlay_first = vec![overlay];
        overlay_first.extend_from_slice(&grid);
        let mut overlay_last = grid.clone();
        overlay_last.push(overlay);
        let mut overlay_middle = grid.clone();
        overlay_middle.insert(128, overlay);
        let touching: Vec<_> = (0..256)
            .map(|index| rectangle(index as f32, 0.0, 1.0, 1.0))
            .collect();
        let degenerate: Vec<_> = (0..256)
            .map(|index| rectangle((index % 7) as f32, 0.0, (index % 3) as f32, 0.0))
            .collect();
        let patterns = [
            ("empty", Vec::new()),
            ("single", vec![overlay]),
            ("disjoint row", row),
            ("disjoint grid", grid),
            ("reversed row", reversed_row),
            ("outside in", outside_in),
            (
                "identical overlaps",
                vec![rectangle(0.0, 0.0, 5.0, 5.0); 256],
            ),
            ("nested shrinking", nested),
            ("nested growing", growing),
            ("alternating clusters", alternating),
            ("overlay first", overlay_first),
            ("overlay last", overlay_last),
            ("overlay middle", overlay_middle),
            ("touching edges", touching),
            ("degenerate", degenerate),
        ];
        let mut tree = BoundsTree::default();
        for (label, bounds) in patterns {
            tree.clear();
            assert_orders(&mut tree, &bounds, label);
            let retained = capacities(&tree);
            tree.clear();
            assert!(tree.bounds.is_empty());
            assert!(tree.maximum.is_none());
            assert!(tree.links.is_empty());
            assert!(tree.exceptional.is_empty());
            assert_eq!(capacities(&tree), retained);
            assert_eq!(tree.find_max_ordering(&overlay), 0);
            assert_orders(&mut tree, &bounds, label);
            assert_eq!(capacities(&tree), retained);
        }
    }

    #[test]
    fn test_oracle_seeded_sparse_and_dense_rectangles() {
        for seed in [0, 1, 44, 1000] {
            for extent in [50.0, 2000.0] {
                let mut random = rand::rngs::StdRng::seed_from_u64(seed);
                let bounds: Vec<_> = (0..1024)
                    .map(|_| {
                        rectangle(
                            random.random_range(-extent..extent),
                            random.random_range(-extent..extent),
                            random.random_range(0.0..50.0),
                            random.random_range(0.0..50.0),
                        )
                    })
                    .collect();
                let label = format!("seed={seed}, extent={extent}");
                let mut tree = BoundsTree::default();
                assert_orders(&mut tree, &bounds, &label);
            }
        }
    }

    #[test]
    fn test_oracle_scalar_types() {
        let bounds: Vec<_> = (0..256)
            .map(|index| {
                rectangle(
                    (index % 31) as f32 - 15.0,
                    (index % 17) as f32 - 8.0,
                    (index % 7) as f32,
                    (index % 11) as f32,
                )
            })
            .collect();
        let integer_bounds: Vec<_> = bounds
            .iter()
            .map(|bounds| bounds.map(|value| value as i32))
            .collect();
        assert_orders(&mut BoundsTree::default(), &integer_bounds, "i32");
        let scaled_bounds: Vec<_> = bounds
            .iter()
            .map(|bounds| bounds.map(crate::ScaledPixels::from))
            .collect();
        assert_orders(&mut BoundsTree::default(), &scaled_bounds, "ScaledPixels");
    }

    #[test]
    fn test_insert() {
        let mut tree = BoundsTree::<f32>::default();
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
        let bounds3 = Bounds {
            origin: Point { x: 10.0, y: 10.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };

        // Insert the bounds into the tree and verify the order is correct
        assert_eq!(tree.insert(bounds1), 1);
        assert_eq!(tree.insert(bounds2), 2);
        assert_eq!(tree.insert(bounds3), 3);

        // Insert non-overlapping bounds and verify they can reuse orders
        let bounds4 = Bounds {
            origin: Point { x: 20.0, y: 20.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };
        let bounds5 = Bounds {
            origin: Point { x: 40.0, y: 40.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };
        let bounds6 = Bounds {
            origin: Point { x: 25.0, y: 25.0 },
            size: Size {
                width: 10.0,
                height: 10.0,
            },
        };
        assert_eq!(tree.insert(bounds4), 1); // bounds4 does not overlap with bounds1, bounds2, or bounds3
        assert_eq!(tree.insert(bounds5), 1); // bounds5 does not overlap with any other bounds
        assert_eq!(tree.insert(bounds6), 2); // bounds6 overlaps with bounds4, so it should have a different order
    }

    #[test]
    fn test_random_iterations() {
        let max_bounds = 100;
        for seed in 1..=1000 {
            // let seed = 44;
            let mut tree = BoundsTree::default();
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
            let mut expected_quads: Vec<(Bounds<f32>, u32)> = Vec::new();

            // Insert a random number of random AABBs into the tree.
            let num_bounds = rng.random_range(1..=max_bounds);
            for _ in 0..num_bounds {
                let min_x: f32 = rng.random_range(-100.0..100.0);
                let min_y: f32 = rng.random_range(-100.0..100.0);
                let width: f32 = rng.random_range(0.0..50.0);
                let height: f32 = rng.random_range(0.0..50.0);
                let bounds = Bounds {
                    origin: Point { x: min_x, y: min_y },
                    size: Size { width, height },
                };

                let expected_ordering = expected_quads
                    .iter()
                    .filter_map(|quad| quad.0.intersects(&bounds).then_some(quad.1))
                    .max()
                    .unwrap_or(0)
                    + 1;
                expected_quads.push((bounds, expected_ordering));

                // Insert the AABB into the tree and collect intersections.
                let actual_ordering = tree.insert(bounds);
                assert_eq!(actual_ordering, expected_ordering);
            }
        }
    }

    #[test]
    fn huge_negative_and_degenerate_bounds_match_oracle() {
        for seed in 0..32 {
            let mut random = rand::rngs::StdRng::seed_from_u64(seed);
            let coordinate = |random: &mut rand::rngs::StdRng| match random.random_range(0..8) {
                0 => f32::MAX,
                1 => -f32::MAX,
                2 => f32::INFINITY,
                3 => f32::NEG_INFINITY,
                4 => f32::NAN,
                _ => random.random_range(-100_000.0..100_000.0),
            };
            let length = |random: &mut rand::rngs::StdRng| match random.random_range(0..6) {
                0 => 0.0,
                1 => f32::MAX,
                2 => f32::INFINITY,
                _ => random.random_range(-128.0..256.0),
            };
            let bounds: Vec<_> = (0..128)
                .map(|_| {
                    rectangle(
                        coordinate(&mut random),
                        coordinate(&mut random),
                        length(&mut random),
                        length(&mut random),
                    )
                })
                .collect();
            assert_orders(&mut BoundsTree::default(), &bounds, &format!("seed={seed}"));
        }
    }
}
