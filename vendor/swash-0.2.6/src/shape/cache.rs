use super::at::FeatureStore;
use super::engine::EngineMetadata;
use super::internal::var::Fvar;
use crate::{charmap::CharmapProxy, metrics::MetricsProxy, FontRef};

use alloc::vec::Vec;

pub type Epoch = u64;

pub struct FontEntry {
    pub metrics: MetricsProxy,
    pub charmap: CharmapProxy,
    pub coord_count: u16,
    pub metadata: EngineMetadata,
}

impl FontEntry {
    pub fn new(font: &FontRef) -> Self {
        Self {
            metrics: MetricsProxy::from_font(font),
            charmap: CharmapProxy::from_font(font),
            coord_count: Fvar::from_font(font)
                .map(|fvar| fvar.axis_count())
                .unwrap_or(0),
            metadata: EngineMetadata::from_font(font),
        }
    }
}

pub struct FeatureEntry {
    pub epoch: Epoch,
    pub id: [u64; 2],
    pub coords: Vec<i16>,
    pub tags: [u32; 4],
    pub store: FeatureStore,
}

pub struct FeatureCache {
    entries: Vec<FeatureEntry>,
    epoch: Epoch,
    max_entries: usize,
}

pub enum FeatureCacheEntry<'a> {
    New(&'a mut FeatureStore),
    Present(&'a mut FeatureStore),
}

impl FeatureCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Default::default(),
            epoch: 0,
            max_entries,
        }
    }

    pub fn entry<'a>(
        &'a mut self,
        id: [u64; 2],
        coords: &[i16],
        has_feature_vars: bool,
        tags: &[u32; 4],
    ) -> FeatureCacheEntry<'a> {
        match self.find_entry(id, coords, has_feature_vars, tags) {
            (true, index) => {
                let entry = &mut self.entries[index];
                entry.epoch = self.epoch;
                FeatureCacheEntry::Present(&mut entry.store)
            }
            (false, index) => {
                self.epoch += 1;
                let entry = &mut self.entries[index];
                entry.epoch = self.epoch;
                FeatureCacheEntry::New(&mut entry.store)
            }
        }
    }

    fn find_entry(
        &mut self,
        id: [u64; 2],
        coords: &[i16],
        has_feature_vars: bool,
        tags: &[u32; 4],
    ) -> (bool, usize) {
        let epoch = self.epoch;
        let mut lowest_serial = epoch;
        let mut lowest_index = 0;
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.id == id && &entry.tags == tags {
                if has_feature_vars && coords != &entry.coords[..] {
                    continue;
                }
                return (true, i);
            }
            if entry.epoch < lowest_serial {
                lowest_serial = entry.epoch;
                lowest_index = i;
            }
        }
        if self.entries.len() < self.max_entries {
            lowest_index = self.entries.len();
            self.entries.push(FeatureEntry {
                epoch,
                id,
                coords: Vec::from(coords),
                store: FeatureStore::default(),
                tags: *tags,
            });
        } else {
            let entry = &mut self.entries[lowest_index];
            entry.epoch = epoch;
            entry.id = id;
            entry.coords.clear();
            entry.coords.extend_from_slice(coords);
            entry.store.clear();
            entry.tags = *tags;
        }
        (false, lowest_index)
    }
}
