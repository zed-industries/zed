use std::num::Wrapping;
use std::u16;

use crate::{AllocatorOptions, DEFAULT_OPTIONS, Allocation, AllocId, Size, Rectangle, point2, size2};

const BIN_BITS: u32 = 12;
const ITEM_BITS: u32 = 12;
const GEN_BITS: u32 = 8;

const BIN_MASK: u32   = (1 << BIN_BITS) - 1;
const ITEM_MASK: u32  = ((1 << ITEM_BITS) - 1) << BIN_BITS;
const GEN_MASK: u32   = ((1 << GEN_BITS) - 1) << (BIN_BITS + ITEM_BITS);

const MAX_ITEMS_PER_BIN: u16 = (ITEM_MASK >> 12) as u16;
const MAX_BIN_COUNT: usize = BIN_MASK as usize;
const MAX_SHELF_COUNT: usize = u16::MAX as usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
struct BucketIndex(u16);

impl BucketIndex {
    fn to_usize(self) -> usize {
        self.0 as usize
    }

    const INVALID: Self = BucketIndex(u16::MAX);
}

#[derive(Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
struct Shelf {
    x: u16,
    y: u16,
    height: u16,
    bucket_width: u16,

    first_bucket: BucketIndex,
}

#[derive(Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
struct Bucket {
    x: u16,
    free_space: u16,

    next: BucketIndex,

    /// Buckets are cleared when their reference count goes back to zero.
    refcount: u16,
    /// Similar to refcount except that the counter is not decremented
    /// when an item is deallocated. We only use this so that allocation
    /// ids are unique within a bucket.
    item_count: u16,
    shelf: u16,
    generation: Wrapping<u8>,
}

/// A faster but less precise Shelf-packing dynamic texture atlas allocator, inspired by https://github.com/mapbox/shelf-pack/
///
/// Items are accumulated into buckets which are laid out in rows (shelves) of variable height.
/// When allocating we first look for a suitable bucket. If none is found, a new shelf of the desired height
/// is pushed.
///
/// Lifetime isn't tracked at item granularity. Instead, items are grouped into buckets and deallocation happens
/// per bucket when all items of the buckets are removed.
/// When the top-most shelf is empty, it is removed, potentially cascading into garbage-collecting the next
/// shelf, etc.
///
/// This allocator works well when there are a lot of small items with similar sizes (typically, glyph atlases).
#[derive(Clone)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct BucketedAtlasAllocator {
    shelves: Vec<Shelf>,
    buckets: Vec<Bucket>,
    available_height: u16,
    width: u16,
    height: u16,
    first_unallocated_bucket: BucketIndex,
    flip_xy: bool,
    alignment: Size,
    current_column: u16,
    column_width: u16,
    num_columns: u16,
    allocated_space: i32,
}

impl BucketedAtlasAllocator {
    /// Create an atlas allocator with provided options.
    pub fn with_options(size: Size, options: &AllocatorOptions) -> Self {
        assert!(size.width < u16::MAX as i32);
        assert!(size.height < u16::MAX as i32);

        let (width, height, shelf_alignment) = if options.vertical_shelves {
            (size.height as u16, size.width as u16, options.alignment.height as u16)
        } else {
            (size.width as u16, size.height as u16, options.alignment.width as u16)
        };

        let mut column_width = width / (options.num_columns as u16);
        column_width = column_width - column_width % shelf_alignment;

        BucketedAtlasAllocator {
            shelves: Vec::new(),
            buckets: Vec::new(),
            available_height: height,
            width,
            height,
            first_unallocated_bucket: BucketIndex::INVALID,
            flip_xy: options.vertical_shelves,
            alignment: options.alignment,
            current_column: 0,
            num_columns: options.num_columns as u16,
            column_width,
            allocated_space: 0,
        }
    }

    /// Create an atlas allocator with default options.
    pub fn new(size: Size) -> Self {
        Self::with_options(size, &DEFAULT_OPTIONS)
    }

    pub fn clear(&mut self) {
        self.shelves.clear();
        self.buckets.clear();
        self.first_unallocated_bucket = BucketIndex::INVALID;
        self.available_height = self.height;
        self.current_column = 0;
        self.allocated_space = 0;
    }

    pub fn size(&self) -> Size {
        let (w, h) = convert_coordinates(self.flip_xy, self.width, self.height);
        size2(w as i32, h as i32)
    }

    pub fn grow(&mut self, new_size: Size) {
        assert!(new_size.width < u16::MAX as i32);
        assert!(new_size.height < u16::MAX as i32);

        let (new_width, new_height) = if self.flip_xy {
            (new_size.height as u16, new_size.width as u16)
        } else {
            (new_size.width as u16, new_size.height as u16)
        };

        assert!(new_width >= self.width);
        assert!(new_height >= self.height);

        self.available_height += new_height - self.height;
        self.width = new_width;
        self.height = new_height;

        if self.num_columns == 1 {
            // Add as many new buckets as possible to the existing shelves.
            let additional_width = self.width - self.column_width;

            let len = self.shelves.len();

            for shelf_index in 0..len {
                let shelf = &self.shelves[shelf_index];
                let mut x = self.column_width;
                let bucket_width = shelf.bucket_width;

                let max_new_buckets = (MAX_BIN_COUNT - self.buckets.len()) as u16;
                let mut num_buckets_to_add = additional_width / bucket_width;
                num_buckets_to_add = num_buckets_to_add.min(max_new_buckets);

                let mut bucket_next = shelf.first_bucket;

                for _ in 0..num_buckets_to_add {
                    let bucket = Bucket {
                        next: bucket_next,
                        x,
                        free_space: bucket_width,
                        refcount: 0,
                        shelf: shelf_index as u16,
                        generation: Wrapping(0),
                        item_count: 0,
                    };

                    x += bucket_width;

                    let bucket_index = self.add_bucket(bucket);

                    bucket_next = bucket_index;
                }

                self.shelves[shelf_index].first_bucket = bucket_next;
            }

            // Resize the existing column.
            self.column_width = self.width;
        } else {
            // Add as many new columns as possible.
            self.num_columns = self.width / self.column_width;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.shelves.is_empty()
    }

    /// Allocate a rectangle in the atlas.
    pub fn allocate(&mut self, mut requested_size: Size) -> Option<Allocation> {
        if requested_size.is_empty()
            || requested_size.width > std::u16::MAX as i32
            || requested_size.height > std::u16::MAX as i32 {
            return None;
        }

        adjust_size(self.alignment.width, &mut requested_size.width);
        adjust_size(self.alignment.height, &mut requested_size.height);

        if requested_size.width > self.column_width as i32 || requested_size.height > self.height as i32 {
            return None;
        }

        let (w, h) = convert_coordinates(self.flip_xy, requested_size.width as u16, requested_size.height as u16);

        let mut selected_shelf = std::usize::MAX;
        let mut selected_bucket = BucketIndex::INVALID;
        let mut best_waste = u16::MAX;

        let can_add_shelf = (self.available_height >= h || self.current_column + 1 < self.num_columns)
            && self.shelves.len() < MAX_SHELF_COUNT
            && self.buckets.len() < MAX_BIN_COUNT;

        'shelves: for (shelf_index, shelf) in self.shelves.iter().enumerate() {
            if shelf.height < h || shelf.bucket_width < w {
                continue;
            }

            let y_waste = shelf.height - h;
            if y_waste > best_waste || (can_add_shelf && y_waste > h) {
                continue;
            }

            let mut bucket_index = shelf.first_bucket;
            while bucket_index != BucketIndex::INVALID {
                let bucket = &self.buckets[bucket_index.to_usize()];

                if bucket.free_space >= w && bucket.item_count < MAX_ITEMS_PER_BIN {
                    if y_waste == 0 && bucket.free_space == w {
                        selected_shelf = shelf_index;
                        selected_bucket = bucket_index;

                        break 'shelves;
                    }

                    if y_waste < best_waste {
                        best_waste = y_waste;
                        selected_shelf = shelf_index;
                        selected_bucket = bucket_index;
                        break;
                    }
                }

                bucket_index = bucket.next;
            }
        }

        if selected_bucket == BucketIndex::INVALID {
            if can_add_shelf {
                selected_shelf = self.add_shelf(w, h);
                selected_bucket = self.shelves[selected_shelf].first_bucket;
            } else {
                // Attempt to merge some empty shelves to make a big enough spot.
                let selected = self.coalesce_shelves(w, h);
                selected_shelf = selected.0;
                selected_bucket = selected.1;
            }
        }

        if selected_bucket != BucketIndex::INVALID {
            return self.alloc_from_bucket(selected_shelf, selected_bucket, w);
        }

        return  None;
    }

    /// Deallocate a rectangle in the atlas.
    ///
    /// Space is only reclaimed when all items of the same bucket are deallocated.
    pub fn deallocate(&mut self, id: AllocId) {
        if self.deallocate_from_bucket(id) {
            self.cleanup_shelves();
        }

        self.check()
    }

    /// Amount of occupied space in the atlas.
    pub fn allocated_space(&self) -> i32 {
        self.allocated_space
    }

    /// How much space is available for future allocations.
    pub fn free_space(&self) -> i32 {
        (self.width as i32 * self.height as i32) - self.allocated_space
    }

    fn alloc_from_bucket(&mut self, shelf_index: usize, bucket_index: BucketIndex, width: u16) -> Option<Allocation> {
        let shelf = &mut self.shelves[shelf_index];
        let bucket = &mut self.buckets[bucket_index.to_usize()];

        debug_assert!(bucket.free_space >= width);

        let min_x = bucket.x + shelf.bucket_width - bucket.free_space;
        let min_y = shelf.y;
        let max_x = min_x + width;
        let max_y = min_y + shelf.height;

        let (min_x, min_y) = convert_coordinates(self.flip_xy, min_x, min_y);
        let (max_x, max_y) = convert_coordinates(self.flip_xy, max_x, max_y);

        bucket.free_space -= width;
        bucket.refcount += 1;
        bucket.item_count += 1;

        let id = AllocId(
            (bucket_index.0 as u32) & BIN_MASK
            | ((bucket.item_count as u32) << 12) & ITEM_MASK
            | (bucket.generation.0 as u32) << 24
        );

        let rectangle = Rectangle {
            min: point2(min_x as i32, min_y as i32),
            max: point2(max_x as i32, max_y as i32),
        };

        self.allocated_space += rectangle.size().area();

        self.check();

        Some(Allocation { id, rectangle })
    }

    fn add_bucket(&mut self, mut bucket: Bucket) -> BucketIndex {
        let mut bucket_index = self.first_unallocated_bucket;

        if bucket_index == BucketIndex::INVALID {
            bucket_index = BucketIndex(self.buckets.len() as u16);
            self.buckets.push(bucket);
        } else {
            let idx = bucket_index.to_usize();
            bucket.generation = self.buckets[idx].generation + Wrapping(1);
            self.first_unallocated_bucket = self.buckets[idx].next;
            self.buckets[idx] = bucket;
        }

        bucket_index
    }

    fn add_shelf(&mut self, width: u16, height: u16) -> usize {

        let can_add_column = self.current_column + 1 < self.num_columns;

        if self.available_height != 0 && self.available_height < height && can_add_column {
            // We have room to add a shelf in a new column but current one doesn't have
            // enough available space. First add a shelf to fill the current column's
            // remaining height.
            self.add_shelf(0, self.available_height);
            debug_assert_eq!(self.available_height, 0);
        }

        if self.available_height == 0 && can_add_column {
            self.current_column += 1;
            self.available_height = self.height;
        }

        let height = shelf_height(height).min(self.available_height);
        let num_buckets = self.num_buckets(width, height);
        let mut bucket_width = self.column_width / num_buckets;
        bucket_width = bucket_width - (bucket_width % self.alignment.width as u16); // TODO
        let y = self.height - self.available_height;
        self.available_height -= height;

        let shelf_index = self.shelves.len();

        // Initialize the buckets for our new shelf.
        let mut x = self.current_column * self.column_width;
        let mut bucket_next = BucketIndex::INVALID;
        for _ in 0..num_buckets {
            let bucket = Bucket {
                next: bucket_next,
                x,
                free_space: bucket_width,
                refcount: 0,
                shelf: shelf_index as u16,
                generation: Wrapping(0),
                item_count: 0,
            };

            x += bucket_width;

            let bucket_index = self.add_bucket(bucket);

            bucket_next = bucket_index;
        }

        self.shelves.push(Shelf {
            x: self.current_column * self.column_width,
            y,
            height,
            bucket_width,
            first_bucket: bucket_next,
        });

        shelf_index
    }

    /// Find a sequence of consecutive shelves that can be coalesced into a single one
    /// tall enough to fit the provided size.
    ///
    /// If such a sequence is found, grow the height of first shelf and squash the other
    /// ones to zero.
    /// The squashed shelves are not removed, their height is just set to zero so no item
    /// can go in, and they will be garbage-collected whenever there's no shelf above them.
    /// For simplicity, the bucket width is not modified.

    fn coalesce_shelves(&mut self, w: u16, h: u16) -> (usize, BucketIndex) {
        let len = self.shelves.len();
        let mut coalesce_range = None;
        let mut coalesced_height = 0;

        'outer: for shelf_index in 0..len {
            if self.shelves[shelf_index].bucket_width < w {
                continue;
            }
            if !self.shelf_is_empty(shelf_index) {
                continue;
            }
            let shelf_x = self.shelves[shelf_index].x;
            coalesced_height = self.shelves[shelf_index].height;
            for i in 1..3 {
                if self.shelves[shelf_index + i].x != shelf_x {
                    // Can't coalesce shelves from different columns.
                    continue 'outer;
                }

                if shelf_index + i >= len {
                    break 'outer;
                }

                if !self.shelf_is_empty(shelf_index + i) {
                    continue 'outer;
                }

                coalesced_height += self.shelves[shelf_index + i].height;

                if coalesced_height >= h {
                    coalesce_range = Some(shelf_index .. (shelf_index + i + 1));
                    break 'outer;
                }
            }
        }

        if let Some(range) = coalesce_range {
            let y_top = self.shelves[range.start].y + coalesced_height;
            for i in range.start + 1 .. range.end {
                self.shelves[i].y = y_top;
                self.shelves[i].height = 0;
            }

            let shelf_index = range.start;
            let shelf = &mut self.shelves[shelf_index];
            shelf.height = coalesced_height;

            return (shelf_index, shelf.first_bucket);
        }

        (0, BucketIndex::INVALID)
    }

    fn num_buckets(&self, width: u16, height: u16) -> u16 {
        match self.column_width / u16::max(width, height) {
            0 ..= 4 => 1,
            5 ..= 16 => 2,
            17 ..= 32 => 4,
            n => (n /16 - 1).next_power_of_two(),
        }.min((MAX_BIN_COUNT - self.buckets.len()) as u16)
    }

    /// Returns true if we should garbage-collect the shelves as a result of
    /// removing this element (we deallocated the last item from the bucket on
    /// the top-most shelf).
    fn deallocate_from_bucket(&mut self, id: AllocId) -> bool {
        let bucket_index = (id.0 & BIN_MASK) as usize;
        let generation = ((id.0 & GEN_MASK) >> 24 ) as u8;

        let bucket = &mut self.buckets[bucket_index];

        let expected_generation = bucket.generation.0;
        assert_eq!(generation, expected_generation);

        assert!(bucket.refcount > 0);
        bucket.refcount -= 1;

        let shelf = &self.shelves[bucket.shelf as usize];

        let bucket_is_empty = bucket.refcount == 0;
        if bucket_is_empty {
            self.allocated_space -= (shelf.bucket_width - bucket.free_space) as i32 * shelf.height as i32;
            bucket.free_space = shelf.bucket_width;
        }

        bucket_is_empty && bucket.shelf as usize == self.shelves.len() - 1
    }

    fn cleanup_shelves(&mut self) {
        while self.shelves.len() > 0 {
            {
                let shelf = self.shelves.last().unwrap();
                let mut bucket_index = shelf.first_bucket;
                let mut last_bucket = shelf.first_bucket;

                while bucket_index != BucketIndex::INVALID {
                    let bucket = &self.buckets[bucket_index.to_usize()];

                    if bucket.refcount != 0 {
                        return;
                    }

                    last_bucket = bucket_index;
                    bucket_index = bucket.next;
                }

                // We didn't run into any bucket on this shelf with live elements,
                // this means we can remove it.

                // Can't have a shelf with no buckets.
                debug_assert!(last_bucket != BucketIndex::INVALID);
                // Add the buckets to the free list.
                self.buckets[last_bucket.to_usize()].next = self.first_unallocated_bucket;
                self.first_unallocated_bucket = shelf.first_bucket;

                if shelf.y == 0 && self.current_column > 0 {
                    self.current_column -= 1;
                    let prev_shelf = &self.shelves[self.shelves.len() - 2];
                    self.available_height = self.height - (prev_shelf.y + prev_shelf.height);
                } else {
                    // Reclaim the height of the shelf.
                    self.available_height += shelf.height;
                }
            }

            self.shelves.pop();
        }
    }

    fn shelf_is_empty(&self, idx: usize) -> bool {
        let shelf = &self.shelves[idx];
        let mut bucket_index = shelf.first_bucket;

        while bucket_index != BucketIndex::INVALID {
            let bucket = &self.buckets[bucket_index.to_usize()];

            if bucket.refcount != 0 {
                return false;
            }

            bucket_index = bucket.next;
        }

        true
    }


    /// Dump a visual representation of the atlas in SVG format.
    pub fn dump_svg(&self, output: &mut dyn std::io::Write) -> std::io::Result<()> {
        use svg_fmt::*;

        writeln!(
            output,
            "{}",
            BeginSvg {
                w: self.width as f32,
                h: self.height as f32
            }
        )?;

        self.dump_into_svg(None, output)?;

        writeln!(output, "{}", EndSvg)
    }


    #[cfg(not(feature = "checks"))]
    fn check(&self) {}

    #[cfg(feature = "checks")]
    fn check(&self) {
        let mut h = 0;
        for shelf in &self.shelves {
            h += shelf.height;
        }
        h += self.available_height;

        // Total height must be a multiple of the actual height, up to height * num_columns.
        assert_eq!(h % self.height, 0);
        assert!(h <= self.height * self.num_columns);
        assert!(h >= self.height);

        assert_eq!(self.is_empty(), self.allocated_space() == 0)
    }

    /// Dump a visual representation of the atlas in SVG, omitting the beginning and end of the
    /// SVG document, so that it can be included in a larger document.
    ///
    /// If a rectangle is provided, translate and scale the output to fit it.
    pub fn dump_into_svg(&self, rect: Option<&Rectangle>, output: &mut dyn std::io::Write) -> std::io::Result<()> {
        use svg_fmt::*;

        let (sx, sy, tx, ty) = if let Some(rect) = rect {
            (
                rect.size().width as f32 / self.width as f32,
                rect.size().height as f32 / self.height as f32,
                rect.min.x as f32,
                rect.min.y as f32,
            )
        } else {
            (1.0, 1.0, 0.0, 0.0)
        };

        writeln!(
            output,
            r#"    {}"#,
            rectangle(tx, ty, self.width as f32 * sx, self.height as f32 * sy)
                .fill(rgb(40, 40, 40))
                .stroke(Stroke::Color(black(), 1.0))
        )?;


        for shelf in &self.shelves {
            let mut bucket_index = shelf.first_bucket;

            let y = shelf.y as f32 * sy;
            let h = shelf.height as f32 * sy;
            while bucket_index != BucketIndex::INVALID {
                let bucket = &self.buckets[bucket_index.to_usize()];

                let x = bucket.x as f32 * sx;
                let w = (shelf.bucket_width - bucket.free_space) as f32 * sx;

                {
                    let (x, y) = if self.flip_xy { (y, x) } else { (x, y) };
                    let (w, h) = if self.flip_xy { (h, w) } else { (w, h) };

                    writeln!(
                        output,
                        r#"    {}"#,
                        rectangle(x + tx, y + ty, w, h)
                            .fill(rgb(70, 70, 180))
                            .stroke(Stroke::Color(black(), 1.0))
                    )?;
                }

                if bucket.free_space > 0 {
                    let x_free = x + w;
                    let w_free = bucket.free_space as f32 * sx;

                    let (x_free, y) = if self.flip_xy { (y, x_free) } else { (x_free, y) };
                    let (w_free, h) = if self.flip_xy { (h, w_free) } else { (w_free, h) };

                    writeln!(
                        output,
                        r#"    {}"#,
                        rectangle(x_free + tx, y + ty, w_free, h)
                            .fill(rgb(50, 50, 50))
                            .stroke(Stroke::Color(black(), 1.0))
                    )?;
                }

                bucket_index = bucket.next;
            }
        }

        Ok(())
    }
}

fn convert_coordinates(flip_xy: bool, x: u16, y: u16) -> (u16, u16) {
    if flip_xy {
        (y, x)
    } else {
        (x, y)
    }
}


fn shelf_height(mut size: u16) -> u16 {
    let alignment = match size {
        0 ..= 31 => 8,
        32 ..= 127 => 16,
        128 ..= 511 => 32,
        _ => 64,
    };

    let rem = size % alignment;
    if rem > 0 {
        size += alignment - rem;
    }

    size
}

fn adjust_size(alignment: i32, size: &mut i32) {
    let rem = *size % alignment;
    if rem > 0 {
        *size += alignment - rem;
    }
}

#[test]
fn atlas_basic() {
    let mut atlas = BucketedAtlasAllocator::new(size2(1000, 1000));

    let full = atlas.allocate(size2(1000, 1000)).unwrap().id;
    assert!(atlas.allocate(size2(1, 1)).is_none());

    atlas.deallocate(full);
    let a = atlas.allocate(size2(10, 10)).unwrap().id;
    let b = atlas.allocate(size2(50, 30)).unwrap().id;
    let c = atlas.allocate(size2(12, 45)).unwrap().id;
    let d = atlas.allocate(size2(60, 45)).unwrap().id;
    let e = atlas.allocate(size2(1, 1)).unwrap().id;
    let f = atlas.allocate(size2(128, 128)).unwrap().id;
    let g = atlas.allocate(size2(256, 256)).unwrap().id;

    atlas.deallocate(b);
    atlas.deallocate(f);
    atlas.deallocate(c);
    atlas.deallocate(e);
    let h = atlas.allocate(size2(500, 200)).unwrap().id;
    atlas.deallocate(a);
    let i = atlas.allocate(size2(500, 200)).unwrap().id;
    atlas.deallocate(g);
    atlas.deallocate(h);
    atlas.deallocate(d);
    atlas.deallocate(i);

    let full = atlas.allocate(size2(1000, 1000)).unwrap().id;
    assert!(atlas.allocate(size2(1, 1)).is_none());
    atlas.deallocate(full);
}

#[test]
fn test_coalesce_shelves() {
    let mut atlas = BucketedAtlasAllocator::new(size2(256, 256));

    // Allocate 7 shelves (leaving 32px of remaining space on top).
    let mut ids = Vec::new();
    for _ in 0..7 {
        for _ in 0..8 {
            ids.push(atlas.allocate(size2(32, 32)).unwrap().id)
        }
    }

    // Free the first shelf.
    for i in 0..8 {
        atlas.deallocate(ids[i]);
    }

    // Free the 3rd and 4th shelf.
    for i in 16..32 {
        atlas.deallocate(ids[i]);
    }

    // Not enough space left in existing shelves and above.
    // even coalescing is not sufficient.
    assert!(atlas.allocate(size2(70, 70)).is_none());

    // Not enough space left in existing shelves and above.
    // The 3rd and 4th row can be coalesced to fit this allocation, though.
    let id = atlas.allocate(size2(64, 64)).unwrap().id;

    // Deallocate everything
    for i in 8..16 {
        atlas.deallocate(ids[i]);
    }

    atlas.deallocate(id);

    for i in 32..56 {
        atlas.deallocate(ids[i]);
    }

    //dump_svg(&atlas, &mut std::fs::File::create("tmp.svg").expect("!!"));

    assert!(atlas.is_empty());
    assert_eq!(atlas.allocated_space(), 0);
}

#[test]
fn grow_vertically() {
    let mut atlas = BucketedAtlasAllocator::new(size2(256, 256));

    // Allocate 7 shelves (leaving 32px of remaining space on top).
    let mut ids = Vec::new();
    for _ in 0..7 {
        for _ in 0..8 {
            ids.push(atlas.allocate(size2(32, 32)).unwrap().id)
        }
    }

    // Free the first shelf.
    for i in 0..8 {
        atlas.deallocate(ids[i]);
    }

    // Free the 3rd and 4th shelf.
    for i in 16..32 {
        atlas.deallocate(ids[i]);
    }

    // Not enough space left in existing shelves and above.
    // even coalescing is not sufficient.
    assert!(atlas.allocate(size2(70, 70)).is_none());

    // Grow just enough vertically to fit the previous region
    atlas.grow(size2(256, 256 + 70 - 32));

    // Allocation should succeed now
    assert!(atlas.allocate(size2(70, 70)).is_some());
}

#[test]
fn grow_horizontally() {
    let mut atlas = BucketedAtlasAllocator::new(size2(256, 256));

    // Allocate 7 shelves (leaving 32px of remaining space on top).
    let mut ids = Vec::new();
    for _ in 0..7 {
        for _ in 0..8 {
            ids.push(atlas.allocate(size2(32, 32)).unwrap().id)
        }
    }

    // Free the first shelf.
    for i in 0..8 {
        atlas.deallocate(ids[i]);
    }

    // Free the 3rd and 4th shelf.
    for i in 16..32 {
        atlas.deallocate(ids[i]);
    }

    // Not enough space left in existing shelves and above.
    // even coalescing is not sufficient.
    assert!(atlas.allocate(size2(512, 32)).is_none());

    // Grow just enough horizontally to add more buckets
    atlas.grow(size2(256 * 2, 256));

    // Allocation should succeed now
    assert!(atlas.allocate(size2(512, 32)).is_some());
}

#[test]
fn grow_to_fit_allocation() {
    let mut atlas = BucketedAtlasAllocator::new(size2(32, 32));

    // Allocate a shelve to make sure we have a non-empty atlas to test the update.
    atlas.allocate(size2(32, 32)).unwrap();

    // Try to make a big allocation that doesn't fit.
    let big_allocation = size2(256, 256);

    assert!(atlas.allocate(big_allocation).is_none());

    // Grow to make enough space for the wanted allocation plus the original shelf.
    atlas.grow(size2(256, 32 + 256));

    // Adding to the original shelf should succeed.
    assert!(atlas.allocate(size2(32, 32)).is_some());

    // Big allocation should also succeed now.
    assert!(atlas.allocate(big_allocation).is_some());
}

#[test]
fn columns() {
    let mut atlas = BucketedAtlasAllocator::with_options(size2(64, 64), &AllocatorOptions {
        num_columns: 2,
        ..DEFAULT_OPTIONS
    });

    let a = atlas.allocate(size2(24, 46)).unwrap();
    let b = atlas.allocate(size2(24, 32)).unwrap();
    let c = atlas.allocate(size2(24, 32)).unwrap();

    fn in_range(val: i32, range: std::ops::Range<i32>) -> bool {
        let ok = val >= range.start && val < range.end;

        if !ok {
            println!("{:?} not in {:?}", val, range);
        }

        ok
    }

    assert!(in_range(a.rectangle.min.x, 0..32));
    assert!(in_range(a.rectangle.max.x, 0..32));
    assert!(in_range(b.rectangle.min.x, 32..64));
    assert!(in_range(b.rectangle.max.x, 32..64));
    assert!(in_range(c.rectangle.min.x, 32..64));
    assert!(in_range(c.rectangle.max.x, 32..64));

    atlas.deallocate(b.id);
    atlas.deallocate(c.id);
    atlas.deallocate(a.id);

    assert!(atlas.is_empty());
    assert_eq!(atlas.allocated_space(), 0);

    let a = atlas.allocate(size2(24, 46)).unwrap();
    let b = atlas.allocate(size2(24, 32)).unwrap();
    let c = atlas.allocate(size2(24, 32)).unwrap();
    let d = atlas.allocate(size2(24, 8)).unwrap();

    assert_eq!(a.rectangle.min.x, 0);
    assert_eq!(b.rectangle.min.x, 32);
    assert_eq!(c.rectangle.min.x, 32);
    assert_eq!(d.rectangle.min.x, 0);
}

#[test]
fn vertical() {
    let mut atlas = BucketedAtlasAllocator::with_options(size2(128, 256), &AllocatorOptions {
        num_columns: 2,
        vertical_shelves: true,
        ..DEFAULT_OPTIONS
    });

    assert_eq!(atlas.size(), size2(128, 256));

    let a = atlas.allocate(size2(32, 16)).unwrap();
    let b = atlas.allocate(size2(16, 32)).unwrap();

    assert!(a.rectangle.size().width >= 32);
    assert!(a.rectangle.size().height >= 16);

    assert!(b.rectangle.size().width >= 16);
    assert!(b.rectangle.size().height >= 32);

    let c = atlas.allocate(size2(128, 128)).unwrap();

    atlas.deallocate(a.id);
    atlas.deallocate(b.id);
    atlas.deallocate(c.id);

    assert!(atlas.is_empty());
    assert_eq!(atlas.allocated_space(), 0);
}

#[test]
fn clear() {
    let mut atlas = BucketedAtlasAllocator::new(size2(2048, 2048));

    // Run a workload a few hundred times to make sure clearing properly resets everything.
    for _ in 0..500 {
        atlas.clear();
        assert_eq!(atlas.allocated_space(), 0);

        atlas.allocate(size2(8, 2)).unwrap();
        atlas.allocate(size2(2, 8)).unwrap();
        atlas.allocate(size2(16, 512)).unwrap();
        atlas.allocate(size2(34, 34)).unwrap();
        atlas.allocate(size2(34, 34)).unwrap();
        atlas.allocate(size2(34, 34)).unwrap();
        atlas.allocate(size2(34, 34)).unwrap();
        atlas.allocate(size2(2, 8)).unwrap();
        atlas.allocate(size2(2, 8)).unwrap();
        atlas.allocate(size2(8, 2)).unwrap();
        atlas.allocate(size2(2, 8)).unwrap();
        atlas.allocate(size2(8, 2)).unwrap();
        atlas.allocate(size2(8, 8)).unwrap();
        atlas.allocate(size2(8, 8)).unwrap();
        atlas.allocate(size2(8, 8)).unwrap();
        atlas.allocate(size2(8, 8)).unwrap();
        atlas.allocate(size2(82, 80)).unwrap();
        atlas.allocate(size2(56, 56)).unwrap();
        atlas.allocate(size2(64, 66)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(40, 40)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(256, 52)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(256, 52)).unwrap();
        atlas.allocate(size2(256, 52)).unwrap();
        atlas.allocate(size2(256, 52)).unwrap();
        atlas.allocate(size2(256, 52)).unwrap();
        atlas.allocate(size2(256, 52)).unwrap();
        atlas.allocate(size2(256, 52)).unwrap();
        atlas.allocate(size2(155, 52)).unwrap();
        atlas.allocate(size2(256, 52)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(24, 24)).unwrap();
        atlas.allocate(size2(64, 64)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(84, 84)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(8, 2)).unwrap();
        atlas.allocate(size2(34, 34)).unwrap();
        atlas.allocate(size2(34, 34)).unwrap();
        atlas.allocate(size2(192, 192)).unwrap();
        atlas.allocate(size2(192, 192)).unwrap();
        atlas.allocate(size2(52, 52)).unwrap();
        atlas.allocate(size2(144, 144)).unwrap();
        atlas.allocate(size2(192, 192)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(144, 144)).unwrap();
        atlas.allocate(size2(24, 24)).unwrap();
        atlas.allocate(size2(192, 192)).unwrap();
        atlas.allocate(size2(192, 192)).unwrap();
        atlas.allocate(size2(432, 243)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
        atlas.allocate(size2(8, 2)).unwrap();
        atlas.allocate(size2(2, 8)).unwrap();
        atlas.allocate(size2(9, 9)).unwrap();
        atlas.allocate(size2(14, 14)).unwrap();
        atlas.allocate(size2(14, 14)).unwrap();
        atlas.allocate(size2(14, 14)).unwrap();
        atlas.allocate(size2(14, 14)).unwrap();
        atlas.allocate(size2(8, 8)).unwrap();
        atlas.allocate(size2(27, 27)).unwrap();
        atlas.allocate(size2(27, 27)).unwrap();
        atlas.allocate(size2(27, 27)).unwrap();
        atlas.allocate(size2(27, 27)).unwrap();
        atlas.allocate(size2(11, 12)).unwrap();
        atlas.allocate(size2(29, 28)).unwrap();
        atlas.allocate(size2(32, 32)).unwrap();
    }
}


#[test]
fn fuzz_01() {
    let mut atlas = BucketedAtlasAllocator::new(size2(1000, 1000));

    assert!(atlas.allocate(size2(65280, 1)).is_none());
    assert!(atlas.allocate(size2(1, 65280)).is_none());
}

#[test]
fn fuzz_02() {
    let mut atlas = BucketedAtlasAllocator::new(size2(1000, 1000));

    assert!(atlas.allocate(size2(255, 65599)).is_none());
}

#[test]
fn fuzz_03() {
    let mut atlas = BucketedAtlasAllocator::new(size2(1000, 1000));

    let sizes = &[
        size2(999, 128),
        size2(168492810, 10),
        size2(45, 96),
        size2(-16711926, 0),
    ];

    let mut allocations = Vec::new();
    let mut allocated_space = 0;

    for size in sizes {
        if let Some(alloc) = atlas.allocate(*size) {
            allocations.push(alloc);
            allocated_space += alloc.rectangle.area();
            assert_eq!(allocated_space, atlas.allocated_space());
        }
    }

    for alloc in &allocations {
        atlas.deallocate(alloc.id);

        allocated_space -= alloc.rectangle.area();
        assert_eq!(allocated_space, atlas.allocated_space());
    }

    assert_eq!(atlas.allocated_space(), 0);
}

#[test]
fn fuzz_04() {
    let mut atlas = BucketedAtlasAllocator::new(size2(1000, 1000));

    assert!(atlas.allocate(size2(2560, 2147483647)).is_none());
}

#[test]
fn fuzz_05() {
    let mut atlas = BucketedAtlasAllocator::new(size2(2048, 2048));

    assert!(atlas.allocate(size2(0, -1978597547)).is_none());
}
