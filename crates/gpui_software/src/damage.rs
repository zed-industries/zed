use gpui::{Bounds, DevicePixels, Point, Size};

use crate::{
    bin_pass::{BinGrid, CELL_WIDTH},
    framebuffer::BAND_HEIGHT,
};

pub struct Damage {
    pub rects: Vec<Bounds<DevicePixels>>,
    pub(crate) dirty_cells: Vec<bool>,
    pub(crate) columns: usize,
}

impl Damage {
    pub fn is_empty(&self) -> bool {
        self.rects.is_empty()
    }
}

pub(crate) fn compute_damage(
    size: Size<DevicePixels>,
    bins: &BinGrid,
    previous_hashes: &[u64],
    force_full: bool,
) -> Damage {
    let dirty_cells: Vec<_> = bins
        .cells()
        .iter()
        .enumerate()
        .map(|(index, cell)| {
            force_full
                || previous_hashes
                    .get(index)
                    .is_none_or(|previous| *previous != cell.hash)
        })
        .collect();
    let mut rects = damage_rectangles(size, bins.columns(), &dirty_cells, true);
    // Avoid exchanging saved bytes for thousands of presentation calls on fragmented damage.
    if rects.len() > 64 {
        rects = damage_rectangles(size, bins.columns(), &dirty_cells, false);
    }
    Damage {
        rects,
        dirty_cells,
        columns: bins.columns(),
    }
}

fn damage_rectangles(
    size: Size<DevicePixels>,
    columns: usize,
    dirty: &[bool],
    separate_runs: bool,
) -> Vec<Bounds<DevicePixels>> {
    let mut rects: Vec<Bounds<DevicePixels>> = Vec::new();
    if columns == 0 {
        return rects;
    }
    let mut active: Vec<Option<usize>> = vec![None; columns];
    for (row, cells) in dirty.chunks(columns).enumerate() {
        let mut column = 0;
        while column < columns {
            if !cells[column] {
                column += 1;
                continue;
            }
            let first = column;
            if separate_runs {
                while column < columns && cells[column] {
                    column += 1;
                }
            } else {
                column = cells
                    .iter()
                    .rposition(|dirty| *dirty)
                    .map_or(first + 1, |last| last + 1);
            }
            let x0 = first * CELL_WIDTH;
            let x1 = (column * CELL_WIDTH).min(size.width.0.max(0) as usize);
            let y0 = row * BAND_HEIGHT;
            let y1 = ((row + 1) * BAND_HEIGHT).min(size.height.0.max(0) as usize);
            let rect = Bounds {
                origin: Point {
                    x: DevicePixels(x0 as i32),
                    y: DevicePixels(y0 as i32),
                },
                size: Size {
                    width: DevicePixels((x1 - x0) as i32),
                    height: DevicePixels((y1 - y0) as i32),
                },
            };
            if let Some(index) = active[first]
                && rects[index].size.width == rect.size.width
                && rects[index].origin.y.0 + rects[index].size.height.0 == rect.origin.y.0
            {
                rects[index].size.height.0 += rect.size.height.0;
            } else {
                active[first] = Some(rects.len());
                rects.push(rect);
            }
            if !separate_runs {
                break;
            }
        }
    }
    rects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparse_damage_preserves_gaps_and_coalesces_vertically() {
        let size = Size {
            width: DevicePixels(192),
            height: DevicePixels(95),
        };
        let atlas = crate::SoftwareAtlas::new();
        let bins = BinGrid::new(size, &[], &atlas.lock());
        let damage = compute_damage(size, &bins, &[1, 0, 1, 1, 0, 1, 1, 0, 1], false);
        assert_eq!(damage.rects.len(), 2);
        assert!(
            damage
                .rects
                .iter()
                .all(|rect| rect.size.width.0 == 64 && rect.size.height.0 == 95)
        );
        assert_eq!(damage.rects[0].origin.x.0, 0);
        assert_eq!(damage.rects[1].origin.x.0, 128);
        let full = compute_damage(size, &bins, &[], true);
        assert_eq!(full.rects, [Bounds::new(Default::default(), size)]);
    }

    #[test]
    fn fragmented_damage_covers_every_dirty_cell_within_the_frame() {
        use rand::{Rng, SeedableRng, rngs::StdRng};
        let mut random = StdRng::seed_from_u64(0xda6a9e);
        for columns in [1, 7, 60] {
            let size = Size {
                width: DevicePixels(columns * 64 - 3),
                height: DevicePixels(95),
            };
            for _ in 0..30 {
                let dirty: Vec<_> = (0..columns * 3).map(|_| random.random()).collect();
                for separate_runs in [false, true] {
                    let rects = damage_rectangles(size, columns as usize, &dirty, separate_runs);
                    for rect in &rects {
                        assert!(rect.origin.x.0 >= 0 && rect.origin.y.0 >= 0);
                        assert!(rect.origin.x.0 + rect.size.width.0 <= size.width.0);
                        assert!(rect.origin.y.0 + rect.size.height.0 <= size.height.0);
                    }
                    for (index, dirty) in dirty.iter().enumerate() {
                        let x = (index % columns as usize * 64) as i32;
                        let y = (index / columns as usize * 32) as i32;
                        let covered = rects.iter().any(|rect| {
                            rect.origin.x.0 <= x
                                && rect.origin.y.0 <= y
                                && rect.origin.x.0 + rect.size.width.0 > x
                                && rect.origin.y.0 + rect.size.height.0 > y
                        });
                        if separate_runs {
                            assert_eq!(covered, *dirty);
                        } else {
                            assert!(!dirty || covered);
                        }
                    }
                }
            }
        }
    }
}
