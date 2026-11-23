use crate::{Bounds, ScaledPixels, scene::PrimitiveKind};
use std::{
    cmp::{self, Ordering},
    fmt::Debug,
    ops::Add,
};

/// Maximum entries per page. Eight lanes of each coordinate fit in two 16-byte
/// aligned vectors for 32-bit coordinates.
const PAGE_CAPACITY: usize = 8;

/// When a full page receives a ninth entry, this many entries stay in the
/// original page and the rest move to a new sibling page.
const SPLIT_LEFT_LEN: usize = 4;

/// Bounds are stored in separate trees by size relative to everything inserted
/// so far, so that large bounds don't inflate the pages holding small ones.
const TIER_COUNT: usize = 3;
const LARGE_TIER_MIN_FRACTION: f32 = 0.25;
const MEDIUM_TIER_MIN_FRACTION: f32 = 0.0625;

/// A coordinate type that [`BoundsTree`] can store.
pub(crate) trait BoundsScalar:
    Clone + Debug + Default + PartialEq + Add<Output = Self>
{
    /// A cheaply comparable encoding of the coordinate whose order is exactly
    /// the coordinate's `PartialOrd`.
    type OrderKey: Copy + Debug + Default + PartialOrd;

    fn to_order_key(&self) -> Self::OrderKey;

    /// The approximate length from `min` to `max`, or zero if `max` isn't
    /// greater. Only used to decide where bounds are stored.
    fn heuristic_extent(min: Self::OrderKey, max: Self::OrderKey) -> f32;

    /// The approximate midpoint of `min` and `max`. Only used to decide where
    /// bounds are stored.
    fn heuristic_center(min: Self::OrderKey, max: Self::OrderKey) -> f32;
}

fn clamped_extent(min: f32, max: f32) -> f32 {
    let extent = max - min;
    if extent > 0.0 { extent } else { 0.0 }
}

impl BoundsScalar for f32 {
    type OrderKey = f32;

    fn to_order_key(&self) -> f32 {
        *self
    }

    fn heuristic_extent(min: f32, max: f32) -> f32 {
        clamped_extent(min, max)
    }

    fn heuristic_center(min: f32, max: f32) -> f32 {
        min * 0.5 + max * 0.5
    }
}

impl BoundsScalar for i32 {
    type OrderKey = i32;

    fn to_order_key(&self) -> i32 {
        *self
    }

    fn heuristic_extent(min: i32, max: i32) -> f32 {
        clamped_extent(min as f32, max as f32)
    }

    fn heuristic_center(min: i32, max: i32) -> f32 {
        min as f32 * 0.5 + max as f32 * 0.5
    }
}

/// Maps between `f32` bits and integers ordered exactly like
/// [`f32::total_cmp`], which `ScaledPixels` uses for its ordering. The mapping
/// is its own inverse.
fn total_order_bits(bits: i32) -> i32 {
    bits ^ (((bits >> 31) as u32 >> 1) as i32)
}

fn decode_scaled_pixels(key: i32) -> f32 {
    f32::from_bits(total_order_bits(key) as u32)
}

impl BoundsScalar for ScaledPixels {
    type OrderKey = i32;

    fn to_order_key(&self) -> i32 {
        total_order_bits(self.0.to_bits() as i32)
    }

    fn heuristic_extent(min: i32, max: i32) -> f32 {
        clamped_extent(decode_scaled_pixels(min), decode_scaled_pixels(max))
    }

    fn heuristic_center(min: i32, max: i32) -> f32 {
        decode_scaled_pixels(min) * 0.5 + decode_scaled_pixels(max) * 0.5
    }
}

/// A spatial tree optimized for finding the maximum key among intersecting bounds.
///
/// This is a paged R-tree specifically designed for assigning z-order to
/// overlapping UI elements. Each bounds has an ordering and a kind; bounds
/// sharing an ordering are drawn in the order of their kinds.
///
/// Every page stores its entries' bounds in structure-of-arrays lanes along with
/// their keys sorted ascending, so searches can scan from the highest key down and
/// stop as soon as no remaining entry can improve on the best match. Within each
/// size tier, all leaf pages are at the same depth.
///
/// Bounds that can never affect a future query result are not stored: a new
/// bounds contained in an intersecting bounds with an equal or greater key is
/// dropped, and a stored bounds contained in a new bounds with an equal or greater
/// key is overwritten by it.
#[derive(Debug)]
pub(crate) struct BoundsTree<U: BoundsScalar> {
    /// From largest to smallest bounds.
    tiers: [PageTree<U>; TIER_COUNT],
    /// The union of every stored bounds.
    scene_bounds: Option<NodeBounds<U::OrderKey>>,
    /// The bounds and key of a stored bounds with the greatest key, which answers
    /// any query that intersects it without searching.
    max_leaf: Option<(NodeBounds<U::OrderKey>, Key)>,
    /// Reusable stack of the internal pages and slots visited while inserting.
    insert_path: Vec<PathStep>,
    /// Reusable stack of internal page indices and their max keys for searching.
    search_stack: Vec<(usize, Key)>,
}

#[derive(Debug)]
struct PageTree<U: BoundsScalar> {
    leaf_pages: Vec<LeafPage<U>>,
    internal_pages: Vec<InternalPage<U>>,
    root: Option<NodeId>,
    /// The union of every bounds in this tree, valid when `root` is set.
    root_bounds: NodeBounds<U::OrderKey>,
    /// The greatest key in this tree, valid when `root` is set.
    root_max_key: Key,
}

/// The ordering of a bounds in the high bits, then its kind, which orders the
/// bounds that share an ordering.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Key(u64);

impl Key {
    /// Orderings start at 1, so this compares below every stored key.
    const MIN: Key = Key(0);

    fn new(ordering: u32, kind: PrimitiveKind) -> Self {
        Self((u64::from(ordering) << 8) | kind as u64)
    }

    fn ordering(self) -> u32 {
        (self.0 >> 8) as u32
    }
}

/// Identifies a leaf page or an internal page.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct NodeId(u32);

enum PageIndex {
    Leaf(usize),
    Internal(usize),
}

impl NodeId {
    const LEAF_PAGE_BIT: u32 = 1 << 31;

    fn leaf_page(index: usize) -> Self {
        debug_assert!(index < Self::LEAF_PAGE_BIT as usize);
        Self(index as u32 | Self::LEAF_PAGE_BIT)
    }

    fn internal_page(index: usize) -> Self {
        debug_assert!(index < Self::LEAF_PAGE_BIT as usize);
        Self(index as u32)
    }

    fn is_leaf_page(self) -> bool {
        self.0 & Self::LEAF_PAGE_BIT != 0
    }

    fn index(self) -> usize {
        (self.0 & !Self::LEAF_PAGE_BIT) as usize
    }

    fn page_index(self) -> PageIndex {
        if self.is_leaf_page() {
            PageIndex::Leaf(self.index())
        } else {
            PageIndex::Internal(self.index())
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PathStep {
    page_index: usize,
    slot: usize,
}

/// Bounds stored as corners.
///
/// The corners are not reordered for negative sizes, so `intersects` agrees with
/// [`Bounds::intersects`] for every input.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct NodeBounds<K> {
    min_x: K,
    min_y: K,
    max_x: K,
    max_y: K,
}

fn node_bounds<U: BoundsScalar>(bounds: &Bounds<U>) -> NodeBounds<U::OrderKey> {
    let max_x = bounds.origin.x.clone() + bounds.size.width.clone();
    let max_y = bounds.origin.y.clone() + bounds.size.height.clone();
    NodeBounds {
        min_x: bounds.origin.x.to_order_key(),
        min_y: bounds.origin.y.to_order_key(),
        max_x: max_x.to_order_key(),
        max_y: max_y.to_order_key(),
    }
}

fn heuristic_half_perimeter<U: BoundsScalar>(bounds: &NodeBounds<U::OrderKey>) -> f32 {
    U::heuristic_extent(bounds.min_x, bounds.max_x)
        + U::heuristic_extent(bounds.min_y, bounds.max_y)
}

impl<K: Copy + PartialOrd> NodeBounds<K> {
    #[inline]
    fn intersects(&self, other: &Self) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
    }

    /// Whenever this holds, every bounds that intersects `inner` also intersects
    /// `self`, regardless of the sign of either size.
    #[inline]
    fn contains(&self, inner: &Self) -> bool {
        self.min_x <= inner.min_x
            && inner.max_x <= self.max_x
            && self.min_y <= inner.min_y
            && inner.max_y <= self.max_y
    }

    fn union(&self, other: &Self) -> Self {
        Self {
            min_x: min_value(self.min_x, other.min_x),
            min_y: min_value(self.min_y, other.min_y),
            max_x: max_value(self.max_x, other.max_x),
            max_y: max_value(self.max_y, other.max_y),
        }
    }
}

/// A bounds with an unordered coordinate (such as an `f32` NaN) never intersects
/// anything, so unions skip such coordinates rather than letting them spread into
/// the bounds of every ancestor.
#[inline]
fn min_value<K: PartialOrd>(a: K, b: K) -> K {
    match a.partial_cmp(&b) {
        Some(Ordering::Greater) => b,
        Some(_) => a,
        None => {
            if a.partial_cmp(&a).is_some() {
                a
            } else {
                b
            }
        }
    }
}

#[inline]
fn max_value<K: PartialOrd>(a: K, b: K) -> K {
    match a.partial_cmp(&b) {
        Some(Ordering::Less) => b,
        Some(_) => a,
        None => {
            if a.partial_cmp(&a).is_some() {
                a
            } else {
                b
            }
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C, align(16))]
struct BoundsLanes<K> {
    min_x: [K; PAGE_CAPACITY],
    min_y: [K; PAGE_CAPACITY],
    max_x: [K; PAGE_CAPACITY],
    max_y: [K; PAGE_CAPACITY],
}

impl<K: Copy + Default + PartialOrd> BoundsLanes<K> {
    fn new() -> Self {
        Self {
            min_x: [K::default(); PAGE_CAPACITY],
            min_y: [K::default(); PAGE_CAPACITY],
            max_x: [K::default(); PAGE_CAPACITY],
            max_y: [K::default(); PAGE_CAPACITY],
        }
    }

    fn get(&self, slot: usize) -> NodeBounds<K> {
        NodeBounds {
            min_x: self.min_x[slot],
            min_y: self.min_y[slot],
            max_x: self.max_x[slot],
            max_y: self.max_y[slot],
        }
    }

    fn set(&mut self, slot: usize, bounds: NodeBounds<K>) {
        self.min_x[slot] = bounds.min_x;
        self.min_y[slot] = bounds.min_y;
        self.max_x[slot] = bounds.max_x;
        self.max_y[slot] = bounds.max_y;
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.min_x.swap(a, b);
        self.min_y.swap(a, b);
        self.max_x.swap(a, b);
        self.max_y.swap(a, b);
    }

    /// Returns a bit per slot, including unused slots, set when the slot
    /// intersects `query`.
    #[inline(always)]
    fn intersection_mask(&self, query: &NodeBounds<K>) -> u8 {
        let mut mask = 0u8;
        // Non-short-circuiting `&` keeps this loop branch-free so it can be
        // vectorized across all lanes.
        for slot in 0..PAGE_CAPACITY {
            let hit = (self.min_x[slot] < query.max_x)
                & (self.max_x[slot] > query.min_x)
                & (self.min_y[slot] < query.max_y)
                & (self.max_y[slot] > query.min_y);
            mask |= (hit as u8) << slot;
        }
        mask
    }

    #[inline(always)]
    fn slot_contains(&self, slot: usize, inner: &NodeBounds<K>) -> bool {
        self.min_x[slot] <= inner.min_x
            && inner.max_x <= self.max_x[slot]
            && self.min_y[slot] <= inner.min_y
            && inner.max_y <= self.max_y[slot]
    }

    #[inline(always)]
    fn is_contained_by(&self, slot: usize, outer: &NodeBounds<K>) -> bool {
        outer.min_x <= self.min_x[slot]
            && self.max_x[slot] <= outer.max_x
            && outer.min_y <= self.min_y[slot]
            && self.max_y[slot] <= outer.max_y
    }
}

/// An entry being moved into a page.
#[derive(Clone, Copy, Debug)]
struct Entry<K, Child> {
    bounds: NodeBounds<K>,
    key: Key,
    child: Child,
}

/// Up to [`PAGE_CAPACITY`] entries, sorted ascending by key.
///
/// In a leaf page, each entry is a stored bounds and its key. In an internal
/// page, each entry is a child page with the union of its bounds and its
/// maximum key.
#[derive(Clone, Debug)]
struct Page<U: BoundsScalar, Child> {
    lanes: BoundsLanes<U::OrderKey>,
    keys: [Key; PAGE_CAPACITY],
    children: [Child; PAGE_CAPACITY],
    len: u8,
}

type LeafPage<U> = Page<U, ()>;
type InternalPage<U> = Page<U, NodeId>;

impl<U: BoundsScalar, Child: Copy + Default> Page<U, Child> {
    fn new() -> Self {
        Self {
            lanes: BoundsLanes::new(),
            keys: [Key::MIN; PAGE_CAPACITY],
            children: [Child::default(); PAGE_CAPACITY],
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn is_full(&self) -> bool {
        self.len() == PAGE_CAPACITY
    }

    fn max_key(&self) -> Key {
        self.keys[self.len() - 1]
    }

    /// Bits of the slots that `query` intersects, excluding unused slots.
    #[inline(always)]
    fn intersection_mask(&self, query: &NodeBounds<U::OrderKey>) -> u8 {
        let used_slots = ((1u16 << self.len) - 1) as u8;
        self.lanes.intersection_mask(query) & used_slots
    }

    fn entry(&self, slot: usize) -> Entry<U::OrderKey, Child> {
        Entry {
            bounds: self.lanes.get(slot),
            key: self.keys[slot],
            child: self.children[slot],
        }
    }

    fn bounds(&self) -> NodeBounds<U::OrderKey> {
        let mut bounds = self.lanes.get(0);
        for slot in 1..self.len() {
            bounds = bounds.union(&self.lanes.get(slot));
        }
        bounds
    }

    fn swap_slots(&mut self, a: usize, b: usize) {
        self.lanes.swap(a, b);
        self.keys.swap(a, b);
        self.children.swap(a, b);
    }

    fn restore_key_order(&mut self, mut slot: usize) {
        while slot > 0 && self.keys[slot - 1] > self.keys[slot] {
            self.swap_slots(slot - 1, slot);
            slot -= 1;
        }
        while slot + 1 < self.len() && self.keys[slot + 1] < self.keys[slot] {
            self.swap_slots(slot, slot + 1);
            slot += 1;
        }
    }

    fn update_slot(&mut self, slot: usize, bounds: NodeBounds<U::OrderKey>, key: Key) {
        self.lanes.set(slot, bounds);
        self.keys[slot] = key;
        self.restore_key_order(slot);
    }

    fn push(&mut self, entry: Entry<U::OrderKey, Child>) {
        debug_assert!(!self.is_full());
        let slot = self.len();
        self.lanes.set(slot, entry.bounds);
        self.keys[slot] = entry.key;
        self.children[slot] = entry.child;
        self.len += 1;
        self.restore_key_order(slot);
    }

    /// Splits this full page and `extra` along the longer axis of their union,
    /// keeping [`SPLIT_LEFT_LEN`] entries here and returning the rest as a new page.
    fn split(&mut self, extra: Entry<U::OrderKey, Child>) -> Self {
        debug_assert!(self.is_full());
        let mut entries: [Entry<U::OrderKey, Child>; PAGE_CAPACITY + 1] =
            std::array::from_fn(|slot| {
                if slot < PAGE_CAPACITY {
                    self.entry(slot)
                } else {
                    extra
                }
            });

        let mut union = entries[0].bounds;
        for entry in &entries[1..] {
            union = union.union(&entry.bounds);
        }
        let width = U::heuristic_extent(union.min_x, union.max_x);
        let height = U::heuristic_extent(union.min_y, union.max_y);
        let split_horizontally = height.partial_cmp(&width) != Some(Ordering::Greater);
        let mut centers: [f32; PAGE_CAPACITY + 1] = std::array::from_fn(|slot| {
            let bounds = &entries[slot].bounds;
            if split_horizontally {
                U::heuristic_center(bounds.min_x, bounds.max_x)
            } else {
                U::heuristic_center(bounds.min_y, bounds.max_y)
            }
        });

        // Insertion sort never panics on an inconsistent order, unlike
        // `sort_by`, which matters when centers are NaN.
        for start in 1..entries.len() {
            let mut slot = start;
            while slot > 0
                && centers[slot - 1].partial_cmp(&centers[slot]) == Some(Ordering::Greater)
            {
                centers.swap(slot - 1, slot);
                entries.swap(slot - 1, slot);
                slot -= 1;
            }
        }

        let mut entries = entries.into_iter();
        self.len = 0;
        for entry in entries.by_ref().take(SPLIT_LEFT_LEN) {
            self.push(entry);
        }
        let mut sibling = Self::new();
        for entry in entries {
            sibling.push(entry);
        }
        sibling
    }
}

impl<U: BoundsScalar> LeafPage<U> {
    /// Finds a stored bounds that `bounds` with `key` makes redundant.
    fn subsumed_slot(&self, bounds: &NodeBounds<U::OrderKey>, key: Key) -> Option<usize> {
        (0..self.len())
            .find(|&slot| self.keys[slot] <= key && self.lanes.is_contained_by(slot, bounds))
    }

    #[inline(always)]
    fn scan(
        &self,
        tier: usize,
        page_index: usize,
        query: &NodeBounds<U::OrderKey>,
        best: &mut SearchResult,
    ) {
        if self.max_key() <= best.key {
            return;
        }
        let mask = self.intersection_mask(query);
        if mask == 0 {
            return;
        }
        // Keys ascend with slots, so the highest set bit has the greatest key.
        let slot = highest_slot(mask);
        if self.keys[slot] > best.key {
            best.key = self.keys[slot];
            best.location = Some(WinnerLocation::Leaf {
                tier,
                page_index,
                slot,
            });
        }
    }
}

impl<U: BoundsScalar> InternalPage<U> {
    fn choose_subtree(&self, bounds: &NodeBounds<U::OrderKey>) -> usize {
        let lanes = &self.lanes;
        let half_perimeters: [f32; PAGE_CAPACITY] = std::array::from_fn(|slot| {
            U::heuristic_extent(lanes.min_x[slot], lanes.max_x[slot])
                + U::heuristic_extent(lanes.min_y[slot], lanes.max_y[slot])
        });
        let enlargements: [f32; PAGE_CAPACITY] = std::array::from_fn(|slot| {
            let union_width = U::heuristic_extent(
                min_value(lanes.min_x[slot], bounds.min_x),
                max_value(lanes.max_x[slot], bounds.max_x),
            );
            let union_height = U::heuristic_extent(
                min_value(lanes.min_y[slot], bounds.min_y),
                max_value(lanes.max_y[slot], bounds.max_y),
            );
            union_width + union_height - half_perimeters[slot]
        });

        let mut best_slot = 0;
        for slot in 1..self.len() {
            let is_better = match enlargements[slot].partial_cmp(&enlargements[best_slot]) {
                Some(Ordering::Less) => true,
                Some(Ordering::Equal) => half_perimeters[slot] < half_perimeters[best_slot],
                _ => false,
            };
            if is_better {
                best_slot = slot;
            }
        }
        best_slot
    }
}

#[inline(always)]
fn highest_slot(mask: u8) -> usize {
    7 - mask.leading_zeros() as usize
}

/// Where the bounds with the greatest intersecting key is stored.
#[derive(Clone, Copy, Debug)]
enum WinnerLocation {
    MaxLeaf,
    Leaf {
        tier: usize,
        page_index: usize,
        slot: usize,
    },
}

struct SearchResult {
    key: Key,
    location: Option<WinnerLocation>,
}

/// A page that split while inserting, described for its parent.
struct Split<K> {
    left: Entry<K, NodeId>,
    right: Entry<K, NodeId>,
}

impl<U: BoundsScalar> PageTree<U> {
    fn clear(&mut self) {
        self.leaf_pages.clear();
        self.internal_pages.clear();
        self.root = None;
    }

    fn search(
        &self,
        tier: usize,
        query: &NodeBounds<U::OrderKey>,
        global_max_key: Key,
        best: &mut SearchResult,
        search_stack: &mut Vec<(usize, Key)>,
    ) {
        let Some(root) = self.root else {
            return;
        };
        if self.root_max_key <= best.key || !self.root_bounds.intersects(query) {
            return;
        }
        search_stack.clear();
        match root.page_index() {
            PageIndex::Leaf(page_index) => {
                self.leaf_pages[page_index].scan(tier, page_index, query, best);
            }
            PageIndex::Internal(page_index) => {
                search_stack.push((page_index, self.root_max_key));
            }
        }

        while let Some((page_index, max_key)) = search_stack.pop() {
            if max_key <= best.key {
                continue;
            }
            let page = &self.internal_pages[page_index];
            let mut mask = page.intersection_mask(query);
            if mask == 0 {
                continue;
            }
            if page.children[0].is_leaf_page() {
                while mask != 0 {
                    let slot = highest_slot(mask);
                    mask &= !(1 << slot);
                    if page.keys[slot] <= best.key {
                        break;
                    }
                    let leaf_index = page.children[slot].index();
                    self.leaf_pages[leaf_index].scan(tier, leaf_index, query, best);
                }
            } else {
                // Pushing in ascending key order pops the highest key first.
                while mask != 0 {
                    let slot = mask.trailing_zeros() as usize;
                    mask &= mask - 1;
                    if page.keys[slot] > best.key {
                        search_stack.push((page.children[slot].index(), page.keys[slot]));
                    }
                }
            }
            if best.key == global_max_key {
                break;
            }
        }
        search_stack.clear();
    }

    fn insert(
        &mut self,
        bounds: NodeBounds<U::OrderKey>,
        key: Key,
        insert_path: &mut Vec<PathStep>,
    ) {
        let Some(root) = self.root else {
            let mut page = LeafPage::new();
            page.push(Entry {
                bounds,
                key,
                child: (),
            });
            self.root = Some(NodeId::leaf_page(self.leaf_pages.len()));
            self.root_bounds = bounds;
            self.root_max_key = key;
            self.leaf_pages.push(page);
            return;
        };
        self.root_bounds = self.root_bounds.union(&bounds);
        self.root_max_key = cmp::max(self.root_max_key, key);

        insert_path.clear();
        let mut node = root;
        let leaf_index = loop {
            match node.page_index() {
                PageIndex::Leaf(page_index) => break page_index,
                PageIndex::Internal(page_index) => {
                    let page = &self.internal_pages[page_index];
                    let slot = page.choose_subtree(&bounds);
                    insert_path.push(PathStep { page_index, slot });
                    node = page.children[slot];
                }
            }
        };

        let mut split = None;
        let leaf_page = &mut self.leaf_pages[leaf_index];
        // The overwritten bounds is contained in the new one, whose key is at
        // least as great, so it can never decide a future query.
        if let Some(slot) = leaf_page.subsumed_slot(&bounds, key) {
            leaf_page.update_slot(slot, bounds, key);
        } else if !leaf_page.is_full() {
            leaf_page.push(Entry {
                bounds,
                key,
                child: (),
            });
        } else {
            let sibling = leaf_page.split(Entry {
                bounds,
                key,
                child: (),
            });
            let left = Entry {
                bounds: leaf_page.bounds(),
                key: leaf_page.max_key(),
                child: NodeId::leaf_page(leaf_index),
            };
            let right = Entry {
                bounds: sibling.bounds(),
                key: sibling.max_key(),
                child: NodeId::leaf_page(self.leaf_pages.len()),
            };
            self.leaf_pages.push(sibling);
            split = Some(Split { left, right });
        }

        for step in insert_path.iter().rev() {
            let page = &mut self.internal_pages[step.page_index];
            let Some(Split { left, right }) = split.take() else {
                // Neither this child's bounds nor its max key change, so no
                // ancestor's does either.
                if key <= page.keys[step.slot] && page.lanes.slot_contains(step.slot, &bounds) {
                    break;
                }
                // Pages below only redistributed their contents, so this page
                // gains exactly the new bounds.
                let merged_bounds = page.lanes.get(step.slot).union(&bounds);
                let merged_key = cmp::max(page.keys[step.slot], key);
                page.update_slot(step.slot, merged_bounds, merged_key);
                continue;
            };

            page.update_slot(step.slot, left.bounds, left.key);
            if !page.is_full() {
                page.push(right);
                continue;
            }
            let sibling = page.split(right);
            let left = Entry {
                bounds: page.bounds(),
                key: page.max_key(),
                child: NodeId::internal_page(step.page_index),
            };
            let right = Entry {
                bounds: sibling.bounds(),
                key: sibling.max_key(),
                child: NodeId::internal_page(self.internal_pages.len()),
            };
            self.internal_pages.push(sibling);
            split = Some(Split { left, right });
        }

        if let Some(Split { left, right }) = split {
            let mut new_root = InternalPage::new();
            new_root.push(left);
            new_root.push(right);
            self.root = Some(NodeId::internal_page(self.internal_pages.len()));
            self.internal_pages.push(new_root);
        }
    }
}

impl<U: BoundsScalar> BoundsTree<U> {
    /// Clears all bounds from the tree.
    pub fn clear(&mut self) {
        for tier in &mut self.tiers {
            tier.clear();
        }
        self.scene_bounds = None;
        self.max_leaf = None;
        self.insert_path.clear();
        self.search_stack.clear();
    }

    /// Inserts bounds of the given kind into the tree and returns its assigned
    /// ordering.
    ///
    /// The ordering is one greater than the maximum ordering of any existing
    /// bounds that intersect with the new bounds. The new bounds share that
    /// maximum ordering instead if every intersecting bounds there has a kind
    /// of at most `max_shared_kind`, since those are drawn first.
    pub fn insert(
        &mut self,
        new_bounds: Bounds<U>,
        kind: PrimitiveKind,
        max_shared_kind: PrimitiveKind,
    ) -> u32 {
        let new_bounds = node_bounds(&new_bounds);
        let max_intersecting = self.find_max_key(&new_bounds);
        let ordering = match &max_intersecting {
            None => 1,
            Some((max_key, _)) if *max_key <= Key::new(max_key.ordering(), max_shared_kind) => {
                max_key.ordering()
            }
            Some((max_key, _)) => max_key.ordering() + 1,
        };
        let key = Key::new(ordering, kind);

        // Any query intersecting the new bounds would also intersect the winner,
        // whose key is at least as great, so storing the new bounds changes nothing.
        if let Some((max_key, winner)) = max_intersecting
            && key <= max_key
            && self.winner_contains(winner, &new_bounds)
        {
            return ordering;
        }

        let is_new_max = match &self.max_leaf {
            None => true,
            Some((_, max_key)) => key >= *max_key,
        };
        if is_new_max {
            self.max_leaf = Some((new_bounds, key));
        }
        let scene_bounds = match &self.scene_bounds {
            None => new_bounds,
            Some(scene_bounds) => scene_bounds.union(&new_bounds),
        };
        self.scene_bounds = Some(scene_bounds);
        let tier = Self::tier_for(&new_bounds, &scene_bounds);
        self.tiers[tier].insert(new_bounds, key, &mut self.insert_path);

        ordering
    }

    fn tier_for(bounds: &NodeBounds<U::OrderKey>, scene_bounds: &NodeBounds<U::OrderKey>) -> usize {
        let half_perimeter = heuristic_half_perimeter::<U>(bounds);
        let scene_half_perimeter = heuristic_half_perimeter::<U>(scene_bounds);
        if half_perimeter > scene_half_perimeter * LARGE_TIER_MIN_FRACTION {
            0
        } else if half_perimeter > scene_half_perimeter * MEDIUM_TIER_MIN_FRACTION {
            1
        } else {
            2
        }
    }

    fn winner_contains(&self, winner: WinnerLocation, inner: &NodeBounds<U::OrderKey>) -> bool {
        match winner {
            WinnerLocation::MaxLeaf => self
                .max_leaf
                .as_ref()
                .is_some_and(|(bounds, _)| bounds.contains(inner)),
            WinnerLocation::Leaf {
                tier,
                page_index,
                slot,
            } => self.tiers[tier].leaf_pages[page_index]
                .lanes
                .slot_contains(slot, inner),
        }
    }

    /// Finds the maximum key among all bounds that intersect with the query,
    /// along with where the bounds that has it is stored.
    fn find_max_key(&mut self, query: &NodeBounds<U::OrderKey>) -> Option<(Key, WinnerLocation)> {
        let (global_max_bounds, global_max_key) = self.max_leaf.as_ref()?;
        if global_max_bounds.intersects(query) {
            return Some((*global_max_key, WinnerLocation::MaxLeaf));
        }
        let global_max_key = *global_max_key;

        let mut best = SearchResult {
            key: Key::MIN,
            location: None,
        };
        for (tier_index, tier) in self.tiers.iter().enumerate() {
            tier.search(
                tier_index,
                query,
                global_max_key,
                &mut best,
                &mut self.search_stack,
            );
            if best.key == global_max_key {
                break;
            }
        }
        Some((best.key, best.location?))
    }
}

impl<U: BoundsScalar> Default for PageTree<U> {
    fn default() -> Self {
        PageTree {
            leaf_pages: Vec::new(),
            internal_pages: Vec::new(),
            root: None,
            root_bounds: NodeBounds::default(),
            root_max_key: Key::MIN,
        }
    }
}

impl<U: BoundsScalar> Default for BoundsTree<U> {
    fn default() -> Self {
        BoundsTree {
            tiers: std::array::from_fn(|_| PageTree::default()),
            scene_bounds: None,
            max_leaf: None,
            insert_path: Vec::new(),
            search_stack: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Bounds, Point, ScaledPixels, Size};
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_insert() {
        let mut tree = BoundsTree::<f32>::default();
        let (quad, shadow) = (PrimitiveKind::Quad, PrimitiveKind::Shadow);
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
        assert_eq!(tree.insert(bounds1, quad, shadow), 1);
        assert_eq!(tree.insert(bounds2, quad, shadow), 2);
        assert_eq!(tree.insert(bounds3, quad, shadow), 3);

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
        assert_eq!(tree.insert(bounds4, quad, shadow), 1); // bounds4 does not overlap with bounds1, bounds2, or bounds3
        assert_eq!(tree.insert(bounds5, quad, shadow), 1); // bounds5 does not overlap with any other bounds
        assert_eq!(tree.insert(bounds6, quad, shadow), 2); // bounds6 overlaps with bounds4, so it should have a different order
    }

    #[test]
    fn test_random_iterations() {
        let max_bounds = 100;
        for seed in 1..=1000 {
            let mut tree = BoundsTree::default();
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
            let mut expected_quads: Vec<(Bounds<f32>, u32, PrimitiveKind)> = Vec::new();
            let kinds = [
                PrimitiveKind::Shadow,
                PrimitiveKind::Quad,
                PrimitiveKind::Path,
                PrimitiveKind::Underline,
            ];

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
                let kind = kinds[rng.random_range(0..kinds.len())];
                let max_shared_kind = kinds[rng.random_range(0..kinds.len())];

                let max_intersecting = expected_quads
                    .iter()
                    .filter(|quad| quad.0.intersects(&bounds))
                    .map(|quad| (quad.1, quad.2))
                    .max();
                let expected_ordering = match max_intersecting {
                    None => 1,
                    Some((ordering, max_kind)) if max_kind <= max_shared_kind => ordering,
                    Some((ordering, _)) => ordering + 1,
                };
                expected_quads.push((bounds, expected_ordering, kind));

                let actual_ordering = tree.insert(bounds, kind, max_shared_kind);
                assert_eq!(actual_ordering, expected_ordering);
            }
        }
    }

    const ITEM_COUNT: usize = 2_500;
    const INVARIANT_CHECK_INTERVAL: usize = 97;

    const ALL_KINDS: [PrimitiveKind; 9] = [
        PrimitiveKind::Shadow,
        PrimitiveKind::Quad,
        PrimitiveKind::Path,
        PrimitiveKind::Underline,
        PrimitiveKind::MonochromeSprite,
        PrimitiveKind::SubpixelSprite,
        PrimitiveKind::PolychromeSprite,
        PrimitiveKind::Surface,
        PrimitiveKind::NativeView,
    ];

    #[derive(Clone, Copy, Debug)]
    enum KindMode {
        Shared,
        MonochromeSpriteUnderline,
        QuadShadow,
        AllKinds,
    }

    const KIND_MODES: [KindMode; 4] = [
        KindMode::Shared,
        KindMode::MonochromeSpriteUnderline,
        KindMode::QuadShadow,
        KindMode::AllKinds,
    ];

    impl KindMode {
        fn kinds(self, rng: &mut impl Rng) -> (PrimitiveKind, PrimitiveKind) {
            match self {
                KindMode::Shared => (PrimitiveKind::Quad, PrimitiveKind::Quad),
                KindMode::MonochromeSpriteUnderline => {
                    (PrimitiveKind::MonochromeSprite, PrimitiveKind::Underline)
                }
                KindMode::QuadShadow => (PrimitiveKind::Quad, PrimitiveKind::Shadow),
                KindMode::AllKinds => (
                    ALL_KINDS[rng.random_range(0..ALL_KINDS.len())],
                    ALL_KINDS[rng.random_range(0..ALL_KINDS.len())],
                ),
            }
        }
    }

    fn bounds<U>(x: U, y: U, width: U, height: U) -> Bounds<U>
    where
        U: Clone + Debug + Default + PartialEq,
    {
        Bounds {
            origin: Point { x, y },
            size: Size { width, height },
        }
    }

    fn expected_ordering<U>(
        inserted: &[(Bounds<U>, u32, PrimitiveKind)],
        new_bounds: &Bounds<U>,
        max_shared_kind: PrimitiveKind,
    ) -> u32
    where
        U: Clone + Debug + Default + PartialEq + PartialOrd + Add<U, Output = U>,
    {
        let max_intersecting = inserted
            .iter()
            .filter(|(bounds, _, _)| bounds.intersects(new_bounds))
            .map(|(_, ordering, kind)| (*ordering, *kind))
            .max();
        match max_intersecting {
            None => 1,
            Some((ordering, max_kind)) if max_kind <= max_shared_kind => ordering,
            Some((ordering, _)) => ordering + 1,
        }
    }

    /// Inserts every item into `tree` and a brute-force oracle, asserting that
    /// they agree on every ordering and that the tree stays well-formed.
    fn assert_matches_oracle<U>(
        tree: &mut BoundsTree<U>,
        items: &[(Bounds<U>, PrimitiveKind, PrimitiveKind)],
        label: &str,
    ) where
        U: BoundsScalar + PartialOrd,
    {
        let mut inserted: Vec<(Bounds<U>, u32, PrimitiveKind)> = Vec::with_capacity(items.len());
        for (item_index, (new_bounds, kind, max_shared_kind)) in items.iter().enumerate() {
            let expected = expected_ordering(&inserted, new_bounds, *max_shared_kind);
            let actual = tree.insert(new_bounds.clone(), *kind, *max_shared_kind);
            assert_eq!(
                actual, expected,
                "{label}: item {item_index} {new_bounds:?} kind {kind:?} max shared {max_shared_kind:?}"
            );
            inserted.push((new_bounds.clone(), expected, *kind));
            if item_index % INVARIANT_CHECK_INTERVAL == 0 {
                assert_invariants(tree, inserted.len(), label);
            }
        }
        assert_invariants(tree, inserted.len(), label);
    }

    fn covers<K: PartialOrd>(outer: &NodeBounds<K>, inner: &NodeBounds<K>) -> bool {
        // Unordered coordinates never intersect anything, so ancestors skip them.
        let is_ordered = |value: &K| value.partial_cmp(value).is_some();
        (!is_ordered(&inner.min_x) || outer.min_x <= inner.min_x)
            && (!is_ordered(&inner.max_x) || inner.max_x <= outer.max_x)
            && (!is_ordered(&inner.min_y) || outer.min_y <= inner.min_y)
            && (!is_ordered(&inner.max_y) || inner.max_y <= outer.max_y)
    }

    fn assert_invariants<U: BoundsScalar>(
        tree: &BoundsTree<U>,
        inserted_count: usize,
        label: &str,
    ) {
        let mut stored_count = 0;
        let mut max_key = None;
        for (tier_index, tier) in tree.tiers.iter().enumerate() {
            let label = format!("{label} tier {tier_index}");
            stored_count += assert_tier_invariants(tier, &label);
            if tier.root.is_some() {
                max_key = cmp::max(max_key, Some(tier.root_max_key));
                let Some(scene_bounds) = &tree.scene_bounds else {
                    panic!("{label}: missing scene bounds");
                };
                assert!(covers(scene_bounds, &tier.root_bounds), "{label}");
            }
        }
        assert!(stored_count <= inserted_count, "{label}");
        assert_eq!(
            tree.max_leaf
                .as_ref()
                .map(|(_, max_leaf_key)| *max_leaf_key),
            max_key,
            "{label}"
        );
        if max_key.is_none() {
            assert!(tree.scene_bounds.is_none(), "{label}");
        }
    }

    /// Returns the number of stored bounds.
    fn assert_tier_invariants<U: BoundsScalar>(tier: &PageTree<U>, label: &str) -> usize {
        let Some(root) = tier.root else {
            assert!(tier.leaf_pages.is_empty(), "{label}");
            assert!(tier.internal_pages.is_empty(), "{label}");
            return 0;
        };

        let mut visited_leaf_pages = vec![false; tier.leaf_pages.len()];
        let mut visited_internal_pages = vec![false; tier.internal_pages.len()];
        let mut leaf_depth = None;
        let mut stored_count = 0;
        let mut stack = vec![(root, 0usize)];
        while let Some((node, depth)) = stack.pop() {
            match node.page_index() {
                PageIndex::Leaf(page_index) => {
                    assert!(!visited_leaf_pages[page_index], "{label}: leaf page reused");
                    visited_leaf_pages[page_index] = true;
                    assert_eq!(
                        *leaf_depth.get_or_insert(depth),
                        depth,
                        "{label}: leaf pages at different depths"
                    );
                    let page = &tier.leaf_pages[page_index];
                    assert!((1..=PAGE_CAPACITY).contains(&page.len()), "{label}");
                    assert!(page.keys[..page.len()].is_sorted(), "{label}");
                    assert!(page.keys[0] > Key::MIN, "{label}");
                    stored_count += page.len();
                }
                PageIndex::Internal(page_index) => {
                    assert!(
                        !visited_internal_pages[page_index],
                        "{label}: internal page reused"
                    );
                    visited_internal_pages[page_index] = true;
                    let page = &tier.internal_pages[page_index];
                    let min_len = if node == root { 2 } else { 1 };
                    assert!((min_len..=PAGE_CAPACITY).contains(&page.len()), "{label}");
                    assert!(page.keys[..page.len()].is_sorted(), "{label}");
                    let children_are_leaves = page.children[0].is_leaf_page();
                    for slot in 0..page.len() {
                        let child = page.children[slot];
                        assert_eq!(child.is_leaf_page(), children_are_leaves, "{label}");
                        let (child_bounds, child_max_key) = page_summary(tier, child);
                        assert_eq!(page.keys[slot], child_max_key, "{label}: stale max key");
                        let slot_bounds = page.lanes.get(slot);
                        assert!(
                            covers(&slot_bounds, &child_bounds),
                            "{label}: {slot_bounds:?} does not cover child {child_bounds:?}"
                        );
                        stack.push((child, depth + 1));
                    }
                }
            }
        }

        assert!(visited_leaf_pages.iter().all(|visited| *visited), "{label}");
        assert!(
            visited_internal_pages.iter().all(|visited| *visited),
            "{label}"
        );
        let (root_page_bounds, root_page_max_key) = page_summary(tier, root);
        assert_eq!(tier.root_max_key, root_page_max_key, "{label}");
        assert!(covers(&tier.root_bounds, &root_page_bounds), "{label}");
        stored_count
    }

    fn page_summary<U: BoundsScalar>(
        tier: &PageTree<U>,
        node: NodeId,
    ) -> (NodeBounds<U::OrderKey>, Key) {
        match node.page_index() {
            PageIndex::Leaf(page_index) => {
                let page = &tier.leaf_pages[page_index];
                (page.bounds(), page.max_key())
            }
            PageIndex::Internal(page_index) => {
                let page = &tier.internal_pages[page_index];
                (page.bounds(), page.max_key())
            }
        }
    }

    fn stored_count<U: BoundsScalar>(tree: &BoundsTree<U>) -> usize {
        tree.tiers
            .iter()
            .flat_map(|tier| &tier.leaf_pages)
            .map(|page| page.len())
            .sum()
    }

    fn tree_height<U: BoundsScalar>(tree: &BoundsTree<U>) -> usize {
        tree.tiers
            .iter()
            .map(|tier| {
                let mut height = 0;
                let mut node = tier.root;
                while let Some(current) = node {
                    height += 1;
                    node = match current.page_index() {
                        PageIndex::Leaf(_) => None,
                        PageIndex::Internal(page_index) => {
                            Some(tier.internal_pages[page_index].children[0])
                        }
                    };
                }
                height
            })
            .max()
            .unwrap_or(0)
    }

    fn log_uniform(rng: &mut impl Rng, min: f32, max: f32) -> f32 {
        rng.random_range(min.ln()..max.ln()).exp()
    }

    fn overlap_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        (0..ITEM_COUNT)
            .map(|_| {
                bounds(
                    rng.random_range(0.0..200.0),
                    rng.random_range(0.0..200.0),
                    rng.random_range(10.0..100.0),
                    rng.random_range(10.0..100.0),
                )
            })
            .collect()
    }

    fn nested_shrinking_scenario() -> Vec<Bounds<f32>> {
        (0..ITEM_COUNT)
            .map(|item_index| {
                let inset = item_index as f32 * 0.2;
                bounds(inset, inset, 1_200.0 - inset * 2.0, 1_000.0 - inset * 2.0)
            })
            .collect()
    }

    fn nested_growing_scenario() -> Vec<Bounds<f32>> {
        let mut items = nested_shrinking_scenario();
        items.reverse();
        items
    }

    fn clusters_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        let centers: Vec<(f32, f32)> = (0..24)
            .map(|_| {
                (
                    rng.random_range(0.0..4_000.0),
                    rng.random_range(0.0..3_000.0),
                )
            })
            .collect();
        (0..ITEM_COUNT)
            .map(|_| {
                let (center_x, center_y) = centers[rng.random_range(0..centers.len())];
                bounds(
                    center_x + rng.random_range(-60.0..60.0),
                    center_y + rng.random_range(-60.0..60.0),
                    rng.random_range(1.0..40.0),
                    rng.random_range(1.0..40.0),
                )
            })
            .collect()
    }

    fn grid_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        let columns = 50;
        (0..ITEM_COUNT)
            .map(|item_index| {
                let column = (item_index % columns) as f32;
                let row = (item_index / columns) as f32;
                if rng.random_ratio(1, 10) {
                    bounds(
                        column * 20.0 - 5.0,
                        row * 20.0 - 5.0,
                        rng.random_range(20.0..90.0),
                        rng.random_range(20.0..90.0),
                    )
                } else {
                    bounds(column * 20.0, row * 20.0, 20.0, 20.0)
                }
            })
            .collect()
    }

    fn stripes_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        (0..ITEM_COUNT)
            .map(|item_index| {
                let offset = (item_index % 200) as f32 * 18.0;
                if item_index % 2 == 0 {
                    bounds(0.0, offset, 3_600.0, rng.random_range(4.0..24.0))
                } else {
                    bounds(offset, 0.0, rng.random_range(4.0..24.0), 3_600.0)
                }
            })
            .collect()
    }

    fn overlay_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        (0..ITEM_COUNT)
            .map(|item_index| {
                if item_index % 250 == 249 {
                    bounds(
                        rng.random_range(-50.0..50.0),
                        rng.random_range(-50.0..50.0),
                        rng.random_range(800.0..1_200.0),
                        rng.random_range(600.0..900.0),
                    )
                } else {
                    bounds(
                        rng.random_range(0.0..1_000.0),
                        rng.random_range(0.0..800.0),
                        rng.random_range(2.0..60.0),
                        rng.random_range(2.0..20.0),
                    )
                }
            })
            .collect()
    }

    fn random_scenario(rng: &mut impl Rng) -> Vec<Bounds<f32>> {
        (0..ITEM_COUNT)
            .map(|_| {
                bounds(
                    rng.random_range(-500.0..3_000.0),
                    rng.random_range(-500.0..2_000.0),
                    log_uniform(rng, 0.5, 2_000.0),
                    log_uniform(rng, 0.5, 2_000.0),
                )
            })
            .collect()
    }

    fn with_kinds<U: Clone>(
        items: &[Bounds<U>],
        mode: KindMode,
        rng: &mut impl Rng,
    ) -> Vec<(Bounds<U>, PrimitiveKind, PrimitiveKind)>
    where
        U: Debug + Default + PartialEq,
    {
        items
            .iter()
            .map(|item| {
                let (kind, max_shared_kind) = mode.kinds(rng);
                (item.clone(), kind, max_shared_kind)
            })
            .collect()
    }

    fn assert_scenario_matches_oracle(name: &str, items: &[Bounds<f32>], seed: u64) {
        for mode in KIND_MODES {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let label = format!("{name} {mode:?} seed {seed}");
            let items = with_kinds(items, mode, &mut rng);
            assert_matches_oracle(&mut BoundsTree::<f32>::default(), &items, &label);

            let scaled_items: Vec<_> = items
                .iter()
                .map(|(item, kind, max_shared_kind)| {
                    (item.map(ScaledPixels), *kind, *max_shared_kind)
                })
                .collect();
            assert_matches_oracle(
                &mut BoundsTree::<ScaledPixels>::default(),
                &scaled_items,
                &format!("{label} scaled"),
            );
        }
    }

    #[test]
    fn test_oracle_overlap() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(1);
        assert_scenario_matches_oracle("overlap", &overlap_scenario(&mut rng), 1);
    }

    #[test]
    fn test_oracle_nested_shrinking() {
        assert_scenario_matches_oracle("nested shrinking", &nested_shrinking_scenario(), 2);
    }

    #[test]
    fn test_oracle_nested_growing() {
        assert_scenario_matches_oracle("nested growing", &nested_growing_scenario(), 3);
    }

    #[test]
    fn test_oracle_clusters() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(4);
        assert_scenario_matches_oracle("clusters", &clusters_scenario(&mut rng), 4);
    }

    #[test]
    fn test_oracle_grid() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(5);
        assert_scenario_matches_oracle("grid", &grid_scenario(&mut rng), 5);
    }

    #[test]
    fn test_oracle_stripes() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(6);
        assert_scenario_matches_oracle("stripes", &stripes_scenario(&mut rng), 6);
    }

    #[test]
    fn test_oracle_overlay() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(7);
        assert_scenario_matches_oracle("overlay", &overlay_scenario(&mut rng), 7);
    }

    #[test]
    fn test_oracle_random() {
        for seed in [8, 9, 10] {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            assert_scenario_matches_oracle("random", &random_scenario(&mut rng), seed);
        }
    }

    #[test]
    fn test_union_keeps_exact_corners() {
        let mut tree = BoundsTree::<f32>::default();
        let (quad, path, shadow) = (
            PrimitiveKind::Quad,
            PrimitiveKind::Path,
            PrimitiveKind::Shadow,
        );
        assert_eq!(tree.insert(bounds(0.0, 0.0, 0.1, 10.0), quad, shadow), 1);
        // Stored as an origin and size, the union of these would put its right
        // edge at `-1000.0 + 1000.1`, which rounds to just below `0.1`.
        assert_eq!(
            tree.insert(bounds(-1000.0, 0.0, 1.0, 10.0), path, shadow),
            1
        );
        assert_eq!(
            tree.insert(bounds(0.09999, 0.0, 1.0, 10.0), quad, shadow),
            2
        );
    }

    #[test]
    fn test_nested_bounds_are_subsumed() {
        let (quad, shadow) = (PrimitiveKind::Quad, PrimitiveKind::Shadow);

        let mut shrinking = BoundsTree::<f32>::default();
        for item in nested_shrinking_scenario() {
            assert_eq!(shrinking.insert(item, quad, quad), 1);
        }
        assert_eq!(stored_count(&shrinking), 1);
        assert_invariants(&shrinking, ITEM_COUNT, "shrinking");

        let mut growing = BoundsTree::<f32>::default();
        for item in nested_growing_scenario() {
            assert_eq!(growing.insert(item, quad, quad), 1);
        }
        assert_eq!(stored_count(&growing), 1);
        assert_invariants(&growing, ITEM_COUNT, "growing");

        // Each bounds needs a new ordering, so it can never be dropped in favor
        // of the bounds containing it, but it does replace the one it contains.
        let mut incrementing = BoundsTree::<f32>::default();
        for (item_index, item) in nested_growing_scenario().into_iter().enumerate() {
            assert_eq!(
                incrementing.insert(item, quad, shadow),
                item_index as u32 + 1
            );
        }
        assert_eq!(stored_count(&incrementing), 1);

        let mut disjoint = BoundsTree::<f32>::default();
        for item_index in 0..ITEM_COUNT {
            let item = bounds(item_index as f32 * 10.0, 0.0, 5.0, 5.0);
            assert_eq!(disjoint.insert(item, quad, shadow), 1);
        }
        assert_eq!(stored_count(&disjoint), ITEM_COUNT);
        assert_invariants(&disjoint, ITEM_COUNT, "disjoint");
        assert!(
            tree_height(&disjoint) <= 6,
            "height {}",
            tree_height(&disjoint)
        );
    }

    #[test]
    fn test_touching_edges() {
        let mut tree = BoundsTree::<f32>::default();
        let (quad, shadow) = (PrimitiveKind::Quad, PrimitiveKind::Shadow);
        assert_eq!(tree.insert(bounds(0.0, 0.0, 10.0, 10.0), quad, shadow), 1);
        assert_eq!(tree.insert(bounds(10.0, 0.0, 10.0, 10.0), quad, shadow), 1);
        assert_eq!(tree.insert(bounds(0.0, 10.0, 10.0, 10.0), quad, shadow), 1);
        assert_eq!(tree.insert(bounds(10.0, 10.0, 10.0, 10.0), quad, shadow), 1);
        assert_eq!(
            tree.insert(bounds(-10.0, -10.0, 10.0, 10.0), quad, shadow),
            1
        );
        assert_eq!(tree.insert(bounds(9.0, 9.0, 2.0, 2.0), quad, shadow), 2);

        let mut rng = rand::rngs::StdRng::seed_from_u64(11);
        let items: Vec<_> = (0..ITEM_COUNT)
            .map(|_| {
                let (kind, max_shared_kind) = KindMode::AllKinds.kinds(&mut rng);
                let item = bounds(
                    rng.random_range(0..40) as f32 * 5.0,
                    rng.random_range(0..40) as f32 * 5.0,
                    rng.random_range(0..6) as f32 * 5.0,
                    rng.random_range(0..6) as f32 * 5.0,
                );
                (item, kind, max_shared_kind)
            })
            .collect();
        assert_matches_oracle(&mut BoundsTree::default(), &items, "touching");
    }

    #[test]
    fn test_zero_and_negative_sizes() {
        let mut tree = BoundsTree::<f32>::default();
        let (quad, shadow) = (PrimitiveKind::Quad, PrimitiveKind::Shadow);
        assert_eq!(tree.insert(bounds(0.0, 0.0, 100.0, 100.0), quad, shadow), 1);
        assert_eq!(tree.insert(bounds(10.0, 10.0, 0.0, 0.0), quad, shadow), 2);
        assert_eq!(tree.insert(bounds(10.0, 10.0, 0.0, 10.0), quad, shadow), 2);
        assert_eq!(
            tree.insert(bounds(50.0, 50.0, -10.0, -10.0), quad, shadow),
            2
        );
        assert_eq!(tree.insert(bounds(50.0, 50.0, 10.0, 10.0), quad, shadow), 2);
        assert_eq!(tree.insert(bounds(200.0, 200.0, 0.0, 0.0), quad, shadow), 1);
        assert_eq!(tree.insert(bounds(200.0, 200.0, 0.0, 0.0), quad, shadow), 1);
        assert_eq!(
            tree.insert(bounds(300.0, 300.0, -10.0, -10.0), quad, shadow),
            1
        );
        assert_eq!(
            tree.insert(bounds(300.0, 300.0, -10.0, -10.0), quad, shadow),
            1
        );

        let mut rng = rand::rngs::StdRng::seed_from_u64(12);
        for mode in KIND_MODES {
            let items: Vec<_> = (0..ITEM_COUNT)
                .map(|_| {
                    let (kind, max_shared_kind) = mode.kinds(&mut rng);
                    let item = bounds(
                        rng.random_range(-300.0..300.0),
                        rng.random_range(-300.0..300.0),
                        rng.random_range(-80.0..80.0),
                        rng.random_range(-80.0..80.0),
                    );
                    (item, kind, max_shared_kind)
                })
                .collect();
            assert_matches_oracle(
                &mut BoundsTree::default(),
                &items,
                &format!("negative sizes {mode:?}"),
            );
        }
    }

    #[test]
    fn test_negative_coordinates() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(13);
        let items: Vec<_> = (0..ITEM_COUNT)
            .map(|_| {
                let (kind, max_shared_kind) = KindMode::AllKinds.kinds(&mut rng);
                let item = bounds(
                    rng.random_range(-5_000.0..-1_000.0),
                    rng.random_range(-5_000.0..-1_000.0),
                    log_uniform(&mut rng, 1.0, 1_500.0),
                    log_uniform(&mut rng, 1.0, 1_500.0),
                );
                (item, kind, max_shared_kind)
            })
            .collect();
        assert_matches_oracle(&mut BoundsTree::default(), &items, "negative coordinates");
    }

    #[test]
    fn test_i32() {
        for mode in KIND_MODES {
            let mut rng = rand::rngs::StdRng::seed_from_u64(14);
            let items: Vec<_> = (0..ITEM_COUNT)
                .map(|_| {
                    let (kind, max_shared_kind) = mode.kinds(&mut rng);
                    let item = bounds(
                        rng.random_range(-1_000..1_000),
                        rng.random_range(-1_000..1_000),
                        rng.random_range(-20..200),
                        rng.random_range(-20..200),
                    );
                    (item, kind, max_shared_kind)
                })
                .collect();
            assert_matches_oracle(
                &mut BoundsTree::<i32>::default(),
                &items,
                &format!("i32 {mode:?}"),
            );
        }
    }

    #[test]
    fn test_clear_reuse() {
        let mut tree = BoundsTree::<f32>::default();
        for seed in 15..18 {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let items = with_kinds(&random_scenario(&mut rng), KindMode::AllKinds, &mut rng);
            tree.clear();
            assert_invariants(&tree, 0, "cleared");
            assert_matches_oracle(&mut tree, &items, &format!("clear reuse seed {seed}"));
        }
    }

    #[test]
    fn test_f32_special_values() {
        let special_values = [
            f32::NAN,
            -f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::MAX,
            f32::MIN,
            0.0,
            -0.0,
        ];
        let mut rng = rand::rngs::StdRng::seed_from_u64(19);
        let value = |rng: &mut rand::rngs::StdRng, min: f32, max: f32| {
            if rng.random_ratio(1, 8) {
                special_values[rng.random_range(0..special_values.len())]
            } else {
                rng.random_range(min..max)
            }
        };
        for mode in KIND_MODES {
            let items: Vec<_> = (0..ITEM_COUNT)
                .map(|_| {
                    let (kind, max_shared_kind) = mode.kinds(&mut rng);
                    let item = bounds(
                        value(&mut rng, -200.0, 200.0),
                        value(&mut rng, -200.0, 200.0),
                        value(&mut rng, -20.0, 100.0),
                        value(&mut rng, -20.0, 100.0),
                    );
                    (item, kind, max_shared_kind)
                })
                .collect();
            assert_matches_oracle(
                &mut BoundsTree::<f32>::default(),
                &items,
                &format!("f32 special values {mode:?}"),
            );
        }
    }

    #[test]
    fn test_scaled_pixels_edge_values() {
        let edge_values = [
            -0.0,
            0.0,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::MAX,
            f32::MIN,
            f32::from_bits(0x7fc0_0001),
            f32::from_bits(0xffc0_0001),
        ];
        let mut rng = rand::rngs::StdRng::seed_from_u64(20);
        let value = |rng: &mut rand::rngs::StdRng, min: f32, max: f32| {
            ScaledPixels(if rng.random_ratio(1, 6) {
                edge_values[rng.random_range(0..edge_values.len())]
            } else {
                rng.random_range(min..max)
            })
        };
        for mode in KIND_MODES {
            let items: Vec<_> = (0..ITEM_COUNT)
                .map(|_| {
                    let (kind, max_shared_kind) = mode.kinds(&mut rng);
                    let item = bounds(
                        value(&mut rng, -200.0, 200.0),
                        value(&mut rng, -200.0, 200.0),
                        value(&mut rng, -20.0, 100.0),
                        value(&mut rng, -20.0, 100.0),
                    );
                    (item, kind, max_shared_kind)
                })
                .collect();
            assert_matches_oracle(
                &mut BoundsTree::<ScaledPixels>::default(),
                &items,
                &format!("scaled pixels edge values {mode:?}"),
            );
        }
    }
}
