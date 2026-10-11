use crate::Bounds;
use std::{
    fmt::Debug,
    ops::Add,
    sync::atomic::{AtomicU64, Ordering},
};

// Glyphs and decorations are usually a few dozen scaled pixels across. A 64-pixel
// cell keeps their duplication small without making every text line one long list.
const CELL_SIZE: f64 = 64.0;
const MAX_CELLS: usize = 256;
const MAX_CELLS_PER_BOUNDS: usize = 16;
const NONE: usize = usize::MAX;
const REPLAY_COMPARISONS: usize = 32_768;
static NEXT_GENERATION: AtomicU64 = AtomicU64::new(1);

/// Assigns one plus the greatest order of previously inserted intersecting bounds.
#[derive(Debug)]
pub(crate) struct BoundsTree<U: Clone + Debug + Default + PartialEq> {
    columns: usize,
    rows: usize,
    cells: Vec<Cell>,
    links: Vec<Link>,
    bounds: Vec<(Bounds<U>, u32)>,
    exceptional: Vec<Exceptional>,
    maximum: Option<usize>,
    generation: u64,
    replay_generation: u64,
    compared: usize,
    changed: Vec<Bounds<U>>,
    comparisons_left: usize,
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

#[derive(Debug)]
struct Exceptional {
    bounds: usize,
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
        self.generation = NEXT_GENERATION.fetch_add(1, Ordering::Relaxed);
        self.replay_generation = 0;
        self.compared = 0;
        self.changed.clear();
        self.comparisons_left = REPLAY_COMPARISONS;
    }

    pub fn len(&self) -> usize {
        self.bounds.len()
    }

    pub fn try_replay_range(&mut self, previous: &Self, range: std::ops::Range<usize>) -> bool {
        if range.start != self.bounds.len() || range.end > previous.bounds.len() {
            return false;
        }
        if self.replay_generation != previous.generation {
            self.replay_generation = previous.generation;
            self.compared = 0;
            self.changed.clear();
        }
        for position in self.compared..range.start {
            let old = &previous.bounds[position];
            let current = &self.bounds[position];
            if old != current {
                self.changed.push(old.0.clone());
                self.changed.push(current.0.clone());
            }
        }
        self.compared = range.start;
        let comparisons = self.changed.len().saturating_mul(range.len());
        let Some(remaining) = self.comparisons_left.checked_sub(comparisons) else {
            return false;
        };
        self.comparisons_left = remaining;
        if previous.bounds[range.clone()].iter().any(|(bounds, _)| {
            self.changed
                .iter()
                .any(|changed| bounds.intersects(changed))
        }) {
            return false;
        }
        // Every prefix difference is disjoint from the whole segment. Its old
        // orders therefore stand, including dependencies between segment nodes.
        for (bounds, order) in &previous.bounds[range.clone()] {
            self.register(bounds.clone(), *order);
        }
        self.compared = range.end;
        true
    }

    /// An unchanged bounds has its old order if no difference in the preceding
    /// insertion stream intersects it. Both sides of each difference count:
    /// removing an overlap can lower the answer just as adding one can raise it.
    /// Prefix entries include their orders, so propagation through earlier reused
    /// bounds is covered too. Layers enter this stream once, not per primitive;
    /// nothing here assumes that primitives sharing a layer order are disjoint.
    pub fn insert_replayed(&mut self, bounds: Bounds<U>, previous: &Self, index: usize) -> u32 {
        if self.replay_generation != previous.generation {
            self.replay_generation = previous.generation;
            self.compared = 0;
            self.changed.clear();
        }
        if index == self.bounds.len()
            && self.comparisons_left > 0
            && let Some((old_bounds, old_order)) = previous.bounds.get(index)
            && *old_bounds == bounds
        {
            for position in self.compared..index {
                let old = &previous.bounds[position];
                let current = &self.bounds[position];
                if old != current {
                    self.changed.push(old.0.clone());
                    self.changed.push(current.0.clone());
                }
            }
            self.compared = index;
            // Bound the cumulative cost of checking differences. Once exhausted,
            // the grid answers all remaining inserts without more replay work.
            if let Some(remaining) = self.comparisons_left.checked_sub(self.changed.len()) {
                self.comparisons_left = remaining;
                if !self
                    .changed
                    .iter()
                    .any(|changed| bounds.intersects(changed))
                {
                    let order = *old_order;
                    self.register(bounds, order);
                    return order;
                }
            } else {
                self.comparisons_left = 0;
            }
        }
        self.insert(bounds)
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
        let extent = Self::extent(bounds);
        let extent = extent.filter(|extent| {
            let columns =
                cell_at(extent.right, self.columns) - cell_at(extent.left, self.columns) + 1;
            let rows = cell_at(extent.bottom, self.rows) - cell_at(extent.top, self.rows) + 1;
            columns * rows <= MAX_CELLS_PER_BOUNDS
        });
        let Some(extent) = extent else {
            // Large overlays would duplicate their boundary lists across the grid.
            // Keep them once, with prefix maxima so reverse searches can stop early.
            self.exceptional.push(Exceptional {
                bounds: index,
                maximum: self
                    .exceptional
                    .last()
                    .map_or(*order, |entry| entry.maximum.max(*order)),
            });
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
        let mut maximum = 0;
        for entry in self.exceptional.iter().rev() {
            if entry.maximum <= maximum {
                break;
            }
            let (bounds, order) = &self.bounds[entry.bounds];
            if *order > maximum && query.intersects(bounds) {
                maximum = *order;
            }
        }
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
            generation: NEXT_GENERATION.fetch_add(1, Ordering::Relaxed),
            replay_generation: 0,
            compared: 0,
            changed: Vec::new(),
            comparisons_left: REPLAY_COMPARISONS,
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

    #[test]
    fn total_order_edges_and_replayed_orders_match_oracle() {
        let values = [
            f32::from_bits(0xffc0_0001),
            f32::NEG_INFINITY,
            -f32::MAX,
            -64.0,
            -0.0,
            0.0,
            f32::from_bits(1),
            64.0,
            f32::MAX,
            f32::INFINITY,
            f32::from_bits(0x7fc0_0001),
        ];
        for seed in 0..12 {
            let mut random = rand::rngs::StdRng::seed_from_u64(seed);
            let bounds: Vec<_> = (0..256)
                .map(|index| {
                    let bounds = if index % 2 == 0 {
                        rectangle(
                            random.random_range(-2048.0..2048.0),
                            random.random_range(-2048.0..2048.0),
                            random.random_range(1.0..2048.0),
                            random.random_range(1.0..2048.0),
                        )
                    } else {
                        rectangle(
                            values[random.random_range(0..values.len())],
                            values[random.random_range(0..values.len())],
                            values[random.random_range(0..values.len())],
                            values[random.random_range(0..values.len())],
                        )
                    };
                    bounds.map(crate::ScaledPixels::from)
                })
                .collect();
            assert_orders(
                &mut BoundsTree::default(),
                &bounds,
                &format!("total order seed={seed}"),
            );
            let mut tree = BoundsTree::default();
            let mut reference = Vec::new();
            for bounds in &bounds {
                let order = random.random_range(1..128);
                tree.register(*bounds, order);
                reference.push((*bounds, order));
                let expected = reference
                    .iter()
                    .filter_map(|(previous, order)| bounds.intersects(previous).then_some(*order))
                    .max()
                    .unwrap_or(0);
                assert_eq!(tree.find_max_ordering(bounds), expected, "seed={seed}");
            }
            for query in &bounds {
                let expected = reference
                    .iter()
                    .filter_map(|(bounds, order)| query.intersects(bounds).then_some(*order))
                    .max()
                    .unwrap_or(0);
                assert_eq!(tree.find_max_ordering(query), expected, "seed={seed}");
            }
        }
        let mut tree = BoundsTree::default();
        tree.register(rectangle(4096.0, 4096.0, 64.0, 64.0), 100);
        tree.register(rectangle(0.0, 0.0, 512.0, 512.0), 100);
        tree.register(rectangle(0.0, 0.0, 32.0, 32.0), 3);
        assert_eq!(tree.find_max_ordering(&rectangle(1.0, 1.0, 8.0, 8.0)), 100);
    }

    #[test]
    fn replay_matches_fresh_orders_across_changed_prefixes() {
        for seed in 0..32 {
            let mut random = rand::rngs::StdRng::seed_from_u64(seed);
            let mut frame: Vec<_> = (0..256)
                .map(|_| {
                    rectangle(
                        random.random_range(-128.0..512.0),
                        random.random_range(-128.0..512.0),
                        random.random_range(-8.0..256.0),
                        random.random_range(-8.0..256.0),
                    )
                })
                .collect();
            let mut previous = BoundsTree::default();
            for bounds in &frame {
                previous.insert(*bounds);
            }
            for iteration in 0..8 {
                let mut replayed = BoundsTree::default();
                let mut fresh = BoundsTree::default();
                if iteration == 4 {
                    replayed.comparisons_left = 1;
                }
                for (index, bounds) in frame.iter_mut().enumerate() {
                    let retained = random.random_range(0..8) != 0;
                    if !retained {
                        bounds.origin.x += random.random_range(-64.0..64.0);
                    }
                    let expected = fresh.insert(*bounds);
                    let actual = if retained {
                        replayed.insert_replayed(*bounds, &previous, index)
                    } else {
                        replayed.insert(*bounds)
                    };
                    assert_eq!(
                        actual, expected,
                        "seed={seed} frame={iteration} index={index}"
                    );
                }
                previous = replayed;
            }
        }
    }

    #[test]
    fn replay_handles_removal_addition_and_a_different_source() {
        let bounds = rectangle(0.0, 0.0, 128.0, 128.0);
        let mut previous = BoundsTree::default();
        let mut other = BoundsTree::default();
        for _ in 0..8 {
            previous.insert(bounds);
            other.insert(bounds);
        }
        let mut target = BoundsTree::default();
        assert_eq!(target.insert_replayed(bounds, &previous, 0), 1);
        assert_eq!(target.insert_replayed(bounds, &other, 1), 2);
        assert_eq!(target.insert_replayed(bounds, &previous, 3), 3);
        assert_eq!(target.insert_replayed(bounds, &previous, 4), 4);
        target.insert(bounds);
        assert_eq!(target.insert_replayed(bounds, &previous, 5), 6);
        assert!(target.changed.is_empty());
    }
}
