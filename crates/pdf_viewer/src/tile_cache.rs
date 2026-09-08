use std::{
    collections::HashMap,
    sync::Arc,
    time::Instant,
};

use gpui::Image;

/// Identifies one renderable tile: a page, at a specific zoom bucket and rotation,
/// at a specific tile grid position (x, y) within that page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileKey {
    pub page_index: usize,
    /// Zoom bucketed to 10% increments to prevent churn during fluid animations.
    /// E.g., 1.0 -> 10, 1.25 -> 13, 1.5 -> 15.
    pub zoom_bucket: u32,
    /// Rotation in degrees (0, 90, 180, 270)
    pub rotation: u16,
    /// Tile column index (0..tiles_wide)
    pub tile_x: u32,
    /// Tile row index (0..tiles_high)
    pub tile_y: u32,
}

impl TileKey {
    pub const TILE_SIZE: u32 = 4096;

    pub fn new(page_index: usize, zoom: f32, rotation: u16, tile_x: u32, tile_y: u32) -> Self {
        Self {
            page_index,
            zoom_bucket: (zoom * 10.0).round() as u32,
            rotation,
            tile_x,
            tile_y,
        }
    }

    /// Zoom scale factor reconstructed from bucket
    pub fn zoom_scale(&self) -> f32 {
        self.zoom_bucket as f32 / 10.0
    }
}

struct CacheEntry {
    image: Arc<Image>,
    byte_size: usize,
    last_accessed: Instant,
}

/// LRU cache for rendered PDF tiles and page overviews, bounded by total memory consumption.
pub struct TileCache {
    entries: HashMap<TileKey, CacheEntry>,
    page_overviews: HashMap<(usize, u16), Arc<Image>>,
    total_bytes: usize,
    max_bytes: usize,
}

impl Default for TileCache {
    fn default() -> Self {
        // Default 256 MB memory limit
        Self::new(256 * 1024 * 1024)
    }
}

impl TileCache {
    pub fn new(max_bytes: usize) -> Self {
        Self {
            entries: HashMap::new(),
            page_overviews: HashMap::new(),
            total_bytes: 0,
            max_bytes,
        }
    }

    /// Returns a tile if cached at the exact key, updating its LRU timestamp.
    pub fn get(&mut self, key: &TileKey) -> Option<Arc<Image>> {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.last_accessed = Instant::now();
            Some(entry.image.clone())
        } else {
            None
        }
    }

    /// Returns the low-res overview image for a whole page at a specific rotation if cached.
    pub fn get_page_overview(&self, page_index: usize, rotation: u16) -> Option<Arc<Image>> {
        self.page_overviews.get(&(page_index, rotation)).cloned()
    }

    /// Stores a low-res overview image for a whole page at a specific rotation.
    pub fn insert_page_overview(&mut self, page_index: usize, rotation: u16, image: Arc<Image>) {
        self.page_overviews.insert((page_index, rotation), image);
    }

    /// Inserts a newly rendered tile into the cache.
    pub fn insert(&mut self, key: TileKey, image: Arc<Image>) {
        // Estimate memory size: file bytes + uncompressed texture overhead (approx 1-2 MB per standard page)
        let byte_size = (image.bytes.len() * 4).max(1024 * 1024);

        if let Some(old) = self.entries.remove(&key) {
            self.total_bytes = self.total_bytes.saturating_sub(old.byte_size);
        }

        self.entries.insert(
            key,
            CacheEntry {
                image,
                byte_size,
                last_accessed: Instant::now(),
            },
        );
        self.total_bytes += byte_size;

        self.evict_if_needed();
    }

    /// Evicts oldest tiles until memory usage is under the limit.
    fn evict_if_needed(&mut self) {
        while self.total_bytes > self.max_bytes && !self.entries.is_empty() {
            let oldest_key = self
                .entries
                .iter()
                .min_by_key(|(_, entry)| entry.last_accessed)
                .map(|(key, _)| *key);

            if let Some(key) = oldest_key {
                if let Some(entry) = self.entries.remove(&key) {
                    self.total_bytes = self.total_bytes.saturating_sub(entry.byte_size);
                }
            } else {
                break;
            }
        }
    }

    /// Prunes tiles for pages that are far outside the current viewport.
    pub fn evict_pages_outside(&mut self, min_page: usize, max_page: usize) {
        let keys_to_remove: Vec<TileKey> = self
            .entries
            .keys()
            .filter(|k| k.page_index < min_page || k.page_index > max_page)
            .copied()
            .collect();

        for key in keys_to_remove {
            if let Some(entry) = self.entries.remove(&key) {
                self.total_bytes = self.total_bytes.saturating_sub(entry.byte_size);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.page_overviews.clear();
        self.total_bytes = 0;
    }
}
